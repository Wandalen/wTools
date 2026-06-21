# TSK-005: Extend CliHelpTemplate — multi-section options, custom usage lines, arguments section

## Execution State

- **State:** ✅ (Complete)
- **Executor:** AI
- **Closes:** null

## MOST Goal

- **Motivated:** `clr --help` requires four named option groups (RUNNER OPTIONS, CLAUDE CODE OPTIONS (forwarded), SUBCOMMANDS, SESSION FLAGS), eight USAGE forms, and an ARGUMENTS section. `CliHelpData` (0.9.1) has a single `options: Vec<OptionEntry>` field, a hardcoded `"Usage: {binary} <command>"` emission, and no ARGUMENTS slot. Claude_runner cannot migrate `help.rs` to `CliHelpTemplate` without these structural capabilities, keeping it on a hand-rolled 262-line printer that will drift from the template's ANSI/width logic on every future help change.
- **Observable:** `CliHelpData` exposes `usage_lines: Vec<String>`, `arguments: Vec<OptionEntry>`, and `option_groups: Vec<OptionGroup>` fields; `render()` calls, in order: `emit_header` (usage lines + tagline + Commands label), `emit_arguments` (when non-empty), `emit_groups` (Commands section), `emit_option_groups` (each `OptionGroup` as a titled section with per-group padding), conditional `emit_options` (suppressed when `option_groups` is non-empty; preserved when `option_groups` is empty — backward compat for callers that set only `options`), `emit_examples`; callers that leave `option_groups` empty still get the existing `"Options:"` section (backward compat); 22 nextest tests pass (`cargo nextest run --all-features`), 1 compile_fail doctest passes (`cargo test --doc --all-features`); `CliHelpData::default()` constructs without field literals; `basic_usage.rs` example compiles using `CliHelpData::default()` + field assignment (`#[non_exhaustive]` also blocks struct update syntax from external crates — E0639); crate version bumped to 0.9.2.
- **Scoped:** Changes confined to `src/help.rs` (struct + `impl CliHelpTemplate` methods + T-A08 compile_fail doctest), `examples/basic_usage.rs` (converted to `CliHelpData::default()` + field assignment; `#[non_exhaustive]` blocks struct update syntax from external crates — E0639), `tests/help.rs` (migrate all existing exhaustive `CliHelpData` struct literals to `CliHelpData::default()` + field assignment + 8 new test cases T-A01–T-A07, T-A09), `Cargo.toml` (version bump), and the downstream TSK-232 task file (item 15 — post-completion task-management notification; out-of-crate write authorized by `## Downstream Dependency` section). No changes to `CliHelpStyle`, `CommandGroup`, `ExampleEntry`, `OptionEntry`, or publish configuration.
- **Testable:** T-A01: `usage_lines: vec!["clr <command>".into()]` → output contains `"  clr <command>"`; T-A02: `arguments: vec![OptionEntry { name: "<MSG>".into(), desc: "Message to send".into() }]` → output contains `"  <MSG>  Message to send"` (single entry, name width=5, exactly 2 spaces as separator between padded name and desc); T-A03: `option_groups: vec![OptionGroup { name: "RUNNER OPTIONS".into(), entries: vec![...] }]` → output contains `"RUNNER OPTIONS:"`; T-A04: `option_groups: vec![]`, `options` non-empty → existing `"Options:"` section renders unchanged; T-A05: `options: vec![OptionEntry { name: "--old".into(), desc: "old".into() }]` + `option_groups: vec![OptionGroup { name: "NEW".into(), entries: vec![OptionEntry { name: "--new".into(), desc: "new".into() }] }]` → output contains `"NEW:"` and `"  --new  new"`; does NOT contain `"--old"` and does NOT contain `"Options:"` (emit_options suppressed when option_groups non-empty); T-A06: two groups — group A entries `OptionEntry { name: "--aa".into(), desc: "flag a".into() }` and `OptionEntry { name: "--bb".into(), desc: "flag b".into() }`; group B entry `OptionEntry { name: "--longer-name".into(), desc: "a long flag".into() }` → output contains `"  --aa  flag a"` and `"  --bb  flag b"` (group A max_len=4, 2-space separator) and `"  --longer-name  a long flag"` (group B max_len=13) — groups compute padding independently; T-A07: `CliHelpData::default()` constructs without panic, all Vec fields empty; T-A08: compile_fail doctest in `src/help.rs` confirms `#[non_exhaustive]` rejects exhaustive external struct literal; T-A09: `cargo test --examples` passes.

