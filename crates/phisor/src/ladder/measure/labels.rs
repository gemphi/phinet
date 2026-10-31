//! Held-out function and terminator readings. Labels enter only through
//! these scorers, after Kinds have already been learned.

use super::{one_decimal, MeasureError, Measured, Unit, FUNCTION_BAR, GAP_BAR};
use crate::guide::holdout::{Breach, HoldoutGuard, Phase};
use crate::guide::{Function, Mark, Part, Split};
use crate::ladder::{shaped, KindReport, Ladder};
use phistd::stats::{f1, purity};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionReport {
    pub purity: Measured,
    pub scored: usize,
    pub unassigned: usize,
    /// Kinds × (explain, instruct, answer).
    pub table: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GapReport {
    pub learned: f64,
    pub hash_null: f64,
    pub kinds: usize,
    pub scored: usize,
    pub difference: Measured,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BestKindF1 {
    pub kind: usize,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TerminatorReport {
    pub purity: Measured,
    pub majority_baseline: f64,
    pub question_f1: Option<BestKindF1>,
    pub scored: usize,
    pub unassigned: usize,
    /// Kinds × (statement, question, exclamation).
    pub table: Vec<Vec<f64>>,
}

pub fn function_purity(
    report: &KindReport,
    split: &Split,
    guard: &mut HoldoutGuard,
) -> Result<FunctionReport, Breach> {
    let test = split.of(Part::Test);
    let (indices, unassigned) = assigned(report, split, &test);
    let table = function_table(report, split, guard, &indices)?;
    let value = one_decimal(100.0 * purity(&table));
    Ok(FunctionReport {
        purity: Measured::at_least(
            "ladder.function_purity.learned",
            value,
            FUNCTION_BAR,
            Unit::Percentage,
        ),
        scored: indices.len(),
        unassigned,
        table,
    })
}

/// Learned purity minus Hash_Null purity on the intersection of their
/// assigned test sentences, and only when their occupied Kind counts match.
pub fn function_gap(
    learned: &KindReport,
    hash: &KindReport,
    split: &Split,
    guard: &mut HoldoutGuard,
) -> Result<GapReport, MeasureError> {
    if learned.level != hash.level {
        return Err(MeasureError::ModeMismatch {
            learned: learned.level.bits() as u8,
            hash_null: hash.level.bits() as u8,
        });
    }
    if learned.occupied != hash.occupied {
        return Err(MeasureError::UnequalKindCount {
            learned: learned.occupied,
            hash_null: hash.occupied,
        });
    }
    let indices: Vec<usize> = split
        .of(Part::Test)
        .into_iter()
        .filter(|&i| {
            shaped(&split.sentences[i]) && learned.of(i).is_some() && hash.of(i).is_some()
        })
        .collect();
    let learned_value = 100.0 * purity(&function_table(learned, split, guard, &indices)?);
    let hash_value = 100.0 * purity(&function_table(hash, split, guard, &indices)?);
    let difference = one_decimal(learned_value - hash_value);
    Ok(GapReport {
        learned: one_decimal(learned_value),
        hash_null: one_decimal(hash_value),
        kinds: learned.occupied,
        scored: indices.len(),
        difference: Measured::at_least(
            "ladder.function_purity.gap",
            difference,
            GAP_BAR,
            Unit::PercentagePoints,
        ),
    })
}

pub fn terminator_purity(
    report: &KindReport,
    split: &Split,
    guard: &mut HoldoutGuard,
) -> Result<TerminatorReport, Breach> {
    let test = split.of(Part::Test);
    let (indices, unassigned) = assigned(report, split, &test);
    let mut table = vec![vec![0.0; 3]; report.occupied];
    let mut scored = 0usize;
    for i in indices {
        let mark = split.sentences[i].mark;
        let Some(label) = terminator(mark) else { continue };
        guard.read_label(Phase::ScoreNumber, TERMINATORS[label])?;
        table[report.of(i).unwrap()][label] += 1.0;
        scored += 1;
    }
    let sums: Vec<f64> = (0..3).map(|j| table.iter().map(|row| row[j]).sum()).collect();
    let majority = one_decimal(100.0 * purity(&[sums]));
    let value = one_decimal(100.0 * purity(&table));
    Ok(TerminatorReport {
        purity: Measured::at_least(
            "ladder.terminator_purity.learned",
            value,
            majority,
            Unit::Percentage,
        ),
        majority_baseline: majority,
        question_f1: best_question(&table),
        scored,
        unassigned,
        table,
    })
}

const TERMINATORS: [&str; 3] = ["statement", "question", "exclamation"];

fn terminator(mark: Mark) -> Option<usize> {
    match mark {
        Mark::Stop => Some(0),
        Mark::Question => Some(1),
        Mark::Exclamation => Some(2),
        Mark::Line => None,
    }
}

fn assigned(report: &KindReport, split: &Split, indices: &[usize]) -> (Vec<usize>, usize) {
    let shaped: Vec<usize> = indices
        .iter()
        .copied()
        .filter(|&i| shaped(&split.sentences[i]))
        .collect();
    let assigned: Vec<usize> = shaped
        .iter()
        .copied()
        .filter(|&i| report.of(i).is_some())
        .collect();
    let unassigned = shaped.len() - assigned.len();
    (assigned, unassigned)
}

fn function_table(
    report: &KindReport,
    split: &Split,
    guard: &mut HoldoutGuard,
    indices: &[usize],
) -> Result<Vec<Vec<f64>>, Breach> {
    let mut table = vec![vec![0.0; Function::ALL.len()]; report.occupied];
    for &i in indices {
        let label = split.sentences[i].function;
        guard.read_label(Phase::ScoreNumber, label.name())?;
        let column = Function::ALL.iter().position(|&f| f == label).unwrap();
        table[report.of(i).unwrap()][column] += 1.0;
    }
    Ok(table)
}

fn best_question(table: &[Vec<f64>]) -> Option<BestKindF1> {
    let expected = table.iter().map(|row| row[1] as usize).sum::<usize>();
    if expected == 0 {
        return None;
    }
    table
        .iter()
        .enumerate()
        .map(|(kind, row)| BestKindF1 {
            kind,
            value: (f1(row[1], row.iter().sum::<f64>() as usize, expected) * 1000.0).round()
                / 1000.0,
        })
        .max_by(|a, b| a.value.total_cmp(&b.value).then_with(|| b.kind.cmp(&a.kind)))
}

impl Ladder<'_> {
    pub fn function_purity(
        &self,
        report: &KindReport,
        guard: &mut HoldoutGuard,
    ) -> Result<FunctionReport, Breach> {
        function_purity(report, self.split(), guard)
    }

    pub fn function_gap(
        &self,
        learned: &KindReport,
        hash: &KindReport,
        guard: &mut HoldoutGuard,
    ) -> Result<GapReport, MeasureError> {
        function_gap(learned, hash, self.split(), guard)
    }

    pub fn terminator_purity(
        &self,
        report: &KindReport,
        guard: &mut HoldoutGuard,
    ) -> Result<TerminatorReport, Breach> {
        terminator_purity(report, self.split(), guard)
    }
}
