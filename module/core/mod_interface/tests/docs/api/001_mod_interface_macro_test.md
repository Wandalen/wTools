# Test Spec: API 001 — mod_interface Macro DSL

## Source

`docs/api/001_mod_interface_macro.md`

## Coverage Matrix

| ID | Claim | Test File | Test Name | Status |
|----|-------|-----------|-----------|--------|
| A01 | Layer assignment to `own`: item accessible only within this module | `tests/tests.rs` via `tests/inc/derive/use_layer/` | `trybuild_tests → use_layer/trybuild.rs` | ✅ |
| A02 | Layer assignment to `orphan`: item propagated to immediate parent's own and root | `../mod_interface_meta/tests/propagation_bug_test.rs` | `orphan_layer_assignment` | ✅ |
| A03 | Layer assignment to `exposed`: item propagated to all ancestor exposed namespaces | `../mod_interface_meta/tests/propagation_bug_test.rs` | `exposed_layer_assignment` | ✅ |
| A04 | Layer assignment to `prelude`: item propagated to all ancestor prelude namespaces | `../mod_interface_meta/tests/integration_test.rs` | `prelude_layer_assignment` | ✅ |
| A05 | Optional rename form supported on any layer assignment directive | `tests/tests.rs` via `tests/inc/derive/use_as/` | `trybuild_tests → use_as/trybuild.rs` | ✅ |
| A06 | Layer wiring directive wires child's orphan→parent's own, exposed→exposed, prelude→prelude | `tests/tests.rs` via `tests/inc/derive/layer_have_layer/` | `trybuild_tests → layer_have_layer/trybuild.rs` | ✅ |
| A07 | Micro-module directive loads external file into named layer namespace | `tests/tests.rs` via `tests/inc/derive/micro_modules/` | `trybuild_tests → micro_modules/trybuild.rs` | ✅ |
| A08 | Debug directive compiles without error and emits expansion message | `tests/tests.rs` via `tests/inc/derive/attr_debug/` | `trybuild_tests → attr_debug/trybuild.rs` | ✅ |
| A09 | Multiple directives in single invocation body compile correctly | `tests/tests.rs` via `tests/inc/derive/layer_have_layer_separate_use_two/` | `trybuild_tests → layer_have_layer_separate_use_two/trybuild.rs` | ✅ |
| A10 | Malformed directive syntax produces compile error | `tests/tests.rs` via `tests/inc/derive/layer_bad_vis/` | `cta_trybuild_tests → layer_bad_vis/trybuild.rs` | ✅ |
| A11 | Unknown visibility keyword produces compile error | `tests/tests.rs` via `tests/inc/derive/use_unknown_vis/` | `cta_trybuild_tests → use_unknown_vis/trybuild.rs` | ✅ |
| A12 | Breaking DSL changes require major version bump | N/A — governance policy, not testable | Changelog / version history | N/A |
| A13 | `mod_interface` and `mod_interface_meta` versioned in lockstep | `Cargo.toml` (inspection) | N/A — structural invariant | ✅ |

## Notes

- A10 and A11 are compile-fail tests run in `cta_trybuild_tests()` (nightly, terminal module only).
- A12 is a process/governance claim — not executable as an automated test.
- A13 is verified by inspecting both crates' `Cargo.toml` version fields; no runtime test needed.