## In Scope

1. Add `#[derive(Default, Debug, Clone)]` and `#[non_exhaustive]` to `CliHelpData` struct in `src/help.rs`
2. Add `usage_lines: Vec<String>` field to `CliHelpData` (default: `vec![]`)
3. Add `arguments: Vec<OptionEntry>` field to `CliHelpData` (default: `vec![]`)
4. Add `pub struct OptionGroup { pub name: String, pub entries: Vec<OptionEntry> }` to `src/help.rs`; add `#[derive(Debug, Clone)]`; locate the `pub mod prelude` block in `src/help.rs` and add `OptionGroup` to its `pub use` exports so downstream callers using `use cli_fmt::prelude::*` can construct `OptionGroup` without a separate import
5. Add `option_groups: Vec<OptionGroup>` field to `CliHelpData` (default: `vec![]`)
6. In `impl CliHelpTemplate`, update `emit_header()`: replace ONLY the first `writeln!` (the hardcoded `{bold}Usage:{rst} {} <command>` line) with a conditional — when `self.data.usage_lines` is non-empty emit each as `writeln!(out, "  {line}").ok()`, else emit the original single-line form; the four remaining `writeln!` calls (blank line, tagline, blank line, `{bold}Commands:{rst}`) emit in both paths unchanged
7. In `impl CliHelpTemplate`, add `fn emit_arguments(&self, out: &mut String, ...)`: guard `if self.data.arguments.is_empty() { return; }`; compute `max_len` = max `entry.name.len()` across `self.data.arguments`; write `"\nArguments:"` header then each entry as `"  {name:<max_len}  {desc}"`
8. In `impl CliHelpTemplate`, add `fn emit_option_group(&self, out: &mut String, ..., name: &str, entries: &[OptionEntry])`: guard empty; compute `max_len` within this group's entries only; write `"\n{name}:"` then entries
9. In `impl CliHelpTemplate`, add `fn emit_option_groups(&self, out: &mut String, ...)`: iterate `self.data.option_groups`, call `emit_option_group()` for each
10. In `impl CliHelpTemplate`, update `render()` call sequence: `emit_header()` → `emit_arguments()` → `emit_groups()` (existing Commands section, keep BEFORE options) → `emit_option_groups()` → `if self.data.option_groups.is_empty() && !self.data.options.is_empty() { emit_options() }` → `emit_examples()` — the existing inner `!options.is_empty()` guard (current line 172) must be preserved inside the new outer guard
11. Add `compile_fail` doctest to the `CliHelpData` doc comment in `src/help.rs` (T-A08): a struct literal listing all 8 fields that fails to compile from outside the crate due to `#[non_exhaustive]`
12. Update `examples/basic_usage.rs`: convert `CliHelpData` literal to `CliHelpData::default()` + field assignment (`#[non_exhaustive]` blocks both exhaustive struct literals AND struct update syntax from external crates — E0639)
13. In `tests/help.rs`, first migrate ALL existing exhaustive `CliHelpData` struct literals (the `two_group_data()` helper and any inline literals in T01–T14) to `CliHelpData::default()` + field assignment; this migration must happen before IS-1 adds `#[non_exhaustive]` or all 14 existing tests fail to compile (note: struct update syntax is also blocked from external crates, so field assignment is the only valid pattern). Then add T-A01 through T-A07 and T-A09 (8 new unit tests).
14. Bump `version = "0.9.1"` → `"0.9.2"` in `Cargo.toml`
15. Update claude_runner TSK-232 task file per `## Downstream Dependency` instructions when this task reaches ✅ Complete

