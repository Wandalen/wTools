# Add `examples/basic.rs` — programmer-facing API demonstration

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** ✅ (Completed)

## Goal

Add `examples/basic.rs` covering the complete `DecoratedText` public API surface with runnable, readable snippets that demonstrate plain text, all three ANSI color encoding schemes (4-bit, 256-color, 24-bit true-color), query methods, and all conversion paths, so that new callers have a single executable reference for the entire API. (Motivated: no runnable API reference currently exists — `manual_color.rs` is a visual ANSI terminal verifier, not a learning reference; Observable: `examples/basic.rs` compiles and runs under `cargo run --example basic --features enabled` with zero warnings; Scoped: `examples/` directory only, no source or test changes; Testable: `cargo run --example basic --features enabled 2>&1 | grep "^error" | wc -l` returns `0`)

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/examples/basic.rs` — new example binary to create
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/examples/readme.md` — add row for `basic.rs`

## Out of Scope

- Documentation updates (already completed by doc_tsk)
- `examples/manual_color.rs` — existing visual verifier, unchanged
- `src/colorful_text.rs` — no source changes
- Any test files

## Description

`color_tools` currently has no programmer-facing example that teaches the API. `manual_color.rs` is a visual verifier — it produces ANSI output for human inspection in a real terminal, not readable code that a new caller can study to learn the API patterns.

`basic.rs` must be structured as a learning document: short named sections (plain construction, color encoding formats, query methods, conversion paths) each followed by an `assert!` or `assert_eq!` so it is also self-validating when run. The example must compile under `--features enabled` (which does not include `serde_support`), so no serde code. Serde usage belongs in `manual_color.rs`.

Current gap identified during the ANSI syntax and translation discussion: callers asking "how do I use 256-color or true-color?" have no concrete reference in this crate. `basic.rs` fills that gap with runnable code for all three encoding schemes documented in `docs/feature/001_decorated_text.md § Color Parameter Syntax`.

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Custom code style applies: 2-space indents, spaces inside angle brackets and function call parens, per `code_style.rulebook.md`
- The example must run successfully under `cargo run --example basic --features enabled`
- No `cargo fmt` — format manually per project style
- The example must cover: plain construction, colored construction, all three ANSI encoding schemes, `with_color`, `render`, `is_colored`, `is_empty`, `From<DecoratedText> for String`, `Display`
- Each section uses `assert!` / `assert_eq!` to be self-validating — no silent failures
- The example must compile with zero warnings under `RUSTFLAGS="-D warnings"`
- No serde usage (serde belongs in `manual_color.rs`, guarded by `serde_support` feature)

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note `code_style.rulebook.md` formatting constraints (2-space indent, spaces in brackets).
2. **Read doc source of truth** — Read `docs/feature/001_decorated_text.md` § Color Parameter Syntax for the three encoding schemes and their examples.
3. **Read existing example** — Read `examples/manual_color.rs` to understand established example style in this crate.
4. **Read examples/readme.md** — Understand current Responsibility Table before adding to it.
5. **Write `examples/basic.rs`** — Implement the 6 named sections listed in the Test Matrix below. Apply project code style. Each section ends with at least one assertion.
6. **Update `examples/readme.md`** — Add row `| \`basic.rs\` | Programmer-facing API reference example covering all encoding schemes and conversion paths |`.
7. **Compile and run** — Execute `RUSTFLAGS="-D warnings" cargo run --example basic --features enabled`. Must exit 0.
8. **Run L3 validation** — `w3 .test level::3` from the crate root. All tests must pass, zero warnings, zero clippy errors.
9. **Walk Validation Checklist** — check every item. Every answer must be YES.

## Test Matrix

