//! The Note G analytic model holds to the live architecture: the four
//! correspondences name the four real modules, the quotations are Lovelace's
//! own verbatim words, and the two readings agree with the code they read.

use super::*;
use phical::keyboard::table;

#[test]
fn the_four_correspondences_name_the_four_real_modules() {
    assert_eq!(Correspondence::ALL.len(), 5);
    let modules: Vec<&str> = Correspondence::ALL.iter().map(|c| c.phisor_module()).collect();
    assert_eq!(modules, ["ground", "machine", "ladder", "truth", "ops"]);
    // Every module name is one word, as the crate's path rule requires.
    assert!(modules.iter().all(|m| m.chars().all(|c| c.is_ascii_lowercase())));
    // The four are distinct ideas and distinct modules.
    let mut terms: Vec<&str> = Correspondence::ALL.iter().map(|c| c.lovelace_term()).collect();
    terms.sort_unstable();
    terms.dedup();
    assert_eq!(terms.len(), 5);
}

#[test]
fn her_famous_lines_are_quoted_verbatim_and_attributed() {
    // The Jacquard-loom weaving line, Note A.
    assert_eq!(
        Correspondence::Weave.quotation(),
        Some(
            "The Analytical Engine weaves algebraical patterns just as the \
             Jacquard-loom weaves flowers and leaves."
        )
    );
    assert_eq!(QUOTATIONS[0].1, "Note A");
    // The originate-nothing line, Note G.
    let objection = Correspondence::Objection.quotation().expect("the objection is quoted");
    assert!(objection.contains("no pretensions whatever to originate anything"));
    assert_eq!(QUOTATIONS[1].1, "Note G");
    // The store, the mill and the cards are her terms, not quoted sentences.
    assert_eq!(Correspondence::Store.quotation(), None);
    assert_eq!(Correspondence::Mill.quotation(), None);
    assert_eq!(Correspondence::Cards.quotation(), None);
    // Every quotation names the Note it is from.
    assert!(QUOTATIONS.iter().all(|(_, source)| source.starts_with("Note ")));
}

#[test]
fn the_dedication_is_the_projects_own_voice() {
    let said = dedication();
    assert!(said.contains("Augusta Ada Lovelace"));
    assert!(said.contains("1815-1852"));
    // The superlative is set in the dedication, where it is plainly the
    // project speaking rather than a claim of record.
    assert!(said.contains("in this project's reckoning"));
}

#[test]
fn operation_cards_reads_the_lowerings_own_ocode_length() {
    let mut lowering = crate::machine::Lowering::new();
    for operator in table() {
        let lowered = lowering
            .lower(&operator)
            .unwrap_or_else(|fault| panic!("{} refused: {fault:?}", operator.name));
        // The reading recomputes nothing: it is the emitted O-code's length.
        assert_eq!(operation_cards(&lowered), lowered.ocode.len());
        // And it stays within the bound the mill was given (R7.5).
        assert!(
            (crate::machine::MIN_OCODE..=crate::machine::MAX_OCODE)
                .contains(&operation_cards(&lowered)),
            "{} emitted {} cards",
            operator.name,
            operation_cards(&lowered)
        );
    }
}

#[test]
fn a_woven_mode_carries_no_thread_of_its_text() {
    // Class ids below the level's class count are structure; nothing here is
    // a word, so the weave holds. A token at or above the count would be a
    // thread of the text, and the weave would not hold.
    let woven = vec![vec![0u16, 5, 12], vec![3, 9], vec![]];
    assert!(weaves_no_thread(&woven, 16));
    assert!(!weaves_no_thread(&woven, 12), "class 12 is a thread at a 12-class mode");
    // An empty weave trivially holds.
    assert!(weaves_no_thread(&[], 16));
}
