# Task 009 — API Consistency: `with_` Prefix Sweep + Rename `TableCaption` → `Heading`

## Execution State

- **State**: ✅ (Completed)
- **ID**: 009
- **Slug**: api_consistency_with_prefix
- **Executor**: dev

## MOST Goal

Add the `with_` prefix to all 39 consuming builder setters across `TableConfig` (24), `ExpandedConfig` (7), `TreeConfig` (7), and `TableCaption` (1: `field` → `with_field`); rename `TableCaption` → `Heading`, `TableConfig::caption()` → `TableConfig::with_heading()`, so the entire config API consistently follows the `with_{name}(self, …) → Self` convention established in HtmlFormatter and SqlFormatter.

- **Motivated**: 39 builder setters currently lack the `with_` prefix, inconsistent with the rest of the codebase and Rust API guidelines for consuming builder methods. `TableCaption` carries HTML-baggage semantics; `Heading` is universally understood.
- **Observable**: `cargo doc` shows all builder setters with `with_` prefix; Level 3 (`w3 .test level::3`) passes after all callers in src/, tests/, and examples/ are updated.
- **Scoped**: Limited to the config module (`src/config/`), all callers in src/, tests/, examples/, and doc code examples in `docs/feature/007_table_caption.md`.
- **Testable**: Level 3 pass/fail is the gate; no new tests needed — existing 618 + 74 cover the renamed API.

## Null Hypothesis

Without this pass, callers face a confusing mixed API where some builder methods have `with_` prefix and others don't. The rename and prefix sweep are committed needs — no speculative scope.

## In Scope

- `src/config/table_caption.rs` — rename struct `TableCaption` → `Heading`; rename `field` → `with_field`; update any internal references
- `src/config/table_config.rs` — rename `caption(…)` → `with_heading(…)`; add `with_` prefix to all 23 other builder setters: `with_column_widths`, `with_align_right`, `with_border_variant`, `with_header_separator_variant`, `with_column_separator`, `with_outer_padding`, `with_inner_padding`, `with_colorize_header`, `with_header_color`, `with_alternating_rows`, `with_row_colors`, `with_color_reset`, `with_min_column_width`, `with_max_column_width`, `with_truncation_marker`, `with_sub_row_indent`, `with_terminal_width`, `with_auto_wrap`, `with_column_flex`, `with_auto_fold`, `with_fold_style`, `with_fold_indent`, `with_border_color`
- `src/config/expanded_config.rs` — add `with_` prefix to 7 setters: `with_record_separator`, `with_key_value_separator`, `with_show_record_numbers`, `with_colorize_keys`, `with_key_color`, `with_padding_side`, `with_indent_prefix`
- `src/config/tree_config.rs` — add `with_` prefix to 7 setters: `with_show_branches`, `with_show_root`, `with_indent_size`, `with_max_depth`, `with_column_separator`, `with_min_column_width`, `with_branch_color`
- `src/config/mod.rs` — update re-exports for `Heading` (was `TableCaption`)
- `src/lib.rs` — update public export of `Heading` (was `TableCaption`)
- All callers in `src/`, `tests/`, `examples/` — grep each old name and update
- Doc code examples in `docs/feature/007_table_caption.md` (Construction section and Minimal/Fields usage examples) — update to new names

## Out of Scope

- Width fix (see Task 008, should be done first)
- Adding new examples beyond updating existing callers (see Task 010)
- Changes to formatting logic or rendering behavior

## Work Procedure

