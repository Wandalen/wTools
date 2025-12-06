# CLI Export and Aggregation - Best Practices Guide

## Overview

This guide demonstrates how to export CLI commands from one crate/module to another and combine multiple CLIs into a single aggregating CLI with optional prefixes using unilang's architecture.

## ✅ Current Architecture Support

**YES** - This is fully supported with the current unilang architecture!

- ✅ Export CLI from module/crate to another
- ✅ Combine multiple CLIs with prefixes
- ✅ Maintain full type safety and validation
- ✅ Preserve all help system functionality
- ✅ Ergonomic, hard-to-misuse API

## 🏗️ Architecture Pattern

### 1. CLI Export Pattern

```rust
// In your CLI module (e.g., math_cli crate)
pub fn export() -> CliModule {
    CliModule::new("math")
        .description("Mathematical operations")
        .simple_command("add", "Add two numbers", args, handler)
        .simple_command("multiply", "Multiply numbers", args, handler)
}
```

### 2. CLI Aggregation Pattern

```rust
// In your main application
let unified_cli = CliAggregator::new()
    .add_module(cmd1_cli::export(), Some("cmd1"))?     // .cmd1.process
    .add_module(cmd2_cli::export(), Some("cmd2"))?     // .cmd2.list
    .add_module(svc1_cli::export(), Some("svc1"))?     // .svc1.connect
    .main_command("info", "Show CLI info", handler)?
    .build();
```

## 🎯 Key Benefits

### Type Safety
- Full argument validation maintained across all modules
- Compile-time checking prevents misuse
- Error handling propagated correctly

### Help System Integration
- All three help methods work: `.command.help`, `??`, `?`
- Auto-generated help for all aggregated commands
- Namespace-aware help navigation

### Ergonomic API
- Builder pattern prevents configuration errors
- Clear separation between export and aggregation
- Intuitive naming that matches mental models

### Hard to Misuse
- Automatic dot prefix handling
- Automatic help enablement for exported commands
- Clear error messages for common mistakes

## 📋 Usage Examples

### Individual Module Testing
```bash
# Test cmd1 module independently
cargo run --example cmd1_cli
> .process a::10 b::5
Result: 10 + 5 = 15
```

### Aggregated CLI Usage
```bash
# Test unified CLI with prefixes
cargo run --example unified_cli
> .cmd1.process a::20 b::15    # Cmd1 module
Result: 20 + 15 = 35

> .cmd2.list path::/tmp        # Cmd2 module
Result: 📁 Listing...

> .svc1.connect host::prod     # Svc1 module
Result: 🔌 Connected to prod...

> .info                        # Main CLI info
Result: 🎯 Unified CLI v1.0...
```

### Help System
```bash
# All help methods work with prefixes
> .cmd1.process.help          # Auto-generated help
> .cmd2.copy "??"             # Help parameter
> .svc1.connect ?             # Traditional help operator
> .                           # List all commands
```

## 🚀 Implementation Steps

### Step 1: Design Your Module CLI
```rust
pub mod your_cli {
    use unilang::prelude::*;

    pub fn export() -> CliModule {
        CliModule::new("your_module")
            .description("Your module description")
            .simple_command("command1", "Description", args, handler1)
            .simple_command("command2", "Description", args, handler2)
    }
}
```

### Step 2: Create the Aggregated CLI
```rust
fn main() -> Result<(), unilang::Error> {
    let cli = CliAggregator::new()
        .add_module(your_cli::export(), Some("prefix"))?
        .add_module(other_cli::export(), Some("other"))?
        .main_command("info", "Main CLI info", info_handler)?
        .build();

    let pipeline = Pipeline::new(cli);
    // Use your unified CLI
    Ok(())
}
```

### Step 3: Test and Deploy
- Individual modules can be tested independently
- Aggregated CLI provides unified interface
- All unilang features work seamlessly

## 🔧 Advanced Features

### Conditional Module Loading
```rust
let mut aggregator = CliAggregator::new();

if cfg!(feature = "math") {
    aggregator = aggregator.add_module(math_cli::export(), Some("math"))?;
}

if cfg!(feature = "database") {
    aggregator = aggregator.add_module(db_cli::export(), Some("db"))?;
}
```

### Dynamic Prefix Assignment
```rust
let modules = vec![
    ("math", math_cli::export()),
    ("fs", file_cli::export()),
    ("db", db_cli::export()),
];

let mut aggregator = CliAggregator::new();
for (prefix, module) in modules {
    aggregator = aggregator.add_module(module, Some(prefix))?;
}
```

### No-Prefix Integration
```rust
// Add commands without prefix (top-level)
aggregator.add_module(core_cli::export(), None)?  // .info, .version, etc.
```

## 📊 Comparison

| Approach | Ergonomics | Type Safety | Help System | Maintenance |
|----------|------------|-------------|-------------|-------------|
| Manual registry merging | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **CliModule pattern** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |

## 🎯 Best Practices

1. **Module Isolation**: Each CLI module should be self-contained
2. **Clear Prefixes**: Use intuitive prefixes that match domain concepts
3. **Help Integration**: Always enable auto-help for exported commands
4. **Error Handling**: Propagate errors properly through the aggregation chain
5. **Testing**: Test both individual modules and the aggregated CLI
6. **Documentation**: Document the prefix scheme and available modules

## 🔍 Error Prevention

The design prevents common mistakes:

- ❌ **Naming Conflicts**: Prefixes prevent command name collisions
- ❌ **Missing Help**: Auto-help enablement ensures help is always available
- ❌ **Type Mismatches**: Full type validation maintained across modules
- ❌ **Configuration Errors**: Builder pattern catches issues at compile time

## 📈 Scalability

This pattern scales well:

- **Small CLIs**: 2-3 modules with simple prefixes
- **Medium CLIs**: 5-10 modules with hierarchical namespaces
- **Large CLIs**: 20+ modules with feature flags and conditional loading
- **Enterprise CLIs**: 100+ modules with dynamic plugin loading

The unilang architecture provides a solid foundation for all scales while maintaining ergonomics and type safety.