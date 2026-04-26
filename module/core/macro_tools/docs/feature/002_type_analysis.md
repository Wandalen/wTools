# Feature: Type Analysis

### Scope
- **Purpose**: Provide extraction and classification of type information within proc-macro ASTs.
- **Responsibility**: Navigate all artifacts for type parameter extraction and container detection.
- **In Scope**: Type parameter extraction by position, container kind detection, quantifier extraction.
- **Out of Scope**: Generic parameter manipulation → feature/003; identifier utilities → feature/004.

### Design
Type analysis operates on parsed type ASTs without re-parsing. Parameter extraction uses
positional range selection, allowing callers to extract specific generic arguments by index
range — for example, the first type argument of a two-argument generic. Container kind
detection identifies wrapping types such as optional wrappers, sequential lists, and
associative maps, enabling derive macros to treat contained vs. bare types differently in generated
code. Quantifier extraction identifies type expression modifiers such as reference or slice
wrappers around the underlying type.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/typ.rs` | Type parameter extraction |
| source | `src/container_kind.rs` | Container type kind detection |
| source | `src/quantifier.rs` | Type quantifier extraction |
| test | `tests/inc/typ_test.rs` | Type extraction correctness |
| test | `tests/inc/container_kind_test.rs` | Container detection correctness |
| test | `tests/inc/quantifier_test.rs` | Quantifier extraction correctness |
| doc | `docs/feature/001_attribute_parsing.md` | Primary macro_tools capability that uses type analysis |
| doc | `docs/feature/003_generic_parameters.md` | Related generic parameter manipulation |
| doc | `docs/feature/004_syntax_tree_helpers.md` | Syntax tree helpers providing identifier utilities |
