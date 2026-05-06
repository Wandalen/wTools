# Files Entity Module

File path type wrappers providing type safety and domain semantics.

### Responsibility Table

| File | Responsibility |
|------|---------------|
| `crate_dir.rs` | Wraps AbsolutePath for crate directory locations |
| `manifest_file.rs` | Wraps AbsolutePath for Cargo.toml file locations |
| `source_file.rs` | Wraps AbsolutePath for source file locations |
| `either.rs` | Provides Either enum for dual-type file path handling |
