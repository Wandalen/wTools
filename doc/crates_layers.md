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

| Crate | Module | L# | Layer | Purpose |
|-------|--------|----|-------|---------|
| `error_tools` | core | 1 | Foundation | Provide a unified error handling namespace across the workspace |
| `data_type` | experimental | 1 | Foundation | Supply foundational type aliases and primal data structures |
| `diagnostics_tools` | experimental | 1 | Foundation | Provide runtime assertion helpers with rich diagnostic context |
| `mem_tools` | experimental | 1 | Foundation | Offer safe memory introspection and alignment utilities |
| `asbytes` | core | 2 | Primitives | Enable zero-copy viewing of POD types as byte slices |
| `deterministic_rand` | core | 2 | Primitives | Generate hierarchical seeded random numbers with switchable determinism |
| `implements` | experimental | 2 | Primitives | Answer at compile time whether a type implements a trait |
| `inspect_type` | core | 2 | Primitives | Print exact Rust type names and sizes at compile time |
| `interval_adapter` | experimental | 2 | Primitives | Unify open, closed, and half-open range types behind one adapter |
| `is_slice` | experimental | 2 | Primitives | Answer at compile time whether an expression is a slice |
| `time_tools` | experimental | 2 | Primitives | Provide minimal time measurement and timestamp utilities |
| `typing_tools` | experimental | 2 | Primitives | Express compile-time type constraints and type guards |
| `winterval` | experimental | 2 | Primitives | Re-export interval_adapter as a standalone dependency |
| `macro_tools` | core | 3 | Macro Framework | Supply all primitives needed to author procedural macros |
| `meta_tools` | experimental | 3 | Macro Framework | Provide token-level macro utilities for metaprogramming |
| `clone_dyn_meta` | core | 3 | Macro Framework | Generate clone_dyn derive implementation (use clone_dyn directly) |
| `component_model_meta` | experimental | 3 | Macro Framework | Generate component_model derive implementation (use component_model directly) |
| `derive_tools_meta` | core | 3 | Macro Framework | Generate derive_tools implementations (use derive_tools directly) |
| `former_meta` | core | 3 | Macro Framework | Generate former builder derive implementation (use former directly) |
| `impls_index_meta` | core | 3 | Macro Framework | Generate impls_index macro wrappers (use impls_index directly) |
| `mod_interface_meta` | core | 3 | Macro Framework | Generate mod_interface namespace macros (use mod_interface directly) |
| `reflect_tools_meta` | experimental | 3 | Macro Framework | Generate reflect_tools introspection code (use reflect_tools directly) |
| `strs_tools_meta` | core | 3 | Macro Framework | Generate strs_tools compile-time operations (use strs_tools directly) |
| `variadic_from_meta` | core | 3 | Macro Framework | Generate variadic_from From impls (use variadic_from directly) |
| `clone_dyn` | core | 4 | Patterns | Make dyn trait objects cloneable via a single derive macro |
| `clone_dyn_types` | core | 4 | Patterns | Expose shared trait contracts consumed by clone_dyn users |
| `component_model` | experimental | 4 | Patterns | Enable type-driven field assignment on complex objects |
| `component_model_types` | experimental | 4 | Patterns | Expose shared traits for the component_model pattern |
| `derive_tools` | core | 4 | Patterns | Add Into, TryInto, IsVariant, and other missing std derives |
| `former` | core | 4 | Patterns | Build complex objects with nested subformers via one derive |
| `former_types` | core | 4 | Patterns | Expose compile-time trait contracts reused by former consumers |
| `impls_index` | core | 4 | Patterns | Wrap impl methods in named macros for navigable indexing |
| `mod_interface` | core | 4 | Patterns | Replace dozens of pub use declarations with a single macro |
| `reflect_tools` | experimental | 4 | Patterns | Inspect struct fields by name and type at runtime |
| `variadic_from` | core | 4 | Patterns | Derive From implementations for tuples of 1 to N elements |
| `async_from` | experimental | 5 | Collections | Provide async versions of From, Into, TryFrom, and TryInto |
| `async_tools` | experimental | 5 | Collections | Supply practical helpers for async task spawning and joining |
| `collection_tools` | core | 5 | Collections | Create std collections inline with ergonomic literal macros |
| `for_each` | experimental | 5 | Collections | Apply any macro to every item in a compile-time list |
| `iter_tools` | experimental | 5 | Collections | Expose the full itertools combinator library via workspace facade |
| `cli_fmt` | core | 6 | String & Format | Structure and colorize CLI terminal output consistently |
| `color_tools` | core | 6 | String & Format | Add ANSI color and text escape formatting to terminal output |
| `data_fmt` | core | 6 | String & Format | Render data as aligned tables and nested tree structures |
| `format_tools` | core | 6 | String & Format | Extend std formatting with structural display and string helpers |
| `include_md` | experimental | 6 | String & Format | Include a markdown file or named section at compile time |
| `strs_tools` | core | 6 | String & Format | Manipulate strings with splitting, indentation, and pattern tools |
| `wstring_tools` | alias | 6 | String & Format | Alias — recommended single dependency for all string utilities |
| `config_hierarchy` | core | 7 | Path & Process | Load layered YAML config with environment variable overrides |
| `config_hierarchy` | experimental | 7 | Path & Process | Evolve config_hierarchy with experimental extensions |
| `fs_tools` | experimental | 7 | Path & Process | Read, write, and traverse files with ergonomic error context |
| `process_tools` | core | 7 | Path & Process | Spawn child processes and capture output reliably |
| `program_tools` | experimental | 7 | Path & Process | Compile and run a Rust source file on demand |
| `pth` | core | 7 | Path & Process | Normalize, resolve, and join paths with workspace-aware helpers |
| `workspace_tools` | core | 7 | Path & Process | Resolve paths relative to workspace root from any execution context |
| `file_tools` | alias | 7 | Path & Process | Alias — recommended single dependency for all filesystem utilities |
| `benchkit` | experimental | 8 | Tooling | Benchmark performance and publish markdown reports |
| `crates_tools` | core | 8 | Tooling | Parse Cargo.toml and analyze crate metadata programmatically |
| `genfile_core` | core | 8 | Tooling | Materialize project scaffolding from versioned template archives |
| `genfile` | core | 8 | Tooling | Manage code generation template archives from the command line |
| `multiline_input` | core | 8 | Tooling | Read multi-line terminal input with readline and paste handling |
| `multiline_input` | experimental | 8 | Tooling | Evolve multiline_input with experimental input handling |
| `test_tools` | experimental | 8 | Tooling | Provide rich assertions and test organization for nextest |
| `wca` | experimental | 8 | Tooling | Define CLI commands as Rust functions with help and errors built in |
| `sqlx_query` | experimental | 9 | Application | Switch between SQLx compile-time and runtime query macros by feature |
| `unitore` | experimental | 9 | Application | Subscribe to RSS and Atom feeds with configurable update intervals |
| `willbe` | experimental | 9 | Application | Publish, version-bump, and consistency-check a Cargo workspace |
| `willbe2` | experimental | 9 | Application | Reimagine willbe with improved architecture |
| `wtools` | experimental | 9 | Application | Aggregate the complete workspace toolkit in one dependency |
| `proper_tools` | alias | 9 | Application | Alias — recommended starting point for general-purpose wTools use |

