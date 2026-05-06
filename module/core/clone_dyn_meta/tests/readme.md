# clone_dyn_meta Tests

## Organization Principles

Tests focus on smoke validation ensuring the procedural macro crate compiles and loads correctly in both local and published build configurations. As a proc-macro crate, functional testing of code generation logic occurs through integration, not unit tests.

## Directory Structure

```
tests/
├── readme.md        # This file
├── manual/          # Manual testing plan and execution history
└── smoke_test.rs    # Smoke tests (local + published builds)
```

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Validate macro crate loads correctly for both local and published build configurations |
| `manual/` | Manual testing plan and exhaustive corner-case test execution history |

### Scope

**Responsibilities:**
Validates clone_dyn_meta procedural macro crate builds, compiles, and loads without errors in both local and published configurations. Ensures proc-macro entry point is accessible and macro infrastructure initializes correctly. Targets Rust stable on all platforms.

**In Scope:**
- Crate compilation in local mode (workspace dependencies)
- Crate compilation in published mode (crates.io dependencies)
- Procedural macro entry point accessibility
- Basic macro infrastructure initialization
- Smoke test execution for regression detection

**Out of Scope:**
- Macro expansion correctness (tested in clone_dyn facade crate integration tests)
- Code generation logic (tested through clone_dyn integration tests)
- Attribute parsing validation (tested indirectly via clone_dyn)
- Generic parameter handling (tested indirectly via clone_dyn)
- Performance benchmarks (would be in benches/)

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Smoke tests (local) | `smoke_test::local_smoke_test` | Macro crate loads with workspace dependencies |
| Smoke tests (published) | `smoke_test::published_smoke_test` | Macro crate loads with published dependencies |

## Adding New Tests

**Q: Testing macro expansion behavior?**
→ Add to clone_dyn facade crate tests (not here - this is proc-macro infrastructure)

**Q: Testing attribute parsing?**
→ Add to clone_dyn facade crate tests (end-to-end validation)

**Q: Testing generic parameter handling?**
→ Add to clone_dyn facade crate tests (integration testing)

**Q: Testing new smoke test variant (e.g., different feature combination)?**
→ Add new test function to `smoke_test.rs` following existing pattern

**Q: Adding compile-fail tests for macro errors?**
→ Create new file `compile_fail_test.rs` using trybuild crate pattern

## File Naming Conventions

- Test files use `snake_case.rs`
- Smoke tests: `smoke_test.rs` (standard pattern)
- Compile-fail tests: `compile_fail_test.rs` (if added)

## Special Considerations

- Procedural macro crates test differently than library crates
- Smoke tests verify crate loading, not functionality
- Functional testing occurs in clone_dyn facade crate
- Test execution time may be slow due to trybuild compilation
- Published smoke test validates crates.io dependency resolution
