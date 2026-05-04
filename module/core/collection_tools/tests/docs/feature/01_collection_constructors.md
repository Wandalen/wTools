# Feature Spec: Collection Constructors

### Scope

- **Element:** `feature/001_collection_constructors`
- **Source:** `docs/feature/001_collection_constructors.md`
- **Feature flag:** `collection_constructors`
- **Prefix:** `FT-`
- **Minimum cases:** 4

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | type_inference_no_annotation | nominal | ⏳ |
| FT-02 | all_nine_strict_macros_correct_type | nominal | ⏳ |
| FT-03 | vec_capacity_exactly_n | capacity | ⏳ |
| FT-04 | dlist_alias_equals_vec | alias | ⏳ |
| FT-05 | empty_construction_all_types | boundary | ⏳ |
| FT-06 | trailing_comma_all_nine_macros | syntax | ⏳ |
| FT-07 | feature_gate_enforced | gate | ⏳ |

---

### FT-01: type inference without annotation

- **Given:** Feature `collection_constructors` is enabled; no explicit type annotation on the binding
- **When:** `let v = collection_tools::vec![1, 2, 3]` is compiled
- **Then:** Compilation succeeds; `v` has type `Vec<i32>` inferred from element literals; `v.len() == 3`

### FT-02: all 9 strict macros construct correct type

- **Given:** Feature `collection_constructors` is enabled; one or more elements supplied to each macro
- **When:** Each of `vec!`, `hmap!`, `hset!`, `bmap!`, `bset!`, `llist!`, `deque!`, `heap!`, `dlist!` is invoked
- **Then:** Each macro returns its declared collection type with all supplied elements; `len() == N` for every macro

### FT-03: Vec capacity equals element count exactly at construction

- **Given:** Feature `collection_constructors` is enabled; N = 5 elements
- **When:** `vec![1, 2, 3, 4, 5]` is constructed
- **Then:** `v.capacity() == 5` exactly; `Vec::with_capacity(5)` is called before inserts; no reallocation occurs during construction
- **Note:** Only Vec guarantees exact capacity; HashMap/HashSet may exceed N due to load-factor rounding

### FT-04: dlist! is a permanent alias for vec!

- **Given:** Feature `collection_constructors` is enabled; identical arguments supplied to both macros
- **When:** Both `collection_tools::vec![1, 2, 3]` and `collection_tools::dlist![1, 2, 3]` are evaluated
- **Then:** Both produce equal `Vec<i32>` values; `vec_result == dlist_result`

### FT-05: empty construction valid for all 9 macros

- **Given:** Feature `collection_constructors` is enabled
- **When:** Each macro is invoked with zero elements: `vec![]`, `hmap!{}`, `hset!{}`, `bmap!{}`, `bset!{}`, `llist!{}`, `deque!{}`, `heap!{}`, `dlist![]`
- **Then:** Each produces an empty collection; `len() == 0` and `is_empty() == true` for all 9

### FT-06: trailing comma accepted in all 9 macros

- **Given:** Feature `collection_constructors` is enabled; trailing comma appended to the last argument or pair
- **When:** Each of the 9 macros is invoked with a trailing comma: `vec![1, 2,]`, `hmap!{1 => 10,}`, `hset!{1,}`, `bmap!{1 => 10,}`, `bset!{1,}`, `llist!{1,}`, `deque!{1,}`, `heap!{1,}`, `dlist![1,]`
- **Then:** Each compiles without error; the result equals the same invocation without the trailing comma

### FT-07: feature gate enforced at compile time

- **Given:** Crate compiled without `collection_constructors` feature (e.g., `--no-default-features`)
- **When:** Any strict macro such as `collection_tools::vec![1]` is referenced in code
- **Then:** Compilation fails; no strict macro is accessible from the crate
