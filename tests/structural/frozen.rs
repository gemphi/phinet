//! The frozen operand set's fingerprint (R7.6): `phistd::mix64` folded over
//! `phinet/crates/phical/tests/keyboard/operands.tsv`, held to the value
//! pinned in `allowlist.golden`.
//!
//! The set is the whole input to the agreement check of R7.6 and was written
//! before the first agreement run. What that criterion asks for — one
//! question put to phiclm's machine and to the Rust reader — only means
//! something if the question does not move between the two, and a frozen
//! file is only frozen if something notices when it thaws. This is that
//! something, and it lives here rather than in phical because the
//! Structure_Check's whole input is files: it reads the record's bytes and
//! the pin's text and links neither crate.
//!
//! The fold is written out here, as the generator's mix is, because this
//! target links no crate it polices. Note that it is *not* `crate::mix64`:
//! phistd's `mix64` adds the golden-ratio constant before the two
//! multiply-shift rounds and the generator's finaliser does not, so the two
//! are different functions over the same constants. The pin was recorded
//! from phical's own run through `phistd::mix64`, so this test agreeing with
//! it is also the two implementations agreeing — a wrong mix here fails as
//! loudly as a modified record.

/// The record, relative to the workspace root.
const RECORD: &str = "crates/phical/tests/keyboard/operands.tsv";

/// The pin's key in the golden record. It is a comment line: the right-hand
/// sides of the uncommented lines are dependency lists, and
/// `the_allowlist_golden_is_the_manifests_true_outside_state` holds their
/// keys to the ten crates, so a fingerprint on a line of its own would read
/// as an eleventh crate named `frozen-operands`.
const KEY: &str = "# frozen-operands =";

/// phistd's `mix64`: splitmix64's finaliser *with* the golden-ratio add.
fn mix64(h: u64) -> u64 {
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The declared fold: [`mix64`] over each byte in turn, a carriage return
/// skipped so a CRLF checkout holds the same record.
fn fingerprint(bytes: &[u8]) -> u64 {
    let mut hash = 0u64;
    for &byte in bytes.iter().filter(|&&b| b != b'\r') {
        hash = mix64(hash ^ byte as u64);
    }
    hash
}

/// The record's bytes as they stand on disk.
fn record() -> Vec<u8> {
    let path = crate::workspace_root().join(RECORD);
    std::fs::read(&path).unwrap_or_else(|e| panic!("{} is unreadable: {e}", path.display()))
}

/// The pinned fingerprint, read from the one comment line that carries it.
fn pinned() -> u64 {
    let line = crate::ALLOWLIST_GOLDEN
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with(KEY))
        .unwrap_or_else(|| panic!("{} holds no `{KEY} 0x…` line", crate::ALLOWLIST_PATH));
    let digits = line[KEY.len()..].trim().trim_start_matches("0x");
    u64::from_str_radix(digits, 16).unwrap_or_else(|e| {
        panic!("{}: `{line}` is not a hex fingerprint: {e}", crate::ALLOWLIST_PATH)
    })
}

/// The record on disk is the record that was pinned. A failure names both
/// numbers and the two files, because the fix depends on which moved: a
/// deliberate change to the set is re-recorded, an accidental one reverted.
///
/// Validates: Requirements 7.6, 11.12
#[test]
fn the_set_is_the_fingerprint_pinned_in_the_golden_record() {
    let found = fingerprint(&record());
    let pinned = pinned();
    assert_eq!(
        found,
        pinned,
        "{RECORD} fingerprints {found:#018x}; {} pins {pinned:#018x}. The frozen operand set \
         is the whole input to the agreement check (R7.6) and was written before the first \
         agreement run: either revert the change to the set, or re-record the pin with the \
         previous fingerprint, the new one and the stated reason (R11.10)",
        crate::ALLOWLIST_PATH
    );
}

/// The pin is worth having only if it moves when the record does. One byte
/// changed at the front, in the middle and at the end is three fingerprints,
/// none of them the pinned one.
///
/// Validates: Requirements 7.6
#[test]
fn a_modified_record_fails_the_pin() {
    let bytes = record();
    let pinned = pinned();
    assert!(bytes.len() > 2, "{RECORD} holds nothing to perturb");
    for at in [0, bytes.len() / 2, bytes.len() - 1] {
        let mut edited = bytes.clone();
        edited[at] = edited[at].wrapping_add(1);
        assert_ne!(
            fingerprint(&edited),
            pinned,
            "{RECORD} with byte {at} changed keeps its fingerprint"
        );
    }
    let mut longer = bytes.clone();
    longer.extend_from_slice(b"level\tdrawn\t07\n");
    assert_ne!(fingerprint(&longer), pinned, "an operand appended keeps the fingerprint");
}

/// A carriage return is not part of the record, so the same set checked out
/// with CRLF line endings fingerprints the same. Without this the pin would
/// fail on Windows and pass on Linux, which is a worse failure than no pin.
///
/// Validates: Requirements 7.6
#[test]
fn the_fingerprint_does_not_turn_on_the_line_ending() {
    let bytes = record();
    let text = String::from_utf8(bytes.clone()).expect("the record is UTF-8");
    let crlf = text.replace('\r', "").replace('\n', "\r\n");
    assert_eq!(fingerprint(&bytes), fingerprint(crlf.as_bytes()));
}

/// The declared fold is phistd's mix and not the generator's finaliser. The
/// two differ by one add, which is easy to lose and silent when lost — the
/// pin would simply be a fingerprint under a different hash, and `mix64`
/// here would no longer be the `phistd::mix64` the requirement names.
#[test]
fn the_declared_mix_is_not_the_generators_finaliser() {
    assert_ne!(mix64(0), crate::mix64(0), "the golden-ratio add has gone missing");
    assert_eq!(mix64(0), 0xE220_A839_7B1D_CDAF, "phistd::mix64 of zero");
}
