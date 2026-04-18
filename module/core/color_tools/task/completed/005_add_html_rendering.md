# Add HTML rendering: `render_html()` and `Color::to_css()`

## Execution State

- **Executor Type:** any
- **Actor:** claude
- **Claimed At:** 2026-04-18
- **Status:** ✅ (Completed)

## Goal

Add `render_html()` to `DecoratedText` (gated by `html_support` feature) and `to_css()` to `Color` (gated same way), so that callers using `with_color_named` can produce browser-usable HTML output without writing an ANSI SGR parser. (Motivated: HTML output requires caller to implement ANSI parser; Observable: `DecoratedText::from("warn").with_color_named(Color::Yellow).render_html()` returns `<span style="color: yellow">warn</span>`; Scoped: cfg-gated additions only, no behavioral change to existing API; Testable: t29–t34 all pass under `--features "enabled,html_support"`)

## In Scope

- `src/decorated_text.rs` — add `render_html()` (cfg-gated), update `with_color_named` to store `named_color`
- `src/color.rs` — add `to_css()` in cfg-gated impl block
- `Cargo.toml` — `html_support = ["enabled"]` feature already present; verified
- `tests/decorated_text_test.rs` — add t29–t34 (TDD-first, all cfg-gated)
- `docs/feature/003_html_rendering.md` — new feature doc
- `docs/feature/readme.md` — add row for 003
- `docs/entities.md` — update feature count to 3
- `docs/doc_graph.yml` — add feature/003 node + 3 edges
- `docs/api/001_decorated_text_type.md` — add `render_html`, `named_color`, cross-ref to feature/003
- `docs/api/002_color_type.md` — add `to_css` row; correct CSS mapping for Bright variants

## Out of Scope

- ANSI SGR parser (`TryFrom<&str> for Color`) — future enhancement
- Non-HTML rendering targets (RTF, log color tags)
- `Color::Bold`, `Color::Italic`, `Color::Underline`

## Requirements

- TDD-first: t29–t34 written before implementation; compile error confirmed before implementing `render_html`
- Plain text (no named_color) returns HTML-escaped text with NO `<span>` wrapper
- `with_color(raw_str)` leaves `named_color` as `None`; `render_html()` returns plain escaped text
- `with_color_named(Color)` stores `Color` in `named_color` when `html_support` enabled
- HTML escaping: `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;`
- `Ansi256(n)` CSS: `var(--ansi256-n)` (caller defines the custom property)
- All 34 tests passing, 0 clippy errors

## Outcomes

Completed 2026-04-18. Phase Gate PASS — all checks green. `render_html()` added (cfg-gated). `to_css()` added to `Color`. t29–t34 all pass. `docs/feature/003_html_rendering.md` created. All documentation updated. L3 clean: 34 tests pass, 11 doc tests, 0 clippy errors.
