//! One arm's kinds read closely: which kinds sense which label (chosen on
//! dev, scored on the held-out sentences), the routine's passes, family
//! resemblance against shuffled kinds, and every kind with its variations,
//! its labels and an example.

use phisor::guide::{Function, Mark, Part, Sentence, Split};
use phisor::kind::{resemblance, Kinds};
use phistd::stats::{mean, z_score};
use std::collections::HashMap;

/// For each label: the kinds that sense it (named on dev) and their F1 on
/// the held-out sentences (`kind::sensing`).
pub fn sensing(kinds: &Kinds, split: &Split) {
    println!("| label | kinds that sense it (chosen on dev) | held out | sensed | hit | F1 |");
    println!("|---|---|---|---|---|---|");
    let [function, mark] = phisor::kind::sensing(kinds, split, phisor::kind::Rule::Majority);
    let names = Function::ALL.iter().map(|f| ("function", f.name())).chain(Mark::ALL.iter().map(|m| ("mark", m.name())));
    for ((set, name), s) in names.zip(function.iter().chain(&mark)) {
        println!("| {set}: {name} | {} | {:.0} | {:.0} | {:.0} | {:.1}% |", s.kinds, s.expected, s.predicted, s.hit, 100.0 * s.f1());
    }
}

/// The sentence-level routine's passes (Babbage: where the differences stop paying).
pub fn passes(kinds: &Kinds) {
    for (d, gains) in kinds.halving.passes.iter().enumerate() {
        let g: Vec<String> = gains.iter().map(|x| format!("{x:.0}")).collect();
        println!("  kinds, depth {}: {} splits kept, {} passes, gains {}", d + 1, kinds.halving.kept.get(d).copied().unwrap_or(0), gains.len(), g.join(" "));
    }
    println!("  kinds: {} at depth {}", kinds.halving.classes, kinds.halving.depth);
}

/// Family resemblance over every kind of two or more variations: the
/// pieces summed and the widest shared feature's mean share, against the
/// same variations dealt into kinds of the same sizes at random.
pub fn resemblance_null(kinds: &Kinds, shuffles: usize) {
    let (tnames, tkinds) = kinds.template_kinds();
    let names: Vec<&str> = tnames.iter().map(String::as_str).collect();
    let mut codes: Vec<usize> = tkinds.to_vec();
    let read = |codes: &[usize]| -> (f64, f64) {
        let mut by: HashMap<usize, Vec<&str>> = HashMap::new();
        for (n, &c) in names.iter().zip(codes) {
            by.entry(c).or_default().push(n);
        }
        let r: Vec<_> = by.values().filter(|v| v.len() >= 2).map(|v| resemblance(v)).collect();
        let pieces = r.iter().map(|x| x.pieces as f64).sum();
        let shares: Vec<f64> = r.iter().map(|x| x.widest as f64 / x.variations as f64).collect();
        (pieces, mean(&shares))
    };
    let (pieces, share) = read(&codes);
    let mut state = 0x2545_f491_4f6c_dd1du64;
    let (mut np, mut ns) = (Vec::new(), Vec::new());
    for _ in 0..shuffles {
        for i in (1..codes.len()).rev() {
            state = phistd::random::xorshift64(state);
            codes.swap(i, (state % (i as u64 + 1)) as usize);
        }
        let (p, s) = read(&codes);
        np.push(p);
        ns.push(s);
    }
    println!("family resemblance: {pieces:.0} pieces over the kinds (shuffled kinds: {:.1}, z {:.1}); the widest shared feature holds {:.1}% of a kind's variations (shuffled: {:.1}%, z {:.1})", mean(&np), z_score(pieces, &np), 100.0 * share, 100.0 * mean(&ns), z_score(share, &ns));
}

/// Every occupied kind: its variations, how its held-out sentences fall
/// under each label, its family resemblance, and an example.
pub fn kinds(kinds: &Kinds, split: &Split) {
    let test = split.of(Part::Test);
    let mut held: HashMap<usize, Vec<usize>> = HashMap::new();
    for &i in &test {
        held.entry(kinds.of(i)).or_default().push(i);
    }
    let mut order: Vec<usize> = (0..kinds.count()).filter(|&k| !kinds.variations(k).is_empty() || held.contains_key(&k)).collect();
    order.sort_by_key(|k| std::cmp::Reverse(held.get(k).map_or(0, Vec::len)));
    println!("| kind | variations | widest shared | pieces | held out | explain · instruct · answer | stop · question · excl. · line | commonest variation | an example |");
    println!("|---|---|---|---|---|---|---|---|---|");
    for k in order {
        let vars = kinds.variations(k);
        let r = resemblance(&vars);
        let members = held.get(&k).cloned().unwrap_or_default();
        let count = |f: &dyn Fn(&Sentence) -> bool| members.iter().filter(|&&i| f(&split.sentences[i])).count().to_string();
        let f: Vec<String> = Function::ALL.iter().map(|&x| count(&|s| s.function == x)).collect();
        let m: Vec<String> = Mark::ALL.iter().map(|&x| count(&|s| s.function != Function::Answer && s.mark == x)).collect();
        let mut by: HashMap<&str, usize> = HashMap::new();
        for &i in &members {
            *by.entry(kinds.variation[i].as_str()).or_default() += 1;
        }
        let common = by.into_iter().max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(a.0))).map_or(String::new(), |(v, n)| format!("`{v}` ×{n}"));
        let example = members.first().map_or(String::new(), |&i| split.sentences[i].words.iter().take(12).cloned().collect::<Vec<_>>().join(" "));
        println!("| {k} | {} | {} | {} | {} | {} | {} | {common} | {example} |", r.variations, r.widest, r.pieces, members.len(), f.join(" · "), m.join(" · "));
    }
}
