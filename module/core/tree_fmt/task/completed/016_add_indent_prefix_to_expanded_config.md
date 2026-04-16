# Add `indent_prefix` to `ExpandedConfig`

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

Add an `indent_prefix: String` field to `ExpandedConfig` so that callers can indent every key-value line by a fixed prefix string (e.g. `"  "` for 2-space indent). This is needed by `gi_forge` to render `.repos.create` output as an indented key-value block under a header line, matching the format specified in `gi/docs/cli/output_format.md`. The field defaults to `""` — zero behavioral change for existing callers. Success is measured by all existing `tree_fmt` tests passing unchanged, plus new tests verifying the indent behavior.

## In Scope

- `src/config.rs` — add `indent_prefix: String` field to `ExpandedConfig`; set default to `""` in `Default` impl, `postgres_style()`, and `property_style()`; add `indent_prefix()` builder method
- `src/formatters/expanded.rs` — prepend `self.config.indent_prefix` before each key-value line (both `BeforeSeparator` and `AfterSeparator` branches); do NOT apply prefix to record separator lines
- `tests/` — add tests exercising `indent_prefix` behavior
- `task/readme.md` — update index with this task

## Out of Scope

- Indenting record separator lines (postgres `"-[ RECORD {} ]"` headers) — they remain flush-left
- Adding indent to other formatters (`TableFormatter`, `TreeFormatter`, `TextFormatter`)
- Publishing `tree_fmt` to crates.io (tracked separately)
- Changes to `gi_forge` or `gi` (covered by gi task 278)

## Description

`ExpandedFormatter` with `ExpandedConfig::property_style()` produces flush-left key-value output:

```
Visibility: private
CloneURL:   https://github.com/alice/my-repo.git
```

The `gi` CLI spec requires this output indented under a header:

```
alice/my-repo
  Visibility: private
  CloneURL:   https://github.com/alice/my-repo.git
```

The header line is printed separately by the caller, but the 2-space indent on key-value lines must come from the formatter. Today `ExpandedConfig` has no field for this. Callers would have to post-process the output string (split lines, prepend spaces, rejoin) which defeats the purpose of a configurable formatter.

Adding `indent_prefix: String` with a default of `""` is backward-compatible and follows the same pattern as existing config fields (`key_value_separator`, `record_separator`, `key_color`).

### Design details

- **Field:** `pub indent_prefix: String` — prepended verbatim to each key-value line
- **Default:** `""` (empty) — existing behavior unchanged
- **Presets:** both `postgres_style()` and `property_style()` set it to `""`
- **Builder:** `pub fn indent_prefix(mut self, prefix: String) -> Self`
- **Formatting:** in `expanded.rs`, insert `output.push_str(&self.config.indent_prefix)` at the top of the key-value loop body (before the `match self.config.padding_side` block), so it applies uniformly to both padding modes
- **Record separators:** NOT indented — the `indent_prefix` is only applied inside the `for cell in &row_node.children` loop, not before the record separator line

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Backward-compatible: empty prefix must produce byte-identical output to current behavior
- All existing `tree_fmt` tests must pass without modification

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note code style (2-space indent, no `cargo fmt`), builder method conventions (return `Self`, `#[must_use]`).

2. **Write Test Matrix** — populate every row before opening any source file.

3. **Write failing tests** — add test file or tests in existing expanded test file. Verify tests fail before implementation.

4. **Implement**:

   a. `src/config.rs` — add field to `ExpandedConfig` struct:
      ```rust
      /// Prefix string prepended to each key-value line (default: empty)
      pub indent_prefix : String,
      ```

   b. `src/config.rs` — add `indent_prefix : String::new()` to:
      - `Default` impl (line ~635)
      - `postgres_style()` (line ~656)
      - `property_style()` (line ~663)

   c. `src/config.rs` — add builder method:
      ```rust
      /// Set indent prefix prepended to each key-value line
      #[ must_use ]
      pub fn indent_prefix( mut self, prefix : String ) -> Self
      {
        self.indent_prefix = prefix;
        self
      }
      ```

   d. `src/formatters/expanded.rs` — in `format()`, add one line at the start of the key-value loop body (inside `for cell in &row_node.children`, before `let key = &cell.name;`):
      ```rust
      output.push_str( &self.config.indent_prefix );
      ```