## Crate Profiles

Per-crate attributes for promotion and publishing.

**Schema:**
- **version** — current version from Cargo.toml; tracks release state
- **status** — adoption risk tier: `stable` (core, high version), `experimental` (experimental/), `alpha` (early, 0.1.x), `alias` (pass-through re-export)
- **no_std** — whether the `no_std` feature is declared in Cargo.toml
- **keywords** — up to 5 crates.io search terms; recommended values for optimal discovery
- **categories** — up to 2 crates.io browse paths
- **pitch** — one sentence written for a potential adopter on crates.io

**Excluded:** `license` (all MIT, no signal), `msrv` (all workspace 1.61, no signal), `published` (none set `publish = false`; live status requires crates.io API), `docs`/`homepage` (fully derivable: `docs.rs/{name}`, GitHub repo path).

---

### Layer 1 · Foundation

#### `error_tools`
- **version**: 0.39.0
- **status**: stable
- **no_std**: yes
- **keywords**: error-handling, anyhow, thiserror, facade, workspace
- **categories**: algorithms, development-tools
- **pitch**: One import replaces separate anyhow and thiserror dependencies across your entire workspace with no runtime overhead.

#### `data_type`
- **version**: 0.25.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, data-types, primitives, type-aliases
- **categories**: algorithms, development-tools
- **pitch**: Foundational type aliases and primal data structures shared across the entire workspace.

