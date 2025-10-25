# Quick Start Guide

This guide will take you from zero to a working CLI application with compile-time command registration in 6 steps (about 5 minutes).

**What you'll create**: A simple greeting CLI with zero runtime overhead and sub-100ns command lookups.

**What you WON'T need to write**:
- ❌ Custom build.rs (unilang provides this)
- ❌ YAML parsing code (happens at compile-time)
- ❌ PHF map generation (automatic)
- ❌ Build dependencies (already included)

---

## Step 1: Create Project

Create a new Rust binary project:

```bash
cargo new my-cli
cd my-cli
```

**Troubleshooting**:
- **"cargo: command not found"** → Install Rust: https://rustup.rs
- **Permission denied** → Check directory permissions or use `sudo` if needed

---

## Step 2: Add Dependency

Add unilang to your `Cargo.toml`:

```toml
[package]
name = "my-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
# Default enables approach_yaml_multi_build (multi-file auto-discovery)
# This includes: static_registry, yaml_parser, multi_file features
unilang = "0.30"
```

**What this does**:
- Adds unilang with default features
- Automatically includes serde_yaml, walkdir, phf (you don't add these)
- Enables multi-file YAML auto-discovery
- Includes unilang's build.rs automatically

**Troubleshooting**:
- **"failed to download"** → Check internet connection, try `cargo update`
- **Version conflict** → Use `cargo tree` to check dependency conflicts
- **"Cargo.lock is out of date"** → Run `cargo update`

---

## Step 3: Create YAML Commands

Create `commands.yaml` in your project root:

```yaml
# commands.yaml - Auto-discovered by unilang's build.rs
- name: ".greet"
  namespace: ""
  description: "Greet someone"
  arguments:
    - name: "name"
      kind: "String"
      description: "Person to greet"
      attributes:
        optional: true
        default: "World"

- name: ".goodbye"
  namespace: ""
  description: "Say goodbye"
  arguments:
    - name: "name"
      kind: "String"
      description: "Person to say goodbye to"
      attributes:
        optional: false
```

**What this does**:
- Defines 2 commands: `.greet` (optional arg) and `.goodbye` (required arg)
- Will be discovered automatically by unilang's build.rs during `cargo build`
- Parsed at compile-time, not runtime
- Generates optimized PHF map for O(1) lookups

**Troubleshooting**:
- **YAML syntax error** → Use a YAML validator or check indentation (use spaces, not tabs)
- **File not found** → Ensure `commands.yaml` is in project root (same dir as Cargo.toml)
- **Wrong extension** → Must be `.yaml` or `.yml` (not `.txt` or `.yaml.txt`)

---

## Step 4: Write Code

Replace `src/main.rs` with:

```rust
use unilang::prelude::*;

// Include compile-time generated commands
// This file is created by unilang's build.rs during cargo build
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main() -> Result< (), unilang::Error >
{
  // Create static registry with zero runtime overhead
  let registry = StaticCommandRegistry::from_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // Get command from argv (handles shell quoting correctly)
  let args : Vec< String > = std::env::args().collect();
  let result = pipeline.process_command_from_argv( &args );

  if result.success
  {
    println!( "{}", result.outputs[ 0 ].content );
  }
  else if let Some( error ) = result.error
  {
    eprintln!( "Error: {}", error );
    std::process::exit( 1 );
  }

  Ok( () )
}
```

**What this does**:
- `include!()` → Brings in compile-time generated command registry
- `StaticCommandRegistry` → Zero overhead, ~80ns lookup time
- `process_command_from_argv()` → Parses command-line arguments correctly
- Handles success and error cases

**Troubleshooting**:
- **"cannot find macro `include`"** → Standard macro, check Rust installation
- **"STATIC_COMMANDS not found"** → Run `cargo build` first (creates static_commands.rs)
- **"OUT_DIR not set"** → This is set by cargo during build, run `cargo build`

---

## Step 5: Build (See Unilang Work)

Build your project:

```bash
cargo build
```

**You should see**:

```
   Compiling unilang v0.30.0

╔══════════════════════════════════════════════════════════╗
║  Unilang: Compile-Time Command Registry                 ║
╟──────────────────────────────────────────────────────────╢
║  Found 1 YAML file                                       ║
║    - commands.yaml                                       ║
║  Generated PHF map with 2 commands                       ║
║  Lookup time: ~80ns (zero runtime overhead)             ║
║                                                          ║
║  ✅ You did NOT need to write build.rs                  ║
║  ✅ YAML parsed at compile-time                         ║
║  ✅ Command registry ready                              ║
║                                                          ║
║  Docs: https://docs.rs/unilang                          ║
╚══════════════════════════════════════════════════════════╝

   Compiling my-cli v0.1.0 (/path/to/my-cli)
    Finished dev [unoptimized + debuginfo] target(s) in 5.23s
```

**This output proves**:
- Unilang discovered your YAML file
- Commands were parsed at compile-time
- PHF map was generated automatically
- You didn't need to write build.rs

**To suppress this output** (for CI builds):
```bash
UNILANG_QUIET_BUILD=1 cargo build
```

**Troubleshooting**:
- **No unilang output shown** → Check that `commands.yaml` exists in project root
- **"YAML parsing error"** → Check YAML syntax, ensure valid structure
- **"Found 0 YAML files"** → Ensure file is named `*.yaml` or `*.yml` and not in tests/ or examples/
- **Build fails** → Check error message, ensure all dependencies resolved

---

## Step 6: Run

Test your CLI:

```bash
# Using default value
$ cargo run -- .greet
Hello, World!

# With argument
$ cargo run -- .greet name::Alice
Hello, Alice!

# Required argument
$ cargo run -- .goodbye name::Bob
Goodbye, Bob!

# Missing required argument (error)
$ cargo run -- .goodbye
Error: Missing required argument: name

# Unknown command (suggestion)
$ cargo run -- .gret name::Alice
Error: Unknown command '.gret'. Did you mean '.greet'?

# Help
$ cargo run -- .greet ?
Command: .greet
Description: Greet someone
...
```

**Troubleshooting**:
- **"Unknown command"** → Check command name starts with `.` (dot)
- **Argument not parsed** → Use `name::value` format (double colon)
- **"Missing required argument"** → Provide all non-optional arguments
- **Quotes issues** → Use `process_command_from_argv()` (already in code) to handle shell quoting

---

## What You Did NOT Write

Let's compare your project to a manual implementation:

### ✅ Your Project (5 minutes)

```
my-cli/
├── Cargo.toml           (5 lines + dependency)
├── commands.yaml        (15 lines)
└── src/main.rs          (20 lines)
────────────────────────
Total: ~40 lines
Build time: ~5 seconds
Lookup time: ~80ns
```

### ❌ Manual Implementation (4+ hours)

```
manual-cli/
├── Cargo.toml           (20+ lines with build-dependencies)
├── build.rs             (220 lines - YAML discovery, parsing, PHF codegen)
├── commands.yaml        (15 lines)
└── src/main.rs          (50+ lines - manual registry, parsing)
────────────────────────
Total: ~300+ lines
Build time: ~10 seconds
Lookup time: ~4,000ns (50x slower if using HashMap instead of PHF)
Common mistakes: Fake PHF (OnceLock<HashMap>), duplicate dependencies
```

**Real-world example**: See [`willbe/module/lib/build.rs`](https://github.com/Wandalen/wTools/tree/master/module/willbe/module/lib/build.rs) (220 lines of unnecessary code with 50x performance degradation).

---

## Next Steps

### Add More Commands

Create additional YAML files (auto-discovered):

```bash
# Organize by module
touch auth.commands.yaml
touch file.commands.yaml
```

All `.yaml` and `.yml` files in your project root are automatically discovered and combined into one static registry.

### Use Namespaces

Organize commands hierarchically:

```yaml
- name: ".list"
  namespace: "file"
  description: "List files"
  # Usage: my-cli file.list
```

### Add Validation

Add validation rules to arguments:

```yaml
arguments:
  - name: "age"
    kind: "Integer"
    validation_rules:
      - Min: 0
      - Max: 150
```

### Enable REPL Mode

Add interactive mode to your CLI:

```rust
use unilang::prelude::*;

fn main() -> Result< (), unilang::Error >
{
  let registry = StaticCommandRegistry::from_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // REPL loop
  loop
  {
    let mut input = String::new();
    std::io::stdin().read_line( &mut input )?;

    if input.trim() == "exit" { break; }

    let result = pipeline.process_command_simple( &input );
    if result.success
    {
      println!( "{}", result.outputs[ 0 ].content );
    }
  }

  Ok( () )
}
```

### Use cargo_unilang Tool

Scaffold new projects faster:

```bash
cargo install cargo_unilang
cargo_unilang new my-other-cli
cd my-other-cli
cargo run -- .example name::Alice
```

### Health Check

Verify your project follows best practices:

```bash
cargo install cargo_unilang
cargo_unilang check
```

Detects:
- Custom build.rs (unnecessary)
- Duplicate dependencies
- Deprecated API usage
- YAML syntax errors

---

## Common Issues

### Issue: "OUT_DIR environment variable not found"

**Cause**: Trying to run code outside of cargo build
**Solution**: Always use `cargo build` or `cargo run`, not `rustc` directly

### Issue: "STATIC_COMMANDS not found"

**Cause**: Build hasn't run yet or build failed
**Solution**: Run `cargo clean && cargo build` and check for build errors

### Issue: "No YAML files found during build"

**Causes**:
- YAML file not in project root
- YAML file in `tests/` or `examples/` (excluded automatically)
- Wrong file extension (must be `.yaml` or `.yml`)

**Solution**: Ensure YAML file is in project root and has correct extension

### Issue: "Unknown command" at runtime

**Causes**:
- Command name doesn't start with `.` (dot)
- YAML not rebuilt after changes

**Solution**:
- Check command name starts with `.`
- Run `cargo clean && cargo build` to rebuild

### Issue: Build output not showing

**Causes**:
- No YAML files discovered
- `UNILANG_QUIET_BUILD` environment variable set
- YAML files in excluded directories

**Solution**: Check YAML file location and remove `UNILANG_QUIET_BUILD` if set

---

## Performance Tips

### Optimize for Production

```bash
# Release build with optimizations
cargo build --release

# Binary at target/release/my-cli
./target/release/my-cli .greet name::Alice
```

**Performance improvements** in release mode:
- ~2x faster parsing
- Smaller binary size
- Link-time optimization (LTO)

### Measure Performance

Use the static examples for benchmarking:

```bash
cargo run --example static_03_performance_comparison
```

Expected results:
- Static registry: ~80-100ns per lookup
- Runtime registry: ~4,000-5,000ns per lookup
- **50x performance difference**

---

## Understanding the Build Process

What happens when you run `cargo build`:

```
1. Cargo reads Cargo.toml
   └→ Sees unilang dependency

2. Cargo builds unilang
   └→ unilang's build.rs runs
       ├→ Discovers *.yaml files in your project root
       ├→ Parses YAML at compile-time
       ├→ Generates PHF map code
       ├→ Writes static_commands.rs to OUT_DIR
       └→ Prints build summary

3. Cargo builds your project
   └→ Your code includes static_commands.rs via include!()
       └→ PHF map compiled into your binary

4. Result: Zero runtime overhead
   └→ O(1) command lookup (~80ns)
```

**Key insight**: All YAML processing happens at compile-time, not runtime.

---

## Further Reading

- [Full Documentation](https://docs.rs/unilang)
- [CLI Definition Approaches](cli_definition_approaches.md) - 21 ways to define commands
- [Examples](../examples/) - Comprehensive examples
- [Optimization Guide](optimization_guide.md) - Performance tuning
- [Specification](../spec.md) - Complete framework specification

---

## Summary

**You've learned**:
- ✅ How to create a zero-overhead CLI in 5 minutes
- ✅ What unilang does automatically (build.rs, YAML parsing, PHF generation)
- ✅ What you DON'T need to write (220 lines of boilerplate)
- ✅ How to avoid common mistakes (custom build.rs, duplicate deps)
- ✅ How to verify everything works (build output, testing, health check)

**Next**: Build your CLI! Start simple, add features as needed. Unilang scales from 1 command to 1,000+ with the same ~80ns lookup performance.
