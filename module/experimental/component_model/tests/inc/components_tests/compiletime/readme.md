# compiletime — Trybuild Compile-Time Fixtures

Source files passed to `trybuild` for compile-time correctness tests. Run under nightly only via the `components_trybuild` test in `tests/inc/mod.rs`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `components_component_from_debug.rs` | Verify ComponentFrom debug attribute compiles and emits expected output |
