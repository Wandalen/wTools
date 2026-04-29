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

Total: 69 crates (3 alias + 34 core + 1 deprecated + 31 experimental)

## Crate Layer Assignments

**Column legend:**

| Column | Meaning |
|--------|---------|
| `Crate` | Crate name as published on crates.io |
| `Module` | Source directory under `module/`: `alias`, `core`, `deprecated`, or `experimental` |
| `L#` | Layer number (1 = lowest / most foundational, 9 = highest / most application-facing) |
| `Layer` | Layer name from the Layer Summary table |
| `Purpose` | One-sentence statement of what the crate does for its callers |
| `Deps` | Total runtime `[dependencies]` count (excludes `[dev-dependencies]` and `[build-dependencies]`) |
| `Int` | Subset of `Deps` that are internal workspace crates (identified by `workspace = true`) |
| `=L` | Internal deps on crates in the **same** layer — lateral coupling |
| `↓L` | Internal deps on crates in **lower** layers — expected, healthy flow |
| `↑L` | Internal deps on crates in **higher** layers — architectural violation; should be zero |
| `State` | Current lifecycle: `stable` (mature, actively used), `experimental` (API may change), `deprecated` (slated for removal) |
| `Target` | Desired lifecycle (same values as State); delta from State shows required action |
| `Readiness` | Quality criteria met: `T`ested, `D`ocumented, `C`lean (↑L=0), `F`eatures (default=[]), `R`eadme, `E`xamples, `M`arkers clean, `S`table deps; `·` = not met |

