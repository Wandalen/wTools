# Feature: Enum Former

### Scope

- **Purpose**: Enables builder-style construction of enum values by generating constructor methods for each variant, dispatching to the appropriate construction strategy based on variant structure and attributes.
- **Responsibility**: Documents the enum builder derivation feature — constructor dispatch rules, supported variant forms, known constraints, and configuration surface.
- **In Scope**: Derive macro behavior on enum types, per-variant constructor generation, behavioral categories, and known limitations.
- **Out of Scope**: Struct builder derivation (→ feature/001_struct_former.md), the full rule table algorithm (→ algorithm/001_variant_constructor_logic.md), attribute contracts (→ api/).

### Design

Enum builder derivation extends the builder model to enum types. Each variant of the annotated enum produces a constructor method. The strategy for each constructor — direct (no arguments), scalar (all fields as arguments), or subformer (returns a nested builder) — is determined at compile time by a rule table that maps variant structural form and applied attribute to a constructor type.

Five behavioral categories organize all enum usage patterns:

- **Struct formers** — foundational struct-level building (not enum-specific)
- **Unit variant formers** — variants with no associated data; always produce direct constructors
- **Tuple variant formers** — variants with positional fields; constructor type driven by field count and attributes
- **Named variant formers** — variants with named fields; behave like struct formers within the variant
- **Complex scenario formers** — advanced combinations and multi-variant patterns

Constraints:
- Enum types with generic type parameters are not supported (→ invariant/002_no_generic_enums.md)
- Multi-variant enums may produce conflicting trait implementations (→ invariant/003_single_variant_enum.md)
- Both struct and enum types require all fields to use owned data (→ invariant/001_owned_types_only.md)

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Crate entry point re-exporting former_meta and former_types |
| doc | [algorithm/001_variant_constructor_logic.md](../algorithm/001_variant_constructor_logic.md) | Full 14-rule decision table for constructor type selection |
| doc | [api/001_item_attributes.md](../api/001_item_attributes.md) | Item-level attributes applicable to enum types |
| doc | [api/002_field_attributes.md](../api/002_field_attributes.md) | Variant-level attributes that drive constructor dispatch |
| doc | [pattern/001_builder_pattern.md](../pattern/001_builder_pattern.md) | Builder pattern structure |
| doc | [invariant/001_owned_types_only.md](../invariant/001_owned_types_only.md) | Constraint: all fields must use owned types |
| doc | [invariant/002_no_generic_enums.md](../invariant/002_no_generic_enums.md) | Constraint: generic type parameters in enums not supported |
| doc | [invariant/003_single_variant_enum.md](../invariant/003_single_variant_enum.md) | Constraint: multi-variant enums may produce trait conflicts |

### Sources

| File | Notes |
|------|-------|
| [../../spec.md](../../spec.md) | Primary source — section 2.1 Target Type Classification, 2.2 Enum Variant Constructor Logic, 2.1.3 Behavioral Categories; also produced 12 sibling instances — see entities.md |
| [../../limitations.md](../../limitations.md) | Combined source for all three enum-related invariants; also produced 3 sibling instances: invariant/001–003 |
| [../../advanced.md](../../advanced.md) | Combined source; enum former internals described in architectural sections; also produced 5 sibling instances: feature/001, feature/003–005, pattern/002 |
