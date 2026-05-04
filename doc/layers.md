# Crates: Architectural Layer Map

Workspace crate inventory organized by architectural dependency layer.
Layers are ordered bottom-up: lower layers have fewer internal dependencies; higher layers depend on lower ones.

## Layer Summary

| L# | Name | Count | Domain |
|----|------|-------|--------|
| 1 | Foundation | 3 | Error handling, diagnostics, memory |
| 2 | Primitives | 9 | Bytes, rand, type checks, intervals, time |
| 3 | Macro Framework | 11 | All proc-macro (`_meta`) crates + macro_tools |
| 4 | Patterns | 11 | Builder, module org, clone dyn, reflect, component model |
| 5 | Collections | 6 | Containers, iterators, async traits |
| 6 | String & Format | 8 | Strings, colors, data display, markdown, type aggregation |
| 7 | Path & Process | 8 | Paths, filesystem, processes, workspace, config |
| 8 | Tooling | 8 | Testing, benchmarking, genfile, CLI, crates analysis |
| 9 | Application | 5 | willbe, unitore, sqlx, aggregators |

Total: 69 crates (0 alias + 30 core + 20 deprecated + 19 experimental)
Note: 22 additional legacy crates (formerly in `module/postponed/`) are co-located in `module/deprecated/` but have no layer assignments and are excluded from this table.

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
| `diagnostics_tools` | deprecated | 1 | Foundation | Supply runtime and compile-time assertion macros with colored diff output | 1 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFRE·S |
| `mem_tools` | experimental | 1 | Foundation | Compare memory addresses, sizes, and byte contents across heterogeneous reference pairs | 0 | 0 | 0 | 0 | 0 | experimental | experimental | TDCFREMS |
| `asbytes` | core | 2 | Primitives | View or consume POD data as bytes via bytemuck-backed traits | 1 | 0 | 0 | 0 | 0 | experimental | experimental | TDCFREMS |
| `deterministic_rand` | core | 2 | Primitives | Generate hierarchical seeded random numbers with switchable determinism | 6 | 2 | 0 | 0 | 2 | experimental | experimental | TD·FRE·S |
| `implements` | experimental | 2 | Primitives | Answer at compile time whether a type implements a trait | 0 | 0 | 0 | 0 | 0 | experimental | stable | TDCFREMS |
| `inspect_type` | core | 2 | Primitives | Print the Rust type name and byte size of any expression at runtime | 0 | 0 | 0 | 0 | 0 | stable | stable | TDCFREMS |
| `interval_adapter` | experimental | 2 | Primitives | Provide uniform interval trait coverage over all Rust range variants — bounded, half-open, and unbounded | 0 | 0 | 0 | 0 | 0 | experimental | stable | TDCFRE·S |
| `is_slice` | deprecated | 2 | Primitives | Answer at compile time whether an expression is a slice | 0 | 0 | 0 | 0 | 0 | experimental | deprecated | TDCFREMS |
| `time_tools` | deprecated | 2 | Primitives | Provide current UNIX epoch timestamps at second, millisecond, and nanosecond resolution | 0 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFRE·S |
| `typing_tools` | deprecated | 2 | Primitives | Aggregate implements, is_slice, and inspect_type into one dependency with per-sub-crate feature flags | 3 | 3 | 3 | 0 | 0 | deprecated | deprecated | TDCFREMS |
| `winterval` | deprecated | 2 | Primitives | Re-export interval_adapter as a standalone dependency | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDC·REMS |
| `macro_tools` | core | 3 | Macro Framework | Supply all primitives needed to author procedural macros | 9 | 4 | 0 | 1 | 3 | stable | stable | TD··RE·S |
| `meta_tools` | deprecated | 3 | Macro Framework | Bundle for_each, impls_index, mod_interface, and identifier-concat macros into one opt-in facade crate | 6 | 5 | 2 | 0 | 3 | deprecated | deprecated | TD·FREM· |
| `clone_dyn_meta` | core | 3 | Macro Framework | Implement #[clone_dyn] attribute macro backend (use clone_dyn crate directly) | 2 | 2 | 1 | 0 | 1 | stable | stable | TD·FR·MS |
| `component_model_meta` | experimental | 3 | Macro Framework | Implement proc-macro backends for Assign, ComponentModel, and related derives (use component_model directly) | 3 | 2 | 1 | 0 | 1 | experimental | stable | TD·FR·MS |
| `derive_tools_meta` | core | 3 | Macro Framework | Generate derive_tools implementations (use derive_tools directly) | 3 | 3 | 1 | 0 | 2 | stable | stable | TD··R··S |
| `former_meta` | core | 3 | Macro Framework | Generate former builder derive implementation (use former directly) | 5 | 4 | 1 | 0 | 3 | stable | stable | TD··R··S |
| `impls_index_meta` | deprecated | 3 | Macro Framework | Generate impls_index macro wrappers (use impls_index directly) | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFR·MS |
| `mod_interface_meta` | core | 3 | Macro Framework | Generate mod_interface namespace macros (use mod_interface directly) | 2 | 2 | 1 | 0 | 1 | stable | stable | TD··R··S |
| `reflect_tools_meta` | experimental | 3 | Macro Framework | Implement #[derive(Reflect)] proc-macro backend (use reflect_tools directly) | 1 | 1 | 1 | 0 | 0 | experimental | experimental | TDCFR·MS |
| `strs_tools_meta` | core | 3 | Macro Framework | Generate optimize_split! and optimize_match! proc macro implementations (use strs_tools directly) | 1 | 1 | 1 | 0 | 0 | experimental | stable | TDCFR·MS |
| `variadic_from_meta` | core | 3 | Macro Framework | Generate variadic_from From impls (use variadic_from directly) | 1 | 1 | 1 | 0 | 0 | stable | stable | TDCFR··S |
| `clone_dyn` | core | 4 | Patterns | Make Box<dyn Trait> cloneable via a single attribute macro | 2 | 2 | 1 | 1 | 0 | stable | stable | TDCFREMS |
| `clone_dyn_types` | core | 4 | Patterns | Supply the CloneDyn trait and clone_into_box for type-erased Box cloning | 0 | 0 | 0 | 0 | 0 | stable | stable | TDC·RE·S |
| `component_model` | experimental | 4 | Patterns | Enable type-driven field assignment on complex objects | 2 | 2 | 1 | 1 | 0 | experimental | stable | TDCFRE·S |
| `component_model_types` | experimental | 4 | Patterns | Expose shared traits for the component_model pattern | 1 | 1 | 0 | 0 | 1 | experimental | stable | TD·FREMS |
| `derive_tools` | core | 4 | Patterns | Add Into, TryInto, IsVariant, and other missing std derives | 6 | 3 | 2 | 1 | 0 | stable | stable | TDC·RE·S |
| `former` | core | 4 | Patterns | Build complex objects with nested subformers via one derive | 4 | 4 | 1 | 1 | 2 | stable | stable | TD··RE·S |
| `former_types` | core | 4 | Patterns | Expose compile-time trait contracts reused by former consumers | 2 | 2 | 1 | 0 | 1 | stable | stable | TD·FRE·S |
| `impls_index` | deprecated | 4 | Patterns | Wrap impl methods in named macros for navigable indexing | 1 | 1 | 0 | 1 | 0 | deprecated | deprecated | TDC·RE·· |
| `mod_interface` | core | 4 | Patterns | Organize module items into five propagation layers with a single declarative macro | 1 | 1 | 0 | 1 | 0 | stable | stable | TDCFREMS |
| `reflect_tools` | experimental | 4 | Patterns | Reflect any value at runtime — type identity, container detection, ordering, and key-value element iteration | 3 | 3 | 1 | 1 | 1 | experimental | experimental | TD··RE·S |
| `variadic_from` | core | 4 | Patterns | Construct structs from 1–3 typed arguments via From1/From2/From3 traits and VariadicFrom derive | 1 | 1 | 0 | 1 | 0 | stable | stable | TDCFREMS |
| `async_from` | experimental | 5 | Collections | Provide async versions of From, Into, TryFrom, and TryInto | 1 | 0 | 0 | 0 | 0 | experimental | experimental | TDCFR··S |
| `async_tools` | deprecated | 5 | Collections | Re-export async_from conversion traits and async_trait macro via a unified namespace facade | 2 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFREMS |
| `collection_tools` | core | 5 | Collections | Provide ergonomic literal macros for inline collection construction, portable to no_std | 1 | 0 | 0 | 0 | 0 | stable | stable | TDCFREMS |
| `for_each` | experimental | 5 | Collections | Apply any macro to every item in a compile-time list | 0 | 0 | 0 | 0 | 0 | experimental | experimental | TDCFREMS |
| `iter_tools` | deprecated | 5 | Collections | Re-export itertools combinators and provide clonable boxed iterators with stop-on-first-error mapping | 2 | 1 | 0 | 1 | 0 | deprecated | deprecated | TDC·RE·S |
| `wtools` | core | 5 | Collections | Thin collections aggregator; re-exports collection_tools macros and constructors | 1 | 1 | 1 | 0 | 0 | experimental | stable | T·CFREMS |
| `cli_fmt` | core | 6 | String & Format | Process CLI command output with head/tail filtering, width truncation, and stream merging | 1 | 1 | 1 | 0 | 0 | experimental | stable | TDCFR·MS |
| `color_tools` | core | 6 | String & Format | Wrap text with typed ANSI color for terminal and HTML rendering | 1 | 0 | 0 | 0 | 0 | experimental | stable | TDCFREMS |
| `data_fmt` | core | 6 | String & Format | Render structured data in multiple visual, text, and serialization formats | 9 | 3 | 2 | 1 | 0 | stable | stable | TDCFREMS |
| `data_type` | deprecated | 6 | String & Format | Aggregate interval, collection, and sum-type re-exports under one unified facade namespace | 3 | 2 | 0 | 2 | 0 | experimental | deprecated | TDC·RE·S |
| `format_tools` | deprecated | 6 | String & Format | Extend std formatting with structural display and string helpers | 3 | 3 | 0 | 3 | 0 | deprecated | deprecated | TDCFRE·S |
| `include_md` | experimental | 6 | String & Format | Include a markdown file or named section at compile time | 0 | 0 | 0 | 0 | 0 | experimental | experimental | TDCFR·MS |
| `strs_tools` | core | 6 | String & Format | Split, indent, and transform strings with ANSI utilities and SIMD acceleration | 9 | 2 | 0 | 2 | 0 | experimental | stable | TDCFREMS |
| `wstring_tools` | deprecated | 6 | String & Format | Alias — recommended single dependency for all string utilities | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDC·REMS |
| `config_hierarchy` | core | 7 | Path & Process | Resolve hierarchical config across runtime, env, file, and default sources | 6 | 1 | 0 | 1 | 0 | experimental | stable | TDC·R·MS |
| `config_hierarchy` | experimental | 7 | Path & Process | Resolve hierarchical config across runtime, env, file, and default sources | 6 | 1 | 0 | 1 | 0 | experimental | stable | TDCFREMS |
| `file_tools` | deprecated | 7 | Path & Process | RAII temp directories, glob re-export, and upward path traversal | 1 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFREMS |
| `process_tools` | core | 7 | Path & Process | Run subprocesses with captured I/O, probe process liveness, and daemonize Unix services | 6 | 4 | 0 | 4 | 0 | stable | stable | TDCFR·MS |
| `program_tools` | experimental | 7 | Path & Process | Rust script runner — compile and execute Rust files as scripts with output capture | 3 | 3 | 0 | 3 | 0 | experimental | experimental | TDCFREMS |
| `pth` | core | 7 | Path & Process | Syntactic path manipulation — normalize, join, query extensions, and typed path wrappers | 5 | 2 | 0 | 2 | 0 | stable | stable | TDC·R··S |
| `workspace_tools` | core | 7 | Path & Process | Resolve workspace-relative paths and optionally load configuration, manage secrets, and discover resources | 11 | 0 | 0 | 0 | 0 | experimental | stable | TDCFR·MS |
| `fs_tools` | deprecated | 7 | Path & Process | Alias — recommended single dependency for all filesystem utilities | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFREM· |
| `benchkit` | experimental | 8 | Tooling | Benchmark performance and publish markdown reports | 9 | 1 | 0 | 1 | 0 | experimental | stable | TDCFREMS |
| `crates_tools` | core | 8 | Tooling | Download and decode .crate archives for in-memory content inspection | 3 | 0 | 0 | 0 | 0 | experimental | stable | TDCFREMS |
| `genfile_core` | core | 8 | Tooling | Render parameterized template archives to output files with a pluggable engine and file system abstraction | 9 | 3 | 0 | 3 | 0 | stable | stable | TDCFREMS |
| `genfile` | core | 8 | Tooling | Manage code generation template archives from the command line | 4 | 3 | 1 | 2 | 0 | experimental | stable | TDCFREMS |
| `multiline_input` | deprecated | 8 | Tooling | Collect multiline terminal input with interactive key editing and Builder-configured validation | 4 | 1 | 0 | 1 | 0 | deprecated | deprecated | TDCFR·MS |
| `multiline_input` | experimental | 8 | Tooling | Collect multiline terminal input with interactive key editing and Builder-configured validation | 4 | 1 | 0 | 1 | 0 | experimental | stable | T·CFREMS |
| `test_tools` | experimental | 8 | Tooling | Aggregate workspace testing utilities into one dev-dependency via a circular-dependency-safe standalone build mode | 11 | 3 | 0 | 3 | 0 | experimental | stable | TDCFRE·· |
| `wca` | experimental | 8 | Tooling | Aggregate CLI commands as typed Rust routines with a parse-verify-execute pipeline | 7 | 4 | 0 | 4 | 0 | experimental | stable | TDCFRE·S |
| `sqlx_query` | deprecated | 9 | Application | Switch between SQLx compile-time and runtime query macros by feature | 0 | 0 | 0 | 0 | 0 | experimental | deprecated | TDCFR·MS |
| `unitore` | experimental | 9 | Application | Subscribe to RSS and Atom feeds from TOML configs, persist entries in embedded SQL, and browse from the terminal | 20 | 3 | 0 | 3 | 0 | experimental | stable | TDCFR··S |
| `willbe` | experimental | 9 | Application | Publish multi-crate workspaces in topological order, run feature-matrix tests, generate CI/CD workflows, and renew readme headers | 40 | 14 | 0 | 14 | 0 | experimental | stable | TDCFR··· |
| `willbe2` | deprecated | 9 | Application | Expose the willbe workspace tool under the willbe2 binary and crate name via complete re-export | 1 | 1 | 1 | 0 | 0 | deprecated | deprecated | TDCFR·MS |
| `proper_tools` | deprecated | 9 | Application | Alias — recommended starting point for general-purpose wTools use | 0 | 0 | 0 | 0 | 0 | deprecated | deprecated | TDCFREMS |