| Crate | Module | L# | Layer | Purpose | Deps | Int | =L | ↓L | ↑L | State | Target | Readiness |
|-------|--------|----|-------|---------|------|-----|----|----|-----|-------|--------|-----------|
| `error_tools` | core | 1 | Foundation | Provide a unified error handling namespace across the workspace | 2 | 0 | 0 | 0 | 0 | stable | stable | TDCFREMS |
| `data_type` | experimental | 1 | Foundation | Supply foundational type aliases and primal data structures | 3 | 2 | 0 | 0 | 2 | experimental | stable | TD··RE·S |
| `diagnostics_tools` | experimental | 1 | Foundation | Provide runtime assertion helpers with rich diagnostic context | 1 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFRE·S |
| `mem_tools` | experimental | 1 | Foundation | Offer safe memory introspection and alignment utilities | 0 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDC·REMS |
| `asbytes` | core | 2 | Primitives | Enable zero-copy viewing of POD types as byte slices | 1 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFREMS |
| `deterministic_rand` | core | 2 | Primitives | Generate hierarchical seeded random numbers with switchable determinism | 6 | 2 | 0 | 0 | 2 | deprecated | deprecated | TD·FRE·S |
| `implements` | experimental | 2 | Primitives | Answer at compile time whether a type implements a trait | 0 | 0 | 0 | 0 | 0 | experimental | stable | TDCFREMS |
| `inspect_type` | core | 2 | Primitives | Print exact Rust type names and sizes at compile time | 0 | 0 | 0 | 0 | 0 | stable | stable | TDCFREMS |
| `interval_adapter` | experimental | 2 | Primitives | Unify open, closed, and half-open range types behind one adapter | 0 | 0 | 0 | 0 | 0 | experimental | stable | TDCFRE·S |
| `is_slice` | experimental | 2 | Primitives | Answer at compile time whether an expression is a slice | 0 | 0 | 0 | 0 | 0 | experimental | stable | TDCFREMS |
| `time_tools` | experimental | 2 | Primitives | Provide minimal time measurement and timestamp utilities | 0 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFRE·S |
| `typing_tools` | experimental | 2 | Primitives | Express compile-time type constraints and type guards | 3 | 3 | 3 | 0 | 0 | deprecated | deprecated | TDCFREMS |
| `winterval` | experimental | 2 | Primitives | Re-export interval_adapter as a standalone dependency | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDC·REMS |
| `macro_tools` | core | 3 | Macro Framework | Supply all primitives needed to author procedural macros | 9 | 4 | 0 | 1 | 3 | stable | stable | TD··RE·S |
| `meta_tools` | experimental | 3 | Macro Framework | Provide token-level macro utilities for metaprogramming | 6 | 5 | 2 | 0 | 3 | deprecated | deprecated | TD·FREM· |
| `clone_dyn_meta` | core | 3 | Macro Framework | Implement #[clone_dyn] attribute macro backend (use clone_dyn crate directly) | 2 | 2 | 1 | 0 | 1 | stable | stable | TD·FR·MS |
| `component_model_meta` | experimental | 3 | Macro Framework | Generate component_model derive implementation (use component_model directly) | 3 | 2 | 1 | 0 | 1 | experimental | stable | TD·FR·MS |
| `derive_tools_meta` | core | 3 | Macro Framework | Generate derive_tools implementations (use derive_tools directly) | 3 | 3 | 1 | 0 | 2 | stable | stable | TD··R··S |
| `former_meta` | core | 3 | Macro Framework | Generate former builder derive implementation (use former directly) | 5 | 4 | 1 | 0 | 3 | stable | stable | TD··R··S |
| `impls_index_meta` | core | 3 | Macro Framework | Generate impls_index macro wrappers (use impls_index directly) | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFR·MS |
| `mod_interface_meta` | core | 3 | Macro Framework | Generate mod_interface namespace macros (use mod_interface directly) | 2 | 2 | 1 | 0 | 1 | stable | stable | TD··R··S |
| `reflect_tools_meta` | experimental | 3 | Macro Framework | Generate reflect_tools introspection code (use reflect_tools directly) | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFR·MS |
| `strs_tools_meta` | core | 3 | Macro Framework | Generate strs_tools compile-time operations (use strs_tools directly) | 1 | 1 | 1 | 0 | 0 | experimental | stable | TDCFR·MS |
| `variadic_from_meta` | core | 3 | Macro Framework | Generate variadic_from From impls (use variadic_from directly) | 1 | 1 | 1 | 0 | 0 | stable | stable | TDCFR··S |
| `clone_dyn` | core | 4 | Patterns | Make Box<dyn Trait> cloneable via a single attribute macro | 2 | 2 | 1 | 1 | 0 | stable | stable | TDC·REMS |
| `clone_dyn_types` | core | 4 | Patterns | Supply the CloneDyn trait and clone_into_box for type-erased Box cloning | 0 | 0 | 0 | 0 | 0 | stable | stable | TDC·RE·S |
| `component_model` | experimental | 4 | Patterns | Enable type-driven field assignment on complex objects | 2 | 2 | 1 | 1 | 0 | experimental | stable | TDC·RE·S |
| `component_model_types` | experimental | 4 | Patterns | Expose shared traits for the component_model pattern | 1 | 1 | 0 | 0 | 1 | experimental | stable | TD·FREMS |
| `derive_tools` | core | 4 | Patterns | Add Into, TryInto, IsVariant, and other missing std derives | 6 | 3 | 2 | 1 | 0 | stable | stable | TDC·RE·S |
| `former` | core | 4 | Patterns | Build complex objects with nested subformers via one derive | 4 | 4 | 1 | 1 | 2 | stable | stable | TD··RE·S |
| `former_types` | core | 4 | Patterns | Expose compile-time trait contracts reused by former consumers | 2 | 2 | 1 | 0 | 1 | stable | stable | TD·FRE·S |
| `impls_index` | core | 4 | Patterns | Wrap impl methods in named macros for navigable indexing | 1 | 1 | 0 | 1 | 0 | deprecated | deprecated | TDC·RE·· |
| `mod_interface` | core | 4 | Patterns | Replace dozens of pub use declarations with a single macro | 1 | 1 | 0 | 1 | 0 | stable | stable | TDC·REMS |
| `reflect_tools` | experimental | 4 | Patterns | Inspect struct fields by name and type at runtime | 3 | 3 | 1 | 1 | 1 | deprecated | deprecated | TD··RE·· |
| `variadic_from` | core | 4 | Patterns | Derive From implementations for tuples of 1 to N elements | 1 | 1 | 0 | 1 | 0 | stable | stable | TDCFREMS |
| `async_from` | experimental | 5 | Collections | Provide async versions of From, Into, TryFrom, and TryInto | 1 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFRE·S |
| `async_tools` | experimental | 5 | Collections | Supply practical helpers for async task spawning and joining | 2 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFREM· |
| `collection_tools` | core | 5 | Collections | Provide ergonomic literal macros for inline collection construction, portable to no_std | 1 | 0 | 0 | 0 | 0 | stable | stable | TDC·RE·S |
| `for_each` | experimental | 5 | Collections | Apply any macro to every item in a compile-time list | 0 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFREMS |
| `iter_tools` | experimental | 5 | Collections | Expose the full itertools combinator library via workspace facade | 2 | 1 | 0 | 1 | 0 | experimental | stable | TDC·RE·S |
| `cli_fmt` | core | 6 | String & Format | Process CLI command output with head/tail filtering, width truncation, and stream merging | 1 | 1 | 1 | 0 | 0 | experimental | stable | TDCFREMS |
| `color_tools` | core | 6 | String & Format | Wrap text with typed ANSI color for terminal and HTML rendering | 1 | 0 | 0 | 0 | 0 | experimental | stable | TDCFREMS |
| `data_fmt` | core | 6 | String & Format | Render data as aligned tables and nested tree structures | 9 | 3 | 2 | 1 | 0 | experimental | stable | TDCFREMS |
| `format_tools` | core | 6 | String & Format | Extend std formatting with structural display and string helpers | 3 | 3 | 0 | 3 | 0 | deprecated | deprecated | TDCFRE·· |
| `include_md` | experimental | 6 | String & Format | Include a markdown file or named section at compile time | 0 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFR·MS |
| `strs_tools` | core | 6 | String & Format | Manipulate strings with splitting, indentation, and pattern tools | 9 | 2 | 0 | 2 | 0 | experimental | stable | TDCFRE·S |
| `wstring_tools` | alias | 6 | String & Format | Alias — recommended single dependency for all string utilities | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDC·REMS |
| `config_hierarchy` | core | 7 | Path & Process | Resolve hierarchical config across runtime, env, file, and default sources | 6 | 1 | 0 | 1 | 0 | experimental | stable | TDC·REMS |
| `config_hierarchy` | experimental | 7 | Path & Process | Resolve hierarchical config across runtime, env, file, and default sources | 6 | 1 | 0 | 1 | 0 | experimental | stable | TDC·REMS |
| `fs_tools` | experimental | 7 | Path & Process | Read, write, and traverse files with ergonomic error context | 1 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDC·REMS |
| `process_tools` | core | 7 | Path & Process | Spawn child processes and capture output reliably | 6 | 4 | 0 | 4 | 0 | stable | stable | TDCFR·MS |
| `program_tools` | experimental | 7 | Path & Process | Compile and run a Rust source file on demand | 5 | 5 | 1 | 4 | 0 | deprecated | deprecated | TDCFR·MS |
| `pth` | core | 7 | Path & Process | Normalize, resolve, and join paths with workspace-aware helpers | 5 | 2 | 0 | 2 | 0 | stable | stable | TDC·RE·S |
| `workspace_tools` | core | 7 | Path & Process | Resolve paths relative to workspace root from any execution context | 11 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFREMS |
| `file_tools` | alias | 7 | Path & Process | Alias — recommended single dependency for all filesystem utilities | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFREM· |
| `benchkit` | experimental | 8 | Tooling | Benchmark performance and publish markdown reports | 9 | 1 | 0 | 1 | 0 | experimental | stable | TDCFREMS |
| `crates_tools` | core | 8 | Tooling | Parse Cargo.toml and analyze crate metadata programmatically | 3 | 0 | 0 | 0 | 0 | stable | stable | TDC·REMS |
| `genfile_core` | core | 8 | Tooling | Materialize project scaffolding from versioned template archives | 9 | 3 | 0 | 3 | 0 | stable | stable | TDC·REMS |
| `genfile` | core | 8 | Tooling | Manage code generation template archives from the command line | 4 | 3 | 1 | 2 | 0 | deprecated | deprecated | TDC·R·MS |
| `multiline_input` | core | 8 | Tooling | Read multi-line terminal input with readline and paste handling | 4 | 1 | 0 | 1 | 0 | deprecated | deprecated | TDCFREMS |
| `multiline_input` | experimental | 8 | Tooling | Evolve multiline_input with experimental input handling | 4 | 1 | 0 | 1 | 0 | deprecated | deprecated | T·CFREMS |
| `test_tools` | experimental | 8 | Tooling | Provide rich assertions and test organization for nextest | 11 | 3 | 0 | 3 | 0 | experimental | stable | TDCFRE·S |
| `wca` | experimental | 8 | Tooling | Define CLI commands as Rust functions with help and errors built in | 7 | 4 | 0 | 4 | 0 | experimental | stable | TDC·RE·S |
| `sqlx_query` | experimental | 9 | Application | Switch between SQLx compile-time and runtime query macros by feature | 0 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFREMS |
| `unitore` | experimental | 9 | Application | Subscribe to RSS and Atom feeds with configurable update intervals | 20 | 3 | 0 | 3 | 0 | experimental | stable | TDC·R··S |
| `willbe` | experimental | 9 | Application | Publish, version-bump, and consistency-check a Cargo workspace | 40 | 14 | 0 | 14 | 0 | experimental | stable | TDC·R··S |
| `willbe2` | deprecated | 9 | Application | Reimagine willbe with improved architecture | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFR·MS |
| `wtools` | experimental | 9 | Application | Aggregate the complete workspace toolkit in one dependency | 12 | 11 | 0 | 11 | 0 | deprecated | deprecated | TDC·REM· |
| `proper_tools` | alias | 9 | Application | Alias — recommended starting point for general-purpose wTools use | 0 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFREMS |


## Deprecation Candidates

Unreachable from any application (willbe, unitore, wca) or test infrastructure (test_tools). Identified by forward reachability analysis from leaf applications through runtime `[dependencies]` only. Alias crates included per policy.

| Crate | Module | Signal | Superseded By |
|-------|--------|--------|---------------|
| `asbytes` | core | Zero dependents; no crate in the workspace uses it | — |
| `async_from` | experimental | Sole dependent `async_tools` is itself deprecated | — |
| `async_tools` | experimental | `private` module is empty; re-exports `async_from` with zero added API | — |
| `deterministic_rand` | core | Zero dependents; ↑L=2 anomaly; no crate in the workspace uses it | — |
| `diagnostics_tools` | experimental | Sole dependent is `wtools` (deprecated) | — |
| `file_tools` | alias | Alias; zero dependents; underlying `fs_tools` also deprecated | — |
| `for_each` | experimental | Sole dependent `meta_tools` is deprecated | — |
| `format_tools` | core | Zero dependents; entire reflect→format chain is dead | — |
| `fs_tools` | experimental | Sole dependent `file_tools` (alias) is deprecated | — |
| `genfile` | core | Zero dependents; CLI wrapper around `genfile_core` (kept via `willbe`) | `genfile_core` directly |
| `impls_index` | core | Only dependents are `meta_tools` (deprecated) and `wtools` (deprecated) | — |
| `impls_index_meta` | core | Only dependents are `impls_index` (deprecated) and `meta_tools` (deprecated) | — |
| `include_md` | experimental | `lib.path` points to `_blank/standard_lib.rs`; all tests/examples commented out | — |
| `mem_tools` | experimental | Sole dependent is `wtools` (deprecated) | — |
| `meta_tools` | experimental | ↑L=3 arch violation; sole dependent is `wtools` (deprecated) | — |
| `multiline_input` | core | Zero dependents; no crate in the workspace uses it | — |
| `multiline_input` | experimental | Identical source to core; zero dependents; core also deprecated | — |
| `program_tools` | experimental | "Data structures only; compilation/execution planned" — YAGNI; zero dependents | — |
| `proper_tools` | alias | Alias; no dependencies; `enabled` feature declares nothing | — |
| `reflect_tools` | experimental | Sole dependent `format_tools` is deprecated; entire chain dead | — |
| `reflect_tools_meta` | experimental | Sole dependent `reflect_tools` is deprecated | — |
| `sqlx_query` | experimental | Zero dependents; `unitore` does not reference it in Cargo.toml or source | — |
| `time_tools` | experimental | Sole dependent is `wtools` (deprecated) | — |
| `typing_tools` | experimental | Sole dependent is `wtools` (deprecated) | — |
| `willbe2` | deprecated | Entire `src/lib.rs` is `pub use ::willbe::*`; zero independent development | `willbe` |
| `winterval` | experimental | Alias for `interval_adapter`; zero dependents within workspace | `interval_adapter` |
| `workspace_tools` | core | Zero dependents; no crate in the workspace uses it despite v0.12.0 | — |
| `wstring_tools` | alias | Alias; zero dependents | — |
| `wtools` | experimental | Aggregate; zero dependents; sole consumer of 10 other deprecated crates | — |

### Dead-End Chains

Complete dependency chains where every crate is unreachable:

1. `reflect_tools_meta` → `reflect_tools` → `format_tools` → (nobody)
2. `for_each` → `meta_tools` → `wtools` → (nobody)
3. `impls_index_meta` → `impls_index` → `wtools` → (nobody)
4. `fs_tools` → `file_tools` → (nobody)
5. `async_from` → `async_tools` → (nobody)

### Cleanup Artifact

| Path | Reason |
|------|--------|
| `module/alias/winterval/` | Empty directory; no `Cargo.toml`; never compiled; leftover stub |

## Promotion Readiness

Experimental crates targeting stable, ranked by readiness. Criteria key: `T`ested `D`ocumented `C`lean `F`eatures `R`eadme `E`xamples `M`arkers `S`table-deps.

### Ready for Promotion

All 8 criteria met — can be promoted to stable without prerequisite work.

| Crate | L# | Readiness |
|-------|----|-----------|
| `implements` | 2 | TDCFREMS |
| `is_slice` | 2 | TDCFREMS |
| `cli_fmt` | 6 | TDCFREMS |
| `color_tools` | 6 | TDCFREMS |
| `data_fmt` | 6 | TDCFREMS |
| `benchkit` | 8 | TDCFREMS |

### Blocked — by criteria count

| Crate | L# | Readiness | Blockers |
|-------|----|-----------|----------|
| `interval_adapter` | 2 | TDCFRE·S | M |
| `strs_tools_meta` | 3 | TDCFR·MS | E |
| `component_model_types` | 4 | TD·FREMS | C |
| `strs_tools` | 6 | TDCFRE·S | M |
| `test_tools` | 8 | TDCFRE·S | M |
| `component_model_meta` | 3 | TD·FR·MS | C, E |
| `component_model` | 4 | TDC·RE·S | F, M |
| `iter_tools` | 5 | TDC·RE·S | F, M |
| `config_hierarchy` (core) | 7 | TDC·REMS | F |
| `config_hierarchy` (experimental) | 7 | TDC·REMS | F |
| `wca` | 8 | TDC·RE·S | F, M |
| `data_type` | 1 | TD··RE·S | C, F, M |
| `unitore` | 9 | TDC·R··S | F, E, M |
| `willbe` | 9 | TDC·R··S | F, E, M |

### Blocker Frequency

| Criterion | Failing | Crates |
|-----------|---------|--------|
| M (Markers) | 9 | data_type, interval_adapter, component_model, iter_tools, strs_tools, test_tools, wca, unitore, willbe |
| F (Features) | 8 | data_type, component_model, iter_tools, config_hierarchy (core), config_hierarchy (experimental), wca, unitore, willbe |
| E (Examples) | 4 | component_model_meta, strs_tools_meta, unitore, willbe |
| C (Clean) | 3 | data_type, component_model_meta, component_model_types |

