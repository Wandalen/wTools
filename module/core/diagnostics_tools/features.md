# Features and Configuration

This document describes the feature flags and configuration options available in `diagnostics_tools`.

## Default Features

By default, the crate enables these features:

```toml
[dependencies]
diagnostics_tools = "0.11"  # Includes: enabled, runtime, compiletime, memory_layout
```

This gives you access to all assertion types:
- Runtime assertions (`a_*` macros)
- Compile-time assertions (`cta_*` macros)  
- Memory layout validation (`cta_type_*`, `cta_ptr_*`, `cta_mem_*`)

## Available Feature Flags

### Core Features

#### `enabled` *(default)*
Master switch that enables the crate functionality. Without this, all macros become no-ops.

```toml
[dependencies]
diagnostics_tools = { version = "0.11", features = ["enabled"] }
```

#### `full` 
Enables all features - equivalent to enabling all individual feature flags.

```toml
[dependencies]
diagnostics_tools = { version = "0.11", features = ["full"] }
```

### Functionality Features

#### `diagnostics_runtime_assertions` *(default)*
Enables runtime assertion macros:
- `a_true!`, `a_false!`
- `a_id!`, `a_not_id!`  
- `a_dbg_true!`, `a_dbg_false!`, `a_dbg_id!`, `a_dbg_not_id!`

```toml
[dependencies]
diagnostics_tools = { version = "0.11", features = ["enabled", "diagnostics_runtime_assertions"] }
```

#### `diagnostics_compiletime_assertions` *(default)*
Enables compile-time assertion macros:
- `cta_true!`

```toml
[dependencies]
diagnostics_tools = { version = "0.11", features = ["enabled", "diagnostics_compiletime_assertions"] }
```

#### `diagnostics_memory_layout` *(default)*
Enables memory layout validation macros:
- `cta_type_same_size!`, `cta_type_same_align!`
- `cta_ptr_same_size!`, `cta_mem_same_size!`

```toml
[dependencies]
diagnostics_tools = { version = "0.11", features = ["enabled", "diagnostics_memory_layout"] }
```

### Environment Features

#### `no_std`
Enables no_std support for embedded and constrained environments.

```toml
[dependencies]
diagnostics_tools = { version = "0.11", features = ["no_std", "enabled"] }
```

When `no_std` is enabled:
- Runtime assertions still work but with limited formatting
- Compile-time assertions work exactly the same
- Memory layout validation works exactly the same

#### `use_alloc`
When using `no_std`, you can still enable heap allocation with `use_alloc`.

```toml
[dependencies]  
diagnostics_tools = { version = "0.11", features = ["no_std", "use_alloc", "enabled"] }
```

## Custom Feature Combinations

### Minimal Runtime Only
For projects that only need runtime assertions:

```toml
[dependencies]
diagnostics_tools = { 
    version = "0.11", 
    default-features = false, 
    features = ["enabled", "diagnostics_runtime_assertions"] 
}
```

### Compile-Time Only  
For projects that only need compile-time validation:

```toml
[dependencies]
diagnostics_tools = { 
    version = "0.11", 
    default-features = false, 
    features = ["enabled", "diagnostics_compiletime_assertions"] 
}
```

### Memory Layout Only
For low-level code that only needs memory validation:

```toml
[dependencies]
diagnostics_tools = { 
    version = "0.11", 
    default-features = false, 
    features = ["enabled", "diagnostics_memory_layout"] 
}
```

### Embedded/No-Std
For embedded projects:

```toml
[dependencies]
diagnostics_tools = { 
    version = "0.11", 
    default-features = false, 
    features = ["no_std", "enabled", "diagnostics_compiletime_assertions", "diagnostics_memory_layout"] 
}
```

## Conditional Compilation

You can conditionally enable features based on your build configuration:

```toml
[dependencies]
diagnostics_tools = { version = "0.11", default-features = false, features = ["enabled"] }

[dependencies.diagnostics_tools.features]
# Only include runtime assertions in debug builds
diagnostics_runtime_assertions = { optional = true }

[features]
default = []
debug_asserts = ["diagnostics_tools/diagnostics_runtime_assertions"]
```

Then use with:
```bash
# Development build with runtime assertions
cargo build --features debug_asserts

# Release build without runtime assertions  
cargo build --release
```

## Performance Impact

### Feature Impact on Binary Size

| Feature | Binary Size Impact | Runtime Impact |
|---------|-------------------|----------------|
| `diagnostics_runtime_assertions` | Medium (includes pretty_assertions) | Same as standard assertions |
| `diagnostics_compiletime_assertions` | None (compile-time only) | None |
| `diagnostics_memory_layout` | None (compile-time only) | None |
| `no_std` | Reduces size | Slightly reduced formatting |

### Recommendation by Use Case

**Testing/Development:**
```toml
diagnostics_tools = "0.11"  # Use all default features
```

**Production Libraries:**
```toml
diagnostics_tools = { 
    version = "0.11", 
    default-features = false, 
    features = ["enabled", "diagnostics_compiletime_assertions", "diagnostics_memory_layout"] 
}
```

**Embedded Systems:**
```toml
diagnostics_tools = { 
    version = "0.11", 
    default-features = false, 
    features = ["no_std", "enabled", "diagnostics_compiletime_assertions"] 
}
```

**High-Performance Applications:**
```toml
# Development
[dependencies.diagnostics_tools]
version = "0.11"

# Production (disable runtime assertions)
[dependencies.diagnostics_tools]  
version = "0.11"
default-features = false
features = ["enabled", "diagnostics_compiletime_assertions", "diagnostics_memory_layout"]
```

## Feature Interaction

Some features have dependencies on each other:

- `enabled` is required for any functionality
- `use_alloc` requires `no_std`
- All diagnostic features require `enabled`

The crate will give compile-time errors if incompatible features are selected.