#### `diagnostics_tools`
- **version**: 0.11.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, diagnostics, assertions, runtime
- **categories**: algorithms, development-tools
- **pitch**: Runtime assertion helpers that produce richer failure messages than std asserts — context without boilerplate.

#### `mem_tools`
- **version**: 0.9.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, memory, alignment, size
- **categories**: algorithms, development-tools
- **pitch**: Safe memory introspection utilities — alignment checks, size comparisons, and copy helpers without unsafe.

---

### Layer 2 · Primitives

#### `asbytes`
- **version**: 0.2.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, bytes, pod, bytemuck
- **categories**: algorithms, development-tools, data-structures
- **pitch**: View any POD type as a byte slice, zero-copy and without unsafe — bytemuck-backed, two-line setup.

#### `deterministic_rand`
- **version**: 0.7.0
- **status**: stable
- **no_std**: yes
- **keywords**: fundamental, general-purpose, random, deterministic, seedable
- **categories**: algorithms, development-tools
- **pitch**: Hierarchical seeded RNG — swap between deterministic and OS-random with one flag and no code changes.

#### `implements`
- **version**: 0.13.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, trait, implements, compile-time
- **categories**: algorithms, development-tools
- **pitch**: `implements!(MyType, Display)` — zero-cost compile-time check whether a type implements a trait.

#### `inspect_type`
- **version**: 0.16.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, diagnostic-purpose, type-name, compile-time
- **categories**: algorithms, development-tools
- **pitch**: Print the exact Rust type of any expression at compile time — the fastest type-debugging shortcut.

#### `interval_adapter`
- **version**: 0.42.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, interval, range, bounds
- **categories**: algorithms, development-tools
- **pitch**: One trait unifies all Rust range types — open, closed, half-open — interchangeable behind a single adapter.

#### `is_slice`
- **version**: 0.14.0
- **status**: experimental
- **no_std**: no
- **keywords**: fundamental, general-purpose, slice, array, type-check
- **categories**: algorithms, development-tools
- **pitch**: `is_slice!(x)` — compile-time check whether an expression is a slice, for use in type guards.

#### `time_tools`
- **version**: 0.2.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, time, timestamp, duration
- **categories**: algorithms, development-tools
- **pitch**: Minimal time utilities — current timestamp, elapsed measurement, and instant comparisons in one crate.

#### `typing_tools`
- **version**: 0.11.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, type-system, compile-time, constraints
- **categories**: algorithms, development-tools
- **pitch**: Compile-time type guards and constraint macros — extend Rust's type system expressively without proc-macros.

#### `winterval`
- **version**: 0.3.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, interval, range, bounds
- **categories**: algorithms, development-tools
- **pitch**: Interval adapter as a standalone dependency — identical capability to interval_adapter, convenient alias.

---

### Layer 3 · Macro Framework

#### `macro_tools`
- **version**: 0.85.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, proc-macro, syn, quote
- **categories**: algorithms, development-tools
- **pitch**: Everything a proc-macro author needs — syn parsing, token generation, and structured error reporting in one toolkit.

