# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts of `color_tools` — properties that must hold across all invocations.
- **Responsibility**: Collect one doc instance per invariant; each instance owns statement, enforcement mechanism, and violation consequences.
- **In Scope**: Formal invariant statements, enforcement mechanisms, and violation impact.
- **Out of Scope**: Feature design rationale (→ `feature/`); API signatures (→ `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Transparent Conversion](001_transparent_conversion.md) | `From<T>` produces no ANSI side effects | ✅ |
| 002 | [Render Reset Contract](002_render_reset_contract.md) | Reset appended iff `color` is `Some` | ✅ |
| 003 | [Emptiness Semantics](003_emptiness_semantics.md) | `is_empty()` tests text field, not render output | ✅ |
| 004 | [Render Is Canonical](004_render_is_canonical.md) | All string conversions delegate to `render()` | ✅ |
