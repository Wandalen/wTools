# Tests Directory

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify crate loads and smoke infrastructure functions |
| `tests.rs` | Aggregate all feature-gated test modules via `mod inc` |
| `experimental.rs` | Experimental test playground for development iterations |
| `integration_test.rs` | End-to-end integration tests for ComponentModel derive |
| `component_model_derive_test.rs` | Core derive macro behavior and generated code shape |
| `comprehensive_coverage_test.rs` | Comprehensive feature matrix coverage for all macros |
| `boolean_ambiguity_test.rs` | Prevent regression for boolean assignment type ambiguity |
| `boolean_fix_verification_test.rs` | Verify boolean fix via field-specific setter methods |
| `edge_cases_test.rs` | Edge cases and boundary conditions for derive macros |
| `error_handling_test.rs` | Error handling and validation error generation behavior |
| `debug_attribute_test.rs` | Debug attribute propagation and generated code |
| `enum_readme_examples_test.rs` | README enum example compilation and runtime behavior |
| `examples_documentation_test.rs` | Verify examples/readme.md matches actual example files |
| `popular_types_test.rs` | Popular type support (Duration, PathBuf) derive behavior |
| `minimal_boolean_error_test.rs` | Minimal reproduction of boolean error and its resolution |
| `inc/` | Shared test infrastructure and per-macro test implementations |

## Test Organization

All tests follow test_organization.rulebook.md standards:
- Tests organized by functional domain, not methodology
- Bug reproducer tests use Five-Section documentation format
- One aspect per test function; no compound assertions
- Explicit parameters, no reliance on default values
