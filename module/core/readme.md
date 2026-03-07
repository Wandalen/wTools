# module/core/

Workspace crates providing core wtools functionality.

| Directory | Responsibility |
|-----------|----------------|
| `asbytes/` | Conversion of types to byte slices. |
| `async_from/` | Async version of the `From` trait. |
| `async_tools/` | Async utility primitives and helpers. |
| `benchkit/` | Benchmark framework for performance testing. |
| `claude_runner/` | Willbe integration lib for `.claude` commands. |
| `claude_runner_cli/` | Standalone CLI binary wrapping Claude Code. |
| `claude_runner_core/` | Builder pattern library for Claude process execution. |
| `claude_session/` | Session path resolution and continuation detection. |
| `claude_storage/` | CLI tool for exploring Claude Code storage. |
| `claude_storage_core/` | Zero-dep core library for Claude storage access. |
| `cli_fmt/` | CLI output formatting utilities. |
| `clone_dyn/` | Dynamic dispatch cloning for trait objects. |
| `clone_dyn_meta/` | Procedural macros for `clone_dyn`. |
| `clone_dyn_types/` | Types and traits for `clone_dyn`. |
| `collection_tools/` | Collection type utilities and abstractions. |
| `component_model/` | Component model for entity composition. |
| `component_model_meta/` | Procedural macros for `component_model`. |
| `component_model_types/` | Types and traits for `component_model`. |
| `config_hierarchy/` | Hierarchical configuration loading and resolution. |
| `crates_tools/` | Crate management and analysis utilities. |
| `data_type/` | Data type utilities and abstractions. |
| `derive_tools/` | Derive macro utilities and extensions. |
| `derive_tools_meta/` | Procedural macros for `derive_tools`. |
| `deterministic_rand/` | Deterministic random number generation. |
| `diagnostics_tools/` | Compiler diagnostics utilities. |
| `error_tools/` | Workspace-standard error handling. |
| `for_each/` | Iteration utilities and for-each patterns. |
| `format_tools/` | Formatting utilities and display helpers. |
| `former/` | Builder pattern derive macro. |
| `former_meta/` | Procedural macros for `former`. |
| `former_types/` | Types and traits for `former`. |
| `fs_tools/` | Filesystem utility functions. |
| `genfile/` | File generation utilities. |
| `genfile_core/` | Core library for file generation. |
| `implements/` | Trait implementation checking utilities. |
| `impls_index/` | Trait implementation index utilities. |
| `impls_index_meta/` | Procedural macros for `impls_index`. |
| `include_md/` | Include markdown content in Rust docs. |
| `inspect_type/` | Type inspection at runtime utilities. |
| `interval_adapter/` | Interval and range adapter utilities. |
| `is_slice/` | Slice type detection utilities. |
| `iter_tools/` | Iterator utilities and extensions. |
| `macro_tools/` | Macro development utilities. |
| `mem_tools/` | Memory utility functions. |
| `meta_tools/` | Meta-programming utility functions. |
| `mod_interface/` | Module interface generation macros. |
| `mod_interface_meta/` | Procedural macros for `mod_interface`. |
| `multiline_input/` | Interactive multiline terminal input. |
| `process_tools/` | Process execution utilities. |
| `program_tools/` | Program-level utility functions. |
| `pth/` | Path manipulation utilities. |
| `reflect_tools/` | Runtime reflection utilities. |
| `reflect_tools_meta/` | Procedural macros for `reflect_tools`. |
| `sqlx_query/` | SQLx query utilities. |
| `strs_tools/` | String manipulation utilities. |
| `strs_tools_meta/` | Procedural macros for `strs_tools`. |
| `test_tools/` | Testing utility helpers. |
| `time_tools/` | Time and duration utilities. |
| `tree_fmt/` | Tree-structured display formatting. |
| `typing_tools/` | Type system utilities. |
| `unitore/` | Unit test runner framework. |
| `variadic_from/` | Variadic `From` trait implementations. |
| `variadic_from_meta/` | Procedural macros for `variadic_from`. |
| `wca/` | Command aggregation utilities. |
| `willbe/` | Workspace tooling and build management. |
| `workspace_tools/` | Workspace management utilities. |
| `wtools/` | Workspace tools main aggregation crate. |