## Deprecation Candidates

Unreachable from any application (willbe, unitore, wca) or test infrastructure (test_tools). Identified by forward reachability analysis from leaf applications through runtime `[dependencies]` only. Alias crates included per policy.

| Crate | Module | Signal | Superseded By |
|-------|--------|--------|---------------|
| `async_tools` | deprecated | `private` module is empty; re-exports `async_from` with zero added API | — |
| `data_type` | deprecated | Pure facade re-exporting `collection_tools` + `interval_adapter` + `either`; zero unique logic; `willbe` must migrate to direct deps | direct deps on constituent crates |
| `diagnostics_tools` | deprecated | Zero dependents | — |
| `fs_tools` | deprecated | Alias; zero dependents; underlying `file_tools` also deprecated | — |
| `format_tools` | deprecated | Zero dependents; reflect_tools is experimental but no other live crate references format_tools | — |
| `file_tools` | deprecated | Sole dependent `fs_tools` (alias) is deprecated | — |
| `impls_index` | deprecated | Only dependent is `meta_tools` (deprecated) | — |
| `impls_index_meta` | deprecated | Only dependents are `impls_index` (deprecated) and `meta_tools` (deprecated) | — |
| `is_slice` | deprecated | Redundant with `implements!` macro + marker traits; sole dependent is `typing_tools` (deprecated) | `implements!` |
| `meta_tools` | deprecated | ↑L=3 arch violation; zero dependents | — |
| `multiline_input` | deprecated | Excluded from workspace (Cargo.toml exclude); moved to module/experimental | `multiline_input` (experimental) |
| `proper_tools` | deprecated | Alias; no dependencies; `enabled` feature declares nothing | — |
| `sqlx_query` | deprecated | Feature-flag toggle between two sqlx macros; 0 deps; 0 workspace consumers; trivially inlineable | — |
| `time_tools` | deprecated | Zero dependents | — |
| `typing_tools` | deprecated | Zero dependents | — |
| `willbe2` | deprecated | Entire `src/lib.rs` is `pub use ::willbe::*`; zero independent development | `willbe` |
| `winterval` | deprecated | Alias for `interval_adapter`; zero dependents within workspace | `interval_adapter` |
| `wstring_tools` | deprecated | Alias; zero dependents | — |
| `iter_tools` | deprecated | Functionality migrated to `macro_tools` (IterTrait, BoxedIter) and direct `itertools` dep; zero dependents | `itertools`, `macro_tools` |