## Out of Scope

- Changes to `CliHelpStyle` or color configuration
- ANSI color variants for group headers
- `OptionGroup` builder API — plain struct construction is sufficient
- Removing the `options: Vec<OptionEntry>` field — it is preserved in the struct for backward compatibility; callers that set only `options` and leave `option_groups` empty are unaffected and still get the existing `"Options:"` section
- Publishing to crates.io — workspace path dep in claude_runner handles this
- Changes to `CommandGroup`, `ExampleEntry`, or `OptionEntry` types
- Item 15 (update downstream TSK-232 task file) writes outside this crate's source tree — this is a task-management notification authorized by the `## Downstream Dependency` section; no other out-of-crate edits are in scope

## Null Hypothesis

Without this change, `CliHelpTemplate` cannot represent multi-section named option groups, custom usage lines, or an arguments section. Claude_runner's `help.rs` stays on a hand-rolled 262-line printer that will diverge from the template's ANSI/width logic on every future help change, producing inconsistent output across subcommands.

## Work Procedure

1. Read `src/help.rs` in full; confirm: (a) `emit_header/groups/options/examples` are all methods on `impl CliHelpTemplate`, (b) all emit methods write to `out: &mut String` using `write!`/`writeln!`, (c) current `render()` order is `emit_header → emit_groups → emit_options → emit_examples`, (d) `OptionEntry` has fields `name: String` and `desc: String`
2. Read `tests/help.rs`; confirm 14 existing tests (T01–T14) and their naming convention; note that ALL existing `CliHelpData` struct literals are exhaustive (listing every field by name)
3. Read `examples/basic_usage.rs`; confirm `CliHelpData` struct literal location and field list
4. In `tests/help.rs` AND `examples/basic_usage.rs`, convert ALL existing exhaustive `CliHelpData` struct literals (the `two_group_data()` helper and any inline literals in T01–T14, plus the literal in `examples/basic_usage.rs`) to `CliHelpData::default()` + field assignment. This migration MUST be completed before step 5a adds `#[non_exhaustive]` to `CliHelpData`; without it, all existing literals outside the defining crate fail to compile. Note: struct update syntax (`..CliHelpData::default()`) is ALSO blocked from external crates by `#[non_exhaustive]` (E0639) — field assignment is the only valid external construction pattern. Doing examples/basic_usage.rs here (rather than in step 6) prevents a compile-broken window between steps 5a and 6.
5. In `src/help.rs`:
   a. Add `#[derive(Default, Debug, Clone)]` and `#[non_exhaustive]` above `pub struct CliHelpData`
   b. Add `pub struct OptionGroup { pub name: String, pub entries: Vec<OptionEntry> }` with `#[derive(Debug, Clone)]` before or after `CliHelpData`; then search for `pub mod prelude` in src/help.rs to locate the prelude block and add `pub use super::OptionGroup;` to its `pub use` exports alongside the other exported types so callers using `use cli_fmt::prelude::*` can construct `OptionGroup` without a separate import
   c. Add three new fields to `CliHelpData`: `pub usage_lines: Vec<String>`, `pub arguments: Vec<OptionEntry>`, `pub option_groups: Vec<OptionGroup>`
   d. In `impl CliHelpTemplate`, update `emit_header(...)`: replace ONLY the first `writeln!` (the `{bold}Usage:{rst} {} <command>` line) with a conditional: `if !self.data.usage_lines.is_empty() { for line in &self.data.usage_lines { writeln!(out, "  {line}").ok(); } } else { writeln!(out, "{bold}Usage:{rst} {} <command>", self.data.binary).ok(); }` — do NOT add `return;`; leave the four remaining writeln! calls (blank, tagline, blank, `{bold}Commands:{rst}`) unchanged so they emit in both paths
   e. In `impl CliHelpTemplate`, add `fn emit_arguments(&self, out: &mut String, bold: &str, opt: &str, rst: &str)`: guard empty; compute `max_len`; write `writeln!(out, "\n{bold}Arguments:{rst}").ok()` using `bold`/`rst` for the header, matching the convention of `emit_options()`; write each entry as `writeln!(out, "  {opt}{name:<max_len}{rst}  {desc}").ok()` (when TTY off, `bold` and `opt` are empty strings so the format degrades gracefully)
   f. In `impl CliHelpTemplate`, add `fn emit_option_group(&self, out: &mut String, bold: &str, opt: &str, rst: &str, name: &str, entries: &[OptionEntry])`: guard empty; compute `max_len` for this group only; write group header then entries
   g. In `impl CliHelpTemplate`, add `fn emit_option_groups(&self, out: &mut String, bold: &str, opt: &str, rst: &str)`: `for group in &self.data.option_groups { self.emit_option_group(out, bold, opt, rst, &group.name, &group.entries); }`
   h. In `impl CliHelpTemplate`, update `render()`: new sequence is `emit_header(...)` → `emit_arguments(...)` → `emit_groups(...)` → `emit_option_groups(...)` → `if self.data.option_groups.is_empty() && !self.data.options.is_empty() { self.emit_options(...); }` → `emit_examples(...)` — the inner `!self.data.options.is_empty()` guard from the existing render() must be preserved
   i. Add compile_fail doctest to `CliHelpData` doc comment:
      ```
      /// ```compile_fail
      /// let _d = cli_fmt::CliHelpData {
      ///     binary: "x".to_string(),
      ///     tagline: "y".to_string(),
      ///     groups: vec![],
      ///     options: vec![],
      ///     examples: vec![],
      ///     usage_lines: vec![],
      ///     arguments: vec![],
      ///     option_groups: vec![],
      /// };
      /// ```
      ```
6. Verify `examples/basic_usage.rs` uses `CliHelpData::default()` + field assignment (done in step 4; confirm here before proceeding; struct update syntax is blocked by E0639)
7. In `tests/help.rs`, add T-A01 through T-A07 and T-A09 after existing T14 (8 new tests; use `style.apply(false, false)` for non-TTY in all new tests); T-A01 requires two assertions: (a) non-empty `usage_lines` case asserts output contains `"  clr <command>"` and (b) default empty `usage_lines` case asserts output contains `"Usage: "` and the binary name — both may be covered in a single test function with two sub-assertions
8. Bump `version = "0.9.1"` → `"0.9.2"` in `Cargo.toml`
9. Run `cargo nextest run --all-features` — verify 22 tests pass (14 existing + 8 new)
10. Run `cargo test --doc --all-features` — verify T-A08 compile_fail doctest passes
11. Run `cargo clippy --all-targets --all-features -- -D warnings` — verify 0 warnings
12. Update claude_runner TSK-232 per `## Downstream Dependency` instructions

