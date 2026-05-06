# Feature: Attribute Parsing

### Scope
- **Purpose**: Provide structured, type-safe parsing of proc-macro attributes into typed property structs.
- **Responsibility**: Navigate all artifacts for the attribute parsing capability.
- **In Scope**: Attribute parsing, property types, attribute component traits, composable parsing via Assign.
- **Out of Scope**: Generic parameter handling → feature/003; type analysis → feature/002.

### Design
Attributes are parsed through a property-based system. Each attribute component declares its
keyword and implements parsing from a syn attribute node. Properties are typed — boolean,
optional-syn, singletone, and their optional variants — and composed via the Assign trait,
enabling field-by-field assignment without manual parser dispatch. The attribute struct receives
properties in any order; the type system dispatches each property to the correct field.
Span-aware errors are produced at the exact token position of malformed attribute content.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/attr.rs` | Core attribute parsing and AttributeComponent trait |
| source | `src/attr_prop.rs` | Property type module router |
| source | `src/attr_prop/boolean.rs` | Boolean attribute property |
| source | `src/attr_prop/boolean_optional.rs` | Optional boolean attribute property |
| source | `src/attr_prop/syn.rs` | Syn-typed attribute property |
| source | `src/attr_prop/syn_optional.rs` | Optional syn-typed attribute property |
| source | `src/attr_prop/singletone.rs` | Single-value attribute property |
| source | `src/attr_prop/singletone_optional.rs` | Optional single-value attribute property |
| source | `src/components.rs` | Component assignment integration |
| test | `tests/inc/attr_test.rs` | Attribute parsing correctness |
| test | `tests/inc/attr_prop_test.rs` | Property type correctness |
| doc | `docs/api/001_attribute_component_api.md` | AttributeComponent and AttributePropertyComponent trait contract |
| doc | `docs/api/002_assign_api.md` | Assign trait contract |
| doc | `docs/pattern/002_property_based_attributes.md` | Design pattern behind this system |
| doc | `docs/feature/002_type_analysis.md` | Type analysis capability — related but out of scope for attribute parsing |
| doc | `docs/feature/003_generic_parameters.md` | Generic parameter handling — related but out of scope for attribute parsing |
| doc | `docs/feature/004_syntax_tree_helpers.md` | Syntax tree helpers — related but out of scope for attribute parsing |
| doc | `docs/feature/005_error_diagnostics.md` | Error creation used for attribute parse failures |
| doc | `docs/feature/006_code_generation_support.md` | Code generation support — downstream consumer of parsed attributes |
