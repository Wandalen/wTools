# Algorithm: Popular Type Generation

### Scope

- **Purpose**: Document how Assign implementations for standard library types (Duration, PathBuf) are generated in user crates without violating the orphan rule.
- **Responsibility**: Explains the string-matching approach, which types are recognized, and what implementations each produces.
- **In Scope**: The matching logic, the recognized type names, and the generated implementation shapes for Duration and PathBuf.
- **Out of Scope**: The Assign trait definition (→ `component_model_types/docs/api/001_assign_trait.md`); the orphan rule constraint (→ `component_model_types/docs/invariant/001_orphan_rule.md`).

### Abstract

When a derive macro expands in user code, both the macro crate and the standard library types are "foreign" to the user crate — but the generated code lands in the user crate, where the orphan rule is satisfied. This algorithm exploits that: by generating Assign implementations for recognized standard library types at derive expansion time, the impls appear in the user's crate and compile without error.

The recognition step uses the rendered string representation of each field's type rather than type identity. This is a deliberate trade-off: type identity requires full resolution at macro time (complex), while string matching on common short names is simple and covers the practical cases.

### Algorithm

1. For each field in the target struct, render the field type to its display string.
2. Compare the display string against a fixed recognition table of standard library type names.
3. For each match, emit a set of Assign implementations appropriate for that type.

**Recognition table:**

| Display string | Recognized as | Generated Assign impls |
|----------------|---------------|----------------------|
| `"Duration"` | `std::time::Duration` | From unsigned 64-bit integer (seconds), from 64-bit float (seconds), from (unsigned 64-bit integer, unsigned 32-bit integer) (secs + nanos) |
| `"PathBuf"` | `std::path::PathBuf` | From string reference, from owned string |

**Important limitation:** Recognition is by short name only — a user-defined type named `Duration` will also trigger the generated impls. This is a known limitation of the string-matching approach. It is documented and accepted: false positives are rare in practice, and the types triggering them are structurally compatible anyway.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/popular_types.rs` | Implementation of `generate_popular_type_assigns()` |
| source | `src/component/component_model.rs` | Call site — invoked during ComponentModel derive expansion |
| doc | [api/001_derive_macros.md](../api/001_derive_macros.md) | Derive macro that invokes this algorithm |
| doc | `component_model_types/docs/invariant/001_orphan_rule.md` | The constraint this algorithm works around |
