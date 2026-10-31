//! Reading a claim and the held relations into the shape the class tests
//! want: the triples the World holds now, a reachability walk for the
//! inherit-cycle test, the readability test (R8.12) and the degeneracy test
//! (R8.5). Kept beside the classifier so `mod.rs` holds only the precedence
//! ladder itself.

use phiecs::quest::{Claim, Degeneracy};
use phiecs::world::World;
use std::collections::{HashMap, HashSet};

/// Every triple the World holds now, in spelling form.
pub(super) fn held_triples(world: &World) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for &word in world.words() {
        let from = world.spelling(word).to_string();
        for e in world.edges.out_of(word) {
            let Some(kind) = world.relations.name(e.kind) else { continue };
            // Slot and range channels (`kind#3`, `kind>`) are the phasor
            // read's own bookkeeping, not told relations; skip them.
            if kind.contains('#') || kind.ends_with('>') {
                continue;
            }
            out.push((from.clone(), kind.to_string(), world.spelling(e.to).to_string()));
        }
    }
    out
}

/// Whether `from` reaches `target` along `kind` over the held edges, and if
/// so the path's edges. Used by the inherit-cycle test: the claim closing
/// `target kind from` would make a cycle.
pub(super) fn reaches(
    held: &[(String, String, String)],
    kind: &str,
    from: &str,
    target: &str,
) -> Option<Vec<(String, String, String)>> {
    let mut adj: HashMap<&str, Vec<&(String, String, String)>> = HashMap::new();
    for t in held.iter().filter(|t| t.1 == kind) {
        adj.entry(t.0.as_str()).or_default().push(t);
    }
    let mut stack: Vec<(&str, Vec<(String, String, String)>)> = vec![(from, Vec::new())];
    let mut seen: HashSet<&str> = HashSet::new();
    while let Some((node, path)) = stack.pop() {
        if node == target {
            return Some(path);
        }
        if !seen.insert(node) {
            continue;
        }
        for &edge in adj.get(node).map(Vec::as_slice).unwrap_or(&[]) {
            let mut next = path.clone();
            next.push(edge.clone());
            stack.push((edge.2.as_str(), next));
        }
    }
    None
}

/// Add a triple to the named rows once, keeping the held order.
pub(super) fn push_unique(rows: &mut Vec<(String, String, String)>, t: (String, String, String)) {
    if !rows.contains(&t) {
        rows.push(t);
    }
}

/// Why a claim is not a readable row, if it is not one (R8.12): a weight off
/// the 0..=1 scale, or a field that cannot be read back as one column
/// because it holds the separator or a control character.
pub(super) fn unreadable(about: &str, claim: &Claim) -> Option<String> {
    if claim.weight.is_nan() {
        return Some("weight is not a number".to_string());
    }
    if !(0.0..=1.0).contains(&claim.weight) {
        return Some(format!("weight {} outside 0..=1", claim.weight));
    }
    let fields = [("about", about), ("kind", claim.kind.as_str()), ("other", claim.other.as_str())];
    fields
        .iter()
        .map(|(field, text)| (field, text.trim()))
        .find(|(_, text)| !text.is_empty() && (text.contains('|') || text.chars().any(char::is_control)))
        .map(|(field, text)| format!("{field} is not one row-shaped field: {text:?}"))
}

/// Which degeneracy test holds, if one does (R8.5). The empty-field test is
/// read first: an empty about beside an empty other is both at once, and the
/// emptiness is the more useful half to record.
pub(super) fn degeneracy(about: &str, claim: &Claim) -> Option<Degeneracy> {
    let (about, kind, other) = (about.trim(), claim.kind.trim(), claim.other.trim());
    if about.is_empty() || kind.is_empty() || other.is_empty() {
        return Some(Degeneracy::EmptyField);
    }
    (other == about || kind == other).then_some(Degeneracy::SelfEdge)
}
