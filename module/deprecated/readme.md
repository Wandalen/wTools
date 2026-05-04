# deprecated/

Crates parked here after being marked deprecated in `doc/layers.md`. Excluded from the workspace (`Cargo.toml` exclude list). Not compiled, not tested, not published. Retained only to preserve git history and docs before final deletion.

## Responsibility Table

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
