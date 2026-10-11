# Support

### Scope

Helpers shared by the integration tests in `tests/`: each test file declares `mod support;` and uses part of it. Nothing here is a test.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Temporary packages, real fixture crates, bounded builds, shared assertions |
