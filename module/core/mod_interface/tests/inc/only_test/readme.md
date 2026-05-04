# tests/inc/only_test

## Scope

Shared assertion fragments included via include!() into both derive/ and manual/ test
modules. Each file contains only #[test] functions asserting specific namespace properties.
These files cannot compile standalone — they rely on the including module's namespace.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `layer_simple_only_test.rs` | Asserts all five namespace levels contain expected items from a two-layer hierarchy |
| `layer_single_only_test.rs` | Asserts single-layer exposure isolation across all namespace levels |
| `layer_have_mod_cfg_test_only.rs` | Asserts cfg-gated module presence and absence under feature flags |
| `micro_modules_only_test.rs` | Asserts micro-module items appear in correct exposure-level namespaces |
| `micro_modules_two_only_test.rs` | Asserts two-module composition — items from both modules in shared namespaces |
| `use_non_layer_only_test.rs` | Asserts use directive on non-layer items propagates correctly to target namespace |
