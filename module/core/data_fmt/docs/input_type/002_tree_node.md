# Input Type: TreeNode

### Scope

- **Purpose**: Document the `TreeNode` generic struct used for both hierarchical and legacy tabular input.
- **Responsibility**: Document TreeNode struct definition, specializations, and trait implementations.
- **In Scope**: Struct fields, type parameter specializations, trait implementations, and usage patterns.
- **Out of Scope**: Conceptual shape (see `../input_model/`), formatter behavior (see `../feature/`).

### Input Models

| File | Relationship |
|------|-------------|
| [002_hierarchical.md](../input_model/002_hierarchical.md) | Conceptual data shape |

### APIs

| File | Relationship |
|------|-------------|
| [001_data_types.md](../api/001_data_types.md) | Public API surface |

### Sources

| File | Relationship |
|------|-------------|
| [`src/data.rs`](../../src/data.rs) | TreeNode struct definition |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../tests/data.rs) | TreeNode tests |

### Type Definition

`TreeNode` has three public fields: `name` holds the node label; `data` holds an optional payload — absent for directory (intermediate) nodes and present for leaf nodes; `children` holds a list of child nodes of the same type.

### Specializations

The same struct serves three distinct roles depending on the type parameter:

#### Specialization: Legacy Tabular (String data)

> **Removed in v0.3.0.** `RowBuilder::build()` and `TableShapedFormatter` were both removed. Use `RowBuilder::build_view()` and the `Format` trait instead.

Tables encoded as trees: root has row children, each row has column-named children with cell data. This specialization is retained as historical reference only.

#### Specialization: Generic Hierarchical (typed data)

File trees, dependency graphs, or any hierarchy with typed leaf data.

```text
root
├── src/
│   ├── main.rs → 150
│   └── lib.rs → 300
└── tests/
    └── test.rs → 50
```

- **Produced by:** `TreeBuilder::build()`
- **Consumed by:** `TreeFormatter::format()`
- **Input model:** Hierarchical

#### Specialization: Multi-Column Hierarchical (ColumnData)

Tree nodes with multiple aligned columns per leaf.

```text
root
├── api_ollama     v0.1.0   (api/ollama)
└── api_openai     v0.2.0   (api/openai)
```

- **Produced by:** Manual construction or `TreeBuilder`
- **Consumed by:** `TreeFormatter::format_aligned()`
- **Input model:** Hierarchical (multi-column variant)

### Trait Implementations

| Trait | Purpose |
|-------|---------|
| `TableShapedView` | Extract headers, rows, check table shape |
| `Debug` | Debug formatting |
| `Clone` | Value cloning |

`TableShapedView` requires the node's data type to implement display formatting to convert leaf values to strings during row extraction.