### Stable Crates with Gaps

Already-stable crates that do not meet all 8 criteria. Not promotion blockers, but maintenance targets.

| Crate | Readiness | Missing |
|-------|-----------|---------|
| `derive_tools_meta` | TD··R··S | C, F, E, M |
| `former_meta` | TD··R··S | C, F, E, M |
| `mod_interface_meta` | TD··R··S | C, F, E, M |
| `macro_tools` | TD··RE·S | C, F, M |
| `former` | TD··RE·S | C, F, M |
| `clone_dyn_meta` | TD·FR·MS | C, E |
| `variadic_from_meta` | TDCFR··S | E, M |
| `clone_dyn_types` | TDC·RE·S | F, M |
| `derive_tools` | TDC·RE·S | F, M |
| `collection_tools` | TDC·RE·S | F, M |
| `pth` | TDC·RE·S | F, M |
| `former_types` | TD·FRE·S | C, M |
| `clone_dyn` | TDC·REMS | F |
| `mod_interface` | TDC·REMS | F |
| `crates_tools` | TDC·REMS | F |
| `genfile_core` | TDC·REMS | F |
| `process_tools` | TDCFR·MS | E |

## Crate Profiles

Per-crate attributes for promotion and publishing.

**Schema:**
- **module** — source directory under `module/`: `alias`, `core`, `deprecated`, or `experimental`
- **layer** — layer number and name from the Layer Summary table
- **state** — current lifecycle: `stable` (mature, actively used), `experimental` (API may change), `deprecated` (slated for removal)
- **target** — desired lifecycle (same values as state); when state ≠ target, action is needed
- **readiness** — quality criteria met: `T`ested, `D`ocumented, `C`lean (↑L=0), `F`eatures (default=[]), `R`eadme, `E`xamples, `M`arkers clean, `S`table deps; `·` = not met
- **purpose** — one-sentence statement matching the Crate Layer Assignments table
- **deps** / **int** / **=L** / **↓L** / **↑L** — dep stats matching the Crate Layer Assignments columns
- **version** — current version from Cargo.toml; tracks release state
- **no_std** — whether the `no_std` feature is declared in Cargo.toml
- **keywords** — up to 5 crates.io search terms; recommended values for optimal discovery
- **categories** — up to 2 crates.io browse paths
- **pitch** — one sentence written for a potential adopter on crates.io

**Excluded:** `license` (all MIT, no signal), `msrv` (all workspace 1.61, no signal), `published` (none set `publish = false`; live status requires crates.io API), `docs`/`homepage` (fully derivable: `docs.rs/{name}`, GitHub repo path).

---

### Layer 1 · Foundation

#### `error_tools`
- **module**: core
- **layer**: 1 · Foundation
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Provide a unified error handling namespace across the workspace
- **deps**: 2 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.39.0
- **no_std**: yes
- **keywords**: error-handling, anyhow, thiserror, facade, workspace
- **categories**: algorithms, development-tools
- **pitch**: One import replaces separate anyhow and thiserror dependencies across your entire workspace with no runtime overhead.

#### `data_type`
- **module**: experimental
- **layer**: 1 · Foundation
- **state**: experimental
- **target**: stable
- **readiness**: TD··RE·S
- **purpose**: Supply foundational type aliases and primal data structures
- **deps**: 3 · **int**: 2 · **=L**: 0 · **↓L**: 0 · **↑L**: 2
- **version**: 0.25.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, data-types, primitives, type-aliases
- **categories**: algorithms, development-tools
- **pitch**: Foundational type aliases and primal data structures shared across the entire workspace.

#### `diagnostics_tools`
- **module**: experimental
- **layer**: 1 · Foundation
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFRE·S
- **purpose**: Provide runtime assertion helpers with rich diagnostic context
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.11.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, diagnostics, assertions, runtime
- **categories**: algorithms, development-tools
- **pitch**: Runtime assertion helpers that produce richer failure messages than std asserts — context without boilerplate.

#### `mem_tools`
- **module**: experimental
- **layer**: 1 · Foundation
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·REMS
- **purpose**: Offer safe memory introspection and alignment utilities
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.9.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, memory, alignment, size
- **categories**: algorithms, development-tools
- **pitch**: Safe memory introspection utilities — alignment checks, size comparisons, and copy helpers without unsafe.

---

### Layer 2 · Primitives

#### `asbytes`
- **module**: core
- **layer**: 2 · Primitives
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Enable zero-copy viewing of POD types as byte slices
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, bytes, pod, bytemuck
- **categories**: algorithms, development-tools, data-structures
- **pitch**: View any POD type as a byte slice, zero-copy and without unsafe — bytemuck-backed, two-line setup.

#### `deterministic_rand`
- **module**: core
- **layer**: 2 · Primitives
- **state**: deprecated
- **target**: deprecated
- **readiness**: TD·FRE·S
- **purpose**: Generate hierarchical seeded random numbers with switchable determinism
- **deps**: 6 · **int**: 2 · **=L**: 0 · **↓L**: 0 · **↑L**: 2
- **version**: 0.7.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, random, deterministic, seedable
- **categories**: algorithms, development-tools
- **pitch**: Hierarchical seeded RNG — swap between deterministic and OS-random with one flag and no code changes.

#### `implements`
- **module**: experimental
- **layer**: 2 · Primitives
- **state**: experimental
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Answer at compile time whether a type implements a trait
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.13.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, trait, implements, compile-time
- **categories**: algorithms, development-tools
- **pitch**: `implements!(MyType, Display)` — zero-cost compile-time check whether a type implements a trait.

#### `inspect_type`
- **module**: core
- **layer**: 2 · Primitives
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Print exact Rust type names and sizes at compile time
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.16.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, diagnostic-purpose, type-name, compile-time
- **categories**: algorithms, development-tools
- **pitch**: Print the exact Rust type of any expression at compile time — the fastest type-debugging shortcut.

#### `interval_adapter`
- **module**: experimental
- **layer**: 2 · Primitives
- **state**: experimental
- **target**: stable
- **readiness**: TDCFRE·S
- **purpose**: Unify open, closed, and half-open range types behind one adapter
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.42.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, interval, range, bounds
- **categories**: algorithms, development-tools
- **pitch**: One trait unifies all Rust range types — open, closed, half-open — interchangeable behind a single adapter.

#### `is_slice`
- **module**: experimental
- **layer**: 2 · Primitives
- **state**: experimental
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Answer at compile time whether an expression is a slice
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.14.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, slice, array, type-check
- **categories**: algorithms, development-tools
- **pitch**: `is_slice!(x)` — compile-time check whether an expression is a slice, for use in type guards.