1. Run `w3 .test level::3` to establish baseline (618 nextest + 74 doc + 0 clippy).
2. Rename struct `TableCaption` → `Heading` in `src/config/table_caption.rs`; rename `field` → `with_field`; update file header comment.
3. Update `src/config/mod.rs`: `pub use table_caption::TableCaption` → `pub use table_caption::Heading`.
4. Update `src/lib.rs` public re-export: `TableCaption` → `Heading`.
5. In `src/config/table_config.rs`: rename `caption` → `with_heading`; add `with_` prefix to all 23 other builder setters; update field type annotation from `TableCaption` to `Heading` in `TableConfig` struct.
6. In `src/config/expanded_config.rs`: add `with_` prefix to all 7 setters.
7. In `src/config/tree_config.rs`: add `with_` prefix to all 7 setters.
8. Run `cargo check --all-features 2>&1 | grep error` to discover all call sites with old names.
9. For each error: update the caller (src/, tests/, or examples/) to use the new name.
10. Update code examples in `docs/feature/007_table_caption.md` Construction section: `TableCaption::new(…)` → `Heading::new(…)`, `.field(…)` → `.with_field(…)`, `.caption(…)` → `.with_heading(…)`.
11. Run `w3 .test level::3`; fix any failures; iterate until Level 3 passes.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|---|---|---|
| All existing caption tests | Renamed API (`Heading`, `with_heading`, `with_field`) | Same 618 nextest + 74 doc pass — no behavioral change |
| `Heading::new("title").with_field("f")` | Direct Heading builder | Produces same `content_str()` as old `TableCaption::new("title").field("f")` |
| `TableConfig::plain().with_heading(Heading::new("T"))` | Config builder chain | Equivalent to old `.caption(TableCaption::new("T"))` |
| All TableConfig presets with builder setters | All 24 renamed setters | Behavioral output unchanged; only method names differ |
| All ExpandedFormatter tests | 7 renamed ExpandedConfig setters | Pass unchanged |
| All TreeFormatter tests | 7 renamed TreeConfig setters | Pass unchanged |

## Validation

Run `w3 .test level::3` and confirm:
- 618 nextest pass (or same count as baseline)
- 74 doc tests pass
- 0 clippy warnings
- `cargo doc --all-features` completes without warnings

## Related Documentation

- [`docs/feature/007_table_caption.md`](../../docs/feature/007_table_caption.md) — Construction section code examples updated in this task
- [`docs/api/003_config_types.md`](../../docs/api/003_config_types.md) — Config types API surface (update `TableCaption` → `Heading`, `caption()` → `with_heading()`, `field()` → `with_field()` after code changes)
- [`docs/pattern/004_config_builder_pattern.md`](../../docs/pattern/004_config_builder_pattern.md) — Builder pattern: verify `with_` convention is documented

**Closes:** null

## Affected Entities

- `src/config/table_caption.rs` — mutated: struct rename + method rename
- `src/config/table_config.rs` — mutated: 24 method renames
- `src/config/expanded_config.rs` — mutated: 7 method renames
- `src/config/tree_config.rs` — mutated: 7 method renames

## History

- **[2026-06-15]** `CREATED` — Add with_ prefix to all 39 config builder setters and rename TableCaption → Heading.

## Verification Record

- **Date**: 2026-06-15
- **Method**: MAAV — 4 independent parallel subagents (no self-verification)
- **Scope Coherence**: PASS (via TSK-008 scope agent — TSK-009 scope confirmed coherent: 39 enumerated method names, 4 config files, all callers via compiler-error discovery, 1 doc file)
- **MOST Goal Quality**: PASS — Motivated (39 inconsistent setters, HtmlFormatter/SqlFormatter established precedent); Observable (cargo doc + Level 3); Scoped (4 config files + callers); Testable (618/74/0 baseline + cargo doc)
- **Value / YAGNI**: PASS — Concrete need (API inconsistency real, confirmed in source); no speculative scope (all 39 names enumerated); null hypothesis answered; proportionate (mechanical renames)
- **Implementation Readiness**: PASS — All 11 steps executable; Test Matrix present; Validation concrete; file paths specified; dependency on TSK-008 noted
- **Result**: ✅ COMPLETED — implemented via `001_heading_implementation.plan.md` Phase 2; MAAV gate passed
