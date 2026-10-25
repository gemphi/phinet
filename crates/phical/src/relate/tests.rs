use super::*;
use crate::{unit_random, Adam, Fno, FnoConfig, Joint, Tie};

/// Families of four: `near` is the next member, `is-a` the family head,
/// `tool` goes by index mod 3. Items 0..16 members, 16..20 heads, 20..23
/// tools, 23 `near`, 24 `is-a`, 25 `tool`.
fn toy() -> Vec<Triple> {
    let mut v = Vec::new();
    for i in 0..16 {
        v.push(Triple { about: i, kind: 23, other: (i / 4) * 4 + (i + 1) % 4 });
        v.push(Triple { about: i, kind: 24, other: 16 + i / 4 });
        v.push(Triple { about: i, kind: 25, other: 20 + i % 3 });
    }
    v
}

#[test]
fn laws_are_counted_not_written() {
    let laws = find_laws(&toy(), 4);
    let near_isa = laws.iter().find(|l| l.k1 == 23 && l.k2 == 24).unwrap();
    assert_eq!(near_isa.confidence(), 1.0);
    let near_tool = laws.iter().find(|l| l.k1 == 23 && l.k2 == 25).unwrap();
    assert!(near_tool.confidence() < 0.5, "{near_tool:?}");
    assert_eq!(laws[0].confidence(), 1.0);
}

#[test]
fn closure_reads_a_word_like_its_law_neighbour() {
    let told = toy();
    let table: Vec<_> = (0..26).map(|i| unit_random(16, 11 ^ i as u64)).collect();
    let cfg = FnoConfig { space: 16, width: 4, layers: 1, modes: 4, margin: 0.3, lr: 1e-2, tie: Tie::Circle };
    let mut joint = Joint::new(Fno::new(cfg, 5), &table, 2);
    let (mut a, mut b) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    let law = Law { k1: 23, k2: 24, support: 16, agree: 16 };
    let pairs = closure_pairs(&told, &[&law]);
    assert_eq!(pairs.len(), 16);
    let items: Vec<usize> = (0..16).collect();
    let first = closure_epoch(&mut joint, &pairs, &items, 1.0, &mut a, &mut b, 0);
    let mut last = first;
    for e in 1..40 {
        last = closure_epoch(&mut joint, &pairs, &items, 1.0, &mut a, &mut b, e);
    }
    assert!(last < first, "closure loss must fall: {first:.3} -> {last:.3}");
}

#[test]
fn the_prior_ranks_by_how_often_a_kind_named_an_answer() {
    let told = toy();
    let (pool, by_kind) = pools(&told);
    let rows = vec![Triple { about: 0, kind: 25, other: 20 }];
    let s = prior(&told, &rows, &pool, &by_kind);
    // Tool 20 was named 6 times, 21 and 22 five: 20 is first of the pool.
    assert_eq!(s.mrr, 1.0);
    assert!((chance(3) - (1.0 + 0.5 + 1.0 / 3.0) / 3.0).abs() < 1e-12);
}

#[test]
fn an_inverse_is_counted_and_implies_the_untold_direction() {
    // 0 unlike 1 and 1 unlike 0 told; 2 unlike 3 told, 3's side not.
    let told = vec![
        Triple { about: 0, kind: 9, other: 1 },
        Triple { about: 1, kind: 9, other: 0 },
        Triple { about: 2, kind: 9, other: 3 },
        Triple { about: 3, kind: 9, other: 4 },
    ];
    let inv = find_inverses(&told, 1);
    let own = inv.iter().find(|i| i.k == 9 && i.k_inv == 9).unwrap();
    assert_eq!((own.support, own.agree), (3, 2));
    let new = implied(&told, &[own]);
    assert!(new.contains(&Triple { about: 3, kind: 9, other: 2 }));
    assert!(new.contains(&Triple { about: 4, kind: 9, other: 3 }));
    assert!(!new.contains(&Triple { about: 1, kind: 9, other: 0 }), "already told");
}

