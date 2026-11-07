//! Host names: the prefix, collisions with the 39, and Property 17.

use super::*;

// ----- the examples -----

#[test]
fn every_host_name_is_prefixed_and_absent_from_the_39() {
    assert_eq!(PhisorHost::core_names().len(), 39);
    for name in NAMES {
        assert!(name.starts_with("phisor-"), "{name} carries no prefix");
    }
    assert_eq!(PhisorHost::check_names(&NAMES), Ok(()));
    assert_eq!(host().names(), &NAMES[..]);
}

/// A host that shadows a core name — what criterion 4 is written against.
struct Shadow;

impl Host for Shadow {
    fn names(&self) -> &'static [&'static str] {
        &["union", "phisor-rows"]
    }

    fn call(&mut self, _: &'static str, _: Vec<Value>) -> Result<Value, String> {
        unreachable!("a shadowing host builds no machine, so nothing can call it")
    }
}

#[test]
fn a_collision_names_both_and_constructs_no_machine() {
    let Err(refused) = checked(Shadow) else {
        panic!("a shadowed core name must be refused")
    };
    assert_eq!(
        refused,
        HostError::CoreNameCollision {
            primitive: "union",
            core: "union"
        }
    );
    let said = refused.to_string();
    assert!(
        said.contains("union") && said.contains("reserved"),
        "{said}"
    );
    // And the message arrives in place of the assert: the real host's five
    // are free, so the real machine still builds.
    assert!(checked(host()).is_ok());
}

// ----- Property 17: a lambda the machine accepts, and no name collides -----

/// A drawn set of candidate host primitive names, one to five of them: a
/// core name straight from the 39, a case variant of one (`Union` against
/// `union`), a dash variant (`union-`, `-union`, `restrictdomain`), or a
/// `phisor-` name known free.
#[derive(Debug)]
struct Names(Vec<String>);

impl Gen for Names {
    fn draw(g: &mut Seeded) -> Self {
        let count = 1 + g.below(5) as usize;
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            let core = *g.pick(PhisorHost::core_names());
            out.push(match g.below(4) {
                0 => core.to_string(),
                1 => upper_first(core),
                2 => g
                    .pick(&[
                        format!("{core}-"),
                        format!("-{core}"),
                        core.replace('-', ""),
                    ])
                    .clone(),
                _ => format!("phisor-{core}"),
            });
        }
        Names(out)
    }
}

/// `union` as `Union`: the case variant the comparison must keep apart.
fn upper_first(core: &str) -> String {
    let mut rest = core.chars();
    match rest.next() {
        Some(first) => first.to_uppercase().collect::<String>() + rest.as_str(),
        None => String::new(),
    }
}

#[test]
fn property_17_a_lowered_operation_is_a_lambda_and_no_host_name_collides() {
    // The lambda half, exhaustive over all 64 Operators rather than sampled.
    let mut m = checked(host()).expect("the five phisor- names are free");
    for op in table() {
        let params: Vec<String> = (0..op.arity).map(|i| format!("a{i}")).collect();
        let bound = format!("phisor-op-{}", op.index);
        let lowered = format!(
            "(define {bound} (lambda ({}) (count (phisor-rows \"is-a\"))))",
            params.join(" ")
        );

        let forms = read(&lowered).unwrap_or_else(|e| panic!("{} does not parse: {e}", op.name));
        assert_eq!(
            forms.len(),
            1,
            "{} lowered to {} forms",
            op.name,
            forms.len()
        );
        let Value::List(items) = &forms[0] else {
            panic!("{} is not a form", op.name)
        };
        assert_eq!(
            items.first(),
            Some(&Value::Sym("define".into())),
            "{} is no define",
            op.name
        );
        let Some(Value::List(lambda)) = items.get(2) else {
            panic!("{} defines no list", op.name)
        };
        assert_eq!(
            lambda.first(),
            Some(&Value::Sym("lambda".into())),
            "{} is no lambda",
            op.name
        );

        // And the machine accepts it, applied to its declared argument count.
        assert_eq!(
            m.run(&lowered),
            Ok(Value::Sym(bound.clone())),
            "{} did not define",
            op.name
        );
        let args = vec!["0"; op.arity as usize].join(" ");
        assert_eq!(
            m.run(&format!("({bound} {args})")),
            Ok(Value::Num(2.0)),
            "{} did not apply",
            op.name
        );
    }

    // The name half: construction succeeds exactly when the set is disjoint
    // from the 39, compared as whole names with case significant.
    let run = for_all::<Names>(0x17, CASES, |drawn| {
        let names: Vec<&str> = drawn.0.iter().map(String::as_str).collect();
        let reserved = names
            .iter()
            .find(|name| PhisorHost::core_names().contains(name));
        match (PhisorHost::check_names(&names), reserved) {
            (Ok(()), None) => true,
            (Err(HostError::CoreNameCollision { primitive, core }), Some(&first)) => {
                primitive == core && core == first
            }
            _ => false,
        }
    });
    assert!(run.is_ok(), "{}", run.unwrap_err());
}
