# Test Spec: Invariant 001 — Propagation Rules

## Source

`docs/invariant/001_propagation_rules.md`

## Coverage Matrix

| ID | Claim | Test File | Test Name | Status |
|----|-------|-----------|-----------|--------|
| I01 | Rule 1: C's prelude items appear in P's prelude namespace | `../mod_interface_meta/tests/propagation_bug_test.rs` | `prelude_propagates_to_parent_prelude` | ✅ |
| I02 | Rule 2: C's exposed items appear in P's exposed namespace | `../mod_interface_meta/tests/propagation_bug_test.rs` | `exposed_propagates_to_parent_exposed` | ✅ |
| I03 | Rule 3: C's orphan items appear in P's own namespace ONLY | `../mod_interface_meta/tests/propagation_bug_test.rs` | `orphan_appears_in_parent_own_only` | ✅ |
| I04 | Rule 3 (negative): C's orphan items do NOT appear in P's orphan namespace | `../mod_interface_meta/tests/propagation_bug_test.rs` | `orphan_not_in_parent_orphan` | ✅ |
| I05 | Rule 4: C itself (module reference) appears in P's own namespace | `../mod_interface_meta/tests/integration_test.rs` | `child_module_in_parent_own` | ✅ |
| I06 | Rule 5: C's own items do NOT propagate to P at all | `../mod_interface_meta/tests/propagation_bug_test.rs` | `own_not_propagated_to_parent` | ✅ |
| I07 | `record_use_implicit` branches on `private_prefix_is_needed()` | `../mod_interface_meta/tests/corner_cases_test.rs` | `explicit_path_prefix_branch` | ✅ |
| I08 | Violation of rule 1/2 causes compile error in dependent code | `tests/tests.rs` via `tests/inc/derive/layer/` | Integration compile test | ✅ |
| I09 | Violation of rule 3 (orphan beyond own) does not occur | `../mod_interface_meta/tests/propagation_bug_test.rs` | `orphan_stops_at_immediate_parent` | ✅ |

## Notes

- Rules 1–5 map directly to the five numbered invariant statements in the source doc.
- Negative coverage (I04, I06, I09) requires compile-fail tests; currently covered by trybuild tests
  in `cta_trybuild_tests()` for bad-visibility cases, and by propagation_bug_test.rs for rule isolation.
- I07 is a structural/algorithmic claim about branching — covered by corner_cases_test.rs exercising
  paths with and without scope prefixes (super::, crate::).
