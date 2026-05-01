# Tests Directory

This directory contains test files for the component_model_meta crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify crate compiles and basic functionality works (local + published) |
| `component_from_duplicate_types_bug.rs` | Reproduce and document issue-001: ComponentFrom with duplicate field types |
| `component_from_generic_types_bug.rs` | Reproduce and document issue-002: ComponentFrom with generic type parameters |
| `assign_duplicate_types_bug.rs` | Reproduce and document issue-003: Assign with duplicate field types |
| `component_model_tuple_struct_limitation.rs` | Reproduce and document issue-004: ComponentModel with tuple structs |
| `manual_examples_comprehensive.rs` | Verify all documented examples from src/lib.rs compile and execute |
| `ui/` | Trybuild UI tests for compile-error diagnostic verification |
| `manual/` | Manual testing artifacts and temporary testing plans |

## Test Organization

All tests follow test_organization.rulebook.md standards:
- Bug reproducer tests have Five-Section documentation (Root Cause, Why Not Caught, Fix Applied, Prevention, Pitfall)
- Bug reproducers marked with `// test_kind: bug_reproducer(issue-NNN)`
- Smoke tests verify basic compilation and functionality
