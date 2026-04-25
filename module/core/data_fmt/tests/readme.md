# Tests

## Purpose
Contains all functional and integration tests for data_fmt library. Tests validate the three-format architecture (Table, Expanded, Tree) and all data conversion functionality.

## Organization Principles
- Tests organized in modular files by functionality domain
- Domain-based organization: tests grouped by what they test (data structures, builders, formatters)
- Tests cover Tasks 012-015: min_column_width floor, ANSI coloring, border variants, Unicode display width
- Test files kept under 1000 lines per file (warning threshold); mandatory split at 1500 lines

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `inc/` | Shared test utilities and helpers |
| `aligned_tree_basic.rs` | Test basic aligned tree formatting |
| `aligned_tree_configuration.rs` | Test aligned tree configuration options |
| `aligned_tree_edge_cases.rs` | Test aligned tree edge cases |
| `builder.rs` | Test TreeBuilder path-based construction |
| `column_data.rs` | Test column data handling |
| `column_truncation.rs` | Test column width truncation |
| `data.rs` | Test core data structures |
| `debug_output_format.rs` | Test debug output formatting |
| `decorated_cells_test.rs` | Test per-cell DecoratedText coloring and per-line ANSI reset invariant |
| `flatten_config.rs` | Test FlattenConfig customization |
| `fluent_api.rs` | Test fluent RowBuilder API |
| `formatters.rs` | Test TableShapedFormatter trait and ExpandedFormatter coloring behaviors |
| `html.rs` | Test HTML output format |
| `logfmt.rs` | Test logfmt output format |
| `manual/` | Store manual testing procedures |
| `multiline_cells.rs` | Test multiline cell support |
| `regression_alignment_column.rs` | Regression guard for historical column alignment bugs |
| `regression_willbe3_alignment.rs` | Regression guard for willbe3 alignment issue |
| `sql.rs` | Test SQL output format |
| `table_styles_compatibility.rs` | Test table style compatibility |
| `table_styles_outputs.rs` | Test table style outputs |
| `table_config_corner_cases.rs` | Test TableConfig API corner cases and edge conditions |
| `table_rendering_borders.rs` | Test border variant rendering: top/bottom borders, inter-row separators, AsciiGrid corners |
| `table_rendering_colors.rs` | Test ANSI header coloring and alternating-row coloring |
| `table_styles_presets.rs` | Test table style presets |
| `text_cli_help.rs` | Test CLI help text formatting |
| `themes.rs` | Test visual themes |
| `unicode_display_width_alignment.rs` | Test Unicode display width alignment |
| `unified_format_trait.rs` | Test unified format trait |
| `verify_alignment_correct.rs` | Verify alignment correctness |
| `word_wrap.rs` | Test `WrapFormatter` / `WrapConfig` / `BreakStrategy` / `Overflow` including bug reproducers |
| `text.rs` | Test TextFormatter general variants: Bullets, Numbered, KeyValue, Compact, Sections |
| `yaml.rs` | Test YamlFormatter YAML list-of-objects output and round-trip correctness |
| `json.rs` | Test JsonFormatter JSON array-of-objects output in pretty and compact modes |
| `toml_fmt.rs` | Test TomlFormatter TOML array-of-tables output and round-trip correctness |
| `expanded_indent_prefix.rs` | Test ExpandedConfig indent_prefix field behavior |
| `expanded_behavior.rs` | Test ExpandedFormatter corner cases: show_record_numbers, alignment, spacing |
| `sub_row_test.rs` | Test sub-row detail lines (tasks 017–018): API, rendering, config, backward compat, ANSI colorization |
| `auto_wrap_test.rs` | Test cell auto-wrapping with terminal-aware budget allocation (task 019) |
| `terminal_width_test.rs` | Test terminal width detection three-tier fallback (task 021) |
| `auto_fold_test.rs` | Test column folding with auto-fold continuation lines (task 020) |

## Test Files
- **data.rs** (14 tests) - Core data structures (`TreeNode`, `RowBuilder`, `TableView` trait)
- **builder.rs** (15 tests) - `TreeBuilder` path-based construction, batch creation
- **fluent_api.rs** (8 tests) - Fluent RowBuilder API, config builder patterns, builder-formatter integration
- **formatters.rs** (12 tests) - TableShapedFormatter trait, generic TableView, Write trait support, ExpandedFormatter key-color corner cases
- **flatten_config.rs** (4 tests) - FlattenConfig customization and integration

## Test Coverage by Category

### Core Data Structures (data.rs - 14 tests)
- TreeNode creation, manipulation, cloning, debug output
- Structure validation: deep nesting, wide trees, large trees (100 nodes)
- RowBuilder construction, header management, row addition
- Edge cases: empty trees, single nodes

### Tree Building (builder.rs - 15 tests)
- Basic construction and empty trees
- Path insertion: single paths, multiple root paths, nested paths
- Edge cases: duplicate paths, empty components, spaces, unicode
- Structure tests: deep nesting, wide trees, insertion order preservation
- Batch creation: `from_items()`, `from_pathbufs()`
- Fluent API chaining

### Fluent APIs (fluent_api.rs - 8 tests)
- Fluent RowBuilder API: single/multiple chains, custom row names, mixed with mutable API
- Config builders: TreeConfig, TableConfig, ExpandedConfig fluent APIs
- Integration: fluent builder with formatter trait polymorphism

### Formatters & Traits (formatters.rs - 12 tests)
- TableShapedFormatter trait: polymorphism with `Box<dyn>`, reference usage
- Generic TableView: works with integers, floats, custom Display types
- Write trait support: stdout output, multiple formatters, zero-allocation I/O
- Generic type integration with formatters
- ExpandedFormatter key-color: empty key_color guard, property_style() defaults, RESET-before-newline invariant, multi-record coloring

### Tree Flattening (flatten_config.rs - 4 tests)
- FlattenConfig: column selection, custom column names, path-only mode
- Integration with TableFormatter for customized output

## Navigation Guide
- **Core data structures**: `data.rs` - `TreeNode` creation, `RowBuilder` operations, `TableView` trait
- **Tree building**: `builder.rs` - Path insertion, `from_items`, deep nesting, unicode handling
- **Fluent APIs**: `fluent_api.rs` - Chainable builder methods, mixed fluent/mutable patterns
- **Configuration**: `fluent_api.rs` - Config builder patterns for all formatters
- **Trait polymorphism**: `formatters.rs` - `TableShapedFormatter` with trait objects
- **Generic support**: `formatters.rs` - `TableView` with integers, floats, custom types
- **I/O integration**: `formatters.rs` - Write trait support for zero-allocation output
- **Custom conversions**: `flatten_config.rs` - `FlattenConfig` for selective column flattening
