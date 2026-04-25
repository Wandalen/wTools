# Examples Directory

This directory contains usage demonstration examples for error_tools.

### Responsibility Table

| File | Responsibility |
|------|---------------|
| error_tools_trivial.rs | Demonstrate minimal error handling with Result and error propagation |
| replace_anyhow.rs | Demonstrate drop-in replacement for anyhow with context and error chaining |
| replace_thiserror.rs | Demonstrate drop-in replacement for thiserror with derive macros and custom errors |
| err_with_example.rs | Demonstrate ErrWith trait usage with closures and references for error context |

## Purpose

Each example demonstrates a specific aspect of error_tools:

- **error_tools_trivial.rs**: Entry-level example showing basic Result<T, E> usage and error propagation
- **replace_anyhow.rs**: Migration guide for users transitioning from anyhow crate
- **replace_thiserror.rs**: Migration guide for users transitioning from thiserror crate
- **err_with_example.rs**: Advanced usage of custom ErrWith trait for rich error context

## Running Examples

```bash
# Run individual examples
cargo run --features full --example error_tools_trivial
cargo run --features full --example replace_anyhow
cargo run --features full --example replace_thiserror
cargo run --features full --example err_with_example

# Run all examples
for example in error_tools_trivial replace_anyhow replace_thiserror err_with_example; do
  cargo run --features full --example $example
done
```