#### `time_tools`
- **module**: experimental
- **layer**: 2 · Primitives
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFRE·S
- **purpose**: Provide minimal time measurement and timestamp utilities
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, time, timestamp, duration
- **categories**: algorithms, development-tools
- **pitch**: Minimal time utilities — current timestamp, elapsed measurement, and instant comparisons in one crate.

#### `typing_tools`
- **module**: experimental
- **layer**: 2 · Primitives
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Express compile-time type constraints and type guards
- **deps**: 3 · **int**: 3 · **=L**: 3 · **↓L**: 0 · **↑L**: 0
- **version**: 0.11.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, type-system, compile-time, constraints
- **categories**: algorithms, development-tools
- **pitch**: Compile-time type guards and constraint macros — extend Rust's type system expressively without proc-macros.

#### `winterval`
- **module**: experimental
- **layer**: 2 · Primitives
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·REMS
- **purpose**: Re-export interval_adapter as a standalone dependency
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.3.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, interval, range, bounds
- **categories**: algorithms, development-tools
- **pitch**: Interval adapter as a standalone dependency — identical capability to interval_adapter, convenient alias.

---

### Layer 3 · Macro Framework

#### `macro_tools`
- **module**: core
- **layer**: 3 · Macro Framework
- **state**: stable
- **target**: stable
- **readiness**: TD··RE·S
- **purpose**: Supply all primitives needed to author procedural macros
- **deps**: 9 · **int**: 4 · **=L**: 0 · **↓L**: 1 · **↑L**: 3
- **version**: 0.85.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, proc-macro, syn, quote
- **categories**: algorithms, development-tools
- **pitch**: Everything a proc-macro author needs — syn parsing, token generation, and structured error reporting in one toolkit.

#### `meta_tools`
- **module**: experimental
- **layer**: 3 · Macro Framework
- **state**: deprecated
- **target**: deprecated
- **readiness**: TD·FREM·
- **purpose**: Provide token-level macro utilities for metaprogramming
- **deps**: 6 · **int**: 5 · **=L**: 2 · **↓L**: 0 · **↑L**: 3
- **version**: 0.12.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, meta, token, stringify
- **categories**: algorithms, development-tools
- **pitch**: Token-level utility macros for metaprogramming — stringify, concatenate, and count macro arguments.

#### `clone_dyn_meta`
- **module**: core
- **layer**: 3 · Macro Framework
- **state**: stable
- **target**: stable
- **readiness**: TD·FR·MS
- **purpose**: Implement #[clone_dyn] attribute macro backend (use clone_dyn crate directly)
- **deps**: 2 · **int**: 2 · **=L**: 1 · **↓L**: 0 · **↑L**: 1
- **version**: 0.58.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, clone, proc-macro, derive
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for clone_dyn — do not depend on directly; add clone_dyn to your Cargo.toml instead.

#### `component_model_meta`
- **module**: experimental
- **layer**: 3 · Macro Framework
- **state**: experimental
- **target**: stable
- **readiness**: TD·FR·MS
- **purpose**: Generate component_model derive implementation (use component_model directly)
- **deps**: 3 · **int**: 2 · **=L**: 1 · **↓L**: 0 · **↑L**: 1
- **version**: 0.17.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, builder-pattern, component, proc-macro
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for component_model — do not depend on directly; use component_model instead.

#### `derive_tools_meta`
- **module**: core
- **layer**: 3 · Macro Framework
- **state**: stable
- **target**: stable
- **readiness**: TD··R··S
- **purpose**: Generate derive_tools implementations (use derive_tools directly)
- **deps**: 3 · **int**: 3 · **=L**: 1 · **↓L**: 0 · **↑L**: 2
- **version**: 0.63.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, derive, proc-macro, codegen
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for derive_tools — do not depend on directly; use derive_tools instead.

#### `former_meta`
- **module**: core
- **layer**: 3 · Macro Framework
- **state**: stable
- **target**: stable
- **readiness**: TD··R··S
- **purpose**: Generate former builder derive implementation (use former directly)
- **deps**: 5 · **int**: 4 · **=L**: 1 · **↓L**: 0 · **↑L**: 3
- **version**: 2.43.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, builder-pattern, derive, proc-macro
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for former — do not depend on directly; use former instead.

#### `impls_index_meta`
- **module**: core
- **layer**: 3 · Macro Framework
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFR·MS
- **purpose**: Generate impls_index macro wrappers (use impls_index directly)
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.13.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, proc-macro, impl, namespace
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for impls_index — do not depend on directly; use impls_index instead.

#### `mod_interface_meta`
- **module**: core
- **layer**: 3 · Macro Framework
- **state**: stable
- **target**: stable
- **readiness**: TD··R··S
- **purpose**: Generate mod_interface namespace macros (use mod_interface directly)
- **deps**: 2 · **int**: 2 · **=L**: 1 · **↓L**: 0 · **↑L**: 1
- **version**: 0.59.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, builder-pattern, module, proc-macro
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for mod_interface — do not depend on directly; use mod_interface instead.

#### `reflect_tools_meta`
- **module**: experimental
- **layer**: 3 · Macro Framework
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFR·MS
- **purpose**: Generate reflect_tools introspection code (use reflect_tools directly)
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.7.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, reflection, proc-macro, introspection
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for reflect_tools — do not depend on directly; use reflect_tools instead.

#### `strs_tools_meta`
- **module**: core
- **layer**: 3 · Macro Framework
- **state**: experimental
- **target**: stable
- **readiness**: TDCFR·MS
- **purpose**: Generate strs_tools compile-time operations (use strs_tools directly)
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.18.0
- **no_std**: no
- **keywords**: procedural-macro, compile-time, optimization, string, codegen
- **categories**: development-tools
- **pitch**: Proc-macro backend for strs_tools — do not depend on directly; use strs_tools instead.

#### `variadic_from_meta`
- **module**: core
- **layer**: 3 · Macro Framework
- **state**: stable
- **target**: stable
- **readiness**: TDCFR··S
- **purpose**: Generate variadic_from From impls (use variadic_from directly)
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.30.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, from, variadic, proc-macro
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for variadic_from — do not depend on directly; use variadic_from instead.

---

### Layer 4 · Patterns

#### `clone_dyn`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TDC·REMS
- **purpose**: Make Box<dyn Trait> cloneable via a single attribute macro
- **deps**: 2 · **int**: 2 · **=L**: 1 · **↓L**: 1 · **↑L**: 0
- **version**: 0.62.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, clone, dyn, trait-object
- **categories**: algorithms, development-tools
- **pitch**: `#[clone_dyn]` on a trait makes `Box<dyn Trait>` cloneable — one attribute replaces four manual `Clone` impl blocks.