#### `meta_tools`
- **version**: 0.12.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, meta, token, stringify
- **categories**: algorithms, development-tools
- **pitch**: Token-level utility macros for metaprogramming — stringify, concatenate, and count macro arguments.

#### `clone_dyn_meta`
- **version**: 0.58.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, clone, proc-macro, derive
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for clone_dyn — do not depend on directly; add clone_dyn to your Cargo.toml instead.

#### `component_model_meta`
- **version**: 0.17.0
- **status**: experimental
- **no_std**: no
- **keywords**: fundamental, general-purpose, builder-pattern, component, proc-macro
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for component_model — do not depend on directly; use component_model instead.

#### `derive_tools_meta`
- **version**: 0.63.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, derive, proc-macro, codegen
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for derive_tools — do not depend on directly; use derive_tools instead.

#### `former_meta`
- **version**: 2.43.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, builder-pattern, derive, proc-macro
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for former — do not depend on directly; use former instead.

#### `impls_index_meta`
- **version**: 0.13.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, proc-macro, impl, namespace
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for impls_index — do not depend on directly; use impls_index instead.

#### `mod_interface_meta`
- **version**: 0.59.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, builder-pattern, module, proc-macro
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for mod_interface — do not depend on directly; use mod_interface instead.

#### `reflect_tools_meta`
- **version**: 0.7.0
- **status**: experimental
- **no_std**: no
- **keywords**: fundamental, general-purpose, reflection, proc-macro, introspection
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for reflect_tools — do not depend on directly; use reflect_tools instead.

#### `strs_tools_meta`
- **version**: 0.18.0
- **status**: stable
- **no_std**: no
- **keywords**: procedural-macro, compile-time, optimization, string, codegen
- **categories**: development-tools
- **pitch**: Proc-macro backend for strs_tools — do not depend on directly; use strs_tools instead.

#### `variadic_from_meta`
- **version**: 0.30.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, from, variadic, proc-macro
- **categories**: algorithms, development-tools
- **pitch**: Proc-macro backend for variadic_from — do not depend on directly; use variadic_from instead.

---

### Layer 4 · Patterns

#### `clone_dyn`
- **version**: 0.62.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, clone, dyn, trait-object
- **categories**: algorithms, development-tools
- **pitch**: `#[derive(CloneDyn)]` makes any trait object cloneable — eliminates the `Box<dyn Trait + Clone>` boilerplate entirely.

#### `clone_dyn_types`
- **version**: 0.48.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, clone, dyn, trait-object
- **categories**: algorithms, development-tools
- **pitch**: Shared trait contracts for clone_dyn — import when you need to name clone_dyn types in your own interfaces.

#### `component_model`
- **version**: 0.17.0
- **status**: experimental
- **no_std**: yes
- **keywords**: builder-pattern, type-safe, zero-cost, fluent-api, configuration
- **categories**: rust-patterns, development-tools
- **pitch**: Build complex objects by assigning fields by type — no named setters, no boilerplate, zero runtime cost.

#### `component_model_types`
- **version**: 0.27.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, builder-pattern, component, types
- **categories**: algorithms, development-tools
- **pitch**: Shared trait contracts for component_model — import when you need to name component_model types in interfaces.

#### `derive_tools`
- **version**: 0.65.0
- **status**: stable
- **no_std**: yes
- **keywords**: fundamental, general-purpose, derive, into, from
- **categories**: algorithms, development-tools
- **pitch**: The std-extending derive collection — Into, TryInto, IsVariant, From, and more via `#[derive]` with no boilerplate.

#### `former`
- **version**: 2.45.0
- **status**: stable
- **no_std**: yes
- **keywords**: fundamental, general-purpose, builder-pattern, derive, nested
- **categories**: algorithms, development-tools
- **pitch**: Builder pattern with one derive — nested subformers, collection builders, and computed defaults, no manual code.