### Dead-End Chains

Complete dependency chains where every crate is unreachable:

1. `format_tools` → (nobody)
2. `meta_tools` → (nobody)
3. `impls_index_meta` → `impls_index` → (nobody)
4. `file_tools` → `fs_tools` → (nobody)
5. `async_tools` → (nobody)
6. `sqlx_query` → (nobody)

## Promotion Readiness

Experimental crates targeting stable, ranked by readiness. Criteria key: `T`ested `D`ocumented `C`lean `F`eatures `R`eadme `E`xamples `M`arkers `S`table-deps.

### Ready for Promotion

All 8 criteria met — can be promoted to stable without prerequisite work.

| Crate | L# | Readiness |
|-------|----|-----------|
| `implements` | 2 | TDCFREMS |
| `color_tools` | 6 | TDCFREMS |
| `strs_tools` | 6 | TDCFREMS |
| `config_hierarchy` (experimental) | 7 | TDCFREMS |
| `genfile` | 8 | TDCFREMS |
| `crates_tools` | 8 | TDCFREMS |
| `benchkit` | 8 | TDCFREMS |

### Blocked — by criteria count

| Crate | L# | Readiness | Blockers |
|-------|----|-----------|----------|
| `strs_tools_meta` | 3 | TDCFR·MS | E |
| `component_model_types` | 4 | TD·FREMS | C |
| `cli_fmt` | 6 | TDCFR·MS | E |
| `workspace_tools` | 7 | TDCFR·MS | E |
| `wtools` | 5 | T·CFREMS | D |
| `test_tools` | 8 | TDCFRE·· | M, S |
| `multiline_input` | 8 | T·CFREMS | D |
| `component_model_meta` | 3 | TD·FR·MS | C, E |
| `component_model` | 4 | TDCFRE·S | M |
| `config_hierarchy` (core) | 7 | TDC·R·MS | F, E |
| `wca` | 8 | TDCFRE·S | M |
| `unitore` | 9 | TDCFR··S | E, M |
| `willbe` | 9 | TDCFR··· | E, M, S |

