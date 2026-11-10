# The naming and size rule (R10.7, R10.8)

`.kiro/steering` holds no `phinet` naming rule and no `phinet` line-cap rule.
Its three files are `buikit-no-hardcoded-layout.md`, `phiray-ui-polish.md`
and `ui-kit-per-app.md`; they speak of the UI kits and `phiray`, not of this
workspace. The cleanup therefore names the rule it applies:

- Names are singular: modules, types and functions (section 7 lists the
  plural names and their replacements).
- No Rust file is longer than 300 lines. `tests/structural/lines/cap.rs`
  holds the cap.
- A file whose name is plural, or that holds several roles, is split by role.
  The live case is `phisor/src/ladder/act.rs`: it is split into one file per
  role it holds, each a singular name.

The map is held to its modules by `tests/structural/cleanup.rs`: it reads the
first column of the section 1 table and fails if a top-level entry of
`crates/phiano/src` (a folder, or a `.rs` file without its extension) has no
row.
