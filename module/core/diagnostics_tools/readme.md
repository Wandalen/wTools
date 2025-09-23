# Diagnostics Tools

[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_diagnostics_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_diagnostics_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/diagnostics_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/diagnostics_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

**Enhanced debugging and testing tools for Rust with better error messages and compile-time checks.**

## Why Choose Diagnostics Tools?

✨ **Better Error Messages** - Get colorful, detailed diffs instead of cryptic assertion failures  
⚡ **Compile-Time Safety** - Catch bugs before your code even runs  
🧠 **Memory Layout Validation** - Ensure your types have the expected size and alignment  
🔧 **Drop-in Replacement** - Works with existing `assert!` macros but provides much more  

## Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
diagnostics_tools = "0.11"
```

## Basic Example

```rust,no_run
use diagnostics_tools::*;

fn main() 
{
    // Instead of cryptic assertion failures, get beautiful diffs:
    a_id!( vec![ 1, 2, 3 ], vec![ 1, 2, 4 ] );
    
    // Outputs:
    // assertion failed: `(left == right)`
    // 
    // Diff < left / right > :
    // [
    //     1,
    //     2,
    // <   3,
    // >   4,
    // ]
}
```

## What Makes It Different?

| Standard Rust | Diagnostics Tools | Advantage |
|---------------|-------------------|-----------|
| `assert_eq!(a, b)` | `a_id!(a, b)` | 🎨 Colorful diff output |
| `assert!(condition)` | `a_true!(condition)` | 📝 Better error context |
| No compile-time checks | `cta_true!(cfg(feature = "x"))` | ⚡ Catch errors at compile time |
| No memory layout validation | `cta_type_same_size!(u32, i32)` | 🔍 Verify type assumptions |

## Core Features

### 🏃 Runtime Assertions
- `a_true!(condition)` / `a_false!(condition)` - Boolean checks with context
- `a_id!(left, right)` / `a_not_id!(left, right)` - Value comparison with diffs
- Debug variants (`a_dbg_*`) that print values even on success

### ⚡ Compile-Time Assertions  
- `cta_true!(condition)` - Validate conditions at compile time
- Perfect for checking feature flags, configurations, or assumptions

### 🧠 Memory Layout Validation
- `cta_type_same_size!(TypeA, TypeB)` - Ensure types have same size
- `cta_type_same_align!(TypeA, TypeB)` - Check alignment requirements
- `cta_ptr_same_size!(ptr1, ptr2)` - Validate pointer sizes
- `cta_mem_same_size!(value1, value2)` - Compare memory footprints

## Learning Path

Explore our numbered examples to learn progressively:

1. [`001_basic_runtime_assertions.rs`](examples/001_basic_runtime_assertions.rs) - Start here!
2. [`002_better_error_messages.rs`](examples/002_better_error_messages.rs) - See the difference
3. [`003_compile_time_checks.rs`](examples/003_compile_time_checks.rs) - Prevent bugs early
4. [`004_memory_layout_validation.rs`](examples/004_memory_layout_validation.rs) - Low-level validation
5. [`005_debug_variants.rs`](examples/005_debug_variants.rs) - Development helpers
6. [`006_real_world_usage.rs`](examples/006_real_world_usage.rs) - Practical scenarios

## Use Cases

- **🧪 Testing**: Get clearer test failure messages
- **🔧 Development**: Debug complex data structures easily  
- **⚙️ Systems Programming**: Validate memory layout assumptions
- **📦 Library Development**: Add compile-time safety checks
- **🚀 Performance Code**: Ensure type sizes match expectations

## Documentation

- [API Reference](https://docs.rs/diagnostics_tools) - Complete API documentation
- [`TECHNICAL_DETAILS.md`](TECHNICAL_DETAILS.md) - Implementation details
- [`MIGRATION_GUIDE.md`](MIGRATION_GUIDE.md) - Switching from standard assertions
- [`FEATURES.md`](FEATURES.md) - Feature flags and configuration

## Try It Online

[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fdiagnostics_tools%2Fexamples%2F001_basic_runtime_assertions.rs,RUN_POSTFIX=--example%20001_basic_runtime_assertions/https://github.com/Wandalen/wTools)

## License

Licensed under MIT license. See [`LICENSE`](LICENSE) for details.