### Blocker Frequency

| Criterion | Failing | Crates |
|-----------|---------|--------|
| M (Markers) | 5 | component_model, test_tools, wca, unitore, willbe |
| F (Features) | 1 | config_hierarchy (core) |
| E (Examples) | 7 | component_model_meta, strs_tools_meta, cli_fmt, workspace_tools, config_hierarchy (core), unitore, willbe |
| C (Clean) | 2 | component_model_meta, component_model_types |
| S (Stable deps) | 2 | test_tools, willbe |
| D (Documented) | 2 | multiline_input (experimental), wtools |

### Stable Crates with Gaps

Already-stable crates that do not meet all 8 criteria. Not promotion blockers, but maintenance targets.

| Crate | Readiness | Missing |
|-------|-----------|---------|
| `derive_tools_meta` | TD··R··S | C, F, E, M |
| `former_meta` | TD··R··S | C, F, E, M |
| `mod_interface_meta` | TD··R··S | C, F, E, M |
| `macro_tools` | TD··RE·S | C, F, M |
| `former` | TD··RE·S | C, F, M |
| `pth` | TDC·R··S | F, E, M |
| `clone_dyn_meta` | TD·FR·MS | C, E |
| `variadic_from_meta` | TDCFR··S | E, M |
| `clone_dyn_types` | TDC·RE·S | F, M |
| `derive_tools` | TDC·RE·S | F, M |
| `former_types` | TD·FRE·S | C, M |
| `process_tools` | TDCFR·MS | E |

## Usefulness Assessment

Utility ranking for 50 non-deprecated crates. Evaluates: internal dependent count, external adopter value, unique functionality (not trivially replaceable), API surface depth.

### Tier Definitions

| Tier | Label | Criteria | Count |
|------|-------|----------|-------|
| 1 | Core | Essential to workspace; 5+ internal dependents; breakage cascades widely | 10 |
| 2 | High | Significant standalone utility; meaningful API surface; active consumers | 15 |
| 3 | Moderate | Useful in domain; narrower audience; some unique functionality | 13 |
| 4 | Low | Thin wrapper or narrow utility; few/zero consumers; easy to inline | 9 |
| 5 | Minimal | Stub, unimplemented, or fully supersedable; future deprecation review candidate | 2 |

### Ranked by Tier

| Crate | L# | Tier | Signal |
|-------|----|------|--------|
| `error_tools` | 1 | 1 | Unified error handling; near-universal workspace dep |
| `macro_tools` | 3 | 1 | Proc-macro framework; used by all `_meta` crates |
| `mod_interface` | 4 | 1 | Module organization framework; used by many crates |
| `mod_interface_meta` | 3 | 1 | Builds mod_interface |
| `former` | 4 | 1 | Builder pattern; ubiquitous across workspace |
| `former_meta` | 3 | 1 | Builds former |
| `former_types` | 4 | 1 | Compile-time contracts reused by all former consumers |
| `derive_tools` | 4 | 1 | Adds missing std derives; widely used |
| `derive_tools_meta` | 3 | 1 | Builds derive_tools |
| `collection_tools` | 5 | 1 | Ergonomic collection literal macros; broadly used |
| `willbe` | 9 | 2 | THE main workspace application; publish + CI/CD |
| `wca` | 8 | 2 | CLI aggregation framework; used by willbe + unitore |
| `unitore` | 9 | 2 | RSS/Atom feed application; standalone product |
| `test_tools` | 8 | 2 | Test harness; used by many crates in dev-deps |
| `process_tools` | 7 | 2 | Subprocess execution; essential for willbe |
| `pth` | 7 | 2 | Path manipulation; essential for willbe |
| `data_fmt` | 6 | 2 | Structured data rendering; rich multi-format API |
| `workspace_tools` | 7 | 2 | Workspace resolution, config loading, secrets |
| `genfile_core` | 8 | 2 | Template rendering engine; used by willbe |
| `genfile` | 8 | 2 | CLI for template archives; wraps genfile_core |
| `clone_dyn` | 4 | 2 | Makes Box<dyn Trait> cloneable; unique capability |
| `clone_dyn_types` | 4 | 2 | CloneDyn trait; supports clone_dyn ecosystem |
| `clone_dyn_meta` | 3 | 2 | Builds clone_dyn |
| `variadic_from` | 4 | 2 | Multi-arg construction; decent standalone utility |
| `variadic_from_meta` | 3 | 2 | Builds variadic_from |
| `config_hierarchy` (core) | 7 | 3 | Hierarchical config resolution; real-world utility |
| `config_hierarchy` (experimental) | 7 | 3 | Hierarchical config resolution; real-world utility |
| `strs_tools` | 6 | 3 | String split/indent/transform with ANSI + SIMD |
| `strs_tools_meta` | 3 | 3 | Builds strs_tools |
| `color_tools` | 6 | 3 | ANSI color wrapping for terminal and HTML |
| `benchkit` | 8 | 3 | Benchmark performance + markdown reports |
| `component_model` | 4 | 3 | Type-driven field assignment pattern |
| `component_model_types` | 4 | 3 | Shared traits for component_model |
| `component_model_meta` | 3 | 3 | Builds component_model |
| `crates_tools` | 8 | 3 | Download and decode .crate archives |
| `implements` | 2 | 3 | Compile-time trait-check mechanism; unique |
| `inspect_type` | 2 | 3 | Runtime type name + size printing; simple but unique |
| `program_tools` | 7 | 3 | Rust script runner; compile + execute Rust files |
| `multiline_input` (experimental) | 8 | 3 | Interactive multiline terminal input |
| `cli_fmt` | 6 | 4 | Thin output filtering wrapper; 0 external deps |
| `for_each` | 5 | 4 | Thin compile-time list macro; easily inlined |
| `async_from` | 5 | 4 | Trivial async trait definitions; 0 live internal deps |
| `interval_adapter` | 2 | 4 | Range normalization traits; sole internal dep is macro_tools |
| `deterministic_rand` | 2 | 4 | No workspace consumer; niche use case |
| `mem_tools` | 1 | 4 | Trivial std memory wrappers; 0 live internal deps |
| `reflect_tools` | 4 | 4 | Runtime reflection; incomplete implementation |
| `reflect_tools_meta` | 3 | 4 | Stub returning empty TokenStream; no real logic |
| `wtools` | 5 | 4 | Thin re-export facade for collection_tools; zero internal dependents |
| `asbytes` | 2 | 5 | Thin bytemuck wrapper; trivially replaceable by direct dep |
| `include_md` | 6 | 5 | Unimplemented stub; 0 logic; 0 consumers |

