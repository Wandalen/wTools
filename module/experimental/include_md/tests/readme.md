# Include_md Tests

## Organization Principles

Tests organized by functional domain (markdown file inclusion, section extraction, compilation features) rather than by methodology (unit, integration). Currently contains only smoke tests verifying basic compilation and package structure.

## Directory Structure

```
tests/
├── manual/
│   └── readme.md   # Manual testing plan
├── readme.md       # This file
└── smoke_test.rs   # Basic compilation and import verification
```

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Smoke testing | `smoke_test.rs` | Basic compilation, package structure, import capability |

