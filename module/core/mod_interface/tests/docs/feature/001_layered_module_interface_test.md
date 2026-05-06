# Test Spec: Feature 001 — Layered Module Interface

## Source

`docs/feature/001_layered_module_interface.md`

## Coverage Matrix

| ID | Claim | Test File | Test Name | Status |
|----|-------|-----------|-----------|--------|
| F01 | Five layers exist: private, own, orphan, exposed, prelude | `tests/tests.rs` via `tests/inc/derive/layer/` | `trybuild_tests → layer/trybuild.rs` | ✅ |
| F02 | Four named namespace modules generated (own, orphan, exposed, prelude) | `tests/tests.rs` via `tests/inc/derive/layer/` | `trybuild_tests → layer/trybuild.rs` | ✅ |
| F03 | Prelude item accessible from all four generated namespaces | `../mod_interface_meta/tests/integration_test.rs` | `prelude_accessible_everywhere` | ✅ |
| F04 | Orphan item accessible in immediate parent's own and root; not in orphan+ | `../mod_interface_meta/tests/propagation_bug_test.rs` | `orphan_stops_at_own` | ✅ |
| F05 | Own item not propagated to parent at all | `../mod_interface_meta/tests/propagation_bug_test.rs` | `own_not_propagated` | ✅ |
| F06 | Layer wiring directive integrates a child submodule into parent cascade | `tests/tests.rs` via `tests/inc/derive/layer_have_layer/` | `trybuild_tests → layer_have_layer/trybuild.rs` | ✅ |
| F07 | Micro-module directive loads external file into named layer namespace | `tests/tests.rs` via `tests/inc/derive/micro_modules/` | `trybuild_tests → micro_modules/trybuild.rs` | ✅ |
| F08 | `#![debug]` directive compiles without error | `tests/tests.rs` via `tests/inc/derive/attr_debug/` | `trybuild_tests → attr_debug/trybuild.rs` | ✅ |
| F09 | No-std compatibility: generated code uses no std runtime features | `tests/smoke_test.rs` | `smoke_test` | ✅ |
| F10 | Bootstrap constraint: mod_interface itself uses conventional org | `src/lib.rs` (inspection) | N/A — structural invariant | ✅ |
| F11 | Optional rename form on layer assignment directive | `tests/tests.rs` via `tests/inc/derive/use_as/` | `trybuild_tests → use_as/trybuild.rs` | ✅ |
| F12 | Multiple directives allowed in single invocation body | `tests/tests.rs` via `tests/inc/derive/layer_have_layer_separate_use_two/` | `trybuild_tests → layer_have_layer_separate_use_two/trybuild.rs` | ✅ |

## Notes

- F10 is a structural/architectural claim verified by reading `src/lib.rs` — no runtime test possible.
- All trybuild pass tests compile and link the generated expansion; they verify the namespace modules exist and are accessible.
