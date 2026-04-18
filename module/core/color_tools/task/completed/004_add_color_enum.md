# Add `Color` enum and `with_color_named` method

## Execution State

- **Executor Type:** any
- **Actor:** claude
- **Claimed At:** 2026-04-18
- **Status:** ✅ (Completed)

## Goal

Add a `Color` enum to `color_tools` with 18 variants (16 named 4-bit + `Ansi256` + `Rgb`) and a `to_ansi()` method that produces correct ANSI SGR sequences, plus a `with_color_named(Color)` method on `DecoratedText` that provides a typed alternative to `with_color(raw_str)` for callers who prefer semantic color references. (Motivated: callers must hand-craft `"\x1b[33m"` strings; Observable: `Color::Yellow.to_ansi()` returns `"\x1b[33m"` exactly; Scoped: `src/color.rs` new file + one new method in `src/decorated_text.rs`; Testable: t22–t28 all pass)

## In Scope

- `src/color.rs` — new file: `Color` enum + `to_ansi()` method
- `src/decorated_text.rs` — add `with_color_named(Color) -> Self` method, add `named_color` field (cfg-gated by `html_support`)
- `src/lib.rs` — add `mod color` and re-export `Color` through the mod_interface chain
- `src/readme.md` — add row for `color.rs`
- `tests/decorated_text_test.rs` — add t22–t28 (TDD-first)
- `docs/api/001_decorated_text_type.md` — add `with_color_named` and `Color Type` sections
- `docs/api/002_color_type.md` — new API doc for `Color`
- `docs/entities.md` — update api count to 2
- `docs/doc_graph.yml` — add `api/002` node + edges

## Out of Scope

- `html_support` feature and `render_html()` (Phase 3 / task 005)
- `Color::to_css()` (Phase 3)
- Any existing test modifications

## Requirements

- TDD-first: t22–t28 written before implementation; compile error confirmed before implementing `Color`
- `Color::Yellow.to_ansi()` MUST equal `"\x1b[33m"` exactly
- `with_color_named` delegates to `with_color` for terminal rendering
- `named_color` field placeholder added for Phase 3, cfg-gated by `html_support`
- 2-space indents, spaces inside brackets, no `cargo fmt`
- 28 total tests passing after implementation

## Outcomes

Completed 2026-04-18. Phase Gate PASS — all checks green. `src/color.rs` created (18 variants, `to_ansi()` method). `with_color_named` added to `DecoratedText`. `named_color` field added (cfg-gated). t22–t28 all pass. L3 clean: 28 tests pass, 0 clippy errors.
