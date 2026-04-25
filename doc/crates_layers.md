# Crates: Architectural Layer Map

Workspace crate inventory organized by architectural dependency layer.
Layers are ordered bottom-up: lower layers have fewer internal dependencies; higher layers depend on lower ones.

## Layer Summary

| L# | Name | Count | Domain |
|----|------|-------|--------|
| 1 | Foundation | 4 | Error handling, primal types, diagnostics, memory |
| 2 | Primitives | 9 | Bytes, rand, type checks, intervals, time |
| 3 | Macro Framework | 11 | All proc-macro (`_meta`) crates + macro_tools |
| 4 | Patterns | 11 | Builder, module org, clone dyn, reflect, component model |
| 5 | Collections | 5 | Containers, iterators, async traits |
| 6 | String & Format | 7 | Strings, colors, data display, markdown |
| 7 | Path & Process | 8 | Paths, filesystem, processes, workspace, config |
| 8 | Tooling | 8 | Testing, benchmarking, genfile, CLI, crates analysis |
| 9 | Application | 6 | willbe, unitore, wtools, sqlx, aggregators |

Total: 69 crates (3 alias + 34 core + 32 experimental)

## Full Table

| Crate | Module | L# | Layer | Description |
|-------|--------|----|-------|-------------|
| `error_tools` | core | 1 | Foundation | Unified error handling facade over anyhow + thiserror |
| `data_type` | experimental | 1 | Foundation | Collection of primal data types |
| `diagnostics_tools` | experimental | 1 | Foundation | Runtime diagnostics utilities |
| `mem_tools` | experimental | 1 | Foundation | Memory manipulation utilities |
| `asbytes` | core | 2 | Primitives | Traits for viewing data as byte slices |
| `deterministic_rand` | core | 2 | Primitives | Hierarchical deterministic RNG for simulations |
| `implements` | experimental | 2 | Primitives | Macro: does a type implement a trait? |
| `inspect_type` | core | 2 | Primitives | Compile-time type inspection and size diagnostics |
| `interval_adapter` | experimental | 2 | Primitives | Adapter for open/closed interval and range types |
| `is_slice` | experimental | 2 | Primitives | Macro: is a value a slice? |
| `time_tools` | experimental | 2 | Primitives | General purpose time abstractions |
| `typing_tools` | experimental | 2 | Primitives | Compile-time type checking utilities |
| `winterval` | experimental | 2 | Primitives | Interval adapter alias |
| `macro_tools` | core | 3 | Macro Framework | Building tools for procedural macros |
| `meta_tools` | experimental | 3 | Macro Framework | General purpose token-level meta tools |
| `clone_dyn_meta` | core | 3 | Macro Framework | Proc-macro for dynamic clone derive |
| `component_model_meta` | experimental | 3 | Macro Framework | Proc-macro for component model derive |
| `derive_tools_meta` | core | 3 | Macro Framework | Proc-macro for derive utilities |
| `former_meta` | core | 3 | Macro Framework | Proc-macro for builder pattern derive |
| `impls_index_meta` | core | 3 | Macro Framework | Proc-macro for implementations index |
| `mod_interface_meta` | core | 3 | Macro Framework | Proc-macro for module interface protocol |
| `reflect_tools_meta` | experimental | 3 | Macro Framework | Proc-macro for reflection mechanisms |
| `strs_tools_meta` | core | 3 | Macro Framework | Proc-macro for string tools |
| `variadic_from_meta` | core | 3 | Macro Framework | Proc-macro for variadic From derive |
| `clone_dyn` | core | 4 | Patterns | Derive to clone dyn trait objects |
| `clone_dyn_types` | core | 4 | Patterns | Types and traits for dynamic clone |
| `component_model` | experimental | 4 | Patterns | Type-safe component assignment via derive macros |
| `component_model_types` | experimental | 4 | Patterns | Trait types for component model pattern |
| `derive_tools` | core | 4 | Patterns | Collection of derives extending std |
| `former` | core | 4 | Patterns | Flexible builder pattern with nested subformers |
| `former_types` | core | 4 | Patterns | Compile-time types and traits for former |
| `impls_index` | core | 4 | Patterns | Index every function via named macros |
| `mod_interface` | core | 4 | Patterns | Modularity protocol with layered interfaces |
| `reflect_tools` | experimental | 4 | Patterns | Reflection and introspection mechanisms |
| `variadic_from` | core | 4 | Patterns | Variadic From/Into conversion derives |
| `async_from` | experimental | 5 | Collections | Async From, Into, TryFrom, TryInto traits |
| `async_tools` | experimental | 5 | Collections | Async programming toolkit |
| `collection_tools` | core | 5 | Collections | General purpose collection manipulation |
| `for_each` | experimental | 5 | Collections | Apply a macro to each element of a list |
| `iter_tools` | experimental | 5 | Collections | Iterator utilities (re-exports itertools) |
| `cli_fmt` | core | 6 | String & Format | CLI output formatting utilities |
| `color_tools` | core | 6 | String & Format | ANSI terminal color and text escape formatting |
| `data_fmt` | core | 6 | String & Format | Data display and tree visualization formatting |
| `format_tools` | core | 6 | String & Format | Mechanisms for formatting and string serialization |
| `include_md` | experimental | 6 | String & Format | Include markdown files or sections at compile time |
| `strs_tools` | core | 6 | String & Format | String manipulation utilities |
| `wstring_tools` | alias | 6 | String & Format | Alias — string tools |
| `config_hierarchy` | core | 7 | Path & Process | Configuration hierarchy with filesystem support |
| `config_hierarchy` | experimental | 7 | Path & Process | Configuration hierarchy (experimental) |
| `fs_tools` | experimental | 7 | Path & Process | File system manipulation tools |
| `process_tools` | core | 7 | Path & Process | Process execution and management |
| `program_tools` | experimental | 7 | Path & Process | Compile and run Rust programs |
| `pth` | core | 7 | Path & Process | Path manipulation and resolution utilities |
| `workspace_tools` | core | 7 | Path & Process | Workspace-relative path resolution and config loading |
| `file_tools` | alias | 7 | Path & Process | Alias — file system tools |
| `benchkit` | experimental | 8 | Tooling | Lightweight benchmarking and report generation |
| `crates_tools` | core | 8 | Tooling | Tools to analyse crate files and crates.io |
| `genfile_core` | core | 8 | Tooling | File generation and template materialization core |
| `genfile` | core | 8 | Tooling | CLI for template archive management |
| `multiline_input` | core | 8 | Tooling | Multi-line terminal input handling |
| `multiline_input` | experimental | 8 | Tooling | Multi-line terminal input (experimental) |
| `test_tools` | experimental | 8 | Tooling | Testing framework and test harness |
| `wca` | experimental | 8 | Tooling | CLI command aggregation framework |
| `sqlx_query` | experimental | 9 | Application | SQLx query/query! selector by feature flag |
| `unitore` | experimental | 9 | Application | RSS feed reader with configurable update frequency |
| `willbe` | experimental | 9 | Application | Multi-crate workspace publishing and consistency tool |
| `willbe2` | experimental | 9 | Application | Workspace publishing tool v2 |
| `wtools` | experimental | 9 | Application | Full workspace aggregator — all tools in one |
| `proper_tools` | alias | 9 | Application | Alias — general purpose tool aggregator |
