# file_tools Examples

Usage examples demonstrating file_tools crate functionality.

## Status

⚠️ **PENDING IMPLEMENTATION**: Examples are placeholders pending specification definition.

## Available Examples

### basic_usage.rs

Demonstrates fundamental file manipulation operations.

**Run:**
```bash
cargo run --example basic_usage --features full
```

**Will demonstrate:**
- File reading operations
- File writing operations
- Path manipulation utilities
- Error handling patterns

## Future Examples

Once specification is defined and functionality implemented, additional examples should cover:

### Planned Examples

| Example | Purpose | Corner Cases Demonstrated |
|---------|---------|---------------------------|
| `read_operations.rs` | File reading patterns | Empty files, large files, binary files, permission errors |
| `write_operations.rs` | File writing patterns | Atomic writes, fsync, disk full handling |
| `path_operations.rs` | Path manipulation | Canonicalization, relative paths, cross-platform paths |
| `directory_ops.rs` | Directory operations | Recursive creation, traversal, permissions |
| `error_handling.rs` | Error handling patterns | All error conditions with recovery strategies |
| `cross_platform.rs` | Platform-specific behavior | Windows vs Unix path handling |

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

## References

- Crate specification (pending creation)
- test_organization.rulebook.md § Examples Organization
- files_structure.rulebook.md § Directory Structure
