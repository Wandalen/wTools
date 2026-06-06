# Decisions

## Design Decisions

| ID | Status | Question | Decision | Rationale |
|----|--------|----------|----------|-----------|
| Q-01 | Decided | How should `include_md!` resolve the path? | Emit `include_str!(path)` from the expanded code | Delegates path resolution to the compiler natively; no proc-macro file I/O needed; identical semantics to the standard built-in; size check done via emitted const assertion |
| Q-02 | Decided | How should `include_md_section!` resolve the path? | Use `CARGO_MANIFEST_DIR` at expansion time | `Span::call_site().source_file()` requires the unstable `proc_macro_span` feature; `CARGO_MANIFEST_DIR` (set by Cargo to the crate's manifest directory) is the stable alternative; resolves path relative to crate root — intentionally distinct from `include_md!`'s source-file-relative resolution; documented in `docs/invariant/001_path_resolution.md` |
| Q-03 | Decided | Which proc-macro framework to use? | `macro_tools` (workspace crate re-exporting `syn`, `quote`, `proc_macro2`) | Workspace convention; provides `syn_err!`/`return_syn_err!` error helpers and consistent patterns used across other workspace proc-macro crates |
| Q-04 | Decided | How to enforce the 10 MB size limit for `include_md!`? | Emit a const assertion: `assert!(include_bytes!(path).len() <= 10_000_000)` | No proc-macro file I/O required; compiler enforces the limit at compile time; the assertion is part of the emitted token stream |
| Q-05 | Decided | Should `include_md_section!` call `proc_macro::tracked_path::path()` to register the file for incremental rebuild? | No — use a code comment documenting the stable-Rust limitation | `proc_macro::tracked_path` requires `#![feature(track_path)]` (nightly-only as of Rust 1.94.1); plan incorrectly stated "stable since Rust 1.77"; on stable Rust users must run `cargo clean` to pick up changes; limitation documented in `src/lib.rs` comment and `docs/invariant/001_path_resolution.md` |