## Downstream Dependency

**cli_fmt TSK-005 is a blocking dependency of claude_runner TSK-232.**

When TSK-005 reaches ✅ Complete, the resolving agent MUST perform the following update to:

`/home/user1/pro/lib/wip_core/agent_kit/task/claude_runner/unverified/232_help_section_split_clihelp_template.md`

1. Remove the `Blocked By: cli_fmt TSK-005` line from `## Execution State`
2. Update Work Procedure step 2 in TSK-232: remove the `[BLOCKED]` label and populate the `option_groups[0]` and `option_groups[1]` entries with the exact `OptionEntry` literals from the now-stable API (replacing the `/* ... from IS-2 list */` comments with concrete code)
3. Add entry to `## History`: `- **[YYYY-MM-DD]** \`UNBLOCKED\` — cli_fmt TSK-005 complete; CliHelpTemplate has option_groups/usage_lines/arguments; help.rs rewrite executable.`

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|---|---|---|
| `usage_lines: vec!["clr <command>".into()]` (non-TTY) | T-A01 positive | output contains `"  clr <command>"` |
| `usage_lines: vec![]` default (non-TTY) | T-A01 fallback | output contains `"Usage: {binary} <command>"` |
| `arguments: vec![OptionEntry { name: "<MSG>".into(), desc: "Message to send".into() }]` (non-TTY) | T-A02 | output contains `"  <MSG>  Message to send"` (single entry, name width=5, 2-space separator between padded name and desc) |
| `arguments: vec![]` default | T-A02 empty | output does NOT contain `"Arguments:"` |
| `option_groups: vec![OptionGroup { name: "RUNNER OPTIONS".into(), entries: vec![OptionEntry { name: "--flag".into(), desc: "A flag".into() }] }]` (non-TTY) | T-A03 | output contains `"RUNNER OPTIONS:"` and `"  --flag  A flag"` |
| `option_groups: vec![]`, `options: vec![OptionEntry { name: "--opt".into(), desc: "desc".into() }]` (non-TTY) | T-A04 | output contains `"Options:"` and `"  --opt  desc"` (backward compat preserved) |
| `options: vec![OptionEntry { name: "--old".into(), desc: "old".into() }]` AND `option_groups: vec![OptionGroup { name: "NEW".into(), entries: vec![OptionEntry { name: "--new".into(), desc: "new".into() }] }]` (non-TTY) | T-A05 | output contains `"NEW:"` and `"  --new  new"`; does NOT contain `"--old"` and does NOT contain `"Options:"` |
| Group A: entries `OptionEntry { name: "--aa".into(), desc: "flag a".into() }` and `OptionEntry { name: "--bb".into(), desc: "flag b".into() }`; Group B: entry `OptionEntry { name: "--longer-name".into(), desc: "a long flag".into() }` (non-TTY) | T-A06 | output contains `"  --aa  flag a"` and `"  --bb  flag b"` (group A max_len=4, names are exactly 4 chars, 2-space separator); output contains `"  --longer-name  a long flag"` (group B max_len=13) — groups compute padding independently |
| `CliHelpData::default()` | T-A07 | Constructs without panic; `usage_lines`, `arguments`, `option_groups` are all empty |
| Exhaustive struct literal on `CliHelpData` from outside crate | T-A08 (compile_fail doctest in src/help.rs) | Fails to compile: `#[non_exhaustive]` enforced |
| `examples/basic_usage.rs` with `..CliHelpData::default()` spread | T-A09 | `cargo test --examples` passes without error |

