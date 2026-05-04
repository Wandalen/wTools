# deprecated/

Excluded from the workspace (`Cargo.toml` exclude list). Not compiled, not tested, not published. Retained only to preserve git history before final deletion.

Two groups co-located here:
- **Architecture-tracked** (20): assigned layers in `doc/layers.md`; targeted for deprecation via reachability analysis
- **Legacy postponed** (22): pre-architecture crates that were never assigned layers; moved here from `module/postponed/`

## Architecture-Tracked (20)

| Path | Responsibility |
|------|----------------|
| `async_tools/` | Deprecated empty re-export of async_from |
| `data_type/` | Deprecated facade re-exporting collection_tools + interval_adapter + either |
| `diagnostics_tools/` | Deprecated assertion macros with colored diff output |
| `file_tools/` | Deprecated RAII temp dirs, glob, upward path traversal |
| `format_tools/` | Deprecated std formatting extensions |
| `fs_tools/` | Deprecated alias for file_tools |
| `impls_index/` | Deprecated impl-method named macro wrappers |
| `impls_index_meta/` | Deprecated proc-macro backend for impls_index |
| `interval_adapter/` | Deprecated interval trait coverage over range variants |
| `is_slice/` | Deprecated compile-time slice check (superseded by implements!) |
| `meta_tools/` | Deprecated macro bundle facade |
| `multiline_input/` | Deprecated core multiline terminal input (moved to experimental) |
| `proper_tools/` | Deprecated alias for general-purpose wTools |
| `sqlx_query/` | Deprecated sqlx feature-flag toggle |
| `time_tools/` | Deprecated UNIX epoch timestamp functions |
| `typing_tools/` | Deprecated type-checking aggregator |
| `willbe2/` | Deprecated transparent re-export of willbe |
| `winterval/` | Deprecated alias for interval_adapter |
| `wstring_tools/` | Deprecated alias for string utilities |
| `wtools/` | Deprecated ten-category utility aggregate |

## Legacy Postponed (22)

| Path | Responsibility |
|------|----------------|
| `automata_tools/` | Postponed automata / state-machine utilities |
| `cargo_will/` | Postponed cargo workspace publish tool (predecessor to willbe) |
| `fundamental_data_type/` | Postponed fundamental data type primitives |
| `graphs_tools/` | Postponed graph data structures and algorithms |
| `instance_of/` | Postponed instance-of type check macro |
| `multilayer/` | Postponed multilayer architecture utilities |
| `non_std/` | Postponed no-std compatibility shims |
| `plot_interface/` | Postponed plotting interface abstraction |
| `proc_macro_tools/` | Postponed proc-macro development utilities |
| `refiner/` | Postponed value refinement and validation utilities |
| `std_tools/` | Postponed std library extension utilities |
| `std_x/` | Postponed std extensions and replacements |
| `type_constructor/` | Postponed type construction macros |
| `unilang_instruction_parser/` | Postponed instruction parser for unilang |
| `_video_experiment/` | Postponed video/media experiment (crate: video_experiment) |
| `wautomata/` | Postponed automata variant (alias/predecessor of automata_tools) |
| `werror/` | Postponed error handling (predecessor to error_tools) |
| `wplot/` | Postponed plot utilities (predecessor to wplot/plot_interface) |
| `wproc_macro/` | Postponed proc-macro utilities (predecessor to proc_macro_tools) |
| `wpublisher/` | Postponed workspace publisher (predecessor to willbe) |
| `wtest/` | Postponed test utilities (predecessor to test_tools) |
| `wtest_basic/` | Postponed basic test utilities (predecessor to test_tools) |
