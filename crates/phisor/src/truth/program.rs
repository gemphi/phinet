//! The program path: a claim that offers a program is answered by running
//! it (R8.6, R8.13).
//!
//! Execution is the judge. Where the classifier core reads a claim against
//! the triples the World already holds, a claim that comes with a program
//! carries its own support: phiclm compiles the source and runs it under two
//! declared caps — 1,000,000 machine steps and 10 seconds of wall-clock time
//! — and every field of what happened is recorded in a [`VerdictRecord`]:
//! whether it compiled, its error, its output, its result and its step count.
//!
//! A program that ran to its end under both caps supports the claim, so the
//! claim stands [`Standing::Consistent`]. A program that did not compile,
//! that faulted, or that reached either cap supports nothing, so the claim
//! stands [`Standing::Unsupported`] — with the error kept and the cap named,
//! never silently dropped (R8.13). The caps are what make this safe to run
//! on generated source: a judged program may not stop, and may not be a
//! program at all.

use super::{Cap, Detector, VerdictRecord};
use phiclm::judge::{judge_under, Caps};
use phiecs::quest::{Claim, Standing};
use phiecs::world::World;

impl Detector {
    /// The two caps the detector holds a program to (R8.6). [`Detector::new`]
    /// sets them to phiclm's own declared pair; a caller may lower them, and
    /// the lowered pair is what is enforced and what the record reports.
    pub fn caps(&self) -> Caps {
        Caps { steps: self.step_cap, seconds: Some(self.seconds_cap) }
    }

    /// Judges one offered program under both caps, and says what the claim
    /// it was offered for stands as.
    ///
    /// The source is run with no input and no files: a claim's program
    /// answers for itself or not at all. The judge returns a verdict for any
    /// source text — truncated, unbalanced or random bytes included — so
    /// this never panics on what an oracle produced.
    ///
    /// Recording is the caller's: the pair comes back whole, and
    /// [`Detector::judge_claim`] is what files a [`super::Reading`] from it,
    /// since a `Reading` names a claim and a bare source is not one.
    pub fn judge_program(&mut self, source: &str) -> (Standing, VerdictRecord) {
        let verdict = judge_under(source, "", &[], self.caps());
        // `supported` is false when the source did not compile, when it
        // faulted, or when it reached a cap — the three cases of R8.13.
        let standing =
            if verdict.supported() { Standing::Consistent } else { Standing::Unsupported };
        (standing, VerdictRecord::from(&verdict))
    }

    /// Classifies a claim that offers a program, and records the reading.
    ///
    /// The precedence of R8.1 is kept: the readability test and then the
    /// degenerate and contradicting tests run first, and the program is not
    /// run at all when one of them holds — a self-edge is degenerate however
    /// well its program runs. Only where the held triples leave the claim
    /// unsupported does the program get to answer, and then it decides
    /// between unsupported and consistent.
    ///
    /// The [`super::Reading`] filed by [`Detector::classify`] is amended
    /// rather than doubled, so one claim leaves one reading, now carrying
    /// the verdict the classifier core leaves `None`.
    pub fn judge_claim(
        &mut self,
        about: &str,
        claim: &Claim,
        world: &World,
        source: &str,
    ) -> Standing {
        let standing = self.classify(about, claim, world);
        if standing != Standing::Unsupported {
            return standing;
        }
        let (judged, record) = self.judge_program(source);
        if let Some(reading) = self.seen.last_mut() {
            reading.standing = judged.clone();
            reading.verdict = Some(record);
        }
        judged
    }
}

/// Why a judged program supported nothing, naming the error and the cap it
/// reached (R8.13); `None` where the program ran to its end under both caps
/// and so supported the claim.
pub fn reason(record: &VerdictRecord) -> Option<String> {
    let error = || record.error.clone().unwrap_or_else(|| "no error given".to_string());
    if !record.compiled {
        return Some(format!("did not compile: {}", error()));
    }
    match record.capped {
        Some(cap) => {
            Some(format!("reached the {} cap after {} steps: {}", named(cap), record.steps, error()))
        }
        None => record
            .error
            .as_ref()
            .map(|e| format!("faulted after {} steps: {e}", record.steps)),
    }
}