### Tier Distribution by Layer

| L# | Layer | T1 | T2 | T3 | T4 | T5 | Total |
|----|-------|----|----|----|----|-----|-------|
| 1 | Foundation | 1 | 0 | 0 | 1 | 0 | 2 |
| 2 | Primitives | 0 | 0 | 2 | 2 | 1 | 5 |
| 3 | Macro Framework | 4 | 2 | 2 | 1 | 0 | 9 |
| 4 | Patterns | 4 | 3 | 2 | 1 | 0 | 10 |
| 5 | Collections | 1 | 0 | 0 | 3 | 0 | 4 |
| 6 | String & Format | 0 | 1 | 2 | 1 | 1 | 5 |
| 7 | Path & Process | 0 | 3 | 2 | 0 | 0 | 5 |
| 8 | Tooling | 0 | 4 | 3 | 0 | 0 | 7 |
| 9 | Application | 0 | 2 | 0 | 0 | 0 | 2 |
| | **Total** | **10** | **15** | **13** | **9** | **2** | **49** |

Note: config_hierarchy counted once in tier table totals. Main ranked table lists both entries separately (50 rows).

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

#### `diagnostics_tools`
- **module**: deprecated
- **layer**: 1 · Foundation
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFRE·S
- **purpose**: Supply runtime and compile-time assertion macros with colored diff output
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.11.0
- **no_std**: yes
- **keywords**: assertions, diagnostics, testing, compile-time, no-std
- **categories**: development-tools, development-tools::testing
- **pitch**: Colored diff runtime assertions, compile-time cfg checks, and memory-layout validation — one crate, no-std compatible.

#### `mem_tools`
- **module**: experimental
- **layer**: 1 · Foundation
- **state**: experimental
- **target**: experimental
- **readiness**: TDCFREMS
- **purpose**: Compare memory addresses, sizes, and byte contents across heterogeneous reference pairs
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.9.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, memory, comparison, no_std
- **categories**: algorithms, development-tools
- **pitch**: Type-agnostic pointer, size, and byte-content comparison across heterogeneous reference pairs — fully safe public API with no_std support.

---

### Layer 2 · Primitives

#### `asbytes`
- **module**: core
- **layer**: 2 · Primitives
- **state**: experimental
- **target**: experimental
- **readiness**: TDCFREMS
- **purpose**: View or consume POD data as bytes via bytemuck-backed traits
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, bytes, pod, bytemuck
- **categories**: algorithms, development-tools, data-structures
- **pitch**: Two traits — AsBytes for borrowing and IntoBytes for consuming — turn any POD type into bytes, zero-copy and without unsafe.

#### `deterministic_rand`
- **module**: core
- **layer**: 2 · Primitives
- **state**: experimental
- **target**: experimental
- **readiness**: TD·FRE·S
- **purpose**: Generate hierarchical seeded random numbers with switchable determinism
- **deps**: 6 · **int**: 2 · **=L**: 0 · **↓L**: 0 · **↑L**: 2
- **version**: 0.7.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose
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
- **pitch**: `implements!(val => Trait)` — zero-cost compile-time check whether an expression's type satisfies a trait bound.

#### `inspect_type`
- **module**: core
- **layer**: 2 · Primitives
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Print the Rust type name and byte size of any expression at runtime
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.16.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, diagnostic-purpose, type-name, sizeof
- **categories**: algorithms, development-tools
- **pitch**: Print the exact Rust type name and byte size of any expression — the fastest runtime type-debugging shortcut.

#### `interval_adapter`
- **module**: experimental
- **layer**: 2 · Primitives
- **state**: experimental
- **target**: stable
- **readiness**: TDCFRE·S
- **purpose**: Provide uniform interval trait coverage over all Rust range variants — bounded, half-open, and unbounded
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.42.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, interval, range, bounds
- **categories**: algorithms, development-tools
- **pitch**: Two complementary traits — one for all range kinds, one for iterable ones — make any Rust range interchangeable as a function parameter.

#### `is_slice`
- **module**: deprecated
- **layer**: 2 · Primitives
- **state**: experimental
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Answer at compile time whether an expression is a slice
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.14.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, slice, array, type-check
- **categories**: algorithms, development-tools
- **pitch**: `is_slice!(x)` — compile-time check whether an expression is a slice, for use in type guards.

#### `time_tools`
- **module**: deprecated
- **layer**: 2 · Primitives
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFRE·S
- **purpose**: Provide current UNIX epoch timestamps at second, millisecond, and nanosecond resolution
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, time, timestamp, unix-epoch
- **categories**: algorithms, development-tools
- **pitch**: Current UNIX epoch timestamp in seconds, milliseconds, and nanoseconds — four retrieval functions, zero dependencies.

#### `typing_tools`
- **module**: deprecated
- **layer**: 2 · Primitives
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Aggregate implements, is_slice, and inspect_type into one dependency with per-sub-crate feature flags
- **deps**: 3 · **int**: 3 · **=L**: 3 · **↓L**: 0 · **↑L**: 0
- **version**: 0.11.0
- **no_std**: yes (feature-gated)
- **keywords**: fundamental, general-purpose, type-system, compile-time, type-checking
- **categories**: algorithms, development-tools
- **pitch**: One dependency for all type-checking macros — implements, is_slice, and inspect_type each activated independently via per-sub-crate feature flags.

#### `winterval`
- **module**: deprecated
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
- **readiness**: TD··RE··
- **purpose**: Supply all primitives needed to author procedural macros
- **deps**: 9 · **int**: 4 · **=L**: 0 · **↓L**: 1 · **↑L**: 3
- **version**: 0.85.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, proc-macro, syn, quote
- **categories**: algorithms, development-tools
- **pitch**: Everything a proc-macro author needs — syn parsing, token generation, and structured error reporting in one toolkit.

#### `meta_tools`
- **module**: deprecated
- **layer**: 3 · Macro Framework
- **state**: deprecated
- **target**: deprecated
- **readiness**: TD·FREM·
- **purpose**: Bundle for_each, impls_index, mod_interface, and identifier-concat macros into one opt-in facade crate
- **deps**: 6 · **int**: 5 · **=L**: 2 · **↓L**: 0 · **↑L**: 3
- **version**: 0.12.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose
- **categories**: algorithms, development-tools
- **pitch**: All-in-one macro utility facade — iterate lists, index impls, organize modules, and concatenate identifiers through a single opt-in dependency.

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
- **keywords**: fundamental, general-purpose, clone, proc-macro, attribute
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for clone_dyn — do not depend on directly; add clone_dyn to your Cargo.toml instead.

