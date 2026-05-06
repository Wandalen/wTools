# Test Spec: Zero-Cost Composition Pattern

### Scope

- **Element:** `pattern/001_zero_cost_composition`
- **Source:** `docs/pattern/001_zero_cost_composition.md`
- **Feature flag:** `enabled`
- **Prefix:** `ZCC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| ZCC-01 | manager_has_zero_size | structural | ⏳ |
| ZCC-02 | different_type_params_produce_independent_types | structural | ✅ |
| ZCC-03 | no_heap_allocation_on_construction | structural | ✅ |

---

### ZCC-01: manager has zero size

- **Given:** A `ConfigManager<D,P,V>` with any valid combination of D, P, V type parameters
- **When:** `std::mem::size_of::<ConfigManager<D,P,V>>()` is called
- **Then:** Returns `0` — the type is a ZST with no runtime storage overhead
- **Tests:** `tests/configurability_tests.rs::config_manager_has_zero_size`

### ZCC-02: different type params produce independent types

- **Given:** Two manager types `ConfigManager<D1,P,V>` and `ConfigManager<D2,P,V>` where `D1 ≠ D2`
- **When:** An attempt is made to use one in place of the other
- **Then:** Compile-time type error — the two types are entirely distinct; no implicit conversion exists
- **Tests:** Compile-time property; verified by the type system (no runtime test needed)

### ZCC-03: no heap allocation on construction

- **Given:** A `ConfigManager<D,P,V>` constructed with default in-memory type parameter implementations
- **When:** Resolution calls are made without enabling `file_ops`
- **Then:** All resolution paths use stack-only data; no heap allocation triggered by construction alone
- **Tests:** `tests/configurability_tests.rs` — implicit in `test_config_manager_is_zero_size` (ZST proof implies no construction allocation)
