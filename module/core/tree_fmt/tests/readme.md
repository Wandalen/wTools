# Tests

## Purpose
Contains all functional and integration tests for tree_fmt library. Tests validate the three-format architecture (Table, Expanded, Tree) and all data conversion functionality.

## Organization Principles
- Tests organized in modular files by functionality domain
- Domain-based organization: tests grouped by what they test (data structures, builders, formatters)
- 49 unit tests + 30 doc tests covering all library functionality
- All test files under 300 lines for maintainability

## Test Files
- **data.rs** (14 tests) - Core data structures (`TreeNode`, `RowBuilder`, `TableView` trait)
- **builder.rs** (15 tests) - `TreeBuilder` path-based construction, batch creation
- **fluent_api.rs** (8 tests) - Fluent RowBuilder API, config builder patterns, builder-formatter integration
- **formatters.rs** (8 tests) - TableShapedFormatter trait, generic TableView, Write trait support
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

### Formatters & Traits (formatters.rs - 8 tests)
- TableShapedFormatter trait: polymorphism with `Box<dyn>`, reference usage
- Generic TableView: works with integers, floats, custom Display types
- Write trait support: stdout output, multiple formatters, zero-allocation I/O
- Generic type integration with formatters

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
