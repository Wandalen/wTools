# crates.md

Dependency reference — which crate to add and when.
Omitted: `_meta` proc-macro internals, `_types` companion crates, thin aliases, CLI-only binaries, unfinished stubs,
and narrow individual utilities that are fully covered by an aggregator crate listed here.

| Crate | Ver | Domain | When to use |
|---|---|---|---|
| `error_tools` | 0.37 | errors | Canonical wTools error handling: `BasicError`, `err!`, unified `Result`/`Error`. Use exclusively — never mix with `anyhow`/`thiserror`. |
| `asbytes` | 0.2 | memory | `AsBytes`/`IntoBytes` traits — view POD types as `&[u8]` or consume into `Vec<u8>` via bytemuck. |
| `collection_tools` | 0.36 | collections | Variadic constructor macros (`hmap!`, `vec!`, `hset!`, …); unified collection re-exports with automatic no_std/hashbrown support. |
| `deterministic_rand` | 0.7 | random | Hierarchical seeded RNG tree — switchable determinism for reproducible concurrent simulations or procedural generation. |
| `former` | 2.42 | patterns | `#[derive(Former)]` — fluent Builder with nested subformers, optional fields, default values, and custom storage. Flagship pattern crate. |
| `mod_interface` | 0.58 | patterns | `mod_interface!` — organize module namespaces into five visibility layers: `private`, `own`, `orphan`, `exposed`, `prelude`. |
| `derive_tools` | 0.61 | patterns | All-in-one derive aggregator: `Former`, `VariadicFrom`, `CloneDyn`, `Display`, `From`, `Into`, `Error`, and 20+ more; prefer over individual derive crates. |
| `variadic_from` | 0.55 | patterns | `#[derive(VariadicFrom)]` — auto-generates `From`/`Into` for structs from single values, tuples, or variadic arguments. |
| `clone_dyn` | 0.58 | patterns | `#[clone_dyn]` on a trait definition — enables `.clone()` on `Box<dyn Trait>` and `Arc<dyn Trait>`. |
| `strs_tools` | 0.43 | strings | Comprehensive string manipulation: structured splitting, parsing, indentation, quoting, multi-line handling; optional SIMD. |
| `format_tools` | 0.6 | format | Data-to-string formatting: debug/display/fallback variants and utilities for formatting arbitrary Rust values into strings. |
| `tree_fmt` | 0.9 | format | Multi-format data visualization — 10 formatter families, 31 variants: table, tree, JSON, HTML, SQL, and more. |
| `cli_fmt` | 0.3 | format | CLI output post-processing: merge stdout/stderr streams, apply head/tail limits, truncate ANSI-colored output. |
| `pth` | 0.35 | path | Typed path wrappers (`AbsolutePath`, `NormalizedPath`) and pure manipulation algorithms — normalizes, rebases, canonicalizes, never does I/O. |
| `process_tools` | 0.29 | process | Execute subprocesses via builder API; capture stdout/stderr; manage environment variables; detect CI/CD context. |
| `workspace_tools` | 0.11 | workspace | Workspace root detection with multi-fallback search; standardized directory layout; config and secret loading. |
| `crates_tools` | 0.23 | workspace | Download `.crate` archives from crates.io and read local `.crate` files — useful for tooling that inspects published crates. |
| `macro_tools` | 0.81 | proc-macro | High-level `syn`/`quote`/`proc-macro2` abstractions: attribute parsing, token manipulation, precise span-aware errors. Use when writing proc-macros. |
| `genfile_core` | 0.9 | codegen | Trait-based file generation: pluggable renderers, archive creation/loading/saving, and directory-to-archive conversion. |
| `claude_storage_core` | 1.0 | ai | Parse, filter, search, and export Claude Code JSONL conversation storage at `~/.claude/`; zero dependencies. |
