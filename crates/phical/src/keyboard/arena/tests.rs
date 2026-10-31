//! The Arena's own tests: the one interning guarantee the equality rule
//! rests on, the stamp that refuses a stray handle, and the depth cap that
//! gives the refusal path a trigger.

use super::*;

/// A seed with no meaning of its own; two Arenas built from different seeds
/// get different ids, which is all the foreign-handle test needs.
const SEED: u64 = 0x5EED;

/// Minting a structurally equal byte form twice returns one handle. This is
/// the whole reason the Arena interns: byte-equal handles must mean
/// structurally equal values, so a second mint of the same canonical bytes
/// cannot hand back a second index.
#[test]
fn a_repeated_form_mints_once() {
    let mut arena = Arena::new(SEED);
    let canon = b"the same bytes";

    let first = arena.mint(Type::Word, canon).unwrap();
    let second = arena.mint(Type::Word, canon).unwrap();

    assert_eq!(first, second, "a structurally equal form must share one handle");
    // epoch() is [words, rows, frames, acts]: one word interned, nothing else.
    assert_eq!(arena.epoch(), [1, 0, 0, 0], "only one word was interned");
}

/// A distinct form gets a distinct handle, so interning does not collapse
/// values that are not equal.
#[test]
fn a_distinct_form_mints_fresh() {
    let mut arena = Arena::new(SEED);

    let a = arena.mint(Type::Word, b"one").unwrap();
    let b = arena.mint(Type::Word, b"two").unwrap();

    assert_ne!(a, b);
    assert_eq!(a.index, 0);
    assert_eq!(b.index, 1);
}

/// A handle minted against one Arena, resolved against another, is refused as
/// a foreign handle rather than indexing whatever bytes happen to sit at that
/// index in the second Arena. The two Arenas hold byte-identical contents so
/// that a resolve by index alone would succeed and return the *wrong* value;
/// the id check is what keeps that from happening silently.
#[test]
fn a_foreign_handle_is_refused_not_misread() {
    let mut mine = Arena::new(SEED);
    let mut theirs = Arena::new(SEED ^ 0xFF);

    let here = mine.mint(Type::Word, b"mine only").unwrap();
    // `theirs` holds a different value at the same index 0.
    theirs.mint(Type::Word, b"theirs only").unwrap();

    match theirs.resolve_from(mine.id, here) {
        Err(Fault::ForeignHandle { expected, given }) => {
            assert_eq!(expected, theirs.id);
            assert_eq!(given, mine.id);
        }
        other => panic!("expected ForeignHandle, got {other:?}"),
    }
}

/// Within its own Arena the handle resolves to exactly the bytes it was
/// minted for, so the refusal above is about provenance and not about the
/// handle being invalid.
#[test]
fn a_home_handle_resolves_to_its_bytes() {
    let mut arena = Arena::new(SEED);
    let h = arena.mint(Type::Frame, b"canonical frame").unwrap();

    assert_eq!(arena.resolve(h).unwrap(), b"canonical frame");
    assert_eq!(arena.resolve_from(arena.id, h).unwrap(), b"canonical frame");
}

/// An index past the table's length is a no-such-handle guard. It is
/// unreachable while minting is the only way to get a handle, so the test
/// forges one.
#[test]
fn an_out_of_range_index_is_no_such_handle() {
    let arena = Arena::new(SEED);
    let forged = Handle::new(Type::Act, 7);

    match arena.resolve(forged) {
        Err(Fault::NoSuchHandle { handle, len }) => {
            assert_eq!(handle, forged);
            assert_eq!(len, 0, "the act table is empty");
        }
        other => panic!("expected NoSuchHandle, got {other:?}"),
    }
}

/// A `Canon` that nests one act inside another, counting its own depth, so an
/// act nested past `MAX_CLAUSE_DEPTH` returns `TooDeep`. This stands in for
/// phisor's real `Act`, which phical cannot see; what matters is that the
/// trait's `write` is the place the depth cap is enforced.
struct DeepAct {
    depth: usize,
}

impl Canon for DeepAct {
    const TYPE: Type = Type::Act;

    fn write(&self, arena: &mut Arena, out: &mut Vec<u8>) -> Result<(), Fault> {
        if self.depth > MAX_CLAUSE_DEPTH {
            return Err(Fault::TooDeep { depth: self.depth, cap: MAX_CLAUSE_DEPTH });
        }
        out.push(self.depth as u8);
        if self.depth > 0 {
            let inner = DeepAct { depth: self.depth - 1 };
            let mut inner_bytes = Vec::new();
            inner.write(arena, &mut inner_bytes)?;
            let h = arena.mint(Type::Act, &inner_bytes)?;
            out.extend_from_slice(&h.index.to_le_bytes());
        }
        Ok(())
    }

    fn read(_arena: &Arena, bytes: &[u8]) -> Result<Self, Fault> {
        Ok(DeepAct { depth: bytes[0] as usize })
    }
}

/// An act nested exactly at the cap canonicalises; one nested past it returns
/// `TooDeep` carrying the depth it reached and the cap it broke.
#[test]
fn an_act_past_the_depth_cap_is_too_deep() {
    let mut arena = Arena::new(SEED);

    let at_cap = DeepAct { depth: MAX_CLAUSE_DEPTH };
    let mut ok = Vec::new();
    assert!(at_cap.write(&mut arena, &mut ok).is_ok(), "the cap itself is allowed");

    let past = DeepAct { depth: MAX_CLAUSE_DEPTH + 1 };
    let mut over = Vec::new();
    match past.write(&mut arena, &mut over) {
        Err(Fault::TooDeep { depth, cap }) => {
            assert_eq!(depth, MAX_CLAUSE_DEPTH + 1);
            assert_eq!(cap, MAX_CLAUSE_DEPTH);
        }
        other => panic!("expected TooDeep, got {other:?}"),
    }
}
