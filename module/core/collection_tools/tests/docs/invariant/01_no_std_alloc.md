# Invariant Spec: No-std Allocation Selection

### Scope

- **Element:** `invariant/001_no_std_alloc`
- **Source:** `docs/invariant/001_no_std_alloc.md`
- **Prefix:** `IN-`
- **Minimum cases:** 2

**Implementation note:** IN-02 through IN-04 require `features = ["use_alloc"]`. The test file
`tests/no_std_alloc_test.rs` is entirely cfg-gated on `feature = "use_alloc"` and implements
those cases. IN-01 is covered by the standard test suite (default features).

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | std_config_hashmap_is_std | nominal | ⏳ |
| IN-02 | use_alloc_hashmap_is_hashbrown | no_std | ⏳ |
| IN-03 | use_alloc_hashset_is_hashbrown | no_std | ⏳ |
| IN-04 | use_alloc_other_types_from_alloc | no_std | ⏳ |

---

### IN-01: standard config HashMap resolves to std::collections::HashMap

- **Given:** Default features active; `collection_tools` compiled in std mode (no `use_alloc`)
- **When:** `let m: std::collections::HashMap<u32, u32> = collection_tools::HashMap::new()` is compiled
- **Then:** Compilation succeeds; `collection_tools::HashMap` and `std::collections::HashMap` are the same type; no coercion required

### IN-02: use_alloc config HashMap resolves to hashbrown::HashMap

- **Given:** Feature `use_alloc` is active (which also activates `no_std` and `hashbrown`)
- **When:** `let m: hashbrown::HashMap<u32, u32> = collection_tools::HashMap::new()` is compiled (via `collection_tools::dependency::hashbrown`)
- **Then:** Compilation succeeds; the direct assignment compiles without coercion; `collection_tools::HashMap` IS `hashbrown::HashMap` in this configuration

### IN-03: use_alloc config HashSet resolves to hashbrown::HashSet

- **Given:** Feature `use_alloc` is active
- **When:** `let s: hashbrown::HashSet<u32> = collection_tools::HashSet::new()` is compiled (via `collection_tools::dependency::hashbrown`)
- **Then:** Compilation succeeds; `collection_tools::HashSet` IS `hashbrown::HashSet` in this configuration

### IN-04: use_alloc config other collection types come from alloc

- **Given:** Feature `use_alloc` is active
- **When:** `collection_tools::Vec`, `BTreeMap`, `BTreeSet`, `LinkedList`, `VecDeque`, `BinaryHeap` are used with their normal operations
- **Then:** All compile and function correctly; types come from `alloc::collections`; no `std` reference required
