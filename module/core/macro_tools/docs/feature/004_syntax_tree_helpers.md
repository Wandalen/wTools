# Feature: Syntax Tree Helpers

### Scope
- **Purpose**: Provide structured access to common AST node types used in derive and attribute macros.
- **Responsibility**: Navigate all artifacts for struct/item parsing, identifier, name, and punctuation utilities.
- **In Scope**: Struct-like parsing, item utilities, identifier manipulation, name utilities, punctuated sequences, phantom type helpers.
- **Out of Scope**: Attribute parsing → feature/001; generic parameters → feature/003.

### Design
Struct-like parsing provides a unified view over struct and tuple struct syntax, allowing
derive macros to handle both forms with a single code path rather than separate branches.
Item utilities expose common transformations on top-level declaration nodes. Identifier
utilities handle case conversion (snake, camel, pascal), generation from string values,
and combination of multiple identifier fragments. Name and path utilities operate on
qualified name nodes. Punctuated sequence utilities simplify operations on comma-separated
or semicolon-separated lists without manual iterator management. Phantom type helpers
generate marker type tokens for parameters that appear only in type constraints
and not in stored fields, satisfying type parameterization requirements.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/item.rs` | Item-level utilities |
| source | `src/item_struct.rs` | Struct-specific item utilities |
| source | `src/struct_like.rs` | Unified struct and tuple-struct interface |
| source | `src/ident.rs` | Identifier manipulation and generation |
| source | `src/name.rs` | Name and path utilities |
| source | `src/punctuated.rs` | Punctuated sequence utilities |
| source | `src/phantom.rs` | PhantomData marker token generation |
| test | `tests/inc/item_test.rs` | Item utility correctness |
| test | `tests/inc/item_struct_test.rs` | Struct-specific item correctness |
| test | `tests/inc/struct_like_test.rs` | Unified struct-like interface correctness |
| test | `tests/inc/ident_test.rs` | Identifier operation correctness |
| test | `tests/inc/ident_cased_test.rs` | Case conversion correctness |
| test | `tests/inc/ident_and_generic_params_test.rs` | Identifier and generic parameter interaction |
| test | `tests/inc/ident_new_from_cased_str_test.rs` | Identifier construction from cased strings |
| test | `tests/inc/phantom_test.rs` | PhantomData token generation correctness |
| doc | `docs/feature/001_attribute_parsing.md` | Attribute parsing — primary capability using syntax tree helpers |
| doc | `docs/feature/002_type_analysis.md` | Type analysis — related capability |
| doc | `docs/feature/003_generic_parameters.md` | Generic parameter handling — related capability |
