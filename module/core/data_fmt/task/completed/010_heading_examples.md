# Task 010 — Add Heading Feature Examples

## Execution State

- **State:** ✅ (Completed)
- **ID:** 010
- **Slug:** heading_examples
- **Executor:** dev

## MOST Goal

Add runnable example binaries demonstrating the `Heading` type (post-Task 009 rename) so callers have concrete reference code for title-only and title+fields usage across multiple table styles.

- **Motivated**: Zero examples currently demonstrate the heading/caption feature; callers discovering `Heading` have no runnable reference. Examples are required for the `E` criterion in TDCFREMS readiness.
- **Observable**: `cargo run --example heading_basic --features enabled` produces formatted table output with a visible heading rule line; Level 3 passes.
- **Scoped**: Limited to `examples/` directory (new files), `examples/readme.md`, and `Cargo.toml` `[[example]]` entries. No changes to src/ or tests/.
- **Testable**: Level 3 passes with 618 nextest + 74 doc + 0 clippy (same as post-Task 009 baseline — this task adds no new tests); `cargo run --example heading_basic --features enabled` exits 0 and prints a caption rule line.

**Dependency**: Task 009 must complete first (this task uses the renamed `Heading` API).

## Null Hypothesis

Without this task, the `E` criterion (examples) for the heading feature remains unmet, and callers have no runnable reference. The need is concrete: heading is a visible, user-facing feature with zero examples.

## In Scope

- `examples/heading_basic.rs` — demonstrates `Heading::new("title").with_field("count")` with `TableConfig::plain()`
- `examples/heading_styles.rs` — demonstrates heading across `grid()` and `unicode_box()` styles
- `examples/readme.md` — add two new rows to Responsibility Table
- `Cargo.toml` — add two `[[example]]` entries with `required-features = ["enabled"]`

## Out of Scope

- Changes to src/ or tests/
- Width fix (Task 008) or API rename (Task 009)
- Examples for other formatters (ExpandedFormatter, TreeFormatter)

## Work Procedure

1. Ensure Task 009 is complete (Heading API available).
2. Create `examples/heading_basic.rs`: import `Heading`, `TableConfig`, `TableFormatter`, `RowBuilder`, `Format`; build a small table with `TableConfig::plain().with_heading(Heading::new("Results").with_field("3 rows"))`; print the output.
3. Create `examples/heading_styles.rs`: demonstrate `Heading` with `grid()` and `unicode_box()` styles; print both outputs.
4. Open `examples/readme.md`; add rows for `heading_basic.rs` and `heading_styles.rs`.
5. Open `Cargo.toml`; add two `[[example]]` entries:
   ```toml
   [[example]]
   name = "heading_basic"
   required-features = ["enabled"]

   [[example]]
   name = "heading_styles"
   required-features = ["enabled"]
   ```
6. Run `RUSTFLAGS="-D warnings" cargo build --examples --all-features` to confirm examples compile.
7. Run `w3 .test level::3` to confirm Level 3 passes.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|---|---|---|
| Title-only heading; plain style | `TableConfig::plain().with_heading(Heading::new("Results"))` | Output starts with `─── Results ─…` then header row |
| Title + field heading; plain style | `.with_heading(Heading::new("Results").with_field("3 rows"))` | Output starts with `─── Results · 3 rows ─…` |
| Heading with grid style | `TableConfig::grid().with_heading(…)` | Heading before `+---+` top border |
| Heading with unicode_box style | `TableConfig::unicode_box().with_heading(…)` | Heading before `┌───┐` top border |

## Validation

Run `w3 .test level::3` and confirm:
- 618 nextest pass (this task adds no new tests; count unchanged from baseline)
- 74 doc tests pass
- 0 clippy warnings
- `cargo run --example heading_basic --features enabled` exits 0 and prints a caption rule line starting with `─── `

## Related Documentation

- [`docs/feature/007_table_caption.md`](../../docs/feature/007_table_caption.md) — heading feature spec (update Sources section to include new example files)
- [`examples/readme.md`](../../examples/readme.md) — updated with new entries in this task

**Closes:** null

## History

- **[2026-06-15]** `CREATED` — Add heading_basic and heading_styles example binaries demonstrating the Heading feature.

## Verification Record

- **Date**: 2026-06-15
- **Method**: MAAV — 4 independent parallel subagents (no self-verification)
- **Scope Coherence**: PASS — In Scope: 4 specific files; Out of Scope: src/, tests/, other formatter examples; observable end-state; tightest scope of the 3 tasks
- **MOST Goal Quality**: Initial finding: Testable bullet said "Same nextest count as baseline" (undefined). Fixed: now says "618 nextest + 74 doc + 0 clippy". Re-verified: PASS
- **Value / YAGNI**: PASS — Concrete need (E criterion unmet, zero examples, TDCFREMS blocks promotion to stable); no speculative scope (2 examples minimum); null hypothesis answered; proportionate
- **Implementation Readiness**: PASS — Steps executable; Test Matrix present (4 rows); Validation concrete with explicit counts + cargo run command; file paths specified; dependency on TSK-009 stated with "must" and explanation
- **Result**: ✅ COMPLETED — implemented via `001_heading_implementation.plan.md` Phase 3; MAAV gate passed
