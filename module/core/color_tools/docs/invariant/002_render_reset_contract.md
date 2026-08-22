# Invariant: Render Reset Contract

### Scope

- **Purpose**: Guarantee that `render()` neither injects escape codes into plain text nor omits the reset from styled text.
- **Responsibility**: Documents the reset-only-when-active contract (active = colored, bold, dim, or any combination) and its enforcement in tests t04, t05, t09, t46-t49.
- **In Scope**: `render()` return value across `color: None`/`Some(...)` and `bold`/`dim` `true`/`false` states.
- **Out of Scope**: Conversion delegation (→ `invariant/004`); emptiness semantics (→ `invariant/003`).

### Abstract

`.render()` appends the ANSI reset sequence `"\x1b[0m"` if and only if the instance is **active** — `color` is `Some`, or `bold` is `true`, or `dim` is `true` (any combination). A fully inactive instance (`color: None, bold: false, dim: false`) is returned as a plain `text` clone with zero escape codes injected.

### Invariant Statement

Let `active = self.color.is_some() || self.bold || self.dim`.

When `active` is `true`, `.render()` builds a prefix in the fixed order **bold → dim → color** — each present component contributes its own separate SGR escape sequence (`"\x1b[1m"` for bold, `"\x1b[2m"` for dim, then the raw `color` string if present) — and returns `format!("{prefix}{}\x1b[0m", self.text)`.
When `active` is `false`, `.render()` returns `self.text.clone()` — no escape bytes appear.

Bold and dim are independent `bool` fields, not mutually exclusive at the type level, but are documented as mutually-exclusive in practice: real terminals typically honor only the last-applied intensity modifier (SGR 1 and SGR 2 both set/clear the same "intensity" state), so combining both is legal but rarely useful.

### Rationale

Consumers must be able to trust that a fully inactive `DecoratedText` round-trips through `.render()` without injecting invisible ANSI codes into log files, serialized output, or non-terminal sinks. The reset-only-when-active guarantee makes the type safe as a transparent `String` substitute regardless of which style dimension (color, bold, dim) is in use.

### Enforcement Mechanism

- Tests `t04_render_uncolored_no_escape` and `t05_render_colored_has_reset` in `tests/decorated_text_test.rs` verify the original color-only branches.
- Test `t09_chain_color_render` validates the exact byte sequence produced for the color-only case.
- Test `t46_with_bold_alone_renders_bold_prefix` verifies the bold-only branch; `t47_with_dim_alone_renders_dim_prefix` verifies the dim-only branch; `t48_bold_and_named_color_combine_as_two_sequences` verifies bold precedes color as two separate SGR sequences; `t49_no_decoration_at_all_returns_bare_text` is the regression guard for the fully-inactive case (no bold, no dim, no color).

### Violation Consequences

Injecting escape codes into uncolored renders corrupts plain-text consumers (log aggregators, CSV exporters, serialization pipelines). Omitting the reset from colored renders causes terminal color bleed into subsequent output.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [DecoratedText](../feature/001_decorated_text.md) | Parent feature |
| doc | [Transparent Conversion](001_transparent_conversion.md) | Sibling — conversion guarantee |
| doc | [Render Is Canonical](004_render_is_canonical.md) | Sibling — render delegation |
