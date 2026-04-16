# Stream Interleaving Test Binaries

Single-file Rust sources compiled and executed during stream-joining tests.

## File Responsibility Table

| Entry | Responsibility |
|-------|---------------|
| err_out_err.rs | Binary: writes stderr, then stdout, then stderr |
| out_err_out.rs | Binary: writes stdout, then stderr, then stdout |
