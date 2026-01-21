# Include_md Tests

## Organization Principles

Tests organized by functional domain (markdown file inclusion, section extraction, compilation features) rather than by methodology (unit, integration). Currently contains only smoke tests verifying basic compilation and package structure.

## Directory Structure

```
tests/
├── readme.md       # This file
└── smoke_test.rs   # Basic compilation and import verification
```

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Smoke testing | `smoke_test.rs` | Basic compilation, package structure, import capability |

## Adding New Tests

**Q: Testing new markdown inclusion functionality (e.g., include_md! macro)?**
→ Create new file `markdown_inclusion.rs` for markdown file inclusion tests

**Q: Testing section extraction (e.g., include_md_section! macro)?**
→ Create new file `section_extraction.rs` for section-based extraction tests

**Q: Testing error handling?**
→ Create new file `error_handling.rs` for error condition tests

**Q: Testing entirely new domain?**
→ 1. Create new file `[domain_name].rs`
   2. Update this readme.md with new domain entry
   3. Add to domain map table above

## File Naming Conventions

- Use lowercase snake_case: `markdown_inclusion.rs`, `section_extraction.rs`
- Name files by functional domain: what feature they test, not how they test it
- Avoid methodology names: don't use `unit_tests.rs` or `integration_tests.rs`
- Domain-based naming: `feature_name.rs` describes what's being tested

## Special Considerations

- **Placeholder Status:** This crate currently has no functionality implemented
- **Smoke Tests Only:** Current tests only verify compilation and structure
- **Future Implementation:** When macros are implemented, add corresponding test files
- **Test Data:** When testing markdown inclusion, test markdown files should go in `tests/assets/` directory
- **Feature Gates:** Heavy integration tests should use `#[cfg(feature = "integration")]` attribute
