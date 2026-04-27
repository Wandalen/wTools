# willbe2 Tests

## Organization Principles

Tests organized by functional domain (binary behavior, library re-exports) rather than by methodology (unit, integration). This crate is a pure alias/re-export crate, so tests focus on verifying the alias works correctly rather than testing core willbe functionality (which belongs in core willbe crate).

## Directory Structure

```
tests/
├── readme.md                 # This file
└── binary_smoke_test.rs     # Binary CLI behavior tests
```

### Scope

#### Responsibilities

Organizes automated tests for willbe2 alias crate functionality. Tests verify that willbe2 correctly re-exports willbe library API and delegates binary CLI commands to core willbe. Does NOT duplicate core willbe functionality tests - only verifies alias/delegation works. Targets Rust 1.70+ across Linux, macOS, Windows.

#### In Scope

- Binary CLI delegation tests (verify willbe2 binary delegates to core willbe)
- Re-export verification tests (verify willbe2::* exposes willbe::* correctly)
- Error propagation tests (verify errors from core willbe pass through correctly)
- Basic smoke tests (verify alias compiles and runs)

#### Out of Scope

- Core willbe functionality tests (belong in core willbe/tests/ directory)
- Duplicate tests of willbe command logic (anti-duplication principle)
- Performance/load testing (see benches/ directory if needed)
- Manual exploratory testing (see tests/manual/readme.md if created)

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Binary CLI delegation | `binary_smoke_test.rs` | willbe2 binary delegates commands to core willbe |

## Adding New Tests

**Q: Testing new willbe command through willbe2 binary?**
→ Add to `binary_smoke_test.rs` (binary delegation domain)

**Q: Testing willbe2 library API access?**
→ Add new test file for re-export verification if needed (currently no library tests)

**Q: Testing core willbe functionality?**
→ Add to core `willbe/tests/` directory (NOT here - anti-duplication principle)

## Test Anti-Duplication

**CRITICAL**: This is an alias crate. We MUST NOT duplicate core willbe tests here. Only test:
1. That alias delegation works (binary calls willbe::run correctly)
2. That re-exports work (willbe2::* exposes willbe::* correctly)

Testing actual command logic (e.g., what `.list` does) belongs in core willbe crate tests.

## Test History

- 2026-01-21: Removed `smoke_test.rs` placeholder tests (violated Loud Failures principle - tests always passed without verifying anything)
