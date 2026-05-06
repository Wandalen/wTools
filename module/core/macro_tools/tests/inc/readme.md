# tests/inc/

Incremental unit tests organized by module functionality. Each test file validates specific module from `src/`.

## Organization

Tests are domain-based, mirroring source module structure. One test file per source module for focused validation.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Re-export test utilities and common imports |
| `attr_prop_test.rs` | Test attribute property parsing and manipulation |
| `attr_test.rs` | Test attribute extraction and analysis |
| `basic_test.rs` | Test basic macro utilities and foundations |
| `compile_time_test.rs` | Test compile-time string formatting utilities |
| `container_kind_test.rs` | Test container type detection and classification |
| `derive_test.rs` | Test derive macro utilities and parsing |
| `diag_test.rs` | Test diagnostic formatting and error reporting |
| `drop_test.rs` | Test drop implementation utilities |
| `equation_test.rs` | Test equation parsing and manipulation |
| `generic_args_test.rs` | Test generic argument extraction and processing |
| `generic_params_ref_refined_test.rs` | Test refined generic parameter reference operations |
| `generic_params_ref_test.rs` | Test generic parameter reference wrapper functionality |
| `generic_params_test.rs` | Test generic parameter parsing and decomposition |
| `ident_and_generic_params_test.rs` | Test combined identifier and generic parameter operations |
| `ident_cased_test.rs` | Test case-converted identifier operations |
| `ident_new_from_cased_str_test.rs` | Test identifier creation from cased strings |
| `ident_test.rs` | Test identifier extraction and manipulation |
| `item_struct_test.rs` | Test struct item parsing and analysis |
| `item_test.rs` | Test general item utilities and operations |
| `phantom_test.rs` | Test phantom type generation and manipulation |
| `quantifier_test.rs` | Test quantifier parsing and validation |
| `struct_like_test.rs` | Test struct-like item detection and processing |
| `tokens_test.rs` | Test token stream utilities and manipulation |
| `typ_test.rs` | Test type analysis and parameter extraction |
