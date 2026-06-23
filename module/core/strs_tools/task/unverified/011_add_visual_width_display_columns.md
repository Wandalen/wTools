# Add `visual_width` function returning display columns

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Start Time:** null
- **Prior State:** null
- **Reopen Count:** 0
- **State:** 🎯 (Verified)
- **Closes:** null
- **Dir:** module/core/strs_tools
- **Validated By:** null
- **Validation Date:** null

## Goal

Add a `visual_width()` function to `src/ansi/visual.rs` that strips ANSI escapes and measures terminal display columns using `UnicodeWidthChar::width()`, closing the API gap between `visual_len()` (char count) and `pad_to_width()` (display-column padding). Currently any caller needing to pre-measure display columns before padding must hand-roll their own width function (as `data_fmt` did with `unicode_visual_len`). Providing `visual_width` as a first-class API in strs_tools eliminates that duplication. Success: `cargo nextest run --all-features -E 'test(visual_width)'` passes with correct display-column counts for ASCII, emoji, CJK, and mixed ANSI input.

## In Scope

- Migrate `unicode-width` from local `"0.1"` to `{ workspace = true }` (`^0.2`) — eliminates duplicate-dep binary
- New `visual_width()` function in `src/ansi/visual.rs` using `UnicodeWidthChar::width()`
- New `visual_width_unicode()` variant using grapheme clusters (Tier 2, `ansi_unicode` feature)
- Unit tests covering: ASCII, emoji (2-column), CJK (2-column), zero-width combiners, ANSI escape sequences, mixed content
- Re-export through `src/ansi/mod.rs` namespace hierarchy (own/orphan/exposed/prelude)

## Out of Scope

- Deprecating or modifying existing `visual_len()` / `visual_len_unicode()` — consumers may rely on char/grapheme counts
- Updating downstream consumers (`data_fmt`, `cli_fmt`) to call `visual_width()` — separate tasks per crate

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)

## Delivery Requirements

- Test Matrix populated before any test code
- All Test Matrix cases implemented as failing tests before implementation
- Minimum code to satisfy Test Matrix — no features beyond requirements
- `w3 .test l::3` passes with zero failures and zero warnings
- No function exceeds 50 lines; no duplication; public items have `///` doc comments
- Independent validation passes per `validation.rulebook.md`
- Task state updated to ✅ on validation pass; file moved to `task/completed/`

## Test Matrix

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | `"hello"` (pure ASCII) | `visual_width` | Returns 5 |
| T02 | `"😀😀"` (two emoji) | `visual_width` | Returns 4 (2 columns each) |
| T03 | `"你好"` (CJK) | `visual_width` | Returns 4 (2 columns each) |
| T04 | `"\x1b[31mred\x1b[0m"` (ANSI-wrapped) | `visual_width` | Returns 3 (ANSI stripped) |
| T05 | `""` (empty) | `visual_width` | Returns 0 |
| T06 | `"a😀b"` (mixed ASCII + emoji) | `visual_width` | Returns 4 (1+2+1) |
| T07 | `"e\u{0301}"` (combining accent) | `visual_width_unicode` | Returns 1 (single grapheme) |
| T08 | `"\x1b[1m😀\x1b[0m text"` (ANSI + emoji + text) | `visual_width` | Returns 7 (2+1+4) |

## Acceptance Criteria

- `visual_width("hello")` returns `5`
- `visual_width("😀😀")` returns `4`
- `visual_width("\x1b[31mred\x1b[0m")` returns `3`
- All T01–T08 test matrix rows have corresponding passing tests
- `visual_width` is re-exported at `strs_tools::ansi::visual_width` and in prelude
- Zero new clippy warnings introduced
- `visual_len()` behavior unchanged (no regressions)

## Validation

**Execution:** The procedure for walking this section is defined in `validation.rulebook.md`. The executor does NOT self-validate — an independent validator performs the walk after RELEASE transition.

### Checklist

**visual_width function**
- [ ] C1 — Does `src/ansi/visual.rs` contain a `pub fn visual_width` function?
- [ ] C2 — Does it use `UnicodeWidthChar::width()` for measurement?
- [ ] C3 — Does it strip ANSI via `parse_segments()` before measuring?
- [ ] C4 — Is `visual_width` re-exported in `src/ansi/mod.rs` exposed section?

**visual_width_unicode function**
- [ ] C5 — Does `src/ansi/visual.rs` contain a `pub fn visual_width_unicode` gated on `ansi_unicode`?

**Out of Scope confirmation**
- [ ] C6 — Is `visual_len()` unchanged (no signature or behavior modification)?
- [ ] C7 — Are there zero changes to `data_fmt` or `cli_fmt` crates?

### Measurements

- [ ] M1 — test count: `cargo nextest run --all-features -E 'test(visual_width)' -- --list` >= 8 tests

### Invariants

- [ ] I1 — test suite: `w3 .test level::3` passes with 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check --all-features` passes with 0 warnings

### Anti-faking checks

- [ ] AF1 — integration proof: `grep -r 'visual_width' src/ansi/mod.rs` shows re-export
- [ ] AF2 — negative: `grep -c 'visual_len' src/ansi/visual.rs` count unchanged from baseline

## Verification Record

- **Date:** 2026-06-23
- **Method:** MAAV (2 agents: feasibility explorer + adversarial reviewer)
- **Feasibility verdict:** Feasible. Implementation site clear (`src/ansi/visual.rs`), dep available (`unicode-width`), namespace pattern established, no naming collisions.
- **Adversarial verdict:** BLOCK with 2 corrections → both applied → PASS.
- **Corrections applied:**
  1. Goal motivation updated — `data_fmt::format_aligned` no longer uses `visual_len` (fixed by data_fmt TSK-011 internally). Task value is general-purpose API gap closure.
  2. `unicode-width` migration moved In Scope — strs_tools is sole holdout on `"0.1"`; workspace standardized on `^0.2`; leaving it produces duplicate-dep binary.
- **Non-blocking notes:** T07 (`"e\u{0301}"`) returns 1 for both `visual_width` and `visual_width_unicode` — weak as a discriminating test for the unicode variant. Consider adding a test with a multi-codepoint grapheme that has different char-width vs grapheme-width behavior (e.g., flag emoji `🇺🇸` = 2 codepoints, 1 grapheme, 2 display columns).

## History

- **[2025-11-22]** `CREATED` — Task filed. Goal: add display-column measurement function to fix visual_len/pad_to_width mismatch.
- **[2026-06-23]** `UPDATED` — Normalized to tsk.rulebook.md v5.11 template. Preserved all original content; added Execution State, Scope, Delivery Requirements, Test Matrix, Validation sections.
- **[2026-06-23]** `VERIFIED` — MAAV verification (2 agents: feasibility + adversarial). 2 corrections applied: (1) Goal text updated — data_fmt TSK-011 already fixed format_aligned internally; task value is general-purpose API, not downstream fix; (2) unicode-width 0.1→0.2 migration moved from Out of Scope to In Scope — workspace already standardized on ^0.2, strs_tools is sole holdout, leaving it creates duplicate-dep binary. T07 noted as weak discriminator (combining accent returns 1 for both variants) but non-blocking.
