# Compile-Fail Tests

Validation that invalid usage of `VariadicFrom` derive macro and `from!` macro produces expected compile-time errors.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `test_0_fields.rs` | Verify derive generates no code for 0-field structs |
| `test_0_fields.stderr` | Expected error output for 0-field struct usage |
| `test_4_fields.rs` | Verify derive generates no code for 4+ field structs |
| `test_4_fields.stderr` | Expected error output for 4-field struct usage |
| `test_from_macro_too_many_args.rs` | Verify from! macro rejects >3 arguments |
| `test_from_macro_too_many_args.stderr` | Expected compile error for >3 arguments |

## Test Methodology

These tests use `trybuild` to verify that invalid code produces expected compile errors, ensuring:
- Struct field count boundaries (0 and >3) are enforced
- `from!` macro argument limits are validated at compile time
- Error messages are clear and actionable

Validates specification requirements in `../../spec.md` § 3.1 and § 3.2.
