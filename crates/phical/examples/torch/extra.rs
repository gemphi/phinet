//! The rest of the reference dump: hrr, the hexagram gates, ranges, and the whole operator with one Adam step.

use phistd::hexagram;
use phistd::hrr;
use phistd::{c64, cosine, Adam, Tensor};
use phical::{Tie, zeta_weights, Fno, FnoConfig};
use serde_json::{json, Map, Value};

fn cx(f: &[c64]) -> Value {
    json!(f.iter().map(|z| [z.re, z.im]).collect::<Vec<_>>())
}
fn tensor(t: &Tensor) -> Value {
    json!({ "shape": t.shape, "data": cx(&t.data) })
}

pub fn spectral_and_misc(out: &mut Map<String, Value>, fs: &[Vec<c64>], space: usize) {
    // ── hrr ──
    let role = hrr::role(space, 3);
    out.insert("role".into(), cx(&role));
    let bound = hrr::bind(&fs[0], &role);
    out.insert("bind".into(), cx(&bound));
    out.insert("unbind".into(), cx(&hrr::unbind(&bound, &role)));
    out.insert("compose".into(), cx(&hrr::compose(&[bound.clone(), hrr::bind(&fs[1], &hrr::role(space, 4))])));

    // ── hexagram ──
    let hx = 0b10_1010u8;
    out.insert("hexagram_id".into(), json!(hx));
    out.insert("hexagram_gate".into(), json!(hexagram::gate(hx, space)));
    out.insert("hexagram_apply".into(), cx(&hexagram::apply(&fs[0], hx)));
    out.insert("hexagram_of_x0".into(), json!(hexagram::hexagram_of(&fs[0])));
    out.insert("hexagram_lines_x0".into(), json!(hexagram::lines(&fs[0])));
    out.insert("hexagram_novelty".into(), json!(hexagram::novelty(&fs[0], hx)));
    out.insert("changing_lines_63_to_hx".into(), json!(hexagram::changing_lines(63, hx)));

    // ── misc ──
    out.insert("zeta_weights_8_s2".into(), json!(zeta_weights(8, 2.0)));
    out.insert("cosine_x0_x1".into(), json!(cosine(&fs[0], &fs[1])));

}

pub fn whole_operator(out: &mut Map<String, Value>, fs: &[Vec<c64>], x: &Tensor, table: &Tensor, space: usize, width: usize) {
    // ── whole operator: forward, and one Adam step on a hinge loss ──
    let cfg = FnoConfig { space, width, layers: 2, modes: 4, margin: 0.2, lr: 1e-2, tie: Tie::Circle };
    let mut fno = Fno::new(cfg, 5);
    let names = ["lift", "lift_b", "l0_r", "l0_w", "l0_b", "l1_r", "l1_w", "l1_b", "proj"];
    let params = fno.params();
    let mut pj = serde_json::Map::new();
    for (n, p) in names.iter().zip(&params) {
        pj.insert(n.to_string(), tensor(fno.tape.value(*p)));
    }
    out.insert("fno_params".into(), Value::Object(pj));
    out.insert("fno_config".into(), json!({"space": space, "width": width, "layers": 2, "modes": 4, "margin": 0.2, "lr": 1e-2}));
    out.insert("fno_parameter_count".into(), json!(fno.parameter_count()));
    let ys = fno.apply(&fs);
    out.insert("fno_apply".into(), json!(ys.iter().map(|f| cx(f)).collect::<Vec<_>>()));
    // one training step: hinge(cos(pred, table[1]), cos(pred, table[0])) on batch row 0 and (3, 2) on row 1
    fno.reset();
    let xin = fno.tape.constant(x.clone());
    let pred = fno.forward(xin);
    let tv = fno.tape.constant(table.clone());
    let pos = fno.tape.gather(tv, &[1, 3]);
    let neg = fno.tape.gather(tv, &[0, 2]);
    let cp = fno.tape.cosine(pred, pos);
    let cn = fno.tape.cosine(pred, neg);
    let l = fno.tape.hinge(cp, cn, 0.2);
    out.insert("fno_step_loss".into(), json!(fno.tape.value(l).data[0].re));
    fno.tape.backward(l);
    let mut gj = serde_json::Map::new();
    for (n, p) in names.iter().zip(&params) {
        gj.insert(n.to_string(), tensor(fno.tape.grad(*p).unwrap()));
    }
    out.insert("fno_step_grads".into(), Value::Object(gj));
    let mut adam = Adam::new(cfg.lr);
    adam.step(&mut fno.tape, &params);
    fno.mask_modes();
    let mut aj = serde_json::Map::new();
    for (n, p) in names.iter().zip(&params) {
        aj.insert(n.to_string(), tensor(fno.tape.value(*p)));
    }
    out.insert("fno_after_adam".into(), Value::Object(aj));
}