#### `former_types`
- **version**: 2.38.0
- **status**: stable
- **no_std**: yes
- **keywords**: fundamental, general-purpose, builder-pattern, types, traits
- **categories**: algorithms, development-tools
- **pitch**: Compile-time trait contracts for former — import when you need to name builder types in your own interfaces.

#### `impls_index`
- **version**: 0.11.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, impl, index, namespace
- **categories**: algorithms, development-tools
- **pitch**: Wrap every method in a named macro so IDE navigation, docs, and grep all work on large impl blocks.

#### `mod_interface`
- **version**: 0.61.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, module, interface, visibility
- **categories**: algorithms, development-tools
- **pitch**: One macro replaces dozens of `pub use` declarations — clean layered module interfaces in a single call.

#### `reflect_tools`
- **version**: 0.7.0
- **status**: experimental
- **no_std**: no
- **keywords**: fundamental, general-purpose, reflection, introspection, struct
- **categories**: algorithms, development-tools
- **pitch**: Inspect struct field names and types at runtime — lightweight reflection without unsafe or external ABI.

#### `variadic_from`
- **version**: 0.59.0
- **status**: stable
- **no_std**: yes
- **keywords**: fundamental, general-purpose, from, variadic, tuple
- **categories**: algorithms, development-tools
- **pitch**: `#[derive(VariadicFrom)]` generates From implementations for 1-element through N-element tuples automatically.

---

### Layer 5 · Collections

#### `async_from`
- **version**: 0.2.0
- **status**: experimental
- **no_std**: no
- **keywords**: fundamental, general-purpose, async, from, into
- **categories**: algorithms, development-tools
- **pitch**: Async versions of the standard conversion traits — From, Into, TryFrom, TryInto — for types that need to await.

#### `async_tools`
- **version**: 0.1.0
- **status**: alpha
- **no_std**: no
- **keywords**: fundamental, general-purpose, async, tokio, runtime
- **categories**: algorithms, development-tools
- **pitch**: Practical async helpers — runtime detection, task spawning, and join utilities that work across runtimes.

#### `collection_tools`
- **version**: 0.38.0
- **status**: stable
- **no_std**: yes
- **keywords**: fundamental, general-purpose, collections, hashmap, hashset
- **categories**: algorithms, development-tools
- **pitch**: `hmap!{ "key" => val }` and friends — ergonomic inline macros for creating any std collection literal.

#### `for_each`
- **version**: 0.10.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, macro, for-each, list
- **categories**: algorithms, development-tools
- **pitch**: Apply any macro to every item in a compile-time list — fills the gap macro_rules! can't close for list-driven codegen.

#### `iter_tools`
- **version**: 0.50.0
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, iterator, itertools, combinators
- **categories**: algorithms, development-tools
- **pitch**: The full itertools combinator library re-exported through the workspace facade — one dependency, all combinators.

---

### Layer 6 · String & Format

#### `cli_fmt`
- **version**: 0.4.0
- **status**: stable
- **no_std**: no
- **keywords**: cli, command-line, output, formatting, display
- **categories**: command-line-utilities, development-tools
- **pitch**: Structure CLI terminal output into tables, sections, and indented blocks — consistent formatting without a heavy framework.

#### `color_tools`
- **version**: 0.4.0
- **status**: stable
- **no_std**: no
- **keywords**: ansi, color, terminal, text, escape
- **categories**: text-processing, command-line-interface
- **pitch**: ANSI terminal color without the bloat — just the escape code utilities you need, zero transitive dependencies.

#### `data_fmt`
- **version**: 0.2.0
- **status**: alpha
- **no_std**: no
- **keywords**: table, formatter, data-format, visualization, tree
- **categories**: text-processing, data-structures
- **pitch**: Render any data as an aligned table or nested tree — visual inspection of complex structures with zero setup.

#### `format_tools`
- **version**: 0.6.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, format, serialization, display
- **categories**: algorithms, development-tools
- **pitch**: Formatting utilities that extend std — structural display, aligned output, and string serialization helpers.