#[test]
fn a_word_inherits_what_its_kind_has() {
    // 0 and 1 are-a 2 (is-a row kind 9); 2 has 5 (kind 8); 0 has 6 too.
    let told = vec![
        Triple { about: 0, kind: 9, other: 2 },
        Triple { about: 1, kind: 9, other: 2 },
        Triple { about: 2, kind: 8, other: 5 },
        Triple { about: 0, kind: 8, other: 6 },
    ];
    let inh = Inherit::from_told(&told, 9, vec![8], 1.0);
    assert_eq!(inh.of(&Triple { about: 1, kind: 8, other: 5 }), &[2]);
    assert!(inh.of(&Triple { about: 1, kind: 9, other: 2 }).is_empty(), "is-a itself does not inherit");
    assert!(inh.of(&Triple { about: 2, kind: 8, other: 5 }).is_empty(), "no told parent");
    let table: Vec<_> = (0..10).map(|i| unit_random(16, 3 ^ i as u64)).collect();
    let cfg = FnoConfig { space: 16, width: 4, layers: 1, modes: 4, margin: 0.3, lr: 1e-2, tie: Tie::Circle };
    let mut joint = Joint::new(Fno::new(cfg, 5), &table, 2);
    let (mut a, mut b) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    let pool = vec![2, 5, 6];
    let first = inherit_epoch(&mut joint, &told, &inh, &pool, &mut a, &mut b, 0);
    let mut last = first;
    for e in 1..60 {
        last = inherit_epoch(&mut joint, &told, &inh, &pool, &mut a, &mut b, e);
    }
    assert!(last <= first, "{first} -> {last}");
    // 1 was never told `has`; through its parent it reads 5.
    let (_, by_kind) = pools(&told);
    let s = score_inherit(&mut joint, &[Triple { about: 1, kind: 8, other: 5 }], &inh, &pool, &by_kind);
    assert!(s.mrr >= 0.5, "{}", s.mrr);
}

#[test]
fn transitive_kinds_read_their_own_parents() {
    // is-a (9): 0 → 2 → 3. has (8) inherits along is-a.
    let told = vec![Triple { about: 0, kind: 9, other: 2 }, Triple { about: 2, kind: 9, other: 3 }, Triple { about: 2, kind: 8, other: 5 }];
    let inh = Inherit::from_told(&told, 9, vec![8], 1.0).with_transitive(&told, &[9]);
    assert_eq!(inh.of(&Triple { about: 0, kind: 9, other: 3 }), &[2], "is-a(0) reads is-a(2)");
    assert_eq!(inh.of(&Triple { about: 0, kind: 8, other: 5 }), &[2], "has still inherits");
}

#[test]
fn a_converse_is_read_both_ways() {
    // has (8) ⇄ part-of (7): 0 has 5 is told; 5 part-of 0 is asked.
    let told = vec![Triple { about: 0, kind: 8, other: 5 }, Triple { about: 1, kind: 8, other: 6 }, Triple { about: 5, kind: 7, other: 0 }];
    let conv = Converse::declared(&[(8, 7)], 1.0);
    assert_eq!((conv.of[&8], conv.of[&7]), (7, 8));
    let table: Vec<_> = (0..10).map(|i| unit_random(16, 5 ^ i as u64)).collect();
    let cfg = FnoConfig { space: 16, width: 4, layers: 1, modes: 4, margin: 0.3, lr: 1e-2, tie: Tie::Circle };
    let mut joint = Joint::new(Fno::new(cfg, 5), &table, 2);
    let (mut a, mut b) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    let pool = vec![0, 1, 5, 6];
    let first = laws_epoch(&mut joint, &told, None, &conv, &pool, &mut a, &mut b, 0);
    let mut last = first;
    for e in 1..80 {
        last = laws_epoch(&mut joint, &told, None, &conv, &pool, &mut a, &mut b, e);
    }
    assert!(last <= first, "{first} -> {last}");
    // 6 part-of 1 was never told; 1 has 6 was. Its own reading of
    // `6 · part-of` was never trained, so the claim is only about the
    // converse half: `1 · has` reads 6 better than the other answers do.
    let tab = joint.functions();
    let mut back = |v: usize| {
        let y = joint.fno.apply(&[super::inherit::context(&tab, v, 8)]);
        crate::cosine(&y[0], &tab[6])
    };
    let (b1, b0, b5) = (back(1), back(0), back(5));
    assert!(b1 > b0 && b1 > b5, "{b1} {b0} {b5}");
    let (_, by_kind) = pools(&told);
    let s = score_laws(&mut joint, &[Triple { about: 6, kind: 7, other: 1 }], None, &conv, &pool, &by_kind);
    assert!(s.mrr > 0.0);
}

#[test]
fn no_converse_reads_as_inheritance_alone() {
    let told = vec![Triple { about: 0, kind: 9, other: 2 }, Triple { about: 2, kind: 8, other: 5 }, Triple { about: 1, kind: 8, other: 6 }];
    let inh = Inherit::from_told(&told, 9, vec![8], 1.0);
    let table: Vec<_> = (0..10).map(|i| unit_random(16, 7 ^ i as u64)).collect();
    let cfg = FnoConfig { space: 16, width: 4, layers: 1, modes: 4, margin: 0.3, lr: 1e-2, tie: Tie::Circle };
    let mut joint = Joint::new(Fno::new(cfg, 5), &table, 2);
    let (pool, by_kind) = pools(&told);
    let a = score_inherit(&mut joint, &told, &inh, &pool, &by_kind);
    let b = score_laws(&mut joint, &told, Some(&inh), &Converse::none(), &pool, &by_kind);
    assert_eq!((a.mrr, a.within), (b.mrr, b.within));
}
