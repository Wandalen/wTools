# file_tools Tests

Test suite for the file_tools crate.

## Organization Principles

Tests are organized by functional domain (what is being tested) rather than by methodology (how it's tested). As this crate evolves, tests should be grouped by the file manipulation functionality they verify (e.g., file reading, writing, path operations) rather than by test type (unit, integration).

## Directory Structure

```
tests/
├── readme.md          # This file
└── smoke_test.rs      # Basic API accessibility tests
```

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| API Surface | `smoke_test.rs` | Module imports, basic function accessibility |

## Adding New Tests

**Q: Testing basic API accessibility?**
→ Add to `smoke_test.rs`

**Q: Testing new file manipulation functionality?**
→ Create new file named after the functionality domain (e.g., `file_read.rs`, `path_operations.rs`)
→ Update this readme.md with new domain entry

**Q: Testing entirely new domain?**
→ 1. Create new file `<domain_name>.rs`
→ 2. Update this readme.md with new domain entry
→ 3. Add to domain map table above

## File Naming Conventions

- Use lowercase_snake_case
- Name files after functional domain (what they test)
- Avoid methodology-based names (no `unit_tests.rs` or `integration_tests.rs`)
- Use descriptive names (e.g., `file_operations.rs` not `test.rs`)

## Test Requirements

All tests in this suite MUST:
- Contain explicit assertions (no silent passing)
- Have comprehensive documentation explaining WHY the test exists
- Follow domain-based organization
- Include test matrix documentation in file-level comments
- Be under 1500 lines per file (target 750-1000 lines)

## Current Test Coverage

- ✅ Basic smoke tests (API accessibility)
- ⏳ File manipulation utilities (pending implementation)
