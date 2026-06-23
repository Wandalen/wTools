# src/

### Responsibility Table

| Name | Responsibility |
|------|---------------|
| `lib.rs` | Crate root with feature gates and module declarations |
| `simd.rs` | SIMD-accelerated delimiter search primitives |
| `ansi/` | ANSI escape sequence detection, parsing, stripping, and visual measurement |
| `string/` | String splitting, isolation, indentation, number parsing, and command parsing |
| `bin/` | Binary entry points for SIMD benchmarking |
