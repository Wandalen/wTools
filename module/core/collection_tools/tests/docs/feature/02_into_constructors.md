# Feature Spec: Into Constructors

### Scope

- **Element:** `feature/002_into_constructors`
- **Source:** `docs/feature/002_into_constructors.md`
- **Feature flag:** `collection_into_constructors`
- **Prefix:** `FT-`
- **Minimum cases:** 4

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | heterogeneous_types_coerced | nominal | ⏳ |
| FT-02 | type_annotation_required_for_maps | annotation | ⏳ |
| FT-03 | into_dlist_alias_equals_into_vec | alias | ⏳ |
| FT-04 | empty_construction_all_nine_into_macros | boundary | ⏳ |
| FT-05 | into_vec_capacity_preallocated | capacity | ⏳ |
| FT-06 | feature_gate_independent_of_strict | gate | ⏳ |

---

### FT-01: heterogeneous types coerced via .into()

- **Given:** Feature `collection_into_constructors` is enabled; target type `Vec<String>` annotated on the binding
- **When:** `let v: Vec<String> = into_vec!["static", String::from("owned"), "another"]` is evaluated
- **Then:** Compilation succeeds; each `&str` literal is coerced to `String` via `.into()`; `v.len() == 3` and all elements are `String`

### FT-02: type annotation required for into map macros

- **Given:** Feature `collection_into_constructors` is enabled; no type annotation on the binding
- **When:** `let m = into_hmap!{"a" => 1}` is compiled without specifying `HashMap<K, V>`
- **Then:** Compilation fails with a type-inference error; the compiler cannot determine K and V without annotation

### FT-03: into_dlist! is a permanent alias for into_vec!

- **Given:** Feature `collection_into_constructors` is enabled; target type `Vec<String>` annotated on both bindings
- **When:** Both `into_vec!["a", "b"]` and `into_dlist!["a", "b"]` are evaluated with identical arguments and type annotation
- **Then:** Both produce equal `Vec<String>` values; `into_vec_result == into_dlist_result`

### FT-04: empty construction valid for all 9 into-macros

- **Given:** Feature `collection_into_constructors` is enabled; explicit type annotation provided for each binding
- **When:** Each of `into_vec!`, `into_hmap!`, `into_hset!`, `into_bmap!`, `into_bset!`, `into_llist!`, `into_vecd!`, `into_heap!`, `into_dlist!` is invoked with zero elements
- **Then:** Each produces an empty collection; `is_empty() == true` for all 9

### FT-05: into-macros pre-allocate capacity

- **Given:** Feature `collection_into_constructors` is enabled; target type `Vec<i32>` annotated; N = 3 elements
- **When:** `let v: Vec<i32> = into_vec![1, 2, 3]` is constructed
- **Then:** `v.capacity() >= 3`; `Vec::with_capacity(3)` is called before inserts, same contract as `vec!`

### FT-06: feature gate independent of collection_constructors

- **Given:** Crate compiled with only `collection_into_constructors` enabled (not `collection_constructors`)
- **When:** Into-macros (`into_vec!`, `into_hmap!`, etc.) are invoked; strict macros (`vec!`, `hmap!`, etc.) are referenced
- **Then:** All 9 into-macros compile and work correctly; all strict macros fail to compile; features are orthogonal
