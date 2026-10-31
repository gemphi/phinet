//! The judge's verdict as the detector records it (R8.6, R8.13). The program
//! path (task 9.3) is what produces a [`VerdictRecord`]; it is named beside
//! the classifier so the `Reading` shape is whole, and the classifier core
//! leaves every `Reading::verdict` `None`.

use phiclm::judge::Verdict;

/// The judge's verdict fields, recorded for a claim that offered a program.
#[derive(Clone, Debug)]
pub struct VerdictRecord {
    pub compiled: bool,
    pub error: Option<String>,
    pub out: String,
    pub result: i64,
    pub steps: usize,
    pub capped: Option<Cap>,
}

/// Which cap a judged program reached, when it reached one (R8.13).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap {
    Steps,
    Seconds,
}

impl From<&Verdict> for VerdictRecord {
    fn from(v: &Verdict) -> Self {
        Self {
            compiled: v.compiled,
            error: v.error.clone(),
            out: v.out.clone(),
            result: v.result,
            steps: v.steps,
            capped: v.capped.map(|c| match c {
                phiclm::machine::Capped::Steps => Cap::Steps,
                phiclm::machine::Capped::Seconds => Cap::Seconds,
            }),
        }
    }
}