#### `include_md`
- **version**: 0.1.0
- **status**: alpha
- **no_std**: yes
- **keywords**: fundamental, general-purpose, markdown, include, compile-time
- **categories**: algorithms, development-tools
- **pitch**: Include a markdown file — or just one named section of it — verbatim into source at compile time.

#### `strs_tools`
- **version**: 0.45.0
- **status**: stable
- **no_std**: yes
- **keywords**: fundamental, general-purpose, string, split, trim
- **categories**: algorithms, development-tools
- **pitch**: String utilities that std forgot — flexible splitting, indentation stripping, and pattern-based manipulation.

#### `wstring_tools`
- **version**: 0.2.0
- **status**: alias
- **no_std**: yes
- **keywords**: fundamental, general-purpose, string, manipulation, utilities
- **categories**: algorithms, development-tools
- **pitch**: All wTools string utilities in one alias — the recommended single dependency for all string handling needs.

---

### Layer 7 · Path & Process

#### `config_hierarchy` (core)
- **version**: 0.4.0
- **status**: stable
- **no_std**: no
- **keywords**: config, configuration, hierarchy, yaml, settings
- **categories**: config, filesystem
- **pitch**: Load layered configuration from YAML files with environment variable overrides — settings that compose cleanly.

#### `config_hierarchy` (experimental)
- **version**: 0.5.0
- **status**: experimental
- **no_std**: no
- **keywords**: config, configuration, hierarchy, yaml, settings
- **categories**: config, filesystem
- **pitch**: Experimental evolution of config_hierarchy — next-generation layered configuration with extended capabilities.

#### `fs_tools`
- **version**: 0.2.0
- **status**: experimental
- **no_std**: no
- **keywords**: fundamental, general-purpose, filesystem, file, path
- **categories**: algorithms, development-tools
- **pitch**: File system utilities with ergonomic error context — read, write, and traverse files with clear failure messages.

#### `process_tools`
- **version**: 0.32.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, process, spawn, shell
- **categories**: algorithms, development-tools
- **pitch**: Spawn child processes and capture their output reliably — ergonomic wrappers with clear exit code handling.

#### `program_tools`
- **version**: 0.1.0
- **status**: alpha
- **no_std**: no
- **keywords**: fundamental, general-purpose, compile, run, program
- **categories**: algorithms, development-tools
- **pitch**: Compile a Rust source file on the fly and run it — the missing link for code generation pipelines.

#### `pth`
- **version**: 0.37.0
- **status**: stable
- **no_std**: yes
- **keywords**: fundamental, general-purpose, path, normalize, resolve
- **categories**: algorithms, development-tools
- **pitch**: Path manipulation utilities — normalize, resolve, and join paths with workspace-aware helpers.

#### `workspace_tools`
- **version**: 0.12.0
- **status**: stable
- **no_std**: no
- **keywords**: workspace, path, cargo, secrets, config
- **categories**: filesystem, development-tools
- **pitch**: Find your workspace root reliably from any execution context — tests, scripts, and CI — and resolve paths from it.

#### `file_tools`
- **version**: 0.1.0
- **status**: alias
- **no_std**: no
- **keywords**: fundamental, general-purpose, filesystem, file, path
- **categories**: algorithms, development-tools
- **pitch**: All wTools file system utilities in one alias — the recommended single dependency for all file handling needs.

---

### Layer 8 · Tooling

#### `benchkit`
- **version**: 0.20.0
- **status**: experimental
- **no_std**: no
- **keywords**: benchmark, performance, toolkit, markdown, reports
- **categories**: development-tools, development-tools::profiling
- **pitch**: Benchmark Rust code and publish markdown performance reports — minimal setup, actionable output, no harness lock-in.

#### `crates_tools`
- **version**: 0.25.0
- **status**: stable
- **no_std**: no
- **keywords**: crates, cargo, toml, metadata, analysis
- **categories**: development-tools, development-tools::cargo-plugins
- **pitch**: Parse Cargo.toml files and analyze crate metadata programmatically — no shell-outs, no manual TOML wrestling.