#### `component_model_meta`
- **module**: experimental
- **layer**: 3 · Macro Framework
- **state**: experimental
- **target**: stable
- **readiness**: TD·FR·MS
- **purpose**: Implement proc-macro backends for Assign, ComponentModel, and related derives (use component_model directly)
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
- **module**: deprecated
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
- **keywords**: modularity, namespace, proc-macro, module, visibility
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for mod_interface — do not depend on directly; use mod_interface instead.

#### `reflect_tools_meta`
- **module**: experimental
- **layer**: 3 · Macro Framework
- **state**: experimental
- **target**: experimental
- **readiness**: TDCFR·MS
- **purpose**: Implement #[derive(Reflect)] proc-macro backend (use reflect_tools directly)
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
- **purpose**: Generate optimize_split! and optimize_match! proc macro implementations (use strs_tools directly)
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.18.0
- **no_std**: no
- **keywords**: procedural-macro, compile-time, optimization
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
- **readiness**: TDCFREMS
- **purpose**: Make Box<dyn Trait> cloneable via a single attribute macro
- **deps**: 2 · **int**: 2 · **=L**: 1 · **↓L**: 1 · **↑L**: 0
- **version**: 0.62.0
- **no_std**: yes
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
- **no_std**: yes
- **keywords**: fundamental, general-purpose, clone, dyn, trait-object
- **categories**: algorithms, development-tools
- **pitch**: Zero-dependency `CloneDyn` trait and `clone_into_box` — import directly when you need the runtime types without the proc-macro.

#### `component_model`
- **module**: experimental
- **layer**: 4 · Patterns
- **state**: experimental
- **target**: stable
- **readiness**: TDCFRE·S
- **purpose**: Enable type-driven field assignment on complex objects
- **deps**: 2 · **int**: 2 · **=L**: 1 · **↓L**: 1 · **↑L**: 0
- **version**: 0.17.0
- **no_std**: yes
- **keywords**: builder-pattern, type-safe, zero-cost, fluent-api, configuration
- **categories**: rust-patterns, development-tools, api-bindings, config
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
- **keywords**: fundamental, general-purpose, builder-pattern
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
- **keywords**: fundamental, general-purpose
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
- **keywords**: fundamental, general-purpose, builder-pattern
- **categories**: algorithms, development-tools
- **pitch**: Compile-time trait contracts for former — import when you need to name builder types in your own interfaces.

#### `impls_index`
- **module**: deprecated
- **layer**: 4 · Patterns
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·RE··
- **purpose**: Wrap impl methods in named macros for navigable indexing
- **deps**: 1 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.11.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, impl, index, namespace
- **categories**: algorithms, development-tools
- **pitch**: Wrap every method in a named macro so IDE navigation, docs, and grep all work on large impl blocks.

#### `mod_interface`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Organize module items into five propagation layers with a single declarative macro
- **deps**: 1 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.61.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, builder-pattern
- **categories**: algorithms, development-tools
- **pitch**: One macro defines five visibility layers (own/orphan/exposed/prelude/private) and wires child modules into the cascade — clean layered module interfaces without manual `pub use` chains.

#### `reflect_tools`
- **module**: experimental
- **layer**: 4 · Patterns
- **state**: experimental
- **target**: experimental
- **readiness**: TD··RE·S
- **purpose**: Reflect any value at runtime — type identity, container detection, ordering, and key-value element iteration
- **deps**: 3 · **int**: 3 · **=L**: 1 · **↓L**: 1 · **↑L**: 1
- **version**: 0.7.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, reflection, introspection
- **categories**: algorithms, development-tools
- **pitch**: Runtime type introspection via Instance/Entity traits — reflect any value to query type name, container status, ordering, and iterate key-value elements over collection types.

#### `variadic_from`
- **module**: core
- **layer**: 4 · Patterns
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Construct structs from 1–3 typed arguments via From1/From2/From3 traits and VariadicFrom derive
- **deps**: 1 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.59.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, from, variadic, tuple
- **categories**: algorithms, development-tools
- **pitch**: `#[derive(VariadicFrom)]` implements From1/From2/From3 on structs with 1–3 fields and provides the `from!(a, b)` macro — eliminates manual tuple-to-struct boilerplate.

---

### Layer 5 · Collections

#### `async_from`
- **module**: experimental
- **layer**: 5 · Collections
- **state**: experimental
- **target**: experimental
- **readiness**: TDCFR··S
- **purpose**: Provide async versions of From, Into, TryFrom, and TryInto
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, async, from, into
- **categories**: algorithms, development-tools
- **pitch**: Async versions of the standard conversion traits — From, Into, TryFrom, TryInto — for types that need to await.

#### `async_tools`
- **module**: deprecated
- **layer**: 5 · Collections
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFREMS
- **purpose**: Re-export async_from conversion traits and async_trait macro via a unified namespace facade
- **deps**: 2 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: no
- **keywords**: async, conversion, from, into, async-trait
- **categories**: algorithms, development-tools
- **pitch**: One import gives access to all four async conversion traits — AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto — and the async_trait macro, without managing separate async_from and async-trait dependencies.

#### `collection_tools`
- **module**: core
- **layer**: 5 · Collections
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
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
- **state**: experimental
- **target**: experimental
- **readiness**: TDCFREMS
- **purpose**: Apply any macro to every item in a compile-time list
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.10.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose
- **categories**: algorithms, development-tools
- **pitch**: Apply any macro to every item in a compile-time list — fills the gap macro_rules! can't close for list-driven codegen.

#### `iter_tools`
- **module**: deprecated
- **layer**: 5 · Collections
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·RE·S
- **purpose**: DEPRECATED — functionality migrated to macro_tools (IterTrait, BoxedIter) and direct itertools dependency
- **deps**: 2 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.50.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, iterator, itertools, combinators
- **categories**: algorithms, development-tools
- **pitch**: DEPRECATED. Use itertools directly for combinators; use macro_tools for IterTrait and BoxedIter types.

