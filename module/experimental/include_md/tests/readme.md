# Include_md Tests

## Organization Principles

Tests organized by functional domain (markdown file inclusion, section extraction, compilation features) rather than by methodology (unit, integration).

## Directory Structure

| File | Responsibility |
|------|----------------|
| `docs/` | Test spec files organized by doc entity type (feature, invariant, api) |
| `manual/` | Manual testing plan for structural and build verification |
| `readme.md` | Test organization, domain map, and directory structure |
| `smoke_test.rs` | Basic compilation and package structure verification |
| `file_inclusion.rs` | Tests for `include_md!` macro: valid file, compile-fail cases |
| `section_extraction.rs` | Tests for `include_md_section!` macro: extraction, compile-fail cases |
| `fixture/` | Fixture files used by file_inclusion and section_extraction tests |

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Smoke testing | `smoke_test.rs` | Basic compilation, package structure, import capability |
| File inclusion | `file_inclusion.rs` | `include_md!` positive test, compile-fail for missing/wrong-arity |
| Section extraction | `section_extraction.rs` | `include_md_section!` positive tests, compile-fail for all error conditions |