## Related Documentation

- `src/help.rs` — implementation target (CliHelpData, impl CliHelpTemplate methods, T-A08 doctest)
- `tests/help.rs` — test target (T-A01–T-A07, T-A09)
- `examples/basic_usage.rs` — #[non_exhaustive] spread update
- `/home/user1/pro/lib/wip_core/agent_kit/task/claude_runner/unverified/232_help_section_split_clihelp_template.md` — downstream dependent (blocked on this task)

## History

- **[2026-06-21]** `CREATED` — Extend CliHelpData with multi-section option groups, custom usage lines, and arguments section; bump to 0.9.2.

## Verification Record

- **Verified:** 2026-06-21
- **D1 Scope Coherence:** PASS — In Scope itemized (15 items), Out of Scope meaningful (7 items), Observable enumerates specific fields, test counts, version string, and default construction behavior.
- **D2 MOST Goal Quality:** PASS — Motivated names structural gaps in CliHelpData 0.9.1; Observable names all six render() steps in correct order (emit_header → emit_arguments → emit_groups → emit_option_groups → conditional emit_options → emit_examples); Scoped lists exactly 5 files with explicit exclusions; Testable T-A01–T-A09 specify exact padding arithmetic and compile_fail verification.
- **D3 Value / YAGNI:** PASS — Null Hypothesis names structural impossibility; downstream TSK-232 at exact file path is the concrete committed need; Out of Scope correctly defers builder API, ANSI variants, and crates.io publish.
- **D4 Implementation Readiness:** PASS — WP step 4 identifies pre-condition ordering risk; WP step 5 broken into lettered sub-steps with exact method names, parameter signatures, and conditional logic; Test Matrix mirrors T-A01–T-A09 with literal OptionEntry structs and expected output strings.
