# file_tools Examples

Usage examples demonstrating file_tools crate functionality.

## Available Examples

### basic_usage.rs

Demonstrates fundamental file manipulation operations.

**Run:**
```bash
cargo run --example basic_usage --features full
```

## Example Requirements

All examples MUST:
- Include comprehensive documentation explaining purpose
- Demonstrate real-world usage patterns
- Show proper error handling
- Be runnable without external dependencies (when possible)
- Use feature gates appropriately (`#[cfg(feature = "enabled")]`)
- Include expected output in comments

## Running Examples

**Run specific example:**
```bash
cargo run --example <name> --features full
```

**List all examples:**
```bash
cargo build --examples --features full
ls -1 target/debug/examples/
```

## Contributing Examples

When adding new examples:

1. **Choose descriptive name** - Reflects functionality demonstrated
2. **Add documentation** - Explain purpose, usage, expected output
3. **Update this readme** - Add entry to "Available Examples" section
4. **Test thoroughly** - Ensure example runs correctly
5. **Keep simple** - Focus on demonstrating one concept per example