#### `clone_dyn_types`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TDC·RE·S
- **purpose**: Supply the CloneDyn trait and clone_into_box for type-erased Box cloning
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.48.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, clone, dyn, trait-object
- **categories**: algorithms, development-tools
- **pitch**: Zero-dependency `CloneDyn` trait and `clone_into_box` — import directly when you need the runtime types without the proc-macro.

#### `component_model`
- **module**: experimental
- **layer**: 4 · Patterns
- **state**: experimental
- **target**: stable
- **readiness**: TDC·RE·S
- **purpose**: Enable type-driven field assignment on complex objects
- **deps**: 2 · **int**: 2 · **=L**: 1 · **↓L**: 1 · **↑L**: 0
- **version**: 0.17.0
- **no_std**: yes
- **keywords**: builder-pattern, type-safe, zero-cost, fluent-api, configuration
- **categories**: rust-patterns, development-tools
- **pitch**: Build complex objects by assigning fields by type — no named setters, no boilerplate, zero runtime cost.

#### `component_model_types`
- **module**: experimental
- **layer**: 4 · Patterns
- **state**: experimental
- **target**: stable
- **readiness**: TD·FREMS
- **purpose**: Expose shared traits for the component_model pattern
- **deps**: 1 · **int**: 1 · **=L**: 0 · **↓L**: 0 · **↑L**: 1
- **version**: 0.27.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, builder-pattern, component, types
- **categories**: algorithms, development-tools
- **pitch**: Shared trait contracts for component_model — import when you need to name component_model types in interfaces.

#### `derive_tools`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TDC·RE·S
- **purpose**: Add Into, TryInto, IsVariant, and other missing std derives
- **deps**: 6 · **int**: 3 · **=L**: 2 · **↓L**: 1 · **↑L**: 0
- **version**: 0.65.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, derive, into, from
- **categories**: algorithms, development-tools
- **pitch**: The std-extending derive collection — Into, TryInto, IsVariant, From, and more via `#[derive]` with no boilerplate.

#### `former`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TD··RE·S
- **purpose**: Build complex objects with nested subformers via one derive
- **deps**: 4 · **int**: 4 · **=L**: 1 · **↓L**: 1 · **↑L**: 2
- **version**: 2.45.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, builder-pattern, derive, nested
- **categories**: algorithms, development-tools
- **pitch**: Builder pattern with one derive — nested subformers, collection builders, and computed defaults, no manual code.

#### `former_types`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TD·FRE·S
- **purpose**: Expose compile-time trait contracts reused by former consumers
- **deps**: 2 · **int**: 2 · **=L**: 1 · **↓L**: 0 · **↑L**: 1
- **version**: 2.38.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, builder-pattern, types, traits
- **categories**: algorithms, development-tools
- **pitch**: Compile-time trait contracts for former — import when you need to name builder types in your own interfaces.

#### `impls_index`
- **module**: core
- **layer**: 4 · Patterns
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·RE··
- **purpose**: Wrap impl methods in named macros for navigable indexing
- **deps**: 1 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.11.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, impl, index, namespace
- **categories**: algorithms, development-tools
- **pitch**: Wrap every method in a named macro so IDE navigation, docs, and grep all work on large impl blocks.

#### `mod_interface`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TDC·REMS
- **purpose**: Replace dozens of pub use declarations with a single macro
- **deps**: 1 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.61.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, module, interface, visibility
- **categories**: algorithms, development-tools
- **pitch**: One macro replaces dozens of `pub use` declarations — clean layered module interfaces in a single call.

#### `reflect_tools`
- **module**: experimental
- **layer**: 4 · Patterns
- **state**: deprecated
- **target**: deprecated
- **readiness**: TD··RE··
- **purpose**: Inspect struct fields by name and type at runtime
- **deps**: 3 · **int**: 3 · **=L**: 1 · **↓L**: 1 · **↑L**: 1
- **version**: 0.7.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, reflection, introspection, struct
- **categories**: algorithms, development-tools
- **pitch**: Inspect struct field names and types at runtime — lightweight reflection without unsafe or external ABI.

#### `variadic_from`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Derive From implementations for tuples of 1 to N elements
- **deps**: 1 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.59.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, from, variadic, tuple
- **categories**: algorithms, development-tools
- **pitch**: `#[derive(VariadicFrom)]` generates From implementations for 1-element through N-element tuples automatically.

---

### Layer 5 · Collections

#### `async_from`
- **module**: experimental
- **layer**: 5 · Collections
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFRE·S
- **purpose**: Provide async versions of From, Into, TryFrom, and TryInto
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, async, from, into
- **categories**: algorithms, development-tools
- **pitch**: Async versions of the standard conversion traits — From, Into, TryFrom, TryInto — for types that need to await.

#### `async_tools`
- **module**: experimental
- **layer**: 5 · Collections
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREM·
- **purpose**: Supply practical helpers for async task spawning and joining
- **deps**: 2 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, async, tokio, runtime
- **categories**: algorithms, development-tools
- **pitch**: Practical async helpers — runtime detection, task spawning, and join utilities that work across runtimes.

#### `collection_tools`
- **module**: core
- **layer**: 5 · Collections
- **state**: stable
- **target**: stable
- **readiness**: TDC·RE·S
- **purpose**: Provide ergonomic literal macros for inline collection construction, portable to no_std
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.38.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, collections, hashmap, hashset
- **categories**: algorithms, development-tools
- **pitch**: `hmap!{ "key" => val }` and friends — ergonomic inline macros for creating any collection literal, with hashbrown fallback for no_std.

#### `for_each`
- **module**: experimental
- **layer**: 5 · Collections
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Apply any macro to every item in a compile-time list
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.10.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, macro, for-each, list
- **categories**: algorithms, development-tools
- **pitch**: Apply any macro to every item in a compile-time list — fills the gap macro_rules! can't close for list-driven codegen.

#### `iter_tools`
- **module**: experimental
- **layer**: 5 · Collections
- **state**: experimental
- **target**: stable
- **readiness**: TDC·RE·S
- **purpose**: Expose the full itertools combinator library via workspace facade
- **deps**: 2 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.50.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, iterator, itertools, combinators
- **categories**: algorithms, development-tools
- **pitch**: The full itertools combinator library re-exported through the workspace facade — one dependency, all combinators.

---

### Layer 6 · String & Format

