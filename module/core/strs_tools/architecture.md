# strs_tools Architecture and Implementation Specification

This document contains detailed technical information about the strs_tools crate implementation, architecture decisions, and compliance with design standards.

## Architecture Overview

### Module Structure

strs_tools follows a layered architecture using the `mod_interface!` pattern:

```
src/
├── lib.rs                 # Main crate entry point
├── simd.rs                # SIMD optimization features
└── string/
    ├── mod.rs             # String module interface
    ├── indentation.rs     # Text indentation tools
    ├── isolate.rs         # String isolation functionality
    ├── number.rs          # Number parsing utilities
    ├── parse_request.rs   # Command parsing tools
    ├── split.rs           # Advanced string splitting
    └── split/
        ├── simd.rs        # SIMD-accelerated splitting
        └── split_behavior.rs  # Split configuration
```

### Design Rulebook Compliance

This crate follows strict Design and Codestyle Rulebook compliance:

#### Core Principles
- **Explicit Lifetimes**: All function signatures with references use explicit lifetime parameters
- **mod_interface Pattern**: Uses `mod_interface!` macro instead of manual namespace definitions  
- **Workspace Dependencies**: All external deps inherit from workspace for version consistency
- **Testing Architecture**: All tests in `tests/` directory, never in `src/`
- **Error Handling**: Uses `error_tools` exclusively, no `anyhow` or `thiserror`

#### Code Style
- **Universal Formatting**: Consistent 2-space indentation and proper attribute spacing
- **Documentation Strategy**: Entry files use `include_str!` to avoid documentation duplication
- **Explicit Exposure**: All `mod_interface!` exports are explicitly listed, never using wildcards
- **Feature Gating**: Every workspace crate has `enabled` and `full` features

## Feature Architecture

### Feature Dependencies

The crate uses a hierarchical feature system:

```toml
default = ["enabled", "string_indentation", "string_isolate", "string_parse_request", "string_parse_number", "string_split", "simd"]
full = ["enabled", "string_indentation", "string_isolate", "string_parse_request", "string_parse_number", "string_split", "simd"]

# Performance optimization
simd = ["memchr", "aho-corasick", "bytecount", "lazy_static"]

# Core functionality
enabled = []
string_split = ["split"]
string_indentation = ["indentation"]
# ... other features
```

### SIMD Optimization

Optional SIMD dependencies provide significant performance improvements:

- **memchr**: Hardware-accelerated byte searching
- **aho-corasick**: Multi-pattern string searching
- **bytecount**: Fast byte counting operations
- **lazy_static**: Cached pattern compilation

Performance benefits:
- 2-10x faster string searching on large datasets
- Parallel pattern matching capabilities
- Reduced CPU cycles for bulk operations

## API Design Principles

### Memory Efficiency

- **Zero-Copy Operations**: String slices returned where possible using `Cow<str>`
- **Lazy Evaluation**: Iterator-based processing avoids unnecessary allocations
- **Reference Preservation**: Original string references maintained when splitting

### Error Handling Strategy

All error handling follows the centralized `error_tools` pattern:

```rust
use error_tools::{ err, Result };

fn parse_operation() -> Result<ParsedData>
{
  // Structured error handling
  match validation_step()
  {
    Ok( data ) => Ok( data ),
    Err( _ ) => Err( err!( ParseError::InvalidFormat ) ),
  }
}
```

### Async-Ready Design

While the current implementation is synchronous, the API is designed to support async operations:

- Iterator-based processing enables easy async adaptation
- No blocking I/O in core operations
- State machines can be made async-aware

## Performance Characteristics

### Benchmarking Results

Performance benchmarks are maintained in the `benchmarks/` directory:

- **Baseline Results**: Standard library comparisons
- **SIMD Benefits**: Hardware acceleration measurements  
- **Memory Usage**: Allocation and reference analysis
- **Scalability**: Large dataset processing metrics

See `benchmarks/readme.md` for current performance data.

### Optimization Strategies

1. **SIMD Utilization**: Vectorized operations for pattern matching
2. **Cache Efficiency**: Minimize memory allocations and copies
3. **Lazy Processing**: Iterator chains avoid intermediate collections
4. **String Interning**: Reuse common patterns and delimiters

## Testing Strategy

### Test Organization

Following the Design Rulebook, all tests are in `tests/`:

```
tests/
├── smoke_test.rs                    # Basic functionality
├── strs_tools_tests.rs             # Main test entry
└── inc/                            # Detailed test modules
    ├── indentation_test.rs
    ├── isolate_test.rs
    ├── number_test.rs
    ├── parse_test.rs
    └── split_test/                 # Comprehensive splitting tests
        ├── basic_split_tests.rs
        ├── quoting_options_tests.rs
        └── ... (other test categories)
```

### Test Matrix Approach

Each test module includes a Test Matrix documenting:

- **Test Factors**: Input variations, configuration options
- **Test Combinations**: Systematic coverage of scenarios
- **Expected Outcomes**: Clearly defined success criteria
- **Edge Cases**: Boundary conditions and error scenarios

### Integration Test Features

Integration tests are feature-gated for flexible CI:

```rust
#![cfg(feature = "integration")]

#[test]
fn test_large_dataset_processing()
{
  // Performance and stress tests
}
```

## Security Considerations

### Input Validation

- **Bounds Checking**: All string operations validate input boundaries
- **Escape Handling**: Raw string slices returned to prevent injection attacks
- **Error Boundaries**: Parsing failures are contained and reported safely

### Memory Safety

- **No Unsafe Code**: All operations use safe Rust constructs
- **Reference Lifetimes**: Explicit lifetime management prevents use-after-free
- **Allocation Control**: Predictable memory usage patterns

## Compatibility and Portability

### Platform Support

- **no_std Compatibility**: Core functionality available in embedded environments
- **SIMD Fallbacks**: Graceful degradation when hardware acceleration unavailable
- **Endianness Agnostic**: Correct operation on all target architectures

### Version Compatibility

- **Semantic Versioning**: API stability guarantees through SemVer
- **Feature Evolution**: Additive changes maintain backward compatibility
- **Migration Support**: Clear upgrade paths between major versions

## Development Workflow

### Code Generation

Some functionality uses procedural macros following the established workflow:

1. **Manual Implementation**: Hand-written reference implementation
2. **Test Development**: Comprehensive test coverage
3. **Macro Creation**: Procedural macro generating equivalent code
4. **Validation**: Comparison testing between manual and generated versions

### Contribution Guidelines

- **Rulebook Compliance**: All code must follow Design and Codestyle rules
- **Test Requirements**: New features require comprehensive test coverage
- **Performance Testing**: Benchmark validation for performance-sensitive changes
- **Documentation**: Rich examples and API documentation required

## Migration from Standard Library

### Common Patterns

| Standard Library | strs_tools Equivalent | Benefits |
|------------------|----------------------|----------|
| `str.split()` | `string::split().src().delimiter().perform()` | Quote awareness, delimiter preservation |
| Manual parsing | `string::parse_request::parse()` | Structured command parsing |
| `str.trim()` + parsing | `string::number::parse()` | Robust number format support |

### Performance Benefits

- **Large Data**: 2-10x improvement with SIMD features
- **Memory Usage**: 50-90% reduction with zero-copy operations
- **Complex Parsing**: 5-20x faster than manual implementations

### API Advantages

- **Type Safety**: Compile-time validation of operations
- **Error Handling**: Comprehensive error types and recovery
- **Extensibility**: Plugin architecture for custom operations
- **Testing**: Built-in test utilities and helpers