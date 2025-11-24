# Comprehensive CLI Definition Approaches for Unilang

⚠️ **IMPORTANT: Opinionated Defaults**

By default, unilang **ONLY** enables **Approach #2** (Multi-YAML Build-Time Static). This is the recommended production approach for 95% of users.

**To use any other approach, you must explicitly enable its feature flag in `Cargo.toml`.**

This document catalogs **all realistic ways** to define CLI commands in the unilang framework - current implementations, planned features, and practical possibilities.

**Complete YAML/JSON parity**: For every YAML approach, there is an equivalent JSON approach (3 YAML + 3 JSON = 6 core variants).

Ridiculous/impractical approaches have been excluded (e.g., Windows Registry, HTTP API, Python scripts for command definitions).

## Quick Navigation

- [Comparison Table](#comparison-table) - All 21 realistic approaches with feature flags
- [Current Implementations](#current-implementations) - What works today
- [Default Approach](#default-approach) - Approach #2 (Multi-YAML Build-Time Static)
- [Using Alternative Approaches](#using-alternative-approaches) - How to enable other approaches
- [Future Enhancements](#future-enhancements) - Planned additions
- [Advanced Possibilities](#advanced-possibilities) - Niche but valid scenarios

---

## Comparison Table

| # | Definition Format | Feature Flag | Default? | Implemented | Easiness | Performance | Flexibility | Recommended Use Case |
|---|-------------------|--------------|----------|-------------|----------|-------------|-------------|----------------------|
| 1 | YAML file → Build-time static | `approach_yaml_single_build` | ❌ | ✅ YES | ✅ Very Easy | ⚡ Best (<100ns) | ⚠️ Medium | Simple projects (<20 commands) |
| **2** | **YAML files → Build-time static** | **`approach_yaml_multi_build`** | **✅ DEFAULT** | **✅ YES** | **✅ Easy** | **⚡ Best (<100ns)** | **⚠️ Medium** | **PRODUCTION - Scalable, modular** |
| 3 | YAML file → Runtime | `approach_yaml_runtime` | ❌ | ✅ YES | ✅ Very Easy | ⚠️ Slow (10-50μs) | ✅ High | Dev/prototyping, plugins |
| 4 | JSON file → Build-time static | `approach_json_single_build` | ❌ | ✅ YES | ✅ Very Easy | ⚡ Best | ⚠️ Medium | JSON-first projects (simple) |
| 5 | JSON files → Build-time static | `approach_json_multi_build` | ❌ | ✅ YES | ✅ Easy | ⚡ Best | ⚠️ Medium | JSON-first projects (large) |
| 6 | JSON file → Runtime | `approach_json_runtime` | ❌ | ✅ YES | ✅ Very Easy | ⚠️ Slow | ✅ High | Runtime config loading |
| 7 | Rust DSL (builder) | *(always available)* | ✅ Core API | ✅ YES | 🔥 Hard | ⚠️ Slow (~4,200ns) | ✅ Highest | Tests, full control |
| 8 | Rust DSL (const fn) → Static | `approach_rust_dsl_const` | ❌ | ✅ YES | 🔥 Hard | ⚡ Best (~80ns) | ⚠️ Medium | High-perf DSL |
| 9 | Declarative macro → Static | `approach_macro_declarative` | ❌ | ❌ NO | ⚠️ Medium | ⚡ Best | ⚠️ Medium | Clean syntax (future) |
| 10 | Proc macro (derive) → Static | `approach_macro_proc` | ❌ | ❌ NO | ✅ Easy | ⚡ Best | ⚠️ Low | Derive-style like clap (future) |
| 11 | TOML file → Build-time static | `approach_toml_single_build` | ❌ | ❌ NO | ✅ Very Easy | ⚡ Best | ⚠️ Medium | Config-heavy projects (future) |
| 11+ | TOML files → Build-time static | `approach_toml_multi_build` | ❌ | ❌ NO | ✅ Easy | ⚡ Best | ⚠️ Medium | Large TOML projects (future) |
| 12 | TOML file → Runtime | `approach_toml_runtime` | ❌ | ❌ NO | ✅ Very Easy | ⚠️ Slow | ✅ High | Runtime TOML config (future) |
| 13 | RON file → Build-time static | `approach_ron_single_build` | ❌ | ❌ NO | ⚠️ Medium | ⚡ Best | ⚠️ Medium | Rust-native syntax (future) |
| 13+ | RON files → Build-time static | `approach_ron_multi_build` | ❌ | ❌ NO | ⚠️ Medium | ⚡ Best | ⚠️ Medium | Large RON projects (future) |
| 14 | RON file → Runtime | `approach_ron_runtime` | ❌ | ❌ NO | ⚠️ Medium | ⚠️ Slow | ✅ High | Rust-native runtime (future) |
| 15 | Protobuf schema → Static | `approach_protobuf` | ❌ | ❌ NO | 🔥 Hard | ⚡ Best | ⚠️ Low | gRPC services (future) |
| 16 | GraphQL schema → Static | `approach_graphql` | ❌ | ❌ NO | 🔥 Hard | ⚡ Best | ⚠️ Low | GraphQL API → CLI (future) |
| 17 | OpenAPI spec → Static | `approach_openapi` | ❌ | ❌ NO | ⚠️ Medium | ⚡ Best | ⚠️ Low | REST API → CLI (future) |
| 18 | Hybrid (static + runtime) | `approach_hybrid` | ❌ | ✅ YES | ⚠️ Medium | ⚡/⚠️ Mixed | ✅ Highest | Base CLI + plugins |
| 19 | Plugin system (.so/.dll) | `approach_plugin` | ❌ | ❌ NO | 🔥 Very Hard | ⚡ + dlopen | ✅ High | True plugin architecture (future) |
| 20 | Binary serialization | `approach_binary` | ❌ | ❌ NO | 🔥 Hard | ⚡ Best | ⚠️ Low | Maximum performance (future) |
| 21 | Inline YAML/JSON literals | `approach_inline_literals` | ❌ | ❌ NO | ✅ Very Easy | ⚡ Best | ⚠️ Medium | Self-contained binaries (future) |

**Total**: 23 approaches (21 base + 2 multi-file variants for TOML/RON)
**Implemented**: 9 approaches
**Default**: Only Approach #2

---

## Legend

### Easiness (User Perspective - How Easy to Use)

- ✅ **Very Easy**: No learning curve, obvious how to use (YAML/JSON/TOML files)
- ✅ **Easy**: Minimal learning required, straightforward (multi-file discovery, proc macros)
- ⚠️ **Medium**: Some learning required (macros, RON syntax, hybrid approach)
- 🔥 **Hard**: Significant learning curve (Rust DSL builder, protobuf, GraphQL, binary formats)
- 🔥 **Very Hard**: Expert-level knowledge required (FFI, dynamic libraries, plugin systems)

### Performance

- ⚡ **Best**: <1μs (Static optimized lookup, O(1) const-time)
- ⚠️ **Slow**: 10-100μs (File I/O, dynamic HashMap with LRU cache)

### Flexibility

- ✅ **Highest**: Can change without recompile, hot-reload possible
- ⚠️ **Medium**: Requires recompile for changes
- 🔥 **Low**: Rigid structure, limited customization

### Complexity (Developer Perspective - Implementation Effort)

- ✅ **Simple**: Straightforward implementation, <1 week
- ⚠️ **Medium**: Requires infrastructure, 1-2 weeks
- 🔥 **Complex**: Significant engineering effort, 2-4 weeks
- 🔥 **Very Complex**: Major architectural work, 1+ months

### Implementation Status

- ✅ **YES**: Fully implemented and tested
- ⚠️ **PARTIAL**: Some functionality exists
- ❌ **NO**: Not implemented

---

## Current Implementations (✅ YES)

### Summary: 10 of 21 approaches implemented ✅ **COMPLETE YAML/JSON PARITY + Row 7/8**

**Implemented (✅ YES)**:
- #1: YAML file → Build-time static ✅
- #2: YAML files (multi) → Build-time static ✅
- #3: YAML file → Runtime loading ✅
- #4: JSON file → Build-time static ✅
- #5: JSON files (multi) → Build-time static ✅
- #6: JSON file → Runtime loading ✅
- #7: Rust DSL (builder) → Dynamic HashMap with inline closures ✅
- #8: Rust DSL (const fn) → Static optimized with named functions ✅ **NEW!**
- #18: Hybrid (static + runtime) ✅

**YAML/JSON Parity Status**: ✅ **6/6 variants (100% complete)**

**Test Coverage**:
- Build-time tests: 7 tests (BT1.1-BT6.1) covering all static variants
- Runtime tests: 10 tests (YAML: 5, JSON: 5) covering runtime loading
- Row 7 tests: 14 tests (IC1.1-IC7.2) covering inline closure registration
- Row 8 tests: 14 tests (CC1.1-CC5.2) covering const fn constructors
- Validation tests: 20 tests (V1.1-V4.3) covering centralized validation
- **Total**: 65 tests ensuring complete coverage

### #1: YAML File → Build-time Static ⭐ RECOMMENDED DEFAULT

**Implementation**: `build.rs` + `unilang.commands.yaml`

**How it works**:
1. Define commands in `unilang.commands.yaml`
2. Build script reads YAML at compile-time
3. Generates optimized static command registry
4. Commands compiled into binary with zero runtime overhead

**Example YAML**:
```yaml
- name: ".video.search"
  namespace: ""
  description: "Search for videos"
  arguments:
    - name: "query"
      kind: "String"
      attributes:
        optional: false
  status: "stable"
  version: "1.0.0"
```

**Performance**: <100ns per command lookup (50x faster than runtime)

**Code locations**:
- Build script: `build.rs:1-409`
- YAML manifest: `unilang.commands.yaml`
- Generated code: `$OUT_DIR/static_commands.rs`
- Static registry: `src/registry.rs:966-1338`

**When to use**: ✅ Production applications, performance-critical CLIs

---

### #2: Multi-file YAML → Build-time Static

**Implementation**: `build.rs` with `walkdir` discovery

**How it works**:
1. Scatter YAML files across project (e.g., `src/commands/*.yaml`)
2. Build script discovers all `.yaml`/`.yml` files recursively
3. Merges definitions and generates optimized static map
4. Automatically excludes `tests/` and `test_data/` directories

**Configuration**:
```bash
# Set discovery paths (colon-separated)
export UNILANG_YAML_DISCOVERY_PATHS="./src/commands:./plugins"
```

**Code location**: `build.rs:63-148`

**When to use**: ✅ Large projects with modular command organization

---

### #3: YAML File → Runtime Loading

**Implementation**: `CommandRegistryBuilder::load_from_yaml_str()`

**How it works**:
```rust
let yaml_content = std::fs::read_to_string("commands.yaml")?;
let registry = CommandRegistry::builder()
  .load_from_yaml_str(&yaml_content)?
  .build();
```

**Performance**: ~10-50μs per command (HashMap + LRU cache)

**Code location**: `src/registry.rs:909-930`

**When to use**:
- ✅ Development and prototyping
- ✅ Plugin systems with user-provided commands
- ✅ Configuration files loaded at startup

**Warning**: ⚠️ 10-50x slower than build-time static approach

---

### #5: JSON File → Runtime Loading

**Implementation**: `CommandRegistryBuilder::load_from_json_str()`

**How it works**:
```rust
let json_content = std::fs::read_to_string("commands.json")?;
let registry = CommandRegistry::builder()
  .load_from_json_str(&json_content)?
  .build();
```

**Code location**: `src/registry.rs:932-954`

**When to use**: ✅ API-driven applications, JSON-first ecosystems

---

### #7: Rust DSL → Dynamic HashMap with Inline Closures

**Implementation**: `CommandRegistryBuilder::command_with_routine()`

**How it works**:
```rust
let registry = CommandRegistry::builder()
  .command_with_routine(
    ".greet",
    "Greets the user by name",
    |cmd, _ctx| {
      let name = cmd.arguments.get("name")
        .and_then(|v| {
          if let Value::String(s) = v {
            Some(s.as_str())
          } else {
            None
          }
        })
        .unwrap_or("World");

      Ok(OutputData {
        content: format!("Hello, {name}!"),
        format: "text".to_string(),
      })
    }
  )
  .build();
```

**Performance**: ⚠️ ~4,200ns per command lookup (HashMap)

**Code location**: `src/registry.rs:956-984`

**When to use**:
- ✅ Prototyping new features
- ✅ Small CLI tools (<100 commands)
- ✅ Applications where development speed > performance
- ✅ Inline closures that capture variables

**Benefits**:
- ✅ Fastest development time (write closures inline)
- ✅ Maximum flexibility (closures can capture variables)
- ✅ Perfect for rapid iteration

**Drawbacks**:
- ⚠️ Runtime lookup cost (~4,200ns vs ~80ns for Row 8)
- ⚠️ No compile-time validation

**Example**: `examples/20_rust_dsl_inline_closures.rs`

---

### #8: Rust DSL → Compile-Time Static with Const Fn Constructors

**Implementation**: `StaticCommandDefinition::new()` + const fn builders

**How it works**:
```rust
// Define static command at compile-time
const GREET_CMD: StaticCommandDefinition = StaticCommandDefinition::new(
  ".greet",
  "",
  "Greets the user by name",
)
.with_hint("Say hello to someone")
.with_version("2.0.0");

// Named routine function (no closures - must be named)
fn greet_routine(cmd: VerifiedCommand, _ctx: ExecutionContext)
  -> Result<OutputData, Box<dyn std::error::Error>>
{
  let name = cmd.arguments.get("name")
    .and_then(|v| {
      if let Value::String(s) = v {
        Some(s.as_str())
      } else {
        None
      }
    })
    .unwrap_or("World");

  Ok(OutputData {
    content: format!("Hello, {name}!"),
    format: "text".to_string(),
  })
}

// Register with named routine
let registry = CommandRegistry::builder()
  .command_with_routine(".greet", "Greets the user", greet_routine)
  .build();
```

**Performance**: ⚡ ~80ns per command lookup (50x faster than Row 7)

**Code location**: `src/static_data.rs:1-400`

**When to use**:
- ✅ Performance-critical applications
- ✅ Large CLI tools (>100 commands)
- ✅ Compile-time command validation needed
- ✅ Long-running services where startup time matters

**Benefits**:
- ✅ Maximum performance (~80ns lookup, 50x faster than Row 7)
- ✅ Zero runtime overhead
- ✅ Compile-time validation
- ✅ No heap allocations for command definitions

**Drawbacks**:
- ⚠️ More verbose than inline closures
- ⚠️ Requires named functions (no inline closures)
- ⚠️ Static lifetime constraints on all data

**Example**: `examples/21_rust_dsl_static.rs`

---

### #17: Hybrid (Static Optimized + Runtime Dynamic)

**Implementation**: `StaticCommandRegistry`

**How it works**:
```rust
let mut registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);

// Static commands from compile-time optimized map (fast, no special dependencies required)
let static_cmd = registry.command(".video.search"); // <100ns

// Add dynamic commands at runtime
registry.register(dynamic_command); // Slower but flexible
```

**Performance**: ⚡ Best for static lookups, ⚠️ Slow for dynamic

**Code location**: `src/registry.rs:987-1338`

**When to use**: ✅ Base CLI + plugin system

---

## Default Approach

### 🎯 Approach #2: Multi-YAML Build-Time Static (ENABLED BY DEFAULT)

**This is the only approach enabled by default. Here's why**:

| Criterion | YAML + Static | Runtime Registration | Rust DSL |
|-----------|------------|---------------------|----------|
| **Easiness** | ✅ Very Easy | ✅ Very Easy | 🔥 Hard (verbose) |
| **Performance** | ⚡ <100ns | ⚠️ 4,200ns (42x slower) | ⚠️ 3,800ns (38x slower) |
| **Memory** | 0 bytes overhead | 512KB HashMap | 512KB HashMap |
| **Type safety** | ✅ Compile-time validation | ❌ Runtime errors | ✅ Compile-time |
| **Simplicity** | ✅ Edit YAML → rebuild | ⚠️ Load files at runtime | ❌ Verbose code |
| **Version control** | ✅ Clear diffs | ✅ Clear diffs | ❌ Code noise |
| **Tooling** | ✅ YAML editors, validators | ✅ YAML editors | ❌ IDE only |

**Migration from runtime to compile-time**:

```rust
// ❌ BEFORE: Runtime registration (slow)
fn main() {
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name(".search")
    .description("Search command")
    .end();

  let routine = Box::new(|cmd, ctx| { /* ... */ });
  registry.command_add_runtime(&cmd, routine)?;
}

// ✅ AFTER: YAML + Build-time Static (50x faster)
// 1. Create unilang.commands.yaml:
//    - name: ".search"
//      description: "Search command"
//      arguments: []
//
// 2. Use static registry in main.rs:
fn main() {
  let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
  // Commands are already registered at compile-time, zero dependencies!
}
```

**Performance improvement**: 50x faster command lookup

---

## Using Alternative Approaches

### How to Enable Other Approaches

By default, ONLY Approach #2 (Multi-YAML Build-Time Static) is available. To use any other approach, you must explicitly enable its feature flag:

**Example 1: Use Approach #1 (Single-YAML Build-Time Static)**

```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = [
  "enabled",
  "approach_yaml_single_build"  # Enable Approach #1
]}
```

**Example 2: Use Approach #3 (YAML Runtime Loading)**

```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = [
  "enabled",
  "approach_yaml_runtime"  # Enable Approach #3
]}
```

**Example 3: Use Approach #7 (Rust DSL Builder)**

```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = [
  "enabled"  # Approach #7 is always available as core API
]}
```

**Example 4: Enable Multiple Approaches**

```toml
[dependencies]
unilang = { version = "0.28", default-features = false, features = [
  "enabled",
  "approach_yaml_multi_build",   # Multi-YAML (build-time)
  "approach_yaml_runtime",        # YAML (runtime)
  "approach_json_runtime"         # JSON (runtime)
]}
```

**Example 5: Enable ALL Implemented Approaches**

```toml
[dependencies]
unilang = { version = "0.28", features = ["full"] }
```

### Feature Flag Reference

**Implemented Approaches (9 features)**:

| Feature Flag | Approach | Description |
|--------------|----------|-------------|
| `approach_yaml_single_build` | #1 | Single YAML → Build-time static |
| `approach_yaml_multi_build` | #2 | Multi-YAML → Build-time static (DEFAULT) |
| `approach_yaml_runtime` | #3 | YAML → Runtime loading |
| `approach_json_single_build` | #4 | Single JSON → Build-time static |
| `approach_json_multi_build` | #5 | Multi-JSON → Build-time static |
| `approach_json_runtime` | #6 | JSON → Runtime loading |
| *(always available)* | #7 | Rust DSL builder (core API) |
| `approach_rust_dsl_const` | #8 | Rust DSL const fn → Static |
| `approach_hybrid` | #18 | Hybrid (static + runtime) |

**Convenience Features**:

| Feature Flag | Enables |
|--------------|---------|
| `all_yaml_approaches` | All 3 YAML approaches (#1, #2, #3) |
| `all_json_approaches` | All 3 JSON approaches (#4, #5, #6) |
| `all_static_approaches` | All static/build-time approaches |
| `all_runtime_approaches` | All runtime approaches |
| `full` | Everything (all implemented approaches) |

**Infrastructure Features** (usually auto-enabled by approaches):

| Feature Flag | Purpose | Enables Dependency |
|--------------|---------|-------------------|
| `static_registry` | Static command registry | (internal) |
| `yaml_parser` | YAML parsing | `serde_yaml` |
| `json_parser` | JSON parsing | `serde_json` |
| `multi_file` | Multi-file discovery | `walkdir` |

### Why Opinionated Defaults?

We chose to enable ONLY Approach #2 by default for several reasons:

1. **Scalability**: Multi-file organization scales to large projects naturally
2. **Performance**: Zero-overhead (<100ns lookups) for production
3. **Best Practice**: Modular command organization is the industry standard
4. **Team Collaboration**: Separate files = fewer merge conflicts
5. **Auto-Discovery**: Drop YAML files anywhere, they're automatically found
6. **Force Conscious Choice**: Alternative approaches require explicit opt-in

If you need something different, we make it easy - just enable the feature! But we believe 95% of users are best served by this default.

---

## Future Enhancements (❌ Not Implemented)

### Priority 1: Approach #8 - Declarative Macro

**Proposed syntax**:
```rust
command! {
  .video.search {
    description: "Search for videos",
    arguments: {
      query: String(required),
      limit: Integer(optional, default: 10),
    },
    routine: |cmd, ctx| {
      let query = cmd.arguments.get("query")?;
      // Implementation
      Ok(OutputData::default())
    }
  }
}
```

**Benefits**:
- ✅ Clean DSL-like syntax
- ✅ Compile-time validation
- ✅ Static-compatible (generates static data)
- ✅ Less verbose than builder pattern

**Implementation effort**: 2-3 weeks

**Why this matters**: Bridges the gap between YAML (declarative) and Rust DSL (programmatic)

---

### Priority 2: Approach #20 - Inline YAML/JSON

**Proposed syntax**:
```rust
const COMMANDS: &str = r#"
- name: example
  description: Example command
  arguments: []
"#;

// Macro parses at compile-time, generates optimized static registry
static_commands_from_yaml!(COMMANDS);
```

**Benefits**:
- ✅ Self-contained single-file binaries
- ✅ No external YAML file dependencies
- ✅ Still gets compile-time optimized performance

**Implementation effort**: 1 week

**Why this matters**: Great for small tools that want zero external dependencies

---

### Priority 3: Approach #10-13 - TOML/RON Support

**Why TOML**:
- Many Rust devs prefer TOML over YAML
- Simpler syntax, better error messages
- Natural fit for Cargo.toml users

**Example TOML**:
```toml
[[command]]
name = ".video.search"
namespace = ""
description = "Search for videos"

[[command.arguments]]
name = "query"
kind = "String"
optional = false
```

**Implementation effort**: 2-3 days (reuse existing build.rs infrastructure)

---

### Priority 4: Approach #4 - JSON Build-time Support

**Why JSON**:
- Already have runtime JSON loading (#5)
- Easy to generate from APIs/codegen
- Just needs build.rs integration

**Implementation effort**: 1 day

---

## Advanced Possibilities

### Approach #14: Protobuf Schema

**Use case**: Generate CLI from existing gRPC service definitions

```protobuf
message SearchRequest {
  string query = 1;
  int32 limit = 2;
}

service VideoService {
  rpc Search(SearchRequest) returns (SearchResponse);
}
```

**Generated CLI**:
```bash
$ mycli .video.search query::"rust tutorial" limit::10
```

**Implementation**: protoc plugin → YAML/JSON → static registry

**Complexity**: 🔥 Complex (3-4 weeks)

**When to use**: Projects with existing protobuf APIs

---

### Approach #15: GraphQL Schema

**Use case**: Generate CLI from GraphQL schema

```graphql
type Query {
  search(query: String!, limit: Int = 10): [Video]!
}
```

**Generated CLI**:
```bash
$ mycli .query.search query::"rust tutorial" limit::10
```

**Implementation**: GraphQL parser → YAML/JSON → static registry

**Complexity**: 🔥 Complex (3-4 weeks)

---

### Approach #16: OpenAPI Spec

**Use case**: Generate CLI from REST API OpenAPI definitions

```yaml
paths:
  /api/search:
    get:
      operationId: search
      parameters:
        - name: query
          in: query
          required: true
          schema:
            type: string
```

**Generated CLI**:
```bash
$ mycli .api.search query::"rust tutorial"
```

**Implementation**: OpenAPI parser → YAML/JSON → static registry

**Complexity**: 🔥 Complex (3-4 weeks)

**Real-world benefit**: Auto-generate CLI clients for any REST API

---

### Approach #18: Plugin System

**Use case**: True plugin architecture with dynamic library loading

```rust
// Plugin: myplugin.so
#[no_mangle]
pub extern "C" fn register_commands() -> Vec<CommandDefinition> {
  vec![/* plugin commands */]
}

// Host application
let plugin = unsafe { libloading::Library::new("myplugin.so")? };
let register: Symbol<fn() -> Vec<CommandDefinition>> =
  unsafe { plugin.get(b"register_commands")? };
let commands = register();
```

**Complexity**: 🔥 Complex (4-6 weeks)

**When to use**: Applications with third-party extension support

---

## Removed Approaches (Ridiculous/Impractical)

These were removed from the table as they don't make practical sense:

1. **SQL schema → CLI commands**: Commands should not live in databases
2. **Python/Lua scripts**: Wrong tool for command definitions (too slow, adds language dependency)
3. **TypeScript/WASM**: Massive overcomplification for CLI definitions
4. **Git repository for commands**: Overcomplicated build process
5. **Environment variables**: Impractical for full command definitions
6. **Cargo features alone**: Only useful for conditional compilation, not full definitions
7. **Database storage (SQLite/Postgres)**: Way overkill for CLI command storage
8. **Windows Registry**: Platform-specific, no cross-platform value
9. **HTTP API**: Too slow, wrong architectural pattern for CLI
10. **YAML with RPC routines**: Doesn't make semantic sense

If you have a genuine use case for any of these, open an issue to discuss.

---

## Performance Benchmark

Real-world benchmark for 1,000 command lookups:

| Approach | Avg Latency | Throughput | Memory Overhead |
|----------|-------------|------------|-----------------|
| **#1: YAML + Static** | **80ns** | **12.5M ops/sec** | **0 bytes** |
| #2: Multi-YAML + Static | 85ns | 11.8M ops/sec | 0 bytes |
| #3: YAML runtime | 4,200ns | 238K ops/sec | 512KB |
| #4: JSON + Static | 80ns | 12.5M ops/sec | 0 bytes |
| #5: Multi-JSON + Static | 85ns | 11.8M ops/sec | 0 bytes |
| #6: JSON runtime | 4,200ns | 238K ops/sec | 512KB |
| #7: Rust DSL inline closures | 4,200ns | 238K ops/sec | 512KB |
| **#8: Rust DSL const fn + Static** | **80ns** | **12.5M ops/sec** | **0 bytes** |
| #17: Hybrid (static) | 80ns | 12.5M ops/sec | 0 bytes |
| #17: Hybrid (dynamic) | 4,200ns | 238K ops/sec | 512KB |

**Conclusion**: Build-time static approaches (#1, #2, #4, #5, #8) are 50x faster and use zero extra memory

**Row 7 vs Row 8 Comparison**:
- Row 7 (Inline Closures): ~4,200ns lookup, maximum flexibility, fastest development
- Row 8 (Const Fn + Static): ~80ns lookup, maximum performance, compile-time validation
- **Performance ratio**: Row 8 is 50x faster than Row 7

---

## Implementation Roadmap

Based on goal: **"Make YAML + build-time static the obvious default choice"**

### Phase 1: Documentation & Guidance (1 week)

1. ✅ Update `readme.md` - YAML-first quick start
2. ✅ Add "Getting Started" guide showing YAML → build-time static workflow
3. ✅ Document build.rs configuration (env vars, discovery paths)
4. ✅ Add deprecation notices to runtime registration docs
5. ✅ Create migration guide (runtime → compile-time)

### Phase 2: Developer Experience (2-3 weeks)

6. ⚡ Implement declarative macro (Approach #8)
7. ⚡ Add inline YAML support (Approach #20)
8. ⚡ Improve build.rs error messages
9. ⚡ Add YAML schema validation at build-time

### Phase 3: Format Expansion (Optional, 1-2 weeks)

10. ⚡ Add TOML support (Approaches #10-11)
11. ⚡ Add JSON build-time support (Approach #4)
12. ⚡ Add RON support (Approaches #12-13)

### Phase 4: Advanced Integrations (Future)

13. ⚡ OpenAPI integration (Approach #16) - most practical
14. ⚡ Protobuf integration (Approach #14) - for gRPC projects
15. ⚡ Plugin system (Approach #18) - for extensible applications

---

## File Locations

### Core Implementation

- **Build script**: `build.rs` - Static registry generation from YAML
- **YAML manifest**: `unilang.commands.yaml` - default command definitions
- **Loader module**: `src/loader.rs` - runtime YAML/JSON loading
- **Registry**: `src/registry.rs` - CommandRegistry, StaticCommandRegistry
- **Static data**: `src/static_data.rs` - Static registry-compatible types
- **Generated code**: `$OUT_DIR/static_commands.rs` - compile-time optimized map

### Test Data

- Runtime YAML: `tests/test_data/dynamic.yaml`
- External files: `tests/test_data/external.yaml`
- Multi-file: `tests/test_data/utils.yaml`
- Compile-time demo: `examples/compile_time_demo.yaml`

---

## Glossary

- **Build-time**: Processed during `cargo build`, code generated and compiled into binary
- **Runtime**: Processed when program executes, dynamic loading from files/memory
- **Static Registry**: Commands compiled into binary with compile-time optimization (fastest, <100ns lookups)
- **Dynamic Registry**: Commands loaded at runtime into HashMap (flexible but slower, ~4-10μs lookups)
- **Hybrid Registry**: Combines static (speed) with dynamic (flexibility) for maximum capability

---

## FAQ

### Q: Why is YAML + build-time static the default?

**A**: It provides the best balance of:
- ✅ Performance (50x faster than runtime)
- ✅ Simplicity (edit YAML, rebuild, done)
- ✅ Type safety (compile-time validation)
- ✅ Tooling (YAML editors, validators, diff tools)

### Q: When should I use runtime registration?

**A**: Only when:
- You're prototyping/developing
- Commands come from user-provided files
- You have a plugin system
- Commands change without recompilation

### Q: Can I mix static and dynamic?

**A**: Yes! Use Approach #17 (Hybrid Registry):
```rust
let mut registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
registry.register(dynamic_command); // Add dynamic commands
```

Static commands are fast (<100ns), dynamic commands are flexible. Zero additional dependencies.

### Q: How do I migrate from runtime to compile-time?

**A**: See [Recommended Approach](#recommended-approach) section for step-by-step guide.

---

## References

- Cargo build scripts: https://doc.rust-lang.org/cargo/reference/build-scripts.html
- Command registry implementation: `src/registry.rs`
- Static command optimization: See `spec.md` Appendix A for implementation details
- Static data structures: `src/static_data.rs`
- Build script implementation: `build.rs:1-409`
- YAML specification: https://yaml.org/spec/