| Section | API Demonstrated | Expected Outcome |
|---------|-----------------|------------------|
| S1 — Plain construction | `From<&str>`, `From<String>`, `color: None`, `render()` uncolored | `render()` equals raw text; no escape codes |
| S2 — 4-bit color | `with_color("\x1b[33m")`, `render()` colored | Output is `"\x1b[33mtext\x1b[0m"` |
| S3 — 256-color | `with_color("\x1b[38;5;208m")` | Output starts with `"\x1b[38;5;208m"` and ends with `"\x1b[0m"` |
| S4 — 24-bit true-color | `with_color("\x1b[38;2;255;165;0m")` | Output starts with `"\x1b[38;2;255;165;0m"` |
| S5 — Query methods | `is_colored()`, `is_empty()` | Correct boolean returns for plain, colored, empty, non-empty |
| S6 — Conversion paths | `String::from(ct)`, `format!("{ct}")`, `Display` | All three equal `ct.render()` |

## Acceptance Criteria

- `examples/basic.rs` exists and compiles under `cargo run --example basic --features enabled`
- The example runs to completion with exit code 0 (all internal assertions pass)
- `examples/readme.md` contains a row for `basic.rs`
- Zero compiler warnings under `RUSTFLAGS="-D warnings"`
- Zero clippy warnings under `cargo clippy --all-targets --all-features -- -D warnings`
- All 6 sections (S1–S6) are present in the example

## Validation

### Checklist

Desired answer for every question is YES.

**Compilation and runtime**
- [ ] Does `RUSTFLAGS="-D warnings" cargo run --example basic --features enabled` exit 0?
- [ ] Does the binary produce no panics?
- [ ] Is there zero output to stderr from the example itself?

**API coverage**
- [ ] Does S1 demonstrate plain construction and uncolored `render()`?
- [ ] Does S2 demonstrate 4-bit color encoding?
- [ ] Does S3 demonstrate 256-color encoding?
- [ ] Does S4 demonstrate 24-bit true-color encoding?
- [ ] Does S5 demonstrate `is_colored()` and `is_empty()` query methods?
- [ ] Does S6 demonstrate all three conversion paths (`String::from`, `format!("{ct}")`, `Display`)?

**Code style**
- [ ] Is the file formatted with 2-space indents?
- [ ] Are spaces present inside angle brackets and function call parens?
- [ ] Was `cargo fmt` NOT used?

**Registration**
- [ ] Does `examples/readme.md` contain a row for `basic.rs`?

**Out of Scope confirmation**
- [ ] Is `examples/manual_color.rs` unchanged?
- [ ] Are `src/`, `tests/`, and `docs/` directories unchanged?

### Measurements

**M1 — Compilation succeeds**
Command: `RUSTFLAGS="-D warnings" cargo run --example basic --features enabled 2>&1; echo "exit:$?"`
Before: file does not exist. Expected: last line `exit:0`. Deviation: non-zero exit code.

**M2 — L3 test suite clean**
Command: `w3 .test level::3 2>&1 | tail -3`
Before: 21 tests pass. Expected: 21+ tests pass, 0 warnings. Deviation: any failure or new warning.

### Anti-faking checks

**AF1 — Basic.rs exists on disk**
Check: `wc -l /home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/examples/basic.rs`
Expected: 40+ lines. Why: catches "created empty file to satisfy existence check" — a real example covering 6 sections needs substantial content.

**AF2 — All three ANSI encoding schemes present**
Check: `grep -c "38;5\|38;2\|\\\\x1b\[3[0-9]m" /home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/examples/basic.rs`
Expected: 3+. Why: confirms 4-bit, 256-color, and 24-bit are all exercised.

**AF3 — readme.md updated**
Check: `grep -c "basic.rs" /home/user1/pro/lib/wip_core/wtools/dev/module/core/color_tools/examples/readme.md`
Expected: 1. Why: confirms Responsibility Table was updated, not just the file created.

## Outcomes

Completed 2026-04-18. Phase Gate PASS — all 13 checks green. `examples/basic.rs` created (146 lines, 25 assertions, all 3 ANSI encoding schemes). `examples/readme.md` updated. `task/002` references updated from `ColorfulText` to `DecoratedText`. L3 clean: 21 tests pass, zero warnings.

