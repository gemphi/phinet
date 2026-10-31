//! The frozen set loads, covers every Frame field, and refuses to load
//! once it has been changed. No property is named for the frozen set —
//! R10.10's Property 27 reads it, and lives with the round trip that owns
//! it — so these are unit tests over the one frozen input.

use super::*;
use crate::ir::{Aspect, Attr, Det, Filler, Force, Modal, Polarity, Role, Tense, Voice, Wh};
use std::collections::BTreeSet;

#[test]
fn the_frozen_set_is_one_hundred_and_fifty_frames_at_the_pinned_fingerprint() {
    let gold = read().unwrap_or_else(|f| panic!("{f}"));
    assert_eq!(gold.len(), COUNT);
    assert_eq!(gold.fingerprint, FINGERPRINT);
    // The ids are the file's own order, 1 to 150, so a Miss can name a row.
    assert!(gold.entries.iter().enumerate().all(|(i, e)| e.id == i + 1));
    assert_eq!(gold.acts().count(), COUNT);
    assert_eq!(gold.pairs().count(), COUNT);
}

#[test]
fn every_force_and_every_frame_field_is_carried() {
    let gold = read().unwrap();
    let frames = || gold.entries.iter().map(|e| &e.act.frame);
    let forces: BTreeSet<Force> = gold.entries.iter().map(|e| e.act.force).collect();
    assert_eq!(forces, Force::ALL.into_iter().collect::<BTreeSet<_>>());
    for t in Tense::ALL {
        assert!(frames().any(|f| f.tense == t), "no frame is {}", t.name());
    }
    for a in Aspect::ALL {
        assert!(frames().any(|f| f.aspect == a), "no frame is {}", a.name());
    }
    for m in Modal::ALL {
        assert!(frames().any(|f| f.modal == Some(m)), "no frame is {}", m.name());
    }
    assert!(frames().any(|f| f.modal.is_none()), "every frame carries a modal");
    for v in Voice::ALL {
        assert!(frames().any(|f| f.voice == v), "no frame is {}", v.name());
    }
    for p in Polarity::ALL {
        assert!(frames().any(|f| f.polarity == p), "no frame is {}", p.name());
    }
    // 0 to 4 roles, which is the span R10.10's generator is written over.
    let widths: BTreeSet<usize> = frames().map(|f| f.slots.len()).collect();
    assert_eq!(widths, (0..=4).collect::<BTreeSet<_>>());
}

