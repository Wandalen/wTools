# API Spec: Collection Constructor Macros

### Scope

- **Element:** `api/001_collection_macros`
- **Source:** `docs/api/001_collection_macros.md`
- **Prefix:** `AP-`
- **Minimum cases:** 4

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| AP-01 | strict_macros_accessible_from_root | accessibility | ⏳ |
| AP-02 | strict_macros_accessible_from_exposed | accessibility | ⏳ |
| AP-03 | into_macros_accessible_from_root | accessibility | ⏳ |
| AP-04 | into_macros_accessible_from_exposed | accessibility | ⏳ |
| AP-05 | aliases_equal_base_macros | alias | ⏳ |
| AP-06 | zero_element_valid_all_macros | boundary | ⏳ |
| AP-07 | trailing_comma_all_macros | syntax | ⏳ |
| AP-08 | type_reexports_full_std_api | reexport | ⏳ |
| AP-09 | features_independent | gate | ⏳ |
| AP-10 | expansion_contract_with_capacity_then_insert | contract | ⏳ |

---

### AP-01: all 9 strict macros accessible from crate root

- **Given:** Feature `collection_constructors` is enabled
- **When:** `collection_tools::vec!`, `::hmap!`, `::hset!`, `::bmap!`, `::bset!`, `::llist!`, `::deque!`, `::heap!`, `::dlist!` are each invoked with one or more elements
- **Then:** All 9 compile and return the correct collection type with the supplied elements

### AP-02: all strict macros accessible from exposed module

- **Given:** Feature `collection_constructors` is enabled
- **When:** `collection_tools::exposed::dlist!`, `::hmap!`, `::hset!`, `::bmap!`, `::bset!`, `::llist!`, `::deque!`, `::heap!` are invoked (the exposed module re-exports `vec` as `dlist`)
- **Then:** All exposed strict macros compile and return the correct collection type

### AP-03: all 9 into-macros accessible from crate root

- **Given:** Feature `collection_into_constructors` is enabled; explicit type annotations provided
- **When:** `collection_tools::into_vec!`, `::into_hmap!`, `::into_hset!`, `::into_bmap!`, `::into_bset!`, `::into_llist!`, `::into_vecd!`, `::into_heap!`, `::into_dlist!` are each invoked
- **Then:** All 9 compile and return the correct collection type

### AP-04: all into-macros accessible from exposed module

- **Given:** Feature `collection_into_constructors` is enabled; explicit type annotations provided
- **When:** The same 9 into-macros are invoked via `collection_tools::exposed::`
- **Then:** All 9 compile and return the correct collection type

### AP-05: alias macros produce identical result to base macros

- **Given:** Features `collection_constructors` and `collection_into_constructors` are enabled
- **When:** `vec![1, 2, 3]` is compared with `dlist![1, 2, 3]`; `into_vec!["a", "b"]` is compared with `into_dlist!["a", "b"]` (both with `Vec<String>` target annotation)
- **Then:** `vec_result == dlist_result`; `into_vec_result == into_dlist_result`

### AP-06: zero-element invocation valid for all 18 macros

- **Given:** Features `collection_constructors` and `collection_into_constructors` are enabled
- **When:** All 18 macros are invoked with zero elements; into-macros receive explicit type annotations
- **Then:** All compile without error; all return empty collections; `is_empty() == true` for each

### AP-07: trailing comma accepted in all 18 macros

- **Given:** Features `collection_constructors` and `collection_into_constructors` are enabled
- **When:** Each of the 18 macros is invoked with one or more elements and a trailing comma after the last element or pair
- **Then:** Each compiles without error; the result equals the same invocation without the trailing comma

### AP-08: re-exported types provide the full std API surface

- **Given:** Feature `enabled` is active; default configuration (std mode, no `use_alloc`)
- **When:** `collection_tools::HashMap` is used with `.insert()`, `.get()`, `.iter()`, `.remove()`, `.len()`, `.is_empty()`, and `.contains_key()`
- **Then:** All operations succeed; `collection_tools::HashMap` is a drop-in replacement for `std::collections::HashMap` with no wrapper types introduced

### AP-09: collection_constructors and collection_into_constructors are independent

- **Given:** Only `collection_constructors` is enabled (not `collection_into_constructors`)
- **When:** Strict macros (`vec!`, `hmap!`, etc.) are invoked; into-macros (`into_vec!`, etc.) are also referenced
- **Then:** All 9 strict macros compile and work; all 9 into-macros fail to compile; enabling one feature does not enable the other

### AP-10: expansion contract — with_capacity then insert for supported types

- **Given:** Feature `collection_constructors` is enabled; N = 3 elements for Vec
- **When:** `vec![1, 2, 3]` is constructed and its result compared to `{ let mut v = Vec::with_capacity(3); v.push(1); v.push(2); v.push(3); v }`
- **Then:** Results are equal; capacity values are identical; `BTreeMap`, `BTreeSet`, `LinkedList` macros use `new()` instead (those types have no `with_capacity` in std Rust)
- **Note:** `with_capacity` applies to Vec, HashMap, HashSet, VecDeque, BinaryHeap only; the 6 macros for BTreeMap/BTreeSet/LinkedList call `new()` and insert without pre-allocation
