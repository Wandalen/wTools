# Feature: Auto-Fit

### Scope

- **Purpose**: Automatically fit table output within terminal width using two cooperating strategies: cell auto-wrapping and column folding, both enabled by default with zero configuration.
- **Responsibility**: Document auto-fit rendering strategies, configuration, and interaction with other features.
- **In Scope**: Cell auto-wrapping, column folding, terminal width detection, progressive degradation, and feature interactions.
- **Out of Scope**: Algorithm details (see `../algorithm/`), API signatures (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/table/mod.rs` | Auto-wrap budget allocation and fold rendering |
| source | `src/config.rs` | ColumnFlex, FoldStyle enums; auto-fit TableConfig fields |
| source | `src/wrap.rs` | WrapFormatter for cell wrapping |
| test | `tests/auto_wrap_test.rs` | Auto-wrap test suite (22 cases) |
| test | `tests/terminal_width_test.rs` | Terminal width three-tier fallback tests (to create) |
| test | `tests/auto_fold_test.rs` | Column folding test suite (22 tests) |
| doc | `../algorithm/004_budget_allocation.md` | Budget allocation algorithm |
| doc | `../algorithm/005_column_fold_detection.md` | Column fold detection algorithm |
| doc | `../invariant/003_auto_wrap_backward_compat.md` | auto_wrap(false) backward compatibility guarantee |
| doc | `../invariant/004_column_fold_invariants.md` | Fold behavioral invariants |
| doc | `../api/003_config_types.md` | TableConfig field reference and builder API |

### Design

#### Related Tasks

- [`task/019`](../../task/019_cell_auto_wrapping_with_budget_allocation.md) — Auto-wrap implementation task
- [`task/020`](../../task/020_column_folding_with_auto_fold.md) — Column folding implementation task
- [`task/021`](../../task/021_terminal_width_detection_tests.md) — Terminal detection test task

#### Motivation

CLI tables with long path or description columns overflow standard 80/120-column terminals. Before auto-fit, callers had three poor options: truncate (loses information), add a wide column (overflows), or manually pre-process strings. Auto-fit solves this at the formatter level.

#### Strategies

#### Strategy 2 — Cell Auto-Wrapping

When a column's content exceeds its allocated width budget, the cell wraps to multiple lines instead of truncating. Cell height grows; column width stays within budget.

```
ID  File                    Lines  Path
--  ----------------------  -----  ---------------------------
b1  governance.rulebook.md  120    /home/user1/pro/genai/
                                   governance/governance.
                                   rulebook.md
```

Cell wrapping reuses the existing multiline cell rendering pipeline. The `WrapFormatter` handles word-boundary splitting.

#### Strategy 1 — Column Folding

When total row width still exceeds terminal after wrapping, overflow columns fold to continuation lines below the row. Continuation lines use labeled format by default.

```
ID  File                    Lines  Rules
--  ----------------------  -----  -----
b1  governance.rulebook.md  120    23
    Path: /home/user1/pro/genai/governance/governance.rulebook.md
```

Column folding reuses the sub-row detail line infrastructure (see `table_formatting.md § Sub-Row Detail Lines`).

#### Combination

Both strategies compose: primary columns have budgeted widths with wrapping; overflow columns fold to continuation lines where folded values can themselves wrap.

```
ID  File                    Lines  Rules
--  ----------------------  -----  -----
b1  governance.rulebook.md  120    23
    Path: /home/user1/pro/genai/governance/
          governance.rulebook.md
```

#### Default Rendering Pipeline

Current pipeline (both strategies implemented):

1. Measure terminal width (auto-detect or use `terminal_width` override; fallback: 120)
2. Compute natural width of each column from content
3. If `sum(natural_widths) + separators ≤ terminal_width` — render normally (no intervention)
4. Classify columns via `column_flex` — `Fixed` columns keep natural width, `Flex` columns share remaining budget
5. Strategy 2 (✅): wrap flex cells that exceed their budget (cell height grows)
6. Strategy 1 (✅): If total still exceeds terminal — fold overflow columns to continuation lines
7. Render combined result

#### Column Classification

Each column is classified as `Fixed` or `Flex`:

| Classification | Behavior | Heuristic (when auto-classified) |
|---------------|----------|----------------------------------|
| `Fixed` | Keeps natural width; never wrapped or folded | Max cell width ≤ 12 display chars |
| `Flex` | Shrinks to budget; content wraps if needed | Max cell width > 12 display chars |

Auto-classification applies when `column_flex` is empty (default). Callers can override with explicit `ColumnFlex` assignments per column.

#### Terminal Width Detection

The auto-fit pipeline begins by resolving the effective terminal width. The `resolve_terminal_width()` method uses a three-tier fallback:

| Priority | Source | Condition | Example |
|----------|--------|-----------|---------|
| 1 | `terminal_width` config field | Caller sets `TableConfig::terminal_width( Some(80) )` | Fixed width for tests or embedded use |
| 2 | `terminal_size` crate | Feature `terminal_size` enabled and stdout is a TTY | Runtime detection via `terminal_size::terminal_size()` |
| 3 | Hardcoded fallback | Neither of the above | 120 columns |

#### Tier 1 — Explicit Override

```rust
let config = TableConfig::plain()
  .terminal_width( Some( 80 ) );
```

When `terminal_width` is `Some(w)`, that value is used directly. A value of `0` is clamped to `1` to prevent division-by-zero in budget allocation.

#### Tier 2 — Runtime Auto-Detection

```toml
# Cargo.toml
data_fmt = { version = "0.12", features = ["terminal_size"] }
```

When the `terminal_size` cargo feature is enabled and `terminal_width` is `None`, the formatter queries `terminal_size::terminal_size()` at render time. This returns the actual terminal dimensions when stdout is connected to a TTY. Falls through to Tier 3 when:
- stdout is redirected to a file or pipe (not a TTY)
- the platform doesn't support terminal size queries

#### Tier 3 — Hardcoded Fallback

When neither Tier 1 nor Tier 2 produces a width, the formatter uses **120 columns** — a reasonable default for modern terminals.

#### When to Use Each Tier

| Scenario | Recommended Tier |
|----------|-----------------|
| Unit/integration tests | Tier 1: explicit width for deterministic output |
| Interactive CLI tools | Tier 2: enable `terminal_size` feature for responsive layout |
| Libraries producing strings | Tier 1 or Tier 3: caller decides width; no TTY assumption |
| CI/CD log output | Tier 3: 120 column fallback is usually appropriate |

#### Configuration

All fields have sensible defaults — auto-fit works without any configuration.

| Field | Type | Default | Status | Behavior |
|-------|------|---------|--------|----------|
| `terminal_width` | `Option<usize>` | `None` (auto-detect) | ✅ | Target width for budget allocation |
| `auto_wrap` | `bool` | `true` | ✅ | Enable Strategy 2 (cell wrapping at budget) |
| `column_flex` | `Vec<ColumnFlex>` | `vec![]` (auto-classify) | ✅ | Per-column flex classification |
| `auto_fold` | `bool` | `true` | ✅ | Enable Strategy 1 (column folding) |
| `fold_style` | `FoldStyle` | `Labeled` | ✅ | Continuation line format |
| `fold_indent` | `String` | `"    "` (4 spaces) | ✅ | Indent prefix for continuation lines |

#### Disabling Auto-Fit

```rust
// Disable wrapping only (folding still active)
let config = TableConfig::plain()
  .auto_wrap( false );

// Disable only folding
let config = TableConfig::plain().auto_fold( false );

// Disable both
let config = TableConfig::plain().auto_wrap( false ).auto_fold( false );
```

#### Progressive Degradation

Strategy 2 (✅ implemented); Strategy 1 (✅ implemented).

| Condition | Strategy 2 | Strategy 1 | Result |
|-----------|-----------|-----------|--------|
| Fits naturally | not needed | not needed | Normal render |
| Tight but flex columns absorb | wraps flex cells | not needed | Taller rows |
| Still overflows after wrapping | wraps remaining | folds overflow cols | Continuation lines with wrapped values |
| Both disabled | — | — | Unlimited width (pre-auto-fit behavior) |

#### Interaction with Existing Features

- **Column truncation** (`max_column_width`): When `auto_wrap` is true and `ColumnOverflow::Wrap` applies, wrapping takes precedence over truncation for flex columns. Fixed columns and explicit `ColumnOverflow::Truncate` still truncate.
- **Multiline cells**: Auto-wrapped cells produce multiline output via the same pipeline as manual `\n` cells.
- **Sub-row detail lines**: Detail lines are emitted after all row content lines (including wrapped lines) and after any folded continuation lines.
- **ANSI coloring**: Wrapped and folded lines respect the per-line color/reset algorithm (no ANSI bleed).
- **CSV/TSV**: Auto-fit is automatically disabled for `csv()` and `tsv()` presets (data formats must not wrap or fold).

#### See Also

- `table_formatting.md` — base table features (multiline cells, truncation, coloring, sub-rows)
- `../api/config_types.md § TableConfig` — field reference and builder API
- `word_wrap.md` — underlying WrapFormatter used by Strategy 2
