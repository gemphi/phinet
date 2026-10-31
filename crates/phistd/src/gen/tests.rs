use super::*;
use std::cell::RefCell;

const SEED: u64 = 0x005E_ED0E;

/// A value whose draw count depends on what it drew, so a case that ran
/// before could only stay out of a later case's way by construction.
#[derive(Debug, PartialEq)]
struct Variable(Vec<u64>);

impl Gen for Variable {
    fn draw(g: &mut Seeded) -> Self {
        let n = g.below(8) as usize + 1;
        Variable((0..n).map(|_| g.u64()).collect())
    }
}

#[test]
fn replay_reproduces_a_reported_failure_exactly() {
    let failure = for_all::<Variable>(SEED, CASES, |x| x.0.len() < 6).unwrap_err();
    assert_eq!(replay::<Variable>(failure.seed, failure.case), failure.value);
    assert_eq!(failure.seed, SEED);
    assert!(failure.value.0.len() >= 6, "the reported value really fails");
    assert!(failure.to_string().contains("replay"), "the report says how to replay");
}

#[test]
fn a_passing_property_reports_no_failure() {
    assert!(for_all::<Variable>(SEED, CASES, |x| !x.0.is_empty()).is_ok());
}

#[test]
fn two_cases_of_one_run_are_independent_of_each_other() {
    // Case 412 reconstructs without running cases 0 to 411.
    let alone = replay::<Variable>(SEED, 412);
    let seen: RefCell<Vec<Variable>> = RefCell::new(Vec::new());
    for_all::<Variable>(SEED, 500, |x| {
        seen.borrow_mut().push(Variable(x.0.clone()));
        true
    })
    .unwrap();
    let seen = seen.into_inner();
    assert_eq!(seen[412], alone);
    assert_ne!(seen[411], seen[412], "neighbouring cases draw different values");
    assert_ne!(seen[0], alone);
}

#[test]
fn a_case_is_a_function_of_seed_and_case_alone() {
    let a: Vec<u64> = (0..8).map(|_| Seeded::for_case(SEED, 9).u64()).collect();
    assert!(a.iter().all(|x| *x == a[0]), "a fresh generator starts over");
    let mut g = Seeded::for_case(SEED, 9);
    let run: Vec<u64> = (0..8).map(|_| g.u64()).collect();
    let mut again = Seeded::for_case(SEED, 9);
    assert_eq!(run, (0..8).map(|_| again.u64()).collect::<Vec<_>>());
    assert_ne!(run[0], run[1], "successive draws move");
    assert_ne!(run[0], Seeded::for_case(SEED ^ 1, 9).u64(), "a new seed draws anew");
}

#[test]
fn the_bounded_draws_stay_inside_their_bounds() {
    let mut g = Seeded::for_case(SEED, 1);
    for _ in 0..CASES {
        assert!(g.below(7) < 7);
        assert_eq!(g.below(1), 0);
        assert_eq!(g.below(0), 0, "a zero bound has no value in it");
        let u = g.unit();
        assert!((0.0..1.0).contains(&u));
        let p = g.phase();
        assert!((0.0..std::f64::consts::TAU).contains(&p));
        assert!([2, 3, 5].contains(g.pick(&[2, 3, 5])));
    }
}

#[test]
fn every_value_of_a_small_bound_is_drawn() {
    let mut counts = [0u32; 4];
    let mut g = Seeded::for_case(SEED, 2);
    for _ in 0..CASES {
        counts[g.below(4) as usize] += 1;
    }
    assert!(counts.iter().all(|c| *c > 0), "no value of 0..4 is missed: {counts:?}");
}

#[test]
fn awkward_f64_draws_the_hard_float_cases() {
    let mut g = Seeded::for_case(SEED, 3);
    let xs: Vec<f64> = (0..CASES * 4).map(|_| g.awkward_f64()).collect();
    assert!(xs.iter().any(|x| *x == 0.0 && x.is_sign_positive()), "+0.0");
    assert!(xs.iter().any(|x| *x == 0.0 && x.is_sign_negative()), "-0.0");
    assert!(xs.iter().any(|x| x.is_subnormal()), "a subnormal");
    assert!(xs.contains(&f64::INFINITY), "+inf");
    assert!(xs.contains(&f64::NEG_INFINITY), "-inf");
    assert!(xs.iter().any(|x| x.is_nan()), "a NaN");
    assert!(xs.iter().any(|x| x.is_normal()), "an ordinary float");
}

#[test]
fn the_drawn_functions_come_from_the_existing_generators() {
    let mut g = Seeded::for_case(SEED, 4);
    let f = g.function(32);
    assert_eq!(f.len(), 32);
    let energy: f64 = f.iter().map(|z| z.norm_sqr()).sum();
    assert!((energy - 1.0).abs() < 1e-9, "unit energy, as unit_random gives: {energy}");
    let r = g.role(32);
    assert_eq!(r.len(), 32);
    assert!(
        r.iter().all(|z| (z.norm() - 1.0).abs() < 1e-12),
        "unit magnitude per sample, as unit_phase gives"
    );
}

/// A pure property's case count clears the 100-case minimum; the machine
/// count is the stated exception, where a case costs a compile and a run.
const _: () = assert!(CASES >= 100);

#[test]
fn the_declared_case_counts_are_the_ones_the_design_names() {
    assert_eq!(CASES, 256);
    assert_eq!(CASES_MACHINE, 64);
}

#[test]
fn no_new_entry_appears_in_the_manifest() {
    const PERMITTED: [&str; 3] = ["num-complex", "serde", "rayon"];
    let manifest = include_str!("../../Cargo.toml");
    let mut in_dependencies = false;
    let mut found: Vec<&str> = Vec::new();
    for line in manifest.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_dependencies = line.contains("dependencies");
            continue;
        }
        if !in_dependencies || line.is_empty() || line.starts_with('#') {
            continue;
        }
        let name = line.split('=').next().unwrap().trim();
        assert!(
            PERMITTED.contains(&name),
            "the seeded generator takes on no outside dependency, and {name} is new"
        );
        found.push(name);
    }
    assert_eq!(found, PERMITTED, "the manifest's dependencies are unchanged");
}