#[test]
fn every_filler_variant_and_every_hole_is_carried() {
    let gold = read().unwrap();
    let mut seen = BTreeSet::new();
    let mut holes = BTreeSet::new();
    let mut attrs = BTreeSet::new();
    let mut dets = BTreeSet::new();
    let mut depth = 0usize;
    for e in &gold.entries {
        walk(&e.act.frame, 1, &mut seen, &mut holes, &mut attrs, &mut dets, &mut depth);
    }
    assert_eq!(
        seen,
        ["clause", "hole", "name", "pronoun", "thing", "word"]
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(holes, Wh::ALL.into_iter().map(Wh::name).collect::<BTreeSet<_>>());
    assert_eq!(
        attrs,
        ["count", "kind", "mod", "order", "via"].into_iter().collect::<BTreeSet<_>>()
    );
    assert_eq!(
        dets,
        ["definite", "far", "indefinite", "near", "owner", "quant"]
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
    // At least one frame nests a clause inside a clause, which is where a
    // reader that flattens is caught.
    assert!(depth >= 3, "the deepest frame is at depth {depth}");
}

/// What a frame carries, gathered by variant name so a missing variant
/// names itself in the failure.
fn walk(
    f: &crate::ir::Frame,
    at: usize,
    seen: &mut BTreeSet<&'static str>,
    holes: &mut BTreeSet<&'static str>,
    attrs: &mut BTreeSet<&'static str>,
    dets: &mut BTreeSet<&'static str>,
    depth: &mut usize,
) {
    *depth = (*depth).max(at);
    for (_, filler) in &f.slots {
        match filler {
            Filler::Thing(t) => {
                seen.insert("thing");
                if let Some(d) = &t.det {
                    dets.insert(match d {
                        Det::Definite => "definite",
                        Det::Indefinite => "indefinite",
                        Det::Near => "near",
                        Det::Far => "far",
                        Det::Quant(_) => "quant",
                        Det::Owner(_) => "owner",
                    });
                }
                for (a, v) in &t.attrs {
                    attrs.insert(match a {
                        Attr::Mod => "mod",
                        Attr::Kind(_) => "kind",
                        Attr::Order => "order",
                        Attr::Count => "count",
                        Attr::Via(_) => "via",
                    });
                    match v {
                        Filler::Word(_) => seen.insert("word"),
                        Filler::Hole(w) => {
                            holes.insert(w.name());
                            seen.insert("hole")
                        }
                        Filler::Name(_) => seen.insert("name"),
                        _ => true,
                    };
                }
            }
            Filler::Name(_) => {
                seen.insert("name");
            }
            Filler::Pronoun(_) => {
                seen.insert("pronoun");
            }
            Filler::Word(_) => {
                seen.insert("word");
            }
            Filler::Hole(w) => {
                seen.insert("hole");
                holes.insert(w.name());
            }
            Filler::Clause(inner) => {
                seen.insert("clause");
                walk(inner, at + 1, seen, holes, attrs, dets, depth);
            }
        }
    }
}

#[test]
fn the_rows_read_as_the_acts_they_are_written_for() {
    let gold = read().unwrap();
    let said = |id: usize| gold.entries[id - 1].act.to_string();
    assert_eq!(said(16), "ASSERT(wear(agent: girl, theme: suit{colour: blue}), present, progressive)");
    assert_eq!(said(73), "ASK(be(theme: ?who, as: king{order: first, of: France}), past)");
    assert_eq!(said(136), "WISH(sing(agent: I), past, can)");
    assert_eq!(said(69), "ASSERT(lose(theme: letter), present, perfect, must, passive)");
    assert_eq!(said(79), "ASK(come(theme: people{count: ?how-many}), past)");
    assert_eq!(
        said(126),
        "COMMIT(promise(agent: I, theme: [try(agent: I, theme: [pay(agent: I, theme: money), present]), present]), present)"
    );
    assert_eq!(said(138), "WISH(stop(theme: [rain(), present, progressive]), past, will)");
    assert_eq!(said(18), "ASSERT(rain(), present, progressive)");
    // The roles keep the sentence's own order, and a role with no name of
    // its own keeps its preposition's.
    let sent = &gold.entries[33];
    assert_eq!(sent.sentence, "the letter was sent to london by helen on friday.");
    let roles: Vec<&str> = sent.act.frame.slots.iter().map(|(r, _)| r.name()).collect();
    assert_eq!(roles, ["theme", "to", "agent", "on"]);
    assert_eq!(sent.act.frame.get(&Role::Via("on".into())), Some(&Filler::name("Friday")));
}

#[test]
fn a_changed_row_fails_the_fingerprint_and_a_dropped_row_fails_the_count() {
    let text = include_str!("../../tests/gold/frames.tsv");
    // One cell changed: the count still reads 150, the fingerprint does not.
    let changed = text.replacen("ASSERT\tboil", "ASSERT\tsimmer", 1);
    assert_ne!(changed, text);
    match of(&changed) {
        Err(Flaw::Fingerprint { read, declared }) => {
            assert_eq!(declared, FINGERPRINT);
            assert_ne!(read, FINGERPRINT);
        }
        other => panic!("a changed row loaded: {other:?}"),
    }
    // One row dropped: the count is 149 and the load stops before the
    // fingerprint, because the count is the more useful half to report.
    let last = text.trim_end().lines().last().unwrap();
    let dropped = text.replacen(last, "", 1);
    assert_eq!(of(&dropped), Err(Flaw::Count { read: COUNT - 1, declared: COUNT }));
    // One row added: the same way round.
    let added = format!("{text}{last}\n");
    assert_eq!(of(&added), Err(Flaw::Count { read: COUNT + 1, declared: COUNT }));
}

#[test]
fn an_unreadable_cell_names_the_row_the_column_and_the_cell() {
    let row = |cells: &str| of(&format!("# head\n{cells}\n"));
    let good = "it is raining.\tASSERT\train\t-\tpresent\tprogressive\t-\tactive\tpositive";
    // A well-formed single row reads, then fails the count: the grammar is
    // not what stopped it.
    assert_eq!(row(good), Err(Flaw::Count { read: 1, declared: COUNT }));
    let flaw = |cells: &str| match row(cells) {
        Err(Flaw::Row { id, column, cell, why }) => (id, column, cell, why),
        other => panic!("{cells:?} read as {other:?}"),
    };
    assert_eq!(
        flaw(&good.replace("ASSERT", "SUGGEST")),
        (1, "force", "SUGGEST".to_string(), "is not one of the seven".to_string())
    );
    assert_eq!(
        flaw(&good.replace("progressive", "habitual")),
        (1, "aspect", "habitual".to_string(), "is not a declared aspect".to_string())
    );
    assert_eq!(flaw(&good.replace("\tactive", "")).1, "row");
    assert_eq!(
        flaw(&good.replace("\t-\tpresent", "\tagent=?whence\tpresent")),
        (1, "roles", "?whence".to_string(), "is not a declared hole".to_string())
    );
    assert_eq!(
        flaw(&good.replace("\t-\tpresent", "\tagent=yonder cat\tpresent")),
        (
            1,
            "roles",
            "yonder cat".to_string(),
            "opens on \"yonder\", which is not a determiner".to_string()
        )
    );
    assert_eq!(flaw(&good.replace("\t-\tpresent", "\tagent\tpresent")).3, "is not `role=filler`");
    assert_eq!(
        flaw(&good.replace("\t-\tpresent", "\ttheme=[go ; agent=^I ; present]\tpresent")).3,
        "holds 3 clause fields, not 7"
    );
}

#[test]
fn the_fingerprint_reads_the_same_under_either_line_ending() {
    let text = include_str!("../../tests/gold/frames.tsv");
    let crlf = text.replace('\n', "\r\n");
    assert_ne!(crlf.len(), text.len());
    assert_eq!(fingerprint(&crlf), fingerprint(text));
    assert_eq!(fingerprint(text), FINGERPRINT);
    // It is a fingerprint rather than a length: a swap of two bytes moves
    // it.
    assert_ne!(fingerprint("ab"), fingerprint("ba"));
}