5. **Green state** — all existing + new tests must pass: `RUSTFLAGS="-D warnings" cargo nextest run --all-features`

6. **Walk Validation Checklist** — every item YES.

7. **Update task status** — ✅ in `task/readme.md`, Priority=0, Advisability=0, move to `task/completed/`.

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---|---|---|
| T01 | Property style, default (no indent_prefix set) | `ExpandedConfig::property_style()` | Output identical to current — lines start at column 0 |
| T02 | Property style, `indent_prefix = "  "` | `.indent_prefix("  ".into())` | Every key-value line starts with `"  "` |
| T03 | Postgres style, `indent_prefix = "  "` | `ExpandedConfig::postgres_style().indent_prefix("  ".into())` | Key-value lines indented; record separator lines (`-[ RECORD 1 ]`) NOT indented |
| T04 | Property style, 2 records, `indent_prefix = "> "` | `.indent_prefix("> ".into())` | All key-value lines from both records start with `"> "` |
| T05 | Empty data (no rows), `indent_prefix = "  "` | `.indent_prefix("  ".into())` | Empty string output (no crash) |
| T06 | `indent_prefix = ""` explicitly set | `.indent_prefix(String::new())` | Identical to default — no prefix |
| T07 | Property style with colorize_keys + indent | `.colorize_keys(true).indent_prefix("  ".into())` | Indent appears before ANSI color code on each line |

## Validation Checklist

Desired answer for every question is YES.

**Config — field and builder**
- [ ] Does `ExpandedConfig` have a `pub indent_prefix: String` field?
- [ ] Does `Default` impl set `indent_prefix` to `""`?
- [ ] Does `postgres_style()` set `indent_prefix` to `""`?
- [ ] Does `property_style()` set `indent_prefix` to `""`?
- [ ] Does builder method `indent_prefix(self, prefix: String) -> Self` exist?
- [ ] Is the builder method `#[must_use]`?

**Formatter — prefix applied correctly**
- [ ] Does `expanded.rs` `format()` prepend `indent_prefix` before each key-value line?
- [ ] Is `indent_prefix` applied inside the `for cell in &row_node.children` loop (not outside)?
- [ ] Is `indent_prefix` NOT applied to record separator lines?

**Backward compatibility**
- [ ] Do all existing expanded tests pass without modification?
- [ ] Does `ExpandedConfig::default()` produce byte-identical output to the version before this change?

**Build and tests**
- [ ] Does `RUSTFLAGS="-D warnings" cargo nextest run --all-features` pass?
- [ ] Does `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` pass?
- [ ] Does `cargo clippy --all-targets --all-features -- -D warnings` pass?

## Validation Procedure

### Measurements

**M1 — `indent_prefix` field present**
```bash
grep -c "indent_prefix" ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/config.rs
```
Before: 0. Expected: ≥5 (field + 3 presets/default + builder). Deviation: not added if 0.

**M2 — `indent_prefix` wired in formatter**
```bash
grep -c "indent_prefix" ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/src/formatters/expanded.rs
```
Before: 0. Expected: ≥1. Deviation: not wired if 0.

**M3 — New tests present**
```bash
grep -rl "indent_prefix" ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt/tests/ | wc -l
```
Before: 0. Expected: ≥1. Deviation: no tests if 0.

**M4 — Full test suite green**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && RUSTFLAGS="-D warnings" cargo nextest run --all-features 2>&1 | tail -3
```
Expected: all tests passed. Deviation: any failure.

### Anti-faking checks

**AF1 — Backward compatibility (empty prefix = no change)**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && cargo test expanded 2>&1 | tail -5
```
Expected: all existing expanded tests pass unchanged. Any modification to existing test assertions is a violation.

**AF2 — Indent actually prepended (not silently ignored)**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && cargo test indent_prefix 2>&1 | grep -c "ok"
```
Expected: ≥1. Field exists but never tested is a violation.

**AF3 — Record separator NOT indented**
```bash
cd ~/pro/lib/wip_core/wtools/dev/module/core/tree_fmt && cargo test indent_prefix 2>&1 | grep -c "PASS\|test result: ok"
```
Expected: T03 specifically validates this. Absence of a postgres-style indent test is a violation.

## Outcomes

*(Completed. Task delivered and verified per acceptance criteria.)*
