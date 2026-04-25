# API: Assign

### Scope
- **Purpose**: Define the contract for component-based field assignment in attribute structs.
- **Responsibility**: Document the Assign trait and its role in composable attribute construction.
- **In Scope**: Generic type-directed assignment of typed components into attribute struct fields.
- **Out of Scope**: Specific property types → feature/001; attribute parsing entry points → api/001.

### Abstract
The Assign trait enables type-directed field assignment. An attribute struct implementing
Assign for a given marker type and value type gains a method that stores a value into the
field associated with that marker. This decouples parsing order from field identity: the
parser iterates tokens in any order and assigns each recognised property to its field by
type dispatch rather than positional or string-keyed logic.

### Operations
- Assign component: given a typed value, store it into the field associated with the value's type marker.
- Type dispatch: the type system selects the correct field at compile time — no runtime string matching or branching.

### Error Handling
Assignment is infallible. All validation occurs during property parsing, before assignment
is reached. A malformed property value returns an error from the parser; assignment itself
never fails.

### Compatibility Guarantees
Stable trait within a major version. Blanket or per-field implementations are provided by
consumer crates. Breaking changes require a major version bump of macro_tools.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/components.rs` | Assign trait definition and component integration |
| doc | `docs/feature/001_attribute_parsing.md` | Context in which Assign is used |
| doc | `docs/api/001_attribute_component_api.md` | Companion trait for attribute parsing |
| doc | `docs/pattern/002_property_based_attributes.md` | Pattern that motivates Assign |
