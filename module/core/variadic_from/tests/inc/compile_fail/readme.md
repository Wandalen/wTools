# Included Compile-Fail Tests

Modular compile-fail test cases included by parent compile-fail test aggregator.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `err_from_0_fields.rs` | Demonstrate FromN trait not implemented for 0-field structs |
| `err_from_4_fields.rs` | Demonstrate FromN trait not implemented for 4-field structs |
| `test_too_many_args.rs` | Verify from! macro compile error for excessive arguments |
| `test_too_many_args.stderr` | Expected error output for too many arguments |

### Purpose

These tests are included by `../compile_fail.rs` to provide modular compile-fail validation. They verify that the derive macro correctly generates no code for unsupported struct sizes, resulting in expected "trait not implemented" errors when attempting to use FromN traits.

Validates the field count boundary invariant documented in `../../../docs/invariant/001_field_count_boundary.md`.