#### `genfile_core`
- **version**: 0.10.0
- **status**: stable
- **no_std**: no
- **keywords**: fundamental, general-purpose, template, codegen, scaffolding
- **categories**: development-tools, template-engine
- **pitch**: Template-driven code generation — materialize project scaffolding from versioned template archives programmatically.

#### `genfile`
- **version**: 0.4.0
- **status**: stable
- **no_std**: no
- **keywords**: template, codegen, cli, scaffolding, generator
- **categories**: command-line-utilities, development-tools
- **pitch**: Create and materialize code generation template archives from the command line — genfile_core made interactive.

#### `multiline_input` (core)
- **version**: 0.2.0
- **status**: alpha
- **no_std**: no
- **keywords**: terminal, input, multiline, cli, interactive
- **categories**: command-line-interface, text-editors
- **pitch**: Read multi-line terminal input cleanly — handles paste, readline edge cases, and buffer flushing out of the box.

#### `multiline_input` (experimental)
- **version**: 0.2.0
- **status**: experimental
- **no_std**: no
- **keywords**: terminal, input, multiline, cli, interactive
- **categories**: command-line-interface, text-editors
- **pitch**: Experimental multi-line input handling — evolving version of the core multiline_input crate.

#### `test_tools`
- **version**: 0.16.0
- **status**: experimental
- **no_std**: no
- **keywords**: fundamental, general-purpose, testing, assertions, harness
- **categories**: algorithms, development-tools
- **pitch**: A complete test harness — rich assertions, test organization helpers, and nextest-compatible test infrastructure.

#### `wca`
- **version**: 0.46.0
- **status**: experimental
- **no_std**: no
- **keywords**: cli, command, aggregation, cui, interface
- **categories**: command-line-interface, command-line-utilities
- **pitch**: Define CLI commands as Rust functions and aggregate them — help generation, error handling, and dispatch built in.

---

### Layer 9 · Application

#### `sqlx_query`
- **version**: 0.2.1
- **status**: experimental
- **no_std**: no
- **keywords**: sqlx, sql, query, compile-time, feature-flag
- **categories**: database, development-tools
- **pitch**: Feature-flag switch between SQLx compile-time `query!` and runtime `query` — same call site, swappable modes.

#### `unitore`
- **version**: 0.1.0
- **status**: alpha
- **no_std**: no
- **keywords**: rss-feed, atom-feed, subscribe, terminal, reader
- **categories**: network-programming, command-line-utilities
- **pitch**: Subscribe to RSS and Atom feeds, configure per-feed update intervals, and browse entries from the terminal.

#### `willbe`
- **version**: 0.35.0
- **status**: experimental
- **no_std**: no
- **keywords**: workspace, publish, cargo, version, consistency
- **categories**: development-tools, development-tools::cargo-plugins
- **pitch**: Publish, version-bump, and consistency-check your entire Cargo workspace with a single command.

#### `willbe2`
- **version**: 0.2.0
- **status**: experimental
- **no_std**: no
- **keywords**: workspace, publish, cargo, multi-crate, automation
- **categories**: development-tools, development-tools::cargo-plugins
- **pitch**: Next-generation workspace publishing and consistency tool — willbe reimagined with improved architecture.

#### `wtools`
- **version**: 0.2.20
- **status**: experimental
- **no_std**: yes
- **keywords**: fundamental, general-purpose, toolkit, wtools, all-in-one
- **categories**: algorithms, development-tools
- **pitch**: The complete wTools suite in one dependency — import the entire workspace toolkit from a single crate.

#### `proper_tools`
- **version**: 0.1.0
- **status**: alias
- **no_std**: yes
- **keywords**: fundamental, general-purpose, toolkit, utilities, workspace
- **categories**: algorithms, development-tools
- **pitch**: A curated wTools utilities alias — the recommended starting point for projects needing general-purpose tools.
