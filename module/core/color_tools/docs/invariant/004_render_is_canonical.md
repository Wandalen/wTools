# Invariant: Render Is Canonical

### Scope

- **Purpose**: Guarantee a single rendering path to prevent `Display` and `From<CT>` from diverging.
- **Responsibility**: Documents the render-is-canonical invariant and its enforcement in tests t06 and t07.
- **In Scope**: `Display::fmt` and `From<DecoratedText> for String` delegation to `render()`.
- **Out of Scope**: What `render()` produces (→ `invariant/002`); conversion semantics for input types (→ `invariant/001`).

### Abstract

All string-producing conversions route through `.render()`. Both `Display::fmt` and `From<DecoratedText> for String` delegate to `.render()`, guaranteeing a single canonical rendering path.

### Invariant Statement

- `impl Display for DecoratedText` calls `f.write_str(&self.render())`.
- `impl From<DecoratedText> for String` returns `ct.render()`.

No alternative string-conversion path exists. Any future trait impl that produces a `String` from `DecoratedText` must also delegate to `.render()`.

### Rationale

A single rendering path eliminates divergence risk. If `Display` and `From<CT> for String` used independent formatting logic, a bug fix in one path could leave the other broken. Centralizing through `.render()` means one fix propagates everywhere.

### Enforcement Mechanism

- Test `t06_from_ct_to_string_is_render` verifies `From<CT> for String` equals `.render()`.
- Test `t07_display_equals_render` verifies `Display` output equals `.render()`.

### Violation Consequences

If `Display` bypassed `.render()`, `format!("{ct}")` and `String::from(ct)` could produce different output for the same value — violating the principle of least surprise and causing subtle bugs in code that mixes both conversion styles.

### Cross-References

| Entity | File | Relationship |
|--------|------|-------------|
| feature/001 | [DecoratedText](../feature/001_decorated_text.md) | Parent feature |
| invariant/002 | [Render Reset Contract](002_render_reset_contract.md) | Sibling — render behavior |
| api/001 | [DecoratedText Type](../api/001_decorated_text_type.md) | Documents the trait impls |