#### `wtools`
- **module**: core
- **layer**: 5 · Collections
- **state**: experimental
- **target**: stable
- **readiness**: T·CFREMS
- **purpose**: Thin collections aggregator; re-exports collection_tools macros and constructors
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.20
- **no_std**: yes
- **keywords**: collections, hashmap, hashset, general-purpose
- **categories**: data-structures, development-tools
- **pitch**: Single entry-point for collection utilities — add wtools and get all collection_tools macros (hmap!, hset!, …) with no_std/hashbrown support by default.

---

### Layer 6 · String & Format

#### `cli_fmt`
- **module**: core
- **layer**: 6 · String & Format
- **state**: experimental
- **target**: stable
- **readiness**: TDCFR·MS
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
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Render structured data in multiple visual, text, and serialization formats
- **deps**: 9 · **int**: 3 · **=L**: 2 · **↓L**: 1 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: table, formatter, data-format, visualization, tree
- **categories**: text-processing, data-structures, visualization
- **pitch**: Multi-format data rendering — 10 formatters, 33 variants from aligned tables to JSON/YAML/HTML/SQL — with granular feature flags for minimal binary size.

#### `data_type`
- **module**: deprecated
- **layer**: 6 · String & Format
- **state**: experimental
- **target**: deprecated
- **readiness**: TDC·RE·S
- **purpose**: Aggregate interval, collection, and sum-type re-exports under one unified facade namespace
- **deps**: 3 · **int**: 2 · **=L**: 0 · **↓L**: 2 · **↑L**: 0
- **version**: 0.25.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, interval, collections, facade
- **categories**: algorithms, development-tools
- **pitch**: One import for Either sum types, interval ranges, and collection macros from a single versioned facade crate.

#### `format_tools`
- **module**: deprecated
- **layer**: 6 · String & Format
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFRE·S
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
- **state**: experimental
- **target**: experimental
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
- **readiness**: TDCFREMS
- **purpose**: Split, indent, and transform strings with ANSI utilities and SIMD acceleration
- **deps**: 9 · **int**: 2 · **=L**: 0 · **↓L**: 2 · **↑L**: 0
- **version**: 0.45.0
- **no_std**: yes
- **keywords**: fundamental, string, split, ansi, simd
- **categories**: algorithms, development-tools
- **pitch**: Comprehensive string toolkit — splitting, indentation, ANSI terminal utilities, command parsing, number parsing, and parser integration with optional SIMD acceleration.

#### `wstring_tools`
- **module**: deprecated
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
- **readiness**: TDC·R·MS
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
- **readiness**: TDCFREMS
- **purpose**: Resolve hierarchical config across runtime, env, file, and default sources
- **deps**: 6 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.5.0
- **no_std**: no
- **keywords**: config, configuration, hierarchy, yaml, settings
- **categories**: config, filesystem
- **pitch**: Trait-based 6-level configuration resolution with source tracking, type detection, and atomic file operations — every value knows where it came from.

#### `file_tools`
- **module**: deprecated
- **layer**: 7 · Path & Process
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDC·REMS
- **purpose**: RAII temp directories, glob re-export, and upward path traversal
- **deps**: 1 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: temp-dir, glob, path-traversal, filesystem, raii
- **categories**: filesystem, development-tools
- **pitch**: RAII-scoped temp directories that clean up on drop, re-exported glob pattern matching, and five utilities for locating files and directories by walking upward through the directory tree.

#### `process_tools`
- **module**: core
- **layer**: 7 · Path & Process
- **state**: stable
- **target**: stable
- **readiness**: TDCFR·MS
- **purpose**: Run subprocesses with captured I/O, probe process liveness, and daemonize Unix services
- **deps**: 6 · **int**: 4 · **=L**: 0 · **↓L**: 4 · **↑L**: 0
- **version**: 0.32.0
- **no_std**: no
- **keywords**: process, subprocess, spawn, daemonize, lifecycle
- **categories**: os, development-tools
- **pitch**: Builder API for subprocess execution with full stdout/stderr capture, signal mapping, process liveness probing, and Unix daemonization — complete process lifecycle management in one crate.

#### `program_tools`
- **module**: experimental
- **layer**: 7 · Path & Process
- **state**: experimental
- **target**: experimental
- **readiness**: TDCFREMS
- **purpose**: Rust script runner — compile and execute Rust files as scripts with output capture
- **deps**: 3 · **int**: 3 · **=L**: 0 · **↓L**: 3 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: no
- **keywords**: script, runner, testing, execute, compile
- **categories**: development-tools::testing, development-tools::build-utils
- **pitch**: Run any Rust file or Cargo project as a script with one call — builds the execution plan, invokes Cargo, and returns stdout/stderr with assertion methods designed for test code.

#### `pth`
- **module**: core
- **layer**: 7 · Path & Process
- **state**: stable
- **target**: stable
- **readiness**: TDC·R··S
- **purpose**: Syntactic path manipulation — normalize, join, query extensions, and typed path wrappers
- **deps**: 5 · **int**: 2 · **=L**: 0 · **↓L**: 2 · **↑L**: 0
- **version**: 0.37.0
- **no_std**: yes
- **keywords**: fundamental, general-purpose, path, normalize, join
- **categories**: algorithms, development-tools
- **pitch**: Purely syntactic path manipulation — normalize, join, and query paths without filesystem access, with AbsolutePath and NormalizedPath types that catch path-category bugs at compile time.

#### `workspace_tools`
- **module**: core
- **layer**: 7 · Path & Process
- **state**: experimental
- **target**: stable
- **readiness**: TDCFR·MS
- **purpose**: Resolve workspace-relative paths and optionally load configuration, manage secrets, and discover resources
- **deps**: 11 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.12.0
- **no_std**: no
- **keywords**: workspace, path, cargo, secrets, config
- **categories**: filesystem, development-tools
- **pitch**: One crate resolves your Cargo workspace root from any context — tests, CI, or installed binary — and optionally loads TOML/JSON/YAML config, manages memory-safe secrets, and discovers resources via glob patterns.

#### `fs_tools`
- **module**: deprecated
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
- **state**: experimental
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Download and decode .crate archives for in-memory content inspection
- **deps**: 3 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.25.0
- **no_std**: no
- **keywords**: crate, archive, tar, gzip, inspection
- **categories**: development-tools
- **pitch**: Download and decode .crate archives for in-memory file listing and byte extraction — no filesystem extraction, no manual tar wrestling.

