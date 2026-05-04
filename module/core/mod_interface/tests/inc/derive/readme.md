# tests/inc/derive

## Scope

Macro-generated test modules — each subdirectory exercises one directive form or
behavioral scenario of the mod_interface! macro. Compile-fail cases verify that invalid
inputs are rejected at compile time.

### Responsibility Table

| Directory | Responsibility |
|-----------|----------------|
| `attr_debug/` | Tests #![debug] inner attribute: macro expansion emitted as compiler message |
| `layer/` | Tests `layer child_module` directive with two-layer hierarchy propagation |
| `layer_bad_vis/` | Compile-fail: `layer` directive rejects invalid visibility keyword |
| `layer_have_layer/` | Tests parent layer containing a child layer with full cascade |
| `layer_have_layer_cfg/` | Tests cfg-gated child layer wired into a parent layer |
| `layer_have_layer_separate_use/` | Tests `layer` + separate `use super::child` on the same module |
| `layer_have_layer_separate_use_two/` | Tests `layer` + two separate `use super::child` directives on same module |
| `layer_have_mod_cfg/` | Tests cfg-gated module declaration inside a layer body |
| `layer_unknown_vis/` | Compile-fail: `layer` directive rejects unknown visibility keyword |
| `layer_use_cfg/` | Tests cfg-gated `use` item assignments inside a layer |
| `layer_use_macro/` | Tests macro symbols in layer body (macro re-export limitation; see task/backlog/004) |
| `micro_modules/` | Tests `micro mod_name` directive across all four exposure levels |
| `micro_modules_bad_vis/` | Compile-fail: micro-module rejects invalid visibility keyword |
| `micro_modules_glob/` | Tests `own use { * }` glob directive pulling all private items into own namespace |
| `micro_modules_two/` | Tests two independent micro-modules each in their own layer |
| `micro_modules_two_joined/` | Tests two micro-modules sharing the same layer namespace |
| `micro_modules_unknown_vis/` | Compile-fail: micro-module rejects unknown visibility keyword |
| `reuse_basic/` | Tests `reuse child` directive propagating a struct type from child module |
| `use_as/` | Tests `exposed use Symbol as Alias` rename form on layer assignment |
| `use_bad_vis/` | Compile-fail: `use` directive rejects invalid visibility keyword |
| `use_basic/` | Tests basic `exposed use Symbol` item assignment to a named layer |
| `use_layer/` | Tests `use super::child` cross-module layer wiring directive |
| `use_private_layers/` | Placeholder: `priv use` private-visibility directive (unimplemented; see task/backlog/003) |
| `use_unknown_vis/` | Compile-fail: `use` directive rejects unknown visibility keyword |
