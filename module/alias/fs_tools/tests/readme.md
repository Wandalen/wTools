# fs_tools Tests

Test suite for the fs_tools crate.

## Organization Principles

Tests are organized by functional domain (what is being tested) rather than by methodology (how it's tested).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Verify file_tools re-export API is accessible |
| `manual/` | Document manual testing procedures |

## Test Requirements

All tests in this suite MUST:
- Contain explicit assertions (no silent passing)
- Have comprehensive documentation explaining WHY the test exists
- Follow domain-based organization
- Include test matrix documentation in file-level comments

