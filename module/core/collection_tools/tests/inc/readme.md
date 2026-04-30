# tests/inc

Test submodules — one per collection type, plus namespace and component tests.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Aggregate all test submodules for the inc/ directory |
| `bmap.rs` | Test BTreeMap re-export, constructor, and iterator |
| `bset.rs` | Test BTreeSet re-export, constructor, and iterator |
| `components.rs` | Placeholder for VectorInterface component tests |
| `deque.rs` | Test VecDeque re-export, constructor, and iterator |
| `heap.rs` | Test BinaryHeap re-export, constructor, and iterator |
| `hmap.rs` | Test HashMap re-export, constructor, and iterator |
| `hset.rs` | Test HashSet re-export, constructor, and iterator |
| `llist.rs` | Test LinkedList re-export, constructor, and iterator |
| `namespace_test.rs` | Verify collection types accessible via exposed namespace |
| `vec.rs` | Test Vec re-export, constructor, and iterator |
