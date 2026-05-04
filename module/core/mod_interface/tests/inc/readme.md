# tests/inc

## Scope

Shared test infrastructure for the `mod_interface` test suite. Contains all inline test
modules organized by construction method (macro-generated vs hand-written) plus shared
assertion fragments and compile-time tests.

### Responsibility Table

| Entry | Responsibility |
|-------|----------------|
| `mod.rs` | Test harness root — assembles derive, manual, and trybuild sub-suites |
| `derive/` | Macro-generated tests — each subdirectory tests one mod_interface! directive form |
| `manual/` | Hand-written tests — baseline layer and namespace behavior without macro expansion |
| `only_test/` | Shared assertion fragments included via include!() into derive and manual tests |
| `trybuild_test.rs` | Compile-time pass and compile-fail tests via trybuild TestCases |
