# tests/inc

Test submodules — one per collection type, plus namespace coverage tests.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `mod.rs` | Aggregate all test submodules for the inc/ directory |
| `bmap.rs` | Test BTreeMap re-export, constructor, and iterator |
| `bset.rs` | Test BTreeSet re-export, constructor, and iterator |
| `deque.rs` | Test VecDeque re-export, constructor, and iterator |
| `heap.rs` | Test BinaryHeap re-export, constructor, and iterator |
| `hmap.rs` | Test HashMap re-export, constructor, and iterator |
| `hset.rs` | Test HashSet re-export, constructor, and iterator |
| `llist.rs` | Test LinkedList re-export, constructor, and iterator |
| `namespace_test.rs` | Verify all collection types and constructor macros accessible from root and exposed modules |
| `vec.rs` | Test Vec re-export, constructor, and iterator |
