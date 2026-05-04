# tests/inc/manual

## Scope

Hand-written test modules verifying layer and namespace behavior without using the
mod_interface! macro. These serve as ground-truth baselines: if the macro output matches
the hand-written form, the propagation semantics are correct.

### Responsibility Table

| Directory | Responsibility |
|-----------|----------------|
| `layer/` | Two-layer hand-written hierarchy baseline for layer directive tests |
| `micro_modules/` | Four-exposure-level hand-written modules baseline for micro_modules tests |
| `micro_modules_two/` | Two-module four-exposure baseline for multi-module composition tests |
| `use_layer/` | Two-layer hand-written cross-wiring baseline for use_layer directive tests |
