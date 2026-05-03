# Tests: Inc Module

### Scope

- **Purpose**: Core integration and unit tests for the `program_tools` crate.
- **In Scope**: Builder API corner cases; runner execution integration; CLI binary integration; output predicate validation.
- **Out of Scope**: Manual testing plans (→ `manual/`); CLI test surface specs (→ `docs/`).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Register inc/ test modules for the suite entry point |
| `basic.rs` | Validate basic builder API with complete nested chain |
| `corner_cases_test.rs` | Validate corner cases for Source/Program/Plan/RunOptions/CapturedOutput |
| `cli_test.rs` | CLI integration tests for the `program_tools run` binary |
| `runner_test.rs` | Test runner execution paths and captured output assertions |