#### `cli_fmt`
- **module**: core
- **layer**: 6 · String & Format
- **state**: experimental
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Process CLI command output with head/tail filtering, width truncation, and stream merging
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.4.0
- **no_std**: no
- **keywords**: cli, command-line, output, formatting, display
- **categories**: command-line-utilities, development-tools
- **pitch**: Head/tail line filtering and ANSI-aware width truncation for CLI output — builder API gives you processed text plus metadata on what was omitted.

#### `color_tools`
- **module**: core
- **layer**: 6 · String & Format
- **state**: experimental
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Wrap text with typed ANSI color for terminal and HTML rendering
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.4.0
- **no_std**: no
- **keywords**: ansi, color, terminal, text, escape
- **categories**: text-processing, command-line-interface
- **pitch**: Typed ANSI color wrapper with builder API — Color enum covers 4-bit, 256, and RGB; renders to both terminal escapes and HTML spans; zero internal dependencies.

#### `data_fmt`
- **module**: core
- **layer**: 6 · String & Format
- **state**: experimental
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Render data as aligned tables and nested tree structures
- **deps**: 9 · **int**: 3 · **=L**: 2 · **↓L**: 1 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: table, formatter, data-format, visualization, tree
- **categories**: text-processing, data-structures
- **pitch**: Render any data as an aligned table or nested tree — visual inspection of complex structures with zero setup.

#### `format_tools`
- **module**: core
- **layer**: 6 · String & Format
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFRE··
- **purpose**: Extend std formatting with structural display and string helpers
- **deps**: 3 · **int**: 3 · **=L**: 0 · **↓L**: 3 · **↑L**: 0
- **version**: 0.6.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, format, serialization, display
- **categories**: algorithms, development-tools
- **pitch**: Formatting utilities that extend std — structural display, aligned output, and string serialization helpers.

#### `include_md`
- **module**: experimental
- **layer**: 6 · String & Format
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFR·MS
- **purpose**: Include a markdown file or named section at compile time
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, markdown, include, compile-time
- **categories**: algorithms, development-tools
- **pitch**: Include a markdown file — or just one named section of it — verbatim into source at compile time.

#### `strs_tools`
- **module**: core
- **layer**: 6 · String & Format
- **state**: experimental
- **target**: stable
- **readiness**: TDCFRE·S
- **purpose**: Manipulate strings with splitting, indentation, and pattern tools
- **deps**: 9 · **int**: 2 · **=L**: 0 · **↓L**: 2 · **↑L**: 0
- **version**: 0.45.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, string, split, trim
- **categories**: algorithms, development-tools
- **pitch**: String utilities that std forgot — flexible splitting, indentation stripping, and pattern-based manipulation.

#### `wstring_tools`
- **module**: alias
- **layer**: 6 · String & Format
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·REMS
- **purpose**: Alias — recommended single dependency for all string utilities
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose
- **categories**: algorithms, development-tools
- **pitch**: All wTools string utilities in one alias — the recommended single dependency for all string handling needs.

---

### Layer 7 · Path & Process

#### `config_hierarchy` (core)
- **module**: core
- **layer**: 7 · Path & Process
- **state**: experimental
- **target**: stable
- **readiness**: TDC·REMS
- **purpose**: Resolve hierarchical config across runtime, env, file, and default sources
- **deps**: 6 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.4.0
- **no_std**: no
- **keywords**: config, configuration, hierarchy, yaml, settings
- **categories**: config, filesystem
- **pitch**: Trait-based 6-level configuration resolution with source tracking, type detection, and atomic file operations — every value knows where it came from.

#### `config_hierarchy` (experimental)
- **module**: experimental
- **layer**: 7 · Path & Process
- **state**: experimental
- **target**: stable
- **readiness**: TDC·REMS
- **purpose**: Resolve hierarchical config across runtime, env, file, and default sources
- **deps**: 6 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.5.0
- **no_std**: no
- **keywords**: config, configuration, hierarchy, yaml, settings
- **categories**: config, filesystem
- **pitch**: Trait-based 6-level configuration resolution with source tracking, type detection, and atomic file operations — every value knows where it came from.

#### `fs_tools`
- **module**: experimental
- **layer**: 7 · Path & Process
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·REMS
- **purpose**: Read, write, and traverse files with ergonomic error context
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, filesystem, file, path
- **categories**: algorithms, development-tools
- **pitch**: File system utilities with ergonomic error context — read, write, and traverse files with clear failure messages.

#### `process_tools`
- **module**: core
- **layer**: 7 · Path & Process
- **state**: stable
- **target**: stable
- **readiness**: TDCFR·MS
- **purpose**: Spawn child processes and capture output reliably
- **deps**: 6 · **int**: 4 · **=L**: 0 · **↓L**: 4 · **↑L**: 0
- **version**: 0.32.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, process, spawn, shell
- **categories**: algorithms, development-tools
- **pitch**: Spawn child processes and capture their output reliably — ergonomic wrappers with clear exit code handling.

#### `program_tools`
- **module**: experimental
- **layer**: 7 · Path & Process
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFR·MS
- **purpose**: Compile and run a Rust source file on demand
- **deps**: 5 · **int**: 5 · **=L**: 1 · **↓L**: 4 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, compile, run, program
- **categories**: algorithms, development-tools
- **pitch**: Compile a Rust source file on the fly and run it — the missing link for code generation pipelines.

#### `pth`
- **module**: core
- **layer**: 7 · Path & Process
- **state**: stable
- **target**: stable
- **readiness**: TDC·RE·S
- **purpose**: Normalize, resolve, and join paths with workspace-aware helpers
- **deps**: 5 · **int**: 2 · **=L**: 0 · **↓L**: 2 · **↑L**: 0
- **version**: 0.37.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, path, normalize, resolve
- **categories**: algorithms, development-tools
- **pitch**: Path manipulation utilities — normalize, resolve, and join paths with workspace-aware helpers.

#### `workspace_tools`
- **module**: core
- **layer**: 7 · Path & Process
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Resolve paths relative to workspace root from any execution context
- **deps**: 11 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.12.0
- **no_std**: no
- **keywords**: workspace, path, cargo, secrets, config
- **categories**: filesystem, development-tools
- **pitch**: Find your workspace root reliably from any execution context — tests, scripts, and CI — and resolve paths from it.

#### `file_tools`
- **module**: alias
- **layer**: 7 · Path & Process
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREM·
- **purpose**: Alias — recommended single dependency for all filesystem utilities
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose
- **categories**: algorithms, development-tools
- **pitch**: All wTools file system utilities in one alias — the recommended single dependency for all file handling needs.

---

### Layer 8 · Tooling