#### `genfile_core`
- **module**: core
- **layer**: 8 · Tooling
- **state**: stable
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Render parameterized template archives to output files with a pluggable engine and file system abstraction
- **deps**: 9 · **int**: 3 · **=L**: 0 · **↓L**: 3 · **↑L**: 0
- **version**: 0.10.0
- **no_std**: no
- **keywords**: fundamental, general-purpose, template, codegen
- **categories**: development-tools, template-engine
- **pitch**: Self-contained template archive engine — embed template files with parameters and values, serialize to JSON/YAML, and materialize output files with a pluggable Handlebars renderer.

#### `genfile`
- **module**: core
- **layer**: 8 · Tooling
- **state**: experimental
- **target**: stable
- **readiness**: TDCFREMS
- **purpose**: Manage code generation template archives from the command line
- **deps**: 4 · **int**: 3 · **=L**: 1 · **↓L**: 2 · **↑L**: 0
- **version**: 0.4.0
- **no_std**: no
- **keywords**: template, codegen, cli, scaffolding, generator
- **categories**: command-line-utilities, development-tools
- **pitch**: Create and materialize code generation template archives from the command line — genfile_core made interactive.

#### `multiline_input` (core)
- **module**: deprecated
- **layer**: 8 · Tooling
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFR·MS
- **purpose**: Collect multiline terminal input with interactive key editing and Builder-configured validation
- **deps**: 4 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: terminal, input, multiline, cli, interactive
- **categories**: command-line-interface, text-editors
- **pitch**: Collect multiline terminal text with ENTER-to-submit, CTRL+ENTER newlines, and Builder-configured validation — excluded from workspace (superseded by module/experimental version).

#### `multiline_input` (experimental)
- **module**: experimental
- **layer**: 8 · Tooling
- **state**: experimental
- **target**: stable
- **readiness**: T·CFREMS
- **purpose**: Collect multiline terminal input with interactive key editing and Builder-configured validation
- **deps**: 4 · **int**: 1 · **=L**: 0 · **↓L**: 1 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: terminal, input, multiline, cli, interactive
- **categories**: command-line-interface, text-editors
- **pitch**: The active workspace version of multiline_input — ENTER to submit, CTRL+ENTER for newlines, Builder API, 4 usage examples, and 9 test files; blocked on docs/ instances (D criterion).

#### `test_tools`
- **module**: experimental
- **layer**: 8 · Tooling
- **state**: experimental
- **target**: stable
- **readiness**: TDCFRE··
- **purpose**: Aggregate workspace testing utilities into one dev-dependency via a circular-dependency-safe standalone build mode
- **deps**: 11 · **int**: 3 · **=L**: 0 · **↓L**: 3 · **↑L**: 0
- **version**: 0.16.0
- **no_std**: no
- **keywords**: testing, test-tools, trybuild, smoke-testing, harness
- **categories**: development-tools::testing, development-tools
- **pitch**: Drop in one dev-dependency and get trybuild, impls_index macros, SmokeModuleTest, and collection utilities — with a standalone build mode that prevents circular dependencies across the workspace.

#### `wca`
- **module**: experimental
- **layer**: 8 · Tooling
- **state**: experimental
- **target**: stable
- **readiness**: TDCFRE·S
- **purpose**: Aggregate CLI commands as typed Rust routines with a parse-verify-execute pipeline
- **deps**: 7 · **int**: 4 · **=L**: 0 · **↓L**: 4 · **↑L**: 0
- **version**: 0.46.0
- **no_std**: no
- **keywords**: cli, command, aggregator, cui, wtools
- **categories**: command-line-interface, command-line-utilities
- **pitch**: Aggregate CLI commands as typed Rust routines — route input through a three-stage parse-verify-execute pipeline, check argument types automatically, generate help, share execution context across routines, and optionally activate fuzzy typo correction.

---

### Layer 9 · Application

#### `sqlx_query`
- **module**: deprecated
- **layer**: 9 · Application
- **state**: experimental
- **target**: deprecated
- **readiness**: TDCFR·MS
- **purpose**: Switch between SQLx compile-time and runtime query macros by feature
- **deps**: 0 · **int**: 0 · **=L**: 0 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.1
- **no_std**: yes
- **keywords**: sqlx, sql, query, compile-time, feature-flag
- **categories**: database, development-tools
- **pitch**: Feature-flag switch between SQLx compile-time `query!` and runtime `query` — same call site, swappable modes.

#### `unitore`
- **module**: experimental
- **layer**: 9 · Application
- **state**: experimental
- **target**: stable
- **readiness**: TDCFR··S
- **purpose**: Subscribe to RSS and Atom feeds from TOML configs, persist entries in embedded SQL, and browse from the terminal
- **deps**: 20 · **int**: 3 · **=L**: 0 · **↓L**: 3 · **↑L**: 0
- **version**: 0.1.0
- **no_std**: no
- **keywords**: rss-feed, atom-feed, subscribe, terminal, reader
- **categories**: network-programming, command-line-utilities
- **pitch**: Subscribe to RSS and Atom feeds from TOML config files, persist entries to an embedded GlueSQL database, and browse or query results from the terminal.

#### `willbe`
- **module**: experimental
- **layer**: 9 · Application
- **state**: experimental
- **target**: stable
- **readiness**: TDCFR···
- **purpose**: Publish multi-crate workspaces in topological order, run feature-matrix tests, generate CI/CD workflows, and renew readme headers
- **deps**: 40 · **int**: 14 · **=L**: 0 · **↓L**: 14 · **↑L**: 0
- **version**: 0.35.0
- **no_std**: no
- **keywords**: workspace, publish, cargo, cicd, consistency
- **categories**: development-tools, development-tools::cargo-plugins
- **pitch**: Manage your entire Cargo workspace with one tool: publish in topological order, run tests across feature matrices, auto-generate CI/CD workflows, and keep readmes and headers in sync.

#### `willbe2`
- **module**: deprecated
- **layer**: 9 · Application
- **state**: deprecated
- **target**: deprecated
- **readiness**: TDCFR·MS
- **purpose**: Expose the willbe workspace tool under the willbe2 binary and crate name via complete re-export
- **deps**: 1 · **int**: 1 · **=L**: 1 · **↓L**: 0 · **↑L**: 0
- **version**: 0.2.0
- **no_std**: no
- **keywords**: willbe, alias, workspace, publish, binary
- **categories**: development-tools, development-tools::cargo-plugins
- **pitch**: Install willbe2 to run the full willbe workspace tool under the willbe2 binary name — all API and CLI functionality is delegated entirely to willbe.

#### `proper_tools`
- **module**: deprecated
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
