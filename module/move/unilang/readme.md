<!-- {{# generate.module_header{} #}} -->

# Module :: unilang
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_unilang_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_unilang_push.yml) [![docs.rs](https://img.shields.io/docsrs/unilang?color=e3e8f0&logo=docs.rs)](https://docs.rs/unilang) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fmove%2Funilang%2Fexamples%2F00_pipeline_basics.rs,RUN_POSTFIX=--example%20module%2Fmove%2Funilang%2Fexamples%2F00_pipeline_basics.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

**Zero-overhead command framework with compile-time command registration**

## Value Proposition

unilang processes command definitions at compile-time, generating Perfect Hash Function (PHF) maps that provide **O(1) command lookups with zero runtime overhead**. This approach delivers:

- **10-50x faster command resolution** compared to runtime HashMap lookups
- **Compile-time validation** of all command definitions and arguments
- **Smaller binary size** through static analysis and dead code elimination
- **SIMD acceleration** for parsing with 4-25x performance improvements
- **Zero memory allocations** for command lookup operations

## Architecture Overview

**Compile-Time Processing:**
```text
YAML definitions → build.rs → PHF maps → Zero-cost lookups
```

**Runtime Execution:**
```text
Command string → O(1) PHF lookup → Validated execution
```

## Quick Start: Compile-Time Registration (Recommended)

### Step 1: Define Commands

Create `unilang.commands.yaml`:
```yaml
- name: "greet"
  namespace: ""
  description: "High-performance greeting command"
  arguments:
    - name: "name"
      kind: "String"
      attributes:
        optional: true
        default: "World"
```

### Step 2: Configure Build Script

Add to `build.rs`:
```rust,ignore
use std::env;
use std::path::Path;

fn main()
{
  println!( "cargo:rerun-if-changed=unilang.commands.yaml" );

  let out_dir = env::var( "OUT_DIR" ).unwrap();
  let dest_path = Path::new( &out_dir ).join( "static_commands.rs" );

  // Generate PHF maps at compile-time
  unilang::build::generate_static_commands( &dest_path, "unilang.commands.yaml" );
}
```

### Step 3: Zero-Cost Execution

```rust,ignore
use unilang::prelude::*;

// Include compile-time generated PHF maps
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main() -> Result< (), unilang::Error >
{
  let registry = StaticCommandRegistry::new( &STATIC_COMMANDS );
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
| **Compile-Time (PHF)** | 1-3 CPU cycles | Zero | Smaller |
| Runtime (HashMap) | 50-150 CPU cycles | Hash tables + allocations | Larger |

**Benchmark Results:**
- **Static lookups:** ~2ns per operation
- **Dynamic lookups:** ~80ns per operation
- **Performance gain:** 40x faster command resolution

## User Guide: Integration Decisions

### Decision 1: How Should I Define Commands?

**10 approaches are currently implemented** (see [full comparison](docs/cli_definition_approaches.md)):

| # | Approach | Lookup Speed | When to Use |
|---|----------|-------------|-------------|
| **1** | **YAML file → Build-time PHF** ⭐ | **~80ns** | **Production apps, best performance, compile-time validation** |
| 2 | Multi-YAML files → Build-time PHF | ~80ns | Large modular projects, multiple CLI tools unified |
| 3 | YAML file → Runtime loading | ~4,200ns | Development, plugin configs loaded at runtime |
| **4** | **JSON file → Build-time PHF** | **~80ns** | **JSON-first projects, API-driven CLI generation** |
| 5 | Multi-JSON files → Build-time PHF | ~80ns | Large JSON projects, modular organization |
| 6 | JSON file → Runtime loading | ~4,200ns | Runtime config loading, dynamic commands |
| 7 | Rust DSL (inline closures) | ~4,200ns | ⚠️ Prototyping/testing only, NOT for production |
| **8** | **Rust DSL (const fn + PHF)** | **~80ns** | **High-performance DSL, type-safe compile-time definitions** |
| 18 | Hybrid (static + runtime) | Mixed | Base CLI + plugin system (best of both worlds) |

**Strongly Recommended:** Row 1 (YAML + build-time PHF) for 50x better performance and compile-time validation.

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

**Recommended: Use defaults** (SIMD enabled automatically)
```toml
[dependencies]
unilang = "0.26"  # Includes SIMD (4-25x parsing speedup), enhanced REPL
```

**Minimal build:**
```toml
unilang = { version = "0.26", default-features = false, features = ["enabled"] }
```

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

unilang excels at aggregating multiple CLI tools into a single unified command interface. This is essential for organizations that want to consolidate developer tools while maintaining namespace isolation.

### Real-World Aggregation Scenario

```rust,ignore
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

```rust
// Database commands become .db.migrate, .db.backup
// File commands become .fs.copy, .fs.move
// Network commands become .net.ping, .net.trace
// No naming conflicts between modules
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
//   → Commands already in memory, fast O(1) lookup
// - dynamic_module_with_prefix(name, PathBuf, prefix)
//   → Commands loaded from YAML file at runtime, ~50x slower lookup
```

### Performance Characteristics

| Approach | Lookup Time | Memory Overhead | Conflict Detection |
|----------|-------------|-----------------|-------------------|
| **Compile-Time** | O(1) PHF | Zero | Build-time |
| Runtime | O(log n) | Hash tables | Runtime |

**Aggregation Scaling:**
- **10 modules, 100 commands each**: ~750ns lookup regardless of module count
- **Single PHF map**: All 1,000 commands accessible in constant time
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
- name: "command_name"           # Required: Command identifier
  namespace: "optional.prefix"   # Optional: Hierarchical organization
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

### Compile-Time Focus Examples
- `static_01_basic_compile_time.rs` - PHF-based zero-cost lookups
- `static_02_yaml_build_integration.rs` - Build script integration patterns
- `static_03_performance_comparison.rs` - Concrete performance measurements
- `static_04_multi_module_aggregation.rs` - Modular command organization

### Traditional Examples
- `01_basic_command_registration.rs` - Runtime registration patterns
- `02_argument_types.rs` - Comprehensive argument type examples
- `07_yaml_json_loading.rs` - Dynamic command loading

### Advanced Features
- `18_help_conventions_demo.rs` - Help system demonstration
- `full_cli_example.rs` - Complete CLI application

### REPL and Interactive
- `12_repl_loop.rs` - Basic REPL implementation
- `15_interactive_repl_mode.rs` - Interactive arguments and secure input
- `17_advanced_repl_features.rs` - History, auto-completion, error recovery

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

### Step 1: Extract Command Definitions
Convert runtime `CommandDefinition` structures to YAML format.

### Step 2: Configure Build Script
Add compile-time generation to `build.rs`.

### Step 3: Update Code
Replace `CommandRegistry::new()` with compile-time command registration via build.rs.

### Step 4: Measure Performance
Use provided benchmarking examples to verify improvements.

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
