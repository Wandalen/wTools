# Invariant Spec: Capacity Pre-allocation

### Scope

- **Element:** `invariant/002_capacity_preallocated`
- **Source:** `docs/invariant/002_capacity_preallocated.md`
- **Prefix:** `IN-`
- **Minimum cases:** 2

**Applicability:** Only the 5 collection types that provide `with_capacity` are covered: `Vec`,
`HashMap`, `HashSet`, `VecDeque`, `BinaryHeap`. The types `BTreeMap`, `BTreeSet`, and
`LinkedList` have no `with_capacity` in std Rust and use `new()` instead; the pre-allocation
invariant does not apply to the 6 macros covering these types.

## Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | vec_capacity_exactly_n | vec | ✅ |
| IN-02 | hashmap_capacity_at_least_n | hashmap | ✅ |
| IN-03 | empty_collection_capacity_zero | boundary | ✅ |
| IN-04 | into_vec_also_preallocates | into | ✅ |
| IN-05 | vecdeque_and_heap_also_preallocate | extended | ✅ |

---

### IN-01: Vec macro pre-allocates exactly N

- **Given:** Feature `collection_constructors` is enabled; N = 5 elements
- **When:** `vec![1, 2, 3, 4, 5]` is constructed
- **Then:** `v.capacity() == 5` exactly; `Vec::with_capacity(5)` allocates exactly N slots; no reallocation occurs during construction

### IN-02: HashMap macro pre-allocates at least N

- **Given:** Feature `collection_constructors` is enabled; N = 5 key-value pairs
- **When:** `hmap!{1 => 10, 2 => 20, 3 => 30, 4 => 40, 5 => 50}` is constructed
- **Then:** `map.capacity() >= 5`; HashMap rounds capacity up to an internal boundary for load-factor reasons; the assertion is `>=` not `==`

### IN-03: empty collection has capacity zero

- **Given:** Feature `collection_constructors` is enabled
- **When:** `vec![]` is constructed with zero elements
- **Then:** `v.capacity() == 0`; `Vec::with_capacity(0)` gives exactly 0

### IN-04: into-macro also pre-allocates

- **Given:** Feature `collection_into_constructors` is enabled; target type `Vec<i32>` annotated; N = 3
- **When:** `let v: Vec<i32> = into_vec![1, 2, 3]` is constructed
- **Then:** `v.capacity() >= 3`; into-macros call `with_capacity` before inserts, same contract as strict macros

### IN-05: VecDeque and BinaryHeap also pre-allocate

- **Given:** Feature `collection_constructors` is enabled; N = 5 elements
- **When:** `deque![1, 2, 3, 4, 5]` and `heap![1, 2, 3, 4, 5]` are constructed
- **Then:** `d.capacity() >= 5` and `h.capacity() >= 5`; both call `with_capacity(5)` before inserts
