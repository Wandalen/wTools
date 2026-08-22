# Variant: Tree Aggregated

### Scope

- **Purpose**: Provide a tree view with subtree totals and aggregate statistics.
- **Responsibility**: Complete attribute descriptor for this output variant preset.
- **In Scope**: All 46 variant attributes, example output, feature flag, compatibility.
- **Out of Scope**: Formatter implementation (see source), attribute schema (see `../data_structure/001_variant_attributes.md`).

### Algorithms

| File | Relationship |
|------|-------------|
| [009_tree_aggregation.md](../algorithm/009_tree_aggregation.md) | Recursive aggregate computation and rendering algorithm backing this variant |

### DataStructures

| File | Relationship |
|------|-------------|
| [001_variant_attributes.md](../data_structure/001_variant_attributes.md) | Attribute definitions for all 46 variant attributes |

### Formatters

| File | Relationship |
|------|-------------|
| [003_tree_formatter.md](../formatter/003_tree_formatter.md) | Parent formatter producing this variant |

### Sources

| File | Relationship |
|------|-------------|
| [`src/formatters/tree/aggregated.rs`](../../src/formatters/tree/aggregated.rs) | TreeFormatter implementation (`format_with_aggregation()`) |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/variant_014_tree_aggregated_test.rs`](../../tests/variant_014_tree_aggregated_test.rs) | Spec tests VT-1..VT-4 for the aggregated variant |

### Identity & Classification

- **formatter**: TreeFormatter
- **variant**: aggregated
- **is_default**: No
- **category**: Visual

### Build & Dependencies

- **feature_flag**: tree_aggregated
- **runtime_deps**: None
- **zero_dependency**: Yes

### Character Set & Encoding

- **charset**: Unicode
- **border_charset**: Box-Drawing
- **requires_unicode_terminal**: Yes
- **supports_ansi_colors**: Yes

### Visual Structure

- **has_borders**: No
- **border_style**: Tree-Lines
- **column_separator**: None
- **row_separator**: Newline
- **header_separator**: None
- **outer_padding**: No
- **inner_padding**: 1

### Data Representation

- **machine_parseable**: Partial
- **human_readable**: Yes
- **supports_hierarchical**: Yes
- **supports_tabular**: No
- **preserves_structure**: Yes
- **supports_multiline_values**: No

### Output Characteristics

- **output_compactness**: Verbose
- **visual_complexity**: Standard
- **alignment**: Left
- **column_alignment**: No

### Usage Context

- **primary_use_case**: Tree with subtree totals
- **terminal_optimized**: Yes
- **file_export_suitable**: Partial
- **streaming_friendly**: Yes
- **grep_friendly**: Partial

### Technical Details

- **escaping_rules**: None
- **output_format**: text/plain
- **standards_compliance**: None
- **supports_custom_colors**: Yes

### API & Construction

- **constructor**: `format_with_aggregation()`
- **config_type**: None
- **customizable_parameters**: 0
- **builder_pattern**: No

### Performance & Size

- **output_overhead**: Medium
- **memory_efficiency**: Buffered

### Compatibility

- **works_on_windows**: Partial
- **works_in_ci**: Yes
- **copy_paste_friendly**: Yes

### Example Output

```
src/ (450 lines, 90.0%)
│   ├── main.rs (150 lines, 30.0%)
│   └── lib.rs (300 lines, 60.0%)
tests/ (50 lines, 10.0%)
    └── test.rs (50 lines, 10.0%)
```

Note: `format_with_aggregation()` never renders the root node (the recursive
helper hardcodes `is_root = true` for the top call and skips the directory-line
branch unconditionally, regardless of `TreeConfig`), and directory roll-up
lines (e.g. `src/ (...)`) are emitted flush-left with no tree-branch prefix or
connector — only leaf value lines are indented and prefixed with `├──`/`└──`.
Verified by running `TreeFormatter::new().format_with_aggregation()` against
`src/formatters/tree/aggregated.rs`.
