# Collection Module

Collection type re-exports and constructor macros.

## Files

| File | Responsibility |
|------|----------------|
| `binary_heap.rs` | BinaryHeap re-exports and heap!/into_heap! macros |
| `btree_map.rs` | BTreeMap re-exports and bmap!/into_bmap! macros |
| `btree_set.rs` | BTreeSet re-exports and bset!/into_bset! macros |
| `hash_map.rs` | HashMap re-exports and hmap!/into_hmap! macros |
| `hash_set.rs` | HashSet re-exports and hset!/into_hset! macros |
| `linked_list.rs` | LinkedList re-exports and llist!/into_llist! macros |
| `mod.rs` | Module aggregation and namespace configuration |
| `vec_deque.rs` | VecDeque re-exports and deque!/into_vecd! macros |
| `vector.rs` | Vec re-exports and vec!/into_vec! macros |

## Module Organization

Each collection type file provides:
- Type re-exports from standard library or hashbrown
- Constructor macros for creating collections with literal syntax
- Into-style constructor macros for type inference scenarios
- Comprehensive documentation with usage examples

The `mod.rs` file configures the 4-tier namespace architecture (own → orphan → exposed → prelude) and aggregates all collection types into the public API.
