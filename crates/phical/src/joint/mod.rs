//! Training the operator and the table of functions together.
//!
//! In [`crate::operator`] the table is constant: the operator learns to map a
//! fixed input to a fixed target. But `cosine` back-propagates into both of
//! its arguments and `gather` scatter-adds into a table, so nothing stops the
//! table from being a parameter. Then the items themselves move under the
//! same objective the operator does.
//!
//! The input is built *on the tape* from the same table: a context is a
//! weighted sum of gathered rows, normalised. So an item receives gradient
//! both as a target (be findable after this context) and as context (make
//! what follows findable).
//!
//! With many more parameters in the table than in the operator, the table
//! memorises unless held: `tether` pulls every row back toward its initial
//! value after each step — a proximal term, so the operator can reshape an
//! item only as far as the objective keeps paying for it.

use crate::operator::Fno;
use crate::tensor::{Adam, Tensor, Var};
use crate::{c64, mix64, Function};

/// One example by index: `(context item, weight)` pairs and the target item.
#[derive(Debug, Clone)]
pub struct IndexExample {
    pub ctx: Vec<(usize, f64)>,
    pub target: usize,
}

/// The operator plus a trainable table of functions on the same tape.
pub struct Joint {
    pub fno: Fno,
    pub table: Var,
    /// Index of the all-zero padding row, used for contexts shorter than the
    /// window. Re-zeroed after every step.
    pad: usize,
    window: usize,
    init: Tensor,
    pub tether: f64,
}

impl Joint {
    /// `window` is the most context items an example carries; the `j`-th most
    /// recent item is weighted by its stored weight (see [`IndexExample`]).
    pub fn new(mut fno: Fno, table: &[Function], window: usize) -> Self {
        let s = fno.config.space;
        let v = table.len();
        let mut t = Tensor::zeros([v + 1, 1, s]);
        for (i, f) in table.iter().enumerate() {
            assert_eq!(f.len(), s);
            t.data[i * s..(i + 1) * s].copy_from_slice(f);
        }
        let init = t.clone();
        let table = fno.attach_param(t);
        Self { fno, table, pad: v, window, init, tether: 0.0 }
    }

    /// Sets the proximal pull toward the initial functions (0 = free).
    pub fn with_tether(mut self, tether: f64) -> Self {
        self.tether = tether;
        self
    }

    fn settle(&mut self) {
        let mut t = self.fno.tape.value(self.table).clone();
        let s = self.fno.config.space;
        for k in 0..s {
            t.data[self.pad * s + k] = c64::new(0.0, 0.0);
        }
        if self.tether > 0.0 {
            let l = self.tether;
            for (z, z0) in t.data.iter_mut().zip(&self.init.data) {
                *z += (z0 - *z) * l;
            }
        }
        self.fno.tape.set_value(self.table, t);
    }

    /// Builds the batch's context functions on the tape from the table.
    fn context_on_tape(&mut self, batch: &[&IndexExample]) -> Var {
        let mut acc: Option<Var> = None;
        for j in 0..self.window {
            let idx: Vec<usize> = batch
                .iter()
                .map(|ex| {
                    let n = ex.ctx.len();
                    match j < n {
                        true => ex.ctx[n - 1 - j].0,
                        false => self.pad,
                    }
                })
                .collect();
            // All examples share the weight schedule of the first that has
            // this position; the padding row is zero so weight is moot there.
            let weight = batch.iter().find_map(|ex| {
                let n = ex.ctx.len();
                match j < n {
                    true => Some(ex.ctx[n - 1 - j].1),
                    false => None,
                }
            });
            let g = self.fno.tape.gather(self.table, &idx);
            let s = self.fno.tape.scale(g, weight.unwrap_or(0.0));
            acc = Some(match acc {
                None => s,
                Some(a) => self.fno.tape.add(a, s),
            });
        }
        let sum = acc.expect("window > 0");
        self.fno.tape.unit(sum)
    }

    /// One epoch. Returns the mean loss.
    pub fn train_epoch(
        &mut self,
        examples: &[IndexExample],
        pool: &[usize],
        batch: usize,
        adam_op: &mut Adam,
        adam_table: &mut Adam,
        seed: u64,
    ) -> f64 {
        let params = self.fno.params();
        let table = self.table;
        let mut total = 0.0;
        let mut batches = 0usize;
        let mut order: Vec<usize> = (0..examples.len()).collect();
        order.sort_by_key(|i| mix64(*i as u64 ^ seed));
        for chunk in order.chunks(batch) {
            self.fno.reset();
            let exs: Vec<&IndexExample> = chunk.iter().map(|&i| &examples[i]).collect();
            let x = self.context_on_tape(&exs);
            let tgt_idx: Vec<usize> = exs.iter().map(|e| e.target).collect();
            let dec_idx: Vec<usize> = chunk
                .iter()
                .enumerate()
                .map(|(j, &i)| Fno::decoy(pool, examples[i].target, mix64(seed ^ (i as u64) << 20 ^ j as u64)))
                .collect();
            let t = self.fno.tape.gather(table, &tgt_idx);
            let d = self.fno.tape.gather(table, &dec_idx);
            let y = self.fno.forward(x);
            let sp = self.fno.tape.cosine(y, t);
            let sn = self.fno.tape.cosine(y, d);
            let loss = self.fno.tape.hinge(sp, sn, self.fno.config.margin);
            total += self.fno.tape.value(loss).get(0, 0, 0).re;
            batches += 1;
            self.fno.tape.backward(loss);
            adam_op.step(&mut self.fno.tape, &params);
            adam_table.step(&mut self.fno.tape, &[table]);
            self.fno.mask_modes();
            self.settle();
        }
        self.fno.reset();
        total / batches.max(1) as f64
    }

    /// The learned functions, in table order.
    pub fn functions(&self) -> Vec<Function> {
        let t = self.fno.tape.value(self.table);
        let s = self.fno.config.space;
        (0..self.pad).map(|i| t.data[i * s..(i + 1) * s].to_vec()).collect()
    }
}

#[cfg(test)]
mod tests;