#### `benchkit`
- **module**: experimental
- **layer**: 8 · Tooling
- **state**: experimental
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Benchmark performance and publish markdown reports
- **deps**: 9 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.20.0
- **no_std**: no
- **keywords**: benchmark, performance, toolkit, markdown, reports
- **categories**: development-tools, development-tools::profiling
- **pitch**: Benchmark Rust code and publish markdown performance reports — minimal setup, actionable output, no harness lock-in.

#### `crates_tools`
- **module**: core
- **layer**: 8 · Tooling
- **state**: stable
- **target**: stable
- **readiness**: TDC·REMS
- **purpose**: Parse Cargo.toml and analyze crate metadata programmatically
- **deps**: 3 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.25.0
- **no_std**: no
- **keywords**: crates, cargo, toml, metadata, analysis
- **categories**: development-tools, development-tools::cargo-plugins
- **pitch**: Parse Cargo.toml files and analyze crate metadata programmatically — no shell-outs, no manual TOML wrestling.

#### `genfile_core`
- **module**: core
- **layer**: 8 · Tooling
- **state**: stable
- **target**: stable
- **readiness**: TDC·REMS
- **purpose**: Materialize project scaffolding from versioned template archives
- **deps**: 9 · **int**: 3 · **=L**: 0 · **↓L**: 3 · **↑L**: 0
- **version**: 0.10.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, template, codegen, scaffolding
- **categories**: development-tools, template-engine
- **pitch**: Template-driven code generation — materialize project scaffolding from versioned template archives programmatically.

#### `genfile`
- **module**: core
- **layer**: 8 · Tooling
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·R·MS
- **purpose**: Manage code generation template archives from the command line
- **deps**: 4 · **int**: 3 · **=L**: 1 · **↓L**: 2 · **↑L**: 0
- **version**: 0.4.0
- **no_std**: no
- **keywords**: template, codegen, cli, scaffolding, generator
- **categories**: command-line-utilities, development-tools
- **pitch**: Create and materialize code generation template archives from the command line — genfile_core made interactive.

#### `multiline_input` (core)
- **module**: core
- **layer**: 8 · Tooling
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Read multi-line terminal input with readline and paste handling
- **deps**: 4 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: terminal, input, multiline, cli, interactive
- **categories**: command-line-interface, text-editors
- **pitch**: Read multi-line terminal input cleanly — handles paste, readline edge cases, and buffer flushing out of the box.

#### `multiline_input` (experimental)
- **module**: experimental
- **layer**: 8 · Tooling
- **state**: deprecated
- **target**: deprecated
- **readiness**: T·CFREMS
- **purpose**: Evolve multiline_input with experimental input handling
- **deps**: 4 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: terminal, input, multiline, cli, interactive
- **categories**: command-line-interface, text-editors
- **pitch**: Experimental multi-line input handling — evolving version of the core multiline_input crate.

#### `test_tools`
- **module**: experimental
- **layer**: 8 · Tooling
- **state**: experimental
- **target**: stable
- **readiness**: TDCFRE·S
- **purpose**: Provide rich assertions and test organization for nextest
- **deps**: 11 · **int**: 3 · **=L**: 0 · **↓L**: 3 · **↑L**: 0
- **version**: 0.16.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, testing, assertions, harness
- **categories**: algorithms, development-tools
- **pitch**: A complete test harness — rich assertions, test organization helpers, and nextest-compatible test infrastructure.

#### `wca`
- **module**: experimental
- **layer**: 8 · Tooling
- **state**: experimental
- **target**: stable
- **readiness**: TDC·RE·S
- **purpose**: Define CLI commands as Rust functions with help and errors built in
- **deps**: 7 · **int**: 4 · **=L**: 0 · **↓L**: 4 · **↑L**: 0
- **version**: 0.46.0
- **no_std**: no
- **keywords**: cli, command, aggregation, cui, interface
- **categories**: command-line-interface, command-line-utilities
- **pitch**: Define CLI commands as Rust functions and aggregate them — help generation, error handling, and dispatch built in.

---

### Layer 9 · Application

#### `sqlx_query`
- **module**: experimental
- **layer**: 9 · Application
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Switch between SQLx compile-time and runtime query macros by feature
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.1
- **no_std**: no
- **keywords**: sqlx, sql, query, compile-time, feature-flag
- **categories**: database, development-tools
- **pitch**: Feature-flag switch between SQLx compile-time `query!` and runtime `query` — same call site, swappable modes.

#### `unitore`
- **module**: experimental
- **layer**: 9 · Application
- **state**: experimental
- **target**: stable
- **readiness**: TDC·R··S
- **purpose**: Subscribe to RSS and Atom feeds with configurable update intervals
- **deps**: 20 · **int**: 3 · **=L**: 0 · **↓L**: 3 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: no
- **keywords**: rss-feed, atom-feed, subscribe, terminal, reader
- **categories**: network-programming, command-line-utilities
- **pitch**: Subscribe to RSS and Atom feeds, configure per-feed update intervals, and browse entries from the terminal.

#### `willbe`
- **module**: experimental
- **layer**: 9 · Application
- **state**: experimental
- **target**: stable
- **readiness**: TDC·R··S
- **purpose**: Publish, version-bump, and consistency-check a Cargo workspace
- **deps**: 40 · **int**: 14 · **=L**: 0 · **↓L**: 14 · **↑L**: 0
- **version**: 0.35.0
- **no_std**: no
- **keywords**: workspace, publish, cargo, version, consistency
- **categories**: development-tools, development-tools::cargo-plugins
- **pitch**: Publish, version-bump, and consistency-check your entire Cargo workspace with a single command.

#### `willbe2`
- **module**: deprecated
- **layer**: 9 · Application
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFR·MS
- **purpose**: Reimagine willbe with improved architecture
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: workspace, publish, cargo, multi-crate, automation
- **categories**: development-tools, development-tools::cargo-plugins
- **pitch**: Next-generation workspace publishing and consistency tool — willbe reimagined with improved architecture.

#### `wtools`
- **module**: experimental
- **layer**: 9 · Application
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·REM·
- **purpose**: Aggregate the complete workspace toolkit in one dependency
- **deps**: 12 · **int**: 11 · **=L**: 0 · **↓L**: 11 · **↑L**: 0
- **version**: 0.2.20
- **no_std**: yes
- **keywords**: fundamental, general-purpose, toolkit, wtools, all-in-one
- **categories**: algorithms, development-tools
- **pitch**: The complete wTools suite in one dependency — import the entire workspace toolkit from a single crate.

#### `proper_tools`
- **module**: alias
- **layer**: 9 · Application
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Alias — recommended starting point for general-purpose wTools use
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, toolkit, utilities, workspace
- **categories**: algorithms, development-tools
- **pitch**: A curated wTools utilities alias — the recommended starting point for projects needing general-purpose tools.
