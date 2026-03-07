# crates.md

Dependency reference — which crate to add and when.
Omitted: `_meta` proc-macro internals, `_types` companion crates, thin aliases, CLI-only binaries, unfinished stubs,
and narrow individual utilities that are fully covered by an aggregator crate listed here.

| Crate | Ver | Domain | When to use |
|---|---|---|---|
| `error_tools` | 0.37 | errors | Canonical wTools error handling: `BasicError`, `err!`, unified `Result`/`Error`. Use exclusively — never mix with `anyhow`/`thiserror`. |
| `diagnostics_tools` | 0.11 | errors | Runtime assertion macros with colorful diffs, compile-time feature-flag checks, and memory layout validation. |
| `mem_tools` | 0.9 | memory | Pointer equality, object size, and byte-level region/data comparison for types that can't implement `PartialEq`. |
| `asbytes` | 0.2 | memory | `AsBytes`/`IntoBytes` traits — view POD types as `&[u8]` or consume into `Vec<u8>` via bytemuck. |
| `collection_tools` | 0.36 | collections | Variadic constructor macros (`hmap!`, `vec!`, `hset!`, …); unified collection re-exports with automatic no_std/hashbrown support. |
| `data_type` | 0.23 | collections | Facade over `interval_adapter` + `collection_tools` + `either`; one dep instead of three. |
| `iter_tools` | 0.46 | collections | Clonable boxed iterators, iterator extension traits, and selective `itertools` re-exports. |
| `interval_adapter` | 0.40 | collections | Unified adapter over all Rust range types; lets you write APIs accepting any `Range`/`RangeInclusive`/`RangeFull`/etc. |
| `deterministic_rand` | 0.7 | random | Hierarchical seeded RNG tree — switchable determinism for reproducible concurrent simulations or procedural generation. |
| `typing_tools` | 0.11 | types | Compile-time trait-check (`implements!`), runtime slice detection, and type-name printing — three utilities in one dep. |
| `meta_tools` | 0.12 | macros | Macro-level utilities in one dep: list iteration (`for_each!`), function indexing (`impls_index!`), `mod_interface!`, and `paste!`. |
| `former` | 2.42 | patterns | `#[derive(Former)]` — fluent Builder with nested subformers, optional fields, default values, and custom storage. Flagship pattern crate. |
| `mod_interface` | 0.58 | patterns | `mod_interface!` — organize module namespaces into five visibility layers: `private`, `own`, `orphan`, `exposed`, `prelude`. |
| `derive_tools` | 0.61 | patterns | All-in-one derive aggregator: `Former`, `VariadicFrom`, `CloneDyn`, `Display`, `From`, `Into`, `Error`, and 20+ more; prefer over individual derive crates. |
| `variadic_from` | 0.55 | patterns | `#[derive(VariadicFrom)]` — auto-generates `From`/`Into` for structs from single values, tuples, or variadic arguments. |
| `clone_dyn` | 0.58 | patterns | `#[clone_dyn]` on a trait definition — enables `.clone()` on `Box<dyn Trait>` and `Arc<dyn Trait>`. |
| `component_model` | 0.15 | patterns | `#[derive(ComponentModel)]` — type-safe typed-component slots on structs: `.assign(val)` / `.component::<T>()`. |
| `strs_tools` | 0.43 | strings | Comprehensive string manipulation: structured splitting, parsing, indentation, quoting, multi-line handling; optional SIMD. |
| `format_tools` | 0.6 | format | Data-to-string formatting: debug/display/fallback variants and utilities for formatting arbitrary Rust values into strings. |
| `tree_fmt` | 0.9 | format | Multi-format data visualization — 10 formatter families, 31 variants: table, tree, JSON, HTML, SQL, and more. |
| `cli_fmt` | 0.3 | format | CLI output post-processing: merge stdout/stderr streams, apply head/tail limits, truncate ANSI-colored output. |
| `pth` | 0.35 | path | Typed path wrappers (`AbsolutePath`, `NormalizedPath`) and pure manipulation algorithms — normalizes, rebases, canonicalizes, never does I/O. |
| `fs_tools` | 0.1 | filesystem | `TempDir` RAII guard for temporary directories, and Unix shell-style glob pattern matching. |
| `process_tools` | 0.29 | process | Execute subprocesses via builder API; capture stdout/stderr; manage environment variables; detect CI/CD context. |
| `workspace_tools` | 0.11 | workspace | Workspace root detection with multi-fallback search; standardized directory layout; config and secret loading. |
| `crates_tools` | 0.23 | workspace | Download `.crate` archives from crates.io and read local `.crate` files — useful for tooling that inspects published crates. |
| `config_hierarchy` | 0.5 | config | Hierarchical configuration with 6-level priority (runtime → env → local → global → default → hardcoded) and per-value source tracking. |
| `macro_tools` | 0.81 | proc-macro | High-level `syn`/`quote`/`proc-macro2` abstractions: attribute parsing, token manipulation, precise span-aware errors. Use when writing proc-macros. |
| `wca` | 0.44 | cli | Type-safe CLI framework: argument parsing, subcommand routing, value converters, and automatic help generation. |
| `multiline_input` | 0.2 | cli | Terminal multiline text input widget — ENTER submits, Ctrl+ENTER inserts newline, with history and cursor movement. |
| `test_tools` | 0.16 | testing | wTools test harness and assertion utilities; `standalone_build` feature resolves circular-dep issues in workspace integration tests. |
| `benchkit` | 0.17 | testing | Performance benchmarking toolkit: regression detection, CI/CD integration, markdown/HTML/JSON report generation. |
| `genfile_core` | 0.9 | codegen | Trait-based file generation: pluggable renderers, archive creation/loading/saving, and directory-to-archive conversion. |
| `claude_runner_core` | 0.1 | ai | Builder-pattern library for assembling and executing Claude Code CLI processes; typed fields for all Claude CLI flags. |
| `claude_session` | 0.1 | ai | Detect active Claude Code sessions, resolve session storage paths, identify existing conversations for resumption. |
| `claude_storage_core` | 1.0 | ai | Parse, filter, search, and export Claude Code JSONL conversation storage at `~/.claude/`; zero dependencies. |
