# Invariant: Emptiness Semantics

### Scope

- **Purpose**: Guarantee that `is_empty()` tests the text field, not render output, enabling formatters to skip empty rows.
- **Responsibility**: Documents the emptiness-semantics invariant and its enforcement in tests t13 and t15.
- **In Scope**: `is_empty()` method semantics; formatter-skip behavior.
- **Out of Scope**: Rendering details (→ `invariant/002`).

### Abstract

`.is_empty()` tests `self.text.is_empty()` — the raw text field — not `self.render().is_empty()`. A colored empty text (e.g. `from("").with_color(...)`) is still considered empty because no visible content will be displayed.

### Invariant Statement

`DecoratedText::is_empty()` returns `self.text.is_empty()`. The presence or absence of a color prefix does not affect the emptiness result.

### Rationale

Formatters use `is_empty()` to decide whether to render a detail row. An empty text with a color attached carries no visible payload — displaying `"\x1b[33m\x1b[0m"` wastes vertical space. By testing the text field only, the formatter can skip empty details regardless of color state.

### Enforcement Mechanism

- Test `t13_is_empty_checks_text` in `tests/decorated_text_test.rs` verifies empty-uncolored, empty-colored, and non-empty cases.
- Test `t15_render_empty_colored_text` documents the render-vs-isEmpty design boundary.

### Violation Consequences

If `is_empty()` tested `render()` output, colored-but-empty text would report non-empty (because render produces `color+reset` bytes), causing formatters to emit blank colored rows — wasting vertical space and confusing users.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [DecoratedText](../feature/001_decorated_text.md) | Parent feature |
| doc | [Render Reset Contract](002_render_reset_contract.md) | Sibling — render behavior |
