# Input Type: TreeNode

### Scope

- **Purpose**: Document the `TreeNode<T>` generic Rust struct used for both hierarchical and legacy tabular input.
- **Responsibility**: Document TreeNode struct definition, specializations, and trait implementations.
- **In Scope**: Struct fields, type parameter specializations, trait implementations, and usage patterns.
- **Out of Scope**: Conceptual shape (see `../input_model/`), formatter behavior (see `../feature/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/data.rs` | TreeNode struct definition |
| test | `tests/data.rs` | TreeNode tests |
| doc | `../input_model/002_hierarchical.md` | Conceptual data shape |
| doc | `../api/001_data_types.md` | Public API surface |

### Type Definition

```rust
pub struct TreeNode< T >
{
  pub name : String,
  pub data : Option< T >,
  pub children : Vec< TreeNode< T > >,
}
```

### Specializations

The same struct serves three distinct roles depending on the type parameter:

#### TreeNode<String> — Legacy Tabular

Tables encoded as trees: root has row children, each row has column-named children with cell data.

```text
root
├── row_1
│   ├── Name: "Alice"
│   └── Age: "30"
└── row_2
    ├── Name: "Bob"
    └── Age: "25"
```

- **Produced by:** `RowBuilder::build()`
- **Consumed by:** `TableShapedFormatter` trait (deprecated — Table, Expanded)
- **Input model:** Tabular (legacy encoding)

#### TreeNode<T> — Generic Hierarchical

File trees, dependency graphs, or any hierarchy with typed leaf data.

```text
root
├── src/
│   ├── main.rs → 150
│   └── lib.rs → 300
└── tests/
    └── test.rs → 50
```

- **Produced by:** `TreeBuilder<T>::build()`
- **Consumed by:** `TreeFormatter::format( tree, render_fn )`
- **Input model:** Hierarchical

#### TreeNode<ColumnData> — Multi-Column Hierarchical

Tree nodes with multiple aligned columns per leaf.

```text
root
├── api_ollama     v0.1.0   (api/ollama)
└── api_openai     v0.2.0   (api/openai)
```

- **Produced by:** Manual construction or `TreeBuilder<ColumnData>`
- **Consumed by:** `TreeFormatter::format_aligned( tree )`
- **Input model:** Hierarchical (multi-column variant)

### Trait Implementations

| Trait | Bound | Purpose |
|-------|-------|---------|
| `TableShapedView` | `T: Display` | Extract headers, rows, check table shape |
| `Debug` | always | Debug formatting |
| `Clone` | always | Value cloning |