/// The cap as the record names it.
fn named(cap: Cap) -> &'static str {
    match cap {
        Cap::Steps => "step",
        Cap::Seconds => "time",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::truth::Declared;
    use phiclm::judge::{SECONDS_CAP, STEP_CAP};

    /// The `sum` Operator lowered to a loop: a vector filled, then reduced
    /// by addition over an `UNTIL`, the sum written and returned.
    /// `0+1+4+9+16+25+36+49 = 140`.
    const SUM: &str = "LET start() = VALOF
         $( LET v = VEC 7
            AND s = 0
            AND j = 0
            FOR i = 0 TO 7 DO v!i := i * i
            UNTIL j > 7 DO
            $( s := s + v!j
               j := j + 1
            $)
            writef(\"sum = %n*n\", s)
            RESULTIS s
         $)";

    /// The same Operator mis-lowered: the index never advances, so the
    /// reduction never ends and the run is stopped by a cap.
    const ENDLESS: &str = "LET start() = VALOF
         $( LET s = 0
            AND j = 0
            UNTIL j > 7 DO s := s + j
            RESULTIS s
         $)";

    fn detector() -> Detector {
        Detector::new(Declared::none())
    }

    fn claim(kind: &str, other: &str, weight: f64) -> Claim {
        Claim { kind: kind.to_string(), other: other.to_string(), weight }
    }

    #[test]
    fn the_caps_are_a_million_steps_and_ten_seconds() {
        let d = detector();
        assert_eq!((d.step_cap, d.seconds_cap), (1_000_000, 10));
        assert_eq!((d.step_cap, d.seconds_cap), (STEP_CAP, SECONDS_CAP));
        assert_eq!(d.caps(), Caps::declared());
    }

    #[test]
    fn an_operator_lowered_to_a_loop_runs_and_every_field_is_recorded() {
        let (standing, r) = detector().judge_program(SUM);
        assert_eq!(standing, Standing::Consistent);
        assert_eq!((r.compiled, r.error.as_deref(), r.result), (true, None, 140));
        assert_eq!(r.out, "sum = 140\n");
        assert!(r.steps > 0 && r.steps < STEP_CAP, "{} steps", r.steps);
        assert_eq!(r.capped, None);
        assert_eq!(reason(&r), None);
    }

    #[test]
    fn a_lowering_that_never_ends_reaches_the_step_cap_and_names_it() {
        let mut d = detector();
        // The declared million steps are reached in the same way, in rather
        // more time than a test should take.
        d.step_cap = 20_000;
        let (standing, r) = d.judge_program(ENDLESS);
        assert_eq!(standing, Standing::Unsupported);
        assert_eq!((r.compiled, r.capped, r.steps), (true, Some(Cap::Steps), 20_000));
        assert!(r.error.is_some() && r.out.is_empty() && r.result == 0, "{r:?}");
        let why = reason(&r).expect("a capped run supported nothing");
        assert!(why.starts_with("reached the step cap after 20000 steps"), "{why}");
    }

    #[test]
    fn a_lowering_that_never_ends_reaches_the_time_cap_and_names_it() {
        let mut d = detector();
        // A zero-second cap has passed by the first clock reading, so the
        // wall-clock path is taken in milliseconds rather than in ten
        // seconds. The steps are left uncapped so the clock is what stops it.
        d.step_cap = usize::MAX;
        d.seconds_cap = 0;
        let (standing, r) = d.judge_program(ENDLESS);
        assert_eq!(standing, Standing::Unsupported);
        assert_eq!((r.compiled, r.capped), (true, Some(Cap::Seconds)));
        assert!(r.steps < STEP_CAP, "stopped by the clock, not the steps: {}", r.steps);
        let why = reason(&r).expect("a capped run supported nothing");
        assert!(why.contains("reached the time cap"), "{why}");
    }

    #[test]
    fn a_program_that_does_not_compile_supports_nothing() {
        let (standing, r) = detector().judge_program("LET start() = VALOF $( RESULTIS");
        assert_eq!(standing, Standing::Unsupported);
        assert_eq!((r.compiled, r.result, r.steps, r.capped), (false, 0, 0, None));
        assert!(r.error.is_some() && r.out.is_empty(), "{r:?}");
        assert!(reason(&r).unwrap().starts_with("did not compile:"));
    }

    #[test]
    fn a_fault_is_not_a_cap_and_still_supports_nothing() {
        let (standing, r) = detector().judge_program("LET start() = 1 / 0");
        assert_eq!(standing, Standing::Unsupported);
        assert_eq!((r.compiled, r.capped), (true, None));
        assert_eq!(r.error.as_deref(), Some("division by zero"));
        assert!(reason(&r).unwrap().starts_with("faulted after"), "{:?}", reason(&r));
    }

    #[test]
    fn a_claim_whose_program_runs_stands_consistent_and_keeps_the_verdict() {
        let mut d = detector();
        let standing = d.judge_claim("sum", &claim("returns", "140", 0.9), &World::new(), SUM);
        assert_eq!(standing, Standing::Consistent);
        // One claim, one reading, now carrying the verdict.
        assert_eq!(d.seen.len(), 1);
        let reading = d.seen.last().expect("the claim was read");
        assert_eq!(reading.standing, Standing::Consistent);
        let r = reading.verdict.as_ref().expect("the program was judged");
        assert_eq!((r.compiled, r.result, r.capped), (true, 140, None));
    }

    #[test]
    fn an_earlier_class_holds_before_any_program_runs() {
        let mut d = detector();
        // A self-edge is degenerate however well its program runs (R8.1).
        let standing = d.judge_claim("sum", &claim("is-a", "sum", 0.9), &World::new(), SUM);
        assert!(matches!(standing, Standing::Degenerate { .. }), "{standing:?}");
        assert!(d.seen.last().expect("the claim was read").verdict.is_none());
    }
}
