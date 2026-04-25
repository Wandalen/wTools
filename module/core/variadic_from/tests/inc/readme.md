# Included Test Modules

Modular test components included by parent test files for comprehensive `VariadicFrom` derive macro validation.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Export test modules for parent aggregators |
| `derive_test.rs` | Test VariadicFrom derive macro for all struct configurations |
| `compile_fail/` | Compile-fail test cases for invalid derive usage |

### Test Coverage

`derive_test.rs` provides comprehensive coverage of:
- Named structs (1, 2, 3 fields)
- Tuple structs (1, 2, 3 fields)
- Generic type parameters
- Identical vs. different field types
- Convenience implementations (From1, From2 for matching types)

All tests verify behavior documented in `../../docs/`.
