# Feature: Auto-Fit

### Scope

- **Purpose**: Automatically fit table output within terminal width using two cooperating strategies: cell auto-wrapping and column folding, both enabled by default with zero configuration.
- **Responsibility**: Document auto-fit rendering strategies, configuration, and interaction with other features.
- **In Scope**: Cell auto-wrapping, column folding, terminal width detection, progressive degradation, and feature interactions.
- **Out of Scope**: Algorithm details (see `../algorithm/`), API signatures (see `../api/`).

### Algorithms

| File | Relationship |
|------|-------------|
| [004_budget_allocation.md](../algorithm/004_budget_allocation.md) | Budget allocation algorithm |
| [005_column_fold_detection.md](../algorithm/005_column_fold_detection.md) | Column fold detection algorithm |

### APIs

| File | Relationship |
|------|-------------|
| [003_config_types.md](../api/003_config_types.md) | TableConfig field reference and builder API |

### Invariants

| File | Relationship |
|------|-------------|
| [003_auto_wrap_backward_compat.md](../invariant/003_auto_wrap_backward_compat.md) | auto_wrap(false) backward compatibility guarantee |
| [004_column_fold_invariants.md](../invariant/004_column_fold_invariants.md) | Fold behavioral invariants |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/table/mod.rs`](../../src/formatters/table/mod.rs) | `format_internal` orchestration; dispatches to auto_fit and rendering |
| [`src/formatters/table/auto_fit.rs`](../../src/formatters/table/auto_fit.rs) | Column classification, budget allocation, fold detection and rendering |
| [`src/formatters/table/rendering.rs`](../../src/formatters/table/rendering.rs) | Row and border rendering primitives used during fold output |
| [`src/config.rs`](../../src/config.rs) | ColumnFlex, FoldStyle enums; auto-fit TableConfig fields |
| [`src/wrap.rs`](../../src/wrap.rs) | WrapFormatter for cell wrapping |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/auto_wrap_test.rs`](../../tests/auto_wrap_test.rs) | Auto-wrap test suite T01–T23 (23 tests) |
| [`tests/auto_wrap_budget_test.rs`](../../tests/auto_wrap_budget_test.rs) | Budget allocation acceptance criteria BA AC-6–AC-8 + invariant WC IN-3 (13 tests) |
| [`tests/auto_fold_test.rs`](../../tests/auto_fold_test.rs) | Column folding test suite T01–T25 (25 tests) |
| [`tests/auto_fold_acceptance_test.rs`](../../tests/auto_fold_acceptance_test.rs) | Fold acceptance criteria CF AC-6–AC-8 + additional (7 tests) |

### Design

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

The auto-fit pipeline begins by resolving the effective terminal width. The `resolve_terminal_width()` method uses a four-tier fallback:

| Priority | Source | Condition | Example |
|----------|--------|-----------|---------|
| 0 | `terminal_width` config field | Caller sets `TableConfig::terminal_width( Some(80) )` | Fixed width for tests or embedded use |
| 1 | `$COLUMNS` environment variable | `COLUMNS` is set to a positive integer | CI/CD pipelines, scripts, non-TTY environments |
| 2 | `terminal_size` crate | Feature `terminal_size` enabled and stdout is a TTY | Runtime detection via the terminal size library |
| 3 | Hardcoded fallback | None of the above | 120 columns |

#### Tier 0 — Explicit Override

When `terminal_width` is `Some(w)`, that value is used directly. A value of `0` is clamped to `1` to prevent division-by-zero in budget allocation.

#### Tier 1 — `$COLUMNS` Environment Variable

When `terminal_width` is `None`, `resolve_terminal_width()` reads the `COLUMNS` environment variable. If it is set to a positive integer (e.g., `COLUMNS=80`), that value is returned immediately. Invalid, empty, or zero values are silently ignored and resolution falls through to Tier 2.

#### Tier 2 — Runtime Auto-Detection

Enable the `terminal_size` cargo feature to activate runtime detection. When the `terminal_size` cargo feature is enabled and `terminal_width` is `None`, the formatter queries `terminal_size::terminal_size()` at render time. This returns the actual terminal dimensions when stdout is connected to a TTY. Falls through to Tier 3 when:
- stdout is redirected to a file or pipe (not a TTY)
- the platform does not support terminal size queries

#### Tier 3 — Hardcoded Fallback

When none of Tier 0–2 produce a width, the formatter uses **120 columns** — a reasonable default for modern terminals.

#### When to Use Each Tier

| Scenario | Recommended Tier |
|----------|-----------------|
| Unit/integration tests | Tier 0: explicit width for deterministic output |
| CI/CD log output | Tier 1: set `COLUMNS=120` in pipeline env |
| Interactive CLI tools | Tier 2: enable `terminal_size` feature for responsive layout |
| Libraries producing strings | Tier 0 or Tier 3: caller decides width; no TTY assumption |

#### Configuration

All fields have sensible defaults — auto-fit works without any configuration.

| Field | Default | Status | Behavior |
|-------|---------|--------|----------|
| `terminal_width` | `None` (auto-detect) | ✅ | Target width for budget allocation |
| `auto_wrap` | `true` | ✅ | Enable Strategy 2 (cell wrapping at budget) |
| `column_flex` | `[]` (auto-classify) | ✅ | Per-column flex classification |
| `auto_fold` | `true` | ✅ | Enable Strategy 1 (column folding) |
| `fold_style` | `Labeled` | ✅ | Continuation line format |
| `fold_indent` | `"    "` (4 spaces) | ✅ | Indent prefix for continuation lines |

#### Disabling Auto-Fit

Auto-fit can be partially or fully disabled via `TableConfig` builder methods: set `auto_wrap(false)` to disable Strategy 2 only, `auto_fold(false)` to disable Strategy 1 only, or both to restore unlimited-width pre-auto-fit behavior.

#### Progressive Degradation

Strategy 2 (✅ implemented); Strategy 1 (✅ implemented).

| Condition | Strategy 2 | Strategy 1 | Result |
|-----------|-----------|-----------|--------|
| Fits naturally | not needed | not needed | Normal render |
| Tight but flex columns absorb | wraps flex cells | not needed | Taller rows |
| Still overflows after wrapping | wraps remaining | folds overflow cols | Continuation lines with wrapped values |
| Both disabled | — | — | Unlimited width (pre-auto-fit behavior) |

#### Interaction with Existing Features

- **Column truncation** (`max_column_width`): When `auto_wrap` is true and wrap overflow mode applies, wrapping takes precedence over truncation for flex columns. Fixed columns and columns with explicit truncate overflow still truncate.
- **Multiline cells**: Auto-wrapped cells produce multiline output via the same pipeline as manual `\n` cells.
- **Sub-row detail lines**: Detail lines are emitted after all row content lines (including wrapped lines) and after any folded continuation lines.
- **ANSI coloring**: Wrapped and folded lines respect the per-line color/reset algorithm (no ANSI bleed).
- **CSV/TSV**: Auto-fit is automatically disabled for `csv()` and `tsv()` presets (data formats must not wrap or fold).

#### See Also

- `table_formatting.md` — base table features (multiline cells, truncation, coloring, sub-rows)
- `../api/config_types.md § TableConfig` — field reference and builder API
- `word_wrap.md` — underlying WrapFormatter used by Strategy 2
