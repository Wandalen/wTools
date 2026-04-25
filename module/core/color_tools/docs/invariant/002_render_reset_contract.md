# Invariant: Render Reset Contract

### Scope

- **Purpose**: Guarantee that `render()` neither injects escape codes into uncolored text nor omits the reset from colored text.
- **Responsibility**: Documents the reset-only-when-colored contract and its enforcement in tests t04, t05, and t09.
- **In Scope**: `render()` return value for both `color: None` and `color: Some(...)` states.
- **Out of Scope**: Conversion delegation (→ `invariant/004`); emptiness semantics (→ `invariant/003`).

### Abstract

`.render()` appends the ANSI reset sequence `"\x1b[0m"` if and only if a color prefix is attached (`color` is `Some`). Uncolored text is returned as a plain `text` clone with zero escape codes injected.

### Invariant Statement

When `self.color` is `Some(c)`, `.render()` returns `format!("{}{}\x1b[0m", c, self.text)`.
When `self.color` is `None`, `.render()` returns `self.text.clone()` — no escape bytes appear.

### Rationale

Consumers must be able to trust that uncolored `DecoratedText` round-trips through `.render()` without injecting invisible ANSI codes into log files, serialized output, or non-terminal sinks. The reset-only-when-colored guarantee makes the type safe as a transparent `String` substitute.

### Enforcement Mechanism

- Tests `t04_render_uncolored_no_escape` and `t05_render_colored_has_reset` in `tests/decorated_text_test.rs` verify both branches.
- Test `t09_chain_color_render` validates the exact byte sequence produced.

### Violation Consequences

Injecting escape codes into uncolored renders corrupts plain-text consumers (log aggregators, CSV exporters, serialization pipelines). Omitting the reset from colored renders causes terminal color bleed into subsequent output.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature/001 | [DecoratedText](../feature/001_decorated_text.md) | Parent feature |
| invariant/001 | [Transparent Conversion](001_transparent_conversion.md) | Sibling — conversion guarantee |
| invariant/004 | [Render Is Canonical](004_render_is_canonical.md) | Sibling — render delegation |
