<!-- {{# generate.module_header{} #}} -->

# Module :: unilang
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_unilang_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_unilang_push.yml) [![docs.rs](https://img.shields.io/docsrs/unilang?color=e3e8f0&logo=docs.rs)](https://docs.rs/unilang) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fmove%2Funilang%2Fexamples%2F00_pipeline_basics.rs,RUN_POSTFIX=--example%20module%2Fmove%2Funilang%2Fexamples%2F00_pipeline_basics.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

**Zero-overhead command framework with compile-time command registration**

## Value Proposition

unilang processes command definitions at compile-time, generating optimized static command registries that provide **O(1) command lookups with zero runtime overhead and zero additional dependencies for downstream crates**. This approach delivers:

- **50x faster command resolution** compared to runtime HashMap lookups (~80ns vs ~4,000ns)
- **Compile-time validation** of all command definitions and arguments
- **Smaller binary size** through static analysis and dead code elimination
- **SIMD acceleration** for parsing with 4-25x performance improvements
- **Zero memory allocations** for command lookup operations

## Architecture Overview

**Compile-Time Processing:**
```text
YAML definitions → build.rs → Static command maps → Zero-cost lookups
```

**Runtime Execution:**
```text
Command string → O(1) static lookup → Validated execution
```

## Quick Start: Compile-Time Registration (Recommended)

### Step 1: Define Commands

Create `unilang.commands.yaml`:
```yaml
- name: ".greet"
  namespace: ""
  description: "High-performance greeting command"
  arguments:
    - name: "name"
      kind: "String"
      attributes:
        optional: true
        default: "World"
```

**Note:** Command names should always start with a dot (`.`). When using build.rs for static generation, you can optionally omit the dot as it will be added automatically, but showing it here reinforces the naming convention.

### Step 2: Configure Cargo.toml

The default configuration already enables compile-time YAML loading:
```toml
[dependencies]
# Multi-YAML build-time approach is enabled by default
unilang = "0.28"
```

For single-file YAML approach, use:
```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = [
  "enabled",
  "approach_yaml_single_build"  # Single YAML file at compile-time
]}
```

### Step 3: Configure Build Script (Optional for Single-File)

If using `approach_yaml_single_build`, add minimal `build.rs`:
```rust,ignore
fn main()
{
  // Rebuild if YAML file changes
  println!( "cargo:rerun-if-changed=unilang.commands.yaml" );

  // Static registry generation happens automatically when
  // approach_yaml_single_build or approach_yaml_multi_build is enabled.
  // The build system discovers YAML files and generates optimized code.
}
```

**Note:** With the default `approach_yaml_multi_build` feature, the build system automatically discovers all `.yaml` files in your project - no build.rs configuration needed!

### Step 4: Zero-Cost Execution

```rust,ignore
use unilang::prelude::*;

// Include compile-time generated commands (created automatically by build system)
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main() -> Result< (), unilang::Error >
{
  // StaticCommandRegistry requires approach_yaml_single_build,
  // approach_yaml_multi_build, or other build-time approach feature
  let registry = StaticCommandRegistry::from_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // O(1) lookup - no hashing overhead
  let result = pipeline.process_command_simple( ".greet name::Alice" );

  println!( "Output: {}", result.outputs[ 0 ].content );
  Ok( () )
}
```

## Performance Comparison

| Approach | Lookup Time | Memory Overhead | Binary Size |
|----------|-------------|-----------------|-------------|
| **Compile-Time (Static)** | ~80ns | Zero | Smaller |
| Runtime (HashMap) | ~4,000ns | Hash tables + allocations | Larger |

**Benchmark Results:**
- **Static lookups:** ~80-100ns (PHF map + zero allocations)
- **Runtime lookups:** ~4,000-5,000ns (HashMap + semantic analysis)
- **Performance gain:** 50x faster command resolution

## User Guide: Integration Decisions

### Decision 1: How Should I Define Commands?

⚠️ **IMPORTANT: Opinionated Defaults**

unilang **ONLY enables Approach #2 by default**. To use any other approach, you must explicitly enable its feature flag in `Cargo.toml`.

**10 approaches are currently implemented** (see [full comparison](docs/cli_definition_approaches.md)):

| # | Approach | Feature Flag | Default | Lookup Speed | When to Use |
|---|----------|--------------|---------|-------------|-------------|
| 1 | YAML file → Build-time static | `approach_yaml_single_build` | ❌ | ~80ns | Single-file projects, compile-time validation |
| **2** | **Multi-YAML files → Build-time static** | `approach_yaml_multi_build` | **✅ DEFAULT** | **~80ns** | **Modular projects, best DX, auto-discovery** |
| 3 | YAML file → Runtime loading | `approach_yaml_runtime` | ❌ | ~4,200ns | Plugin configs, runtime loading |
| 4 | JSON file → Build-time static | `approach_json_single_build` | ❌ | ~80ns | JSON-first projects, API generation |
| 5 | Multi-JSON files → Build-time static | `approach_json_multi_build` | ❌ | ~80ns | Large JSON projects, modular organization |
| 6 | JSON file → Runtime loading | `approach_json_runtime` | ❌ | ~4,200ns | Runtime config loading, dynamic commands |
| 7 | Rust DSL (builder API) | *(always available)* | ✅ | ~4,200ns | Core API, prototyping, type-safe definitions |
| 8 | Rust DSL (const fn + static) | `approach_rust_dsl_const` | ❌ | ~80ns | High-performance DSL, compile-time |
| 18 | Hybrid (static + runtime) | `approach_hybrid` | ❌ | Mixed | Base CLI + plugin system |

**Why Approach #2 is Default:**
- **Best developer experience** with auto-discovery of command files
- **Optimal for modular projects** splitting commands across multiple files
- **Compile-time validation** catches errors before runtime
- **Zero overhead** with static registry generation

**See full comparison:** [21 approaches documented](docs/cli_definition_approaches.md) including planned features like declarative macros, proc macros, TOML, RON, Protobuf, GraphQL, OpenAPI.

### Decision 2: How Do I Execute Commands?

```rust,ignore
// CLI applications: Avoids shell quoting issues
let result = pipeline.process_command_from_argv( &std::env::args().collect() );

// REPL/interactive applications: String-based parsing
let result = pipeline.process_command_simple( ".greet name::Alice" );
```

### Decision 3: What Are the Naming Rules?

✅ **Commands should start with a dot:**
```bash
.greet name::Alice           # Recommended
greet name::Alice            # Not recommended
```

### Decision 4: What Features Should I Enable?

**Recommended: Use defaults** (Approach #2 + SIMD + Enhanced REPL)
```toml
[dependencies]
unilang = "0.28"  # Multi-YAML build-time + SIMD (4-25x parsing) + Enhanced REPL
```

**Alternative approach (JSON single-file):**
```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = [
  "enabled",
  "approach_json_single_build"  # Switch to JSON single-file approach
]}
```

**Minimal build (Rust DSL only):**
```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = ["enabled"] }
```

**All features enabled:**
```toml
[dependencies]
unilang = { version = "0.28", features = ["full"] }  # All 21 approaches available
```

**Feature Architecture:**

The framework uses **approach-based feature flags**:
- Each CLI definition approach has its own feature (e.g., `approach_yaml_multi_build`)
- Approach features automatically enable required infrastructure (`static_registry`, `yaml_parser`, etc.)
- Only **Approach #2** enabled by default for optimal performance and minimal binary size
- See [Feature Flags Documentation](docs/cli_definition_approaches.md#using-alternative-approaches) for complete list

### Decision 5: How Does Help Work?

Three methods available:
```bash
.command ?                   # Traditional operator (bypasses validation)
.command ??                  # Modern parameter
.command.help                # Auto-generated help command
```

### Decision 6: Error Handling

Unknown parameters are **always detected** with Levenshtein distance suggestions. This cannot be disabled and ensures command correctness.

### Decision 7: Advanced Use Cases

- **REPL applications:** Use `enhanced_repl` feature for history, completion, secure input
- **WASM deployment:** Full framework support in browsers with SIMD acceleration
- **Interactive arguments:** Prompt for missing required arguments in REPL mode

## CLI Aggregation: Unifying Multiple Tools

⚠️ **Feature Required:** `multi_file` (automatically enabled by default `approach_yaml_multi_build`)

unilang excels at aggregating multiple CLI tools into a single unified command interface. This is essential for organizations that want to consolidate developer tools while maintaining namespace isolation.

### Real-World Aggregation Scenario

```rust,ignore
// Requires: approach_yaml_multi_build (default) or manually enable 'multi_file' feature
use unilang::multi_yaml::CliBuilder;

// Aggregate multiple CLI tools into one unified command
let unified_cli = CliBuilder::new()
  .static_module_with_prefix( "database", "db", database_commands )
  .static_module_with_prefix( "filesystem", "fs", file_commands )
  .static_module_with_prefix( "network", "net", network_commands )
  .static_module_with_prefix( "build", "build", build_commands )
  .detect_conflicts( true )
  .build_static();

// Usage: unified-cli .db.migrate, unified-cli .fs.copy src dest
```

### Compile-Time Aggregation Benefits

**Before Aggregation:**
```bash
# Separate tools requiring individual installation and learning
db-cli migrate --direction up
file-cli copy --src ./source --dest ./target --recursive
net-cli ping google.com --count 10
build-cli compile --target release
```

**After Aggregation:**
```bash
# Single unified tool with consistent interface
unified-cli .db.migrate direction::up
unified-cli .fs.copy source::./source destination::./target recursive::true
unified-cli .net.ping host::google.com count::10
unified-cli .build.compile target::release
```

### Key Aggregation Features

#### Namespace Isolation
Each CLI module maintains its own command space with automatic prefix application:

```text
Database commands become .db.migrate, .db.backup
File commands become .fs.copy, .fs.move
Network commands become .net.ping, .net.trace
No naming conflicts between modules
```

#### Conflict Detection
```rust,ignore
let registry = CliBuilder::new()
  .static_module_with_prefix( "tools", "tool", cli_a_commands )
  .static_module_with_prefix( "utils", "tool", cli_b_commands )  // Conflict!
  .detect_conflicts( true )  // Catches duplicate prefixes at build time
  .build_static();
```

#### Help System Integration
```bash
# All aggregated commands support unified help
unified-cli .db.migrate.help       # Detailed help for database migrations
unified-cli .fs.copy ??            # Interactive help during command construction
unified-cli .net.ping ?            # Traditional help operator
```

### Advanced Aggregation Patterns

#### Conditional Module Loading
```rust,ignore
let registry = CliBuilder::new()
  .conditional_module( "docker", docker_commands, &[ "feature_docker" ] )
  .conditional_module( "k8s", kubernetes_commands, &[ "feature_k8s" ] )
  .build_static();

// Only includes modules when features are enabled
```

#### Multi-Source Aggregation
```rust,ignore
use std::path::PathBuf;

// Combine static commands (in-memory) and dynamic YAML loading
let registry = CliBuilder::new()
  .static_module_with_prefix( "core", ".core", core_commands )
  .dynamic_module_with_prefix( "plugins", PathBuf::from( "plugins.yaml" ), ".plugins" )
  .build_hybrid();

// Key differences:
// - static_module_with_prefix(name, prefix, Vec<CommandDefinition>)
//   → Commands already in memory, fast O(1) lookup (~80-100ns)
// - dynamic_module_with_prefix(name, PathBuf, prefix)
//   → Commands loaded from YAML file at runtime (~4,000ns, 50x slower)
```

### Performance Characteristics

| Approach | Lookup Time | Memory Overhead | Conflict Detection |
|----------|-------------|-----------------|-------------------|
| **Compile-Time** | O(1) static | Zero | Build-time |
| Runtime | O(log n) | Hash tables | Runtime |

**Aggregation Scaling:**
- **10 modules, 100 commands each**: ~80-100ns lookup regardless of module count
- **Single static map**: All 1,000 commands accessible in constant time with O(1) complexity
- **Namespace resolution**: Zero runtime overhead with compile-time prefixing

### Complete Example

See `examples/practical_cli_aggregation.rs` for a comprehensive demonstration showing:

- Individual CLI module definitions
- Runtime and compile-time aggregation approaches
- Namespace organization and conflict prevention
- Unified command execution patterns
- Performance comparison between approaches

```bash
# Run the complete aggregation demo
cargo run --example practical_cli_aggregation
```

This example demonstrates aggregating database, file, network, and build CLIs into a single unified tool while maintaining type safety, performance, and usability.

## Command Definition Format

### Basic Command Structure
```yaml
- name: ".command_name"          # Required: Command identifier (must start with dot)
  namespace: ""                  # Optional: Hierarchical organization (e.g., "math", "file")
  description: "What it does"    # Required: User-facing description
  arguments:                     # Optional: Command parameters
    - name: "arg_name"
      kind: "String"             # String, Integer, Float, Boolean, Path, etc.
      attributes:
        optional: false          # Required by default
        default: "value"         # Default value if optional
```

### Supported Argument Types
- **Basic Types:** String, Integer, Float, Boolean
- **Path Types:** Path, File, Directory
- **Complex Types:** Url, DateTime, Pattern (regex)
- **Collections:** List, Map with custom delimiters
- **Special Types:** JsonString, Object, Enum

### Validation Rules
```yaml
arguments:
  - name: "count"
    kind: "Integer"
    validation_rules:
      - Min: 1
      - Max: 100
  - name: "email"
    kind: "String"
    validation_rules:
      - Pattern: "^[^@]+@[^@]+\\.[^@]+$"
      - MinLength: 5
```

## Command Execution Patterns

### Standard Execution
```rust,ignore
let result = pipeline.process_command_simple( ".namespace.command arg::value" );
if result.success
{
  println!( "Success: {}", result.outputs[ 0 ].content );
}
```

### Batch Processing
```rust,ignore
let commands = vec!
[
  ".file.create name::test.txt",
  ".file.write name::test.txt content::data",
  ".file.list pattern::*.txt",
];

let batch_result = pipeline.process_batch( &commands, ExecutionContext::default() );
println!( "Success rate: {:.1}%", batch_result.success_rate() * 100.0 );
```

### Error Handling
```rust,ignore
match pipeline.process_command_simple( ".command arg::value" )
{
  result if result.success =>
  {
    // Process successful execution
    for output in result.outputs
    {
      println!( "Output: {}", output.content );
    }
  }
  result =>
  {
    if let Some( error ) = result.error
    {
      eprintln!( "Command failed: {}", error );
    }
  }
}
```

## Help System

unilang provides comprehensive help with three access methods:

### Traditional Help Operator
```bash
.command ?                    # Instant help, bypasses validation
```

### Modern Help Parameter
```bash
.command ??                   # Clean help access
.command arg1::value ??       # Help with partial arguments
```

### Auto-Generated Help Commands
```bash
.command.help                 # Direct help command access
.namespace.command.help       # Works with namespaced commands
```

## Feature Configuration

### Core Features
```toml
[dependencies]
unilang = "0.10"              # Default: enhanced_repl + simd + enabled
```

### Performance Optimized
```toml
[dependencies]
unilang = { version = "0.10", features = ["simd", "enhanced_repl"] }
```

### Minimal Footprint
```toml
[dependencies]
unilang = { version = "0.10", default-features = false, features = ["enabled"] }
```

### Available Features
- **`enabled`** - Core functionality (required)
- **`simd`** - SIMD optimizations for 4-25x parsing performance
- **`enhanced_repl`** - Advanced REPL with history, completion, secure input
- **`repl`** - Basic REPL functionality
- **`on_unknown_suggest`** - Fuzzy command suggestions

## Examples and Learning Path

### 🚀 **Start Here: Recommended Learning Path**

**1. Quick Start (Runtime, Educational Only)**
- `00_quick_start.rs` - Get something working in 5 minutes (⚠️ runtime registration, slow)
- `01_basic_command_registration.rs` - Understand the runtime API (⚠️ 50x slower than compile-time)

**2. Production Approach (Compile-Time, Recommended)**
- `static_01_basic_compile_time.rs` - **READ THIS FIRST** - Explains proper YAML + build.rs pattern
- `static_02_yaml_build_integration.rs` - Multi-YAML file aggregation
- `static_03_performance_comparison.rs` - Benchmark compile-time vs runtime (proves 50x speedup)
- `static_04_multi_module_aggregation.rs` - Organize commands across modules

**3. Advanced Type System**
- `02_argument_types.rs` - String, Integer, Float, Boolean, Path, etc. (⚠️ requires `json_parser`)
- `03_collection_types.rs` - Lists, Maps with custom delimiters
- `14_advanced_types_validation.rs` - Complex validation rules (⚠️ requires `json_parser`)

**4. Help & User Experience**
- `06_help_system.rs` - Comprehensive help system
- `18_help_conventions_demo.rs` - Three help access methods (?, ??, .help)

**5. REPL Applications**
- `12_repl_loop.rs` - Basic REPL implementation
- `15_interactive_repl_mode.rs` - Interactive arguments + secure input (⚠️ requires `enhanced_repl`)
- `17_advanced_repl_features.rs` - History, completion, recovery (⚠️ requires `enhanced_repl`)

**6. Complete Applications**
- `full_cli_example.rs` - Full-featured CLI with all concepts integrated
- `practical_cli_aggregation.rs` - Real-world multi-tool aggregation (⚠️ requires `multi_file`)

### ⚠️ **Feature Requirements Legend**
- No marker = Works with default features
- ⚠️ `json_parser` = Requires JSON support feature
- ⚠️ `enhanced_repl` = Requires advanced REPL features
- ⚠️ `multi_file` = Requires multi-file aggregation (default includes this)

## WebAssembly Support

unilang provides full WebAssembly compatibility for browser deployment:

```bash
cd examples/wasm-repl
wasm-pack build --target web --release
cd www && python3 -m http.server 8000
```

**WASM Features:**
- Complete framework functionality in browsers
- SIMD acceleration where supported
- Optimized bundle size (800KB-1.2MB compressed)
- Seamless Rust-to-JavaScript integration

## Migration from Runtime to Compile-Time

Migrate from runtime registration (slow) to compile-time registration (50x faster) in 4 steps.

### Step 1: Extract Command Definitions to YAML

**Before (Runtime, in main.rs):**
```rust,ignore
#[allow(deprecated)]
let mut registry = CommandRegistry::new();

let greet_cmd = CommandDefinition {
  name: ".greet".to_string(),
  namespace: String::new(),
  description: "Greeting command".to_string(),
  hint: "Say hello".to_string(),
  arguments: vec![
    ArgumentDefinition {
      name: "name".to_string(),
      kind: Kind::String,
      description: "Person's name".to_string(),
      hint: "Name".to_string(),
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("World".to_string()),
        ..Default::default()
      },
      validation_rules: vec![],
      aliases: vec![],
      tags: vec![],
    }
  ],
  // ... other fields
  status: "stable".to_string(),
  version: "1.0.0".to_string(),
  aliases: vec![],
  tags: vec![],
  permissions: vec![],
  idempotent: true,
  deprecation_message: String::new(),
  http_method_hint: String::new(),
  examples: vec![],
  routine_link: None,
  auto_help_enabled: false,
};

#[allow(deprecated)]
registry.command_add_runtime(&greet_cmd, greet_routine)?;
```

**After (Compile-Time, in unilang.commands.yaml):**
```yaml
- name: ".greet"
  namespace: ""
  description: "Greeting command"
  hint: "Say hello"
  status: "stable"
  version: "1.0.0"
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: ""
  auto_help_enabled: false
  examples: []
  arguments:
    - name: "name"
      kind: "String"
      description: "Person's name"
      hint: "Name"
      attributes:
        optional: true
        default: "World"
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: []
      tags: []
  routine_link: null
```

### Step 2: Update Cargo.toml

**Add feature flag:**
```toml
[dependencies]
# Enable single-file YAML compile-time approach
unilang = { version = "0.28", features = ["approach_yaml_single_build"] }

# Or use default (multi-file auto-discovery)
unilang = "0.28"
```

### Step 3: Configure Build Script (Single-File Only)

For `approach_yaml_single_build`, create `build.rs`:
```rust,ignore
fn main()
{
  // Rebuild if YAML file changes
  println!("cargo:rerun-if-changed=unilang.commands.yaml");

  // Static registry generation happens automatically
  // No manual code needed - the feature flag handles it
}
```

**Note:** With default `approach_yaml_multi_build`, no build.rs needed - auto-discovery handles everything!

### Step 4: Update Code to Use Static Registry

**Before (Runtime):**
```rust,ignore
use unilang::prelude::*;

fn main() -> Result<(), unilang::Error> {
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Manual registration (slow)
  #[allow(deprecated)]
  registry.command_add_runtime(&greet_cmd, greet_routine)?;

  let pipeline = Pipeline::new(registry);
  let result = pipeline.process_command_simple(".greet name::Alice");
  Ok(())
}
```

**After (Compile-Time):**
```rust,ignore
use unilang::prelude::*;

// Include compile-time generated commands (auto-generated by build system)
include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));

fn main() -> Result<(), unilang::Error> {
  // Zero-cost static registry (~80ns lookup vs ~4,000ns runtime)
  let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);

  let pipeline = Pipeline::new(registry);
  let result = pipeline.process_command_simple(".greet name::Alice");
  Ok(())
}
```

### Step 5: Measure Performance Improvement

**Run benchmarks:**
```bash
cargo run --example static_03_performance_comparison
```

**Expected results:**
- **Runtime registration**: ~4,000ns per command lookup
- **Compile-time registration**: ~80-100ns per command lookup
- **Performance gain**: 50x faster

## Performance Optimization Guidelines

### Compile-Time Best Practices
- Use static command definitions for all known commands
- Leverage multi-module aggregation for organization
- Enable SIMD features for maximum parsing performance
- Utilize conflict detection during build process

### Runtime Considerations
- Reserve runtime registration for truly dynamic scenarios
- Minimize command modifications during execution
- Use batch processing for multiple commands
- Implement proper error handling and recovery
