# Comprehensive Plan: Preventing Unilang Misuse (Final - CLI Rulebook Compliant)

**Created**: 2025-10-25
**Revised**: 2025-10-25 (CLI Rulebook Compliance Update)
**Status**: Ready for Implementation - FULLY CLI RULEBOOK COMPLIANT
**Priority**: CRITICAL
**Goal**: Make the invisible visible - eliminate framework misuse through progressive prevention

**Estimated Total Effort**: 27.5 hours over 4 weeks (updated for compliance features)
**Success Criteria**: Zero greet_cli-style mistakes within 2 months

**CLI Rulebook Compliance**: ✅ COMPLETE
- All commands use dot-prefix format (`.new`, `.check`)
- All parameters use param::value format (`project::name`)
- Help system implemented (`.`, `.help`, `.command.help`)
- Verbosity system implemented (levels 0-5)
- Exit codes standardized (0, 1, 2, 3)
- Input validation specified (security guidelines compliant)
- Audit report: task/-cargo_unilang_cli_audit_report.md

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Root Cause Analysis](#root-cause-analysis)
3. [Solution Architecture](#solution-architecture)
4. [Implementation Plan](#implementation-plan)
   - [Phase 1: Make Invisible Visible (Week 1)](#phase-1-make-invisible-visible-week-1)
   - [Phase 2: Tooling Infrastructure (Week 2-3)](#phase-2-tooling-infrastructure-week-2-3)
   - [Phase 3: Polish & Documentation (Week 4)](#phase-3-polish--documentation-week-4)
5. [Success Metrics](#success-metrics)
6. [Risk Mitigation](#risk-mitigation)
7. [Maintenance Plan](#maintenance-plan)

---

## Executive Summary

### The Problem

**Root Cause**: Unilang's build.rs is **INVISIBLE**. Users don't know it exists, so they reimplement it.

**Evidence**: greet_cli developer wrote 220-line custom build.rs at `/home/user1/pro/lib/willbe/module/lib/build.rs`, duplicating exactly what unilang already provides.

**Impact**:
- 50x performance degradation (fake PHF using `OnceLock<HashMap>` vs real PHF)
- Duplicate dependencies (serde_yaml, walkdir, phf_codegen)
- Violated all 6 spec requirements (NFR-PERF-1, NFR-PERF-2, etc.)
- Wasted 4+ hours of development time

### The Solution

**Principle**: Make the invisible VISIBLE through 3-layer progressive prevention

```
┌─────────────────────────────────────────────────────────┐
│  BEFORE they code → WHILE they code → WHEN they run    │
│       ↓                  ↓                  ↓           │
│  Prevention          Detection          Validation      │
└─────────────────────────────────────────────────────────┘
```

**Layer 1 - BEFORE they code** (Prevention):
- README starts with "⚠️ DON'T DO THIS" warning
- Package description says "no build.rs needed"
- Minimal example impossible to overlook
- Scaffolding tool creates correct structure

**Layer 2 - WHILE they code** (Detection):
- **Build output shows what unilang did** (NEW - most critical)
- Compile errors guide to solution
- Examples/docs use only current API

**Layer 3 - WHEN they run** (Validation):
- Runtime checks in debug builds
- Health check tooling (cargo_unilang .check)
- Automatic misconfiguration detection

### Key Innovation

**Build Output Visibility** - The game-changing feature:

```bash
$ cargo build
   Compiling unilang v0.30.0

╔══════════════════════════════════════════════════════════╗
║  Unilang: Compile-Time Command Registry                 ║
╟──────────────────────────────────────────────────────────╢
║  Found 2 YAML files                                      ║
║    - greet.commands.yaml                                 ║
║    - auth.commands.yaml                                  ║
║  Generated PHF map with 5 commands                       ║
║  Lookup time: ~80ns (zero runtime overhead)             ║
║                                                          ║
║  ✅ You did NOT need to write build.rs                  ║
║  ✅ YAML parsed at compile time                         ║
║  ✅ Command registry ready                              ║
╚══════════════════════════════════════════════════════════╝

   Compiling my-cli v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s
```

**Why this matters**: Developer SEES unilang worked, knows not to write build.rs

### Expected Outcomes

**Week 1**: Every cargo build shows visible output, README warns users
**Month 1**: cargo_unilang published, developers use scaffolding
**Month 2-3**: Zero GitHub issues about "how to use unilang"

---

## Root Cause Analysis

### Timeline: How The Mistake Happened

**greet_cli developer's experience reconstructed:**

```
Minute 0:   Discovers unilang framework
Minute 2:   Sees dependency: unilang = { features = ["approach_yaml_multi_build"] }
Minute 3:   Thinks: "What does approach_yaml_multi_build mean?"
Minute 5:   Skips documentation (wants to "just code")
Minute 10:  Creates greet_cli.commands.yaml with command definitions
Minute 15:  Thinks: "Now I need to process this YAML at build time"
Minute 20:  Creates build.rs, starts writing YAML parser
Hour 1:     Implements YAML discovery with walkdir
Hour 2:     Implements fake PHF with OnceLock<HashMap>
Hour 3:     Project compiles and "works" (but 50x slower)
Never:      Realizes unilang already provides all of this
```

### Critical Failure Points

**5 points where prevention could have stopped this:**

1. **README structure** - Doesn't start with "DON'T write build.rs"
2. **Build visibility** - No output showing unilang's build.rs ran
3. **Package description** - Doesn't communicate "no build.rs needed"
4. **Scaffolding tool** - No `cargo_unilang .new` to create correct structure
5. **Runtime validation** - No warning that custom build.rs is wrong

### Why Documentation Alone Fails

**Assumption**: "If we write better docs, people will read them"

**Reality**: **Developers don't read docs before coding**

**Evidence from greet_cli**:
- spec.md exists (1,020 lines) - not read
- examples/ directory exists - not studied
- build.rs exists (in unilang crate) - not discovered
- Result: 220 lines of unnecessary code

**Conclusion**: We need **ACTIVE ENFORCEMENT**, not passive documentation

---

## Solution Architecture

### Design Principle: Progressive Prevention

Users encounter prevention at **3 critical moments**:

```
┌────────────────────────────────────────────────────────────┐
│                    User Journey                            │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  BEFORE they code:                                         │
│  ├─ Read README (sees warning)                            │
│  ├─ See package description on crates.io                  │
│  └─ Run scaffolding tool (creates correct structure)      │
│                                                            │
│  WHILE they code:                                          │
│  ├─ cargo build (sees visible output)  ← CRITICAL        │
│  ├─ Compile errors (helpful messages)                     │
│  └─ Examples use correct API                              │
│                                                            │
│  WHEN they run:                                            │
│  ├─ Debug build (runtime checks)                          │
│  ├─ cargo_unilang .check (validation)                     │
│  └─ Tests fail loudly if wrong                            │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

---

## Implementation Plan

### Overview: 3 Phases, 11 Actions, 25.5 Hours

```
Phase 1 (Week 1):   Make Invisible Visible       9.0 hours
Phase 2 (Week 2-3): Tooling Infrastructure      13.5 hours
Phase 3 (Week 4):   Polish & Documentation       3.0 hours
────────────────────────────────────────────────────────────
Total:                                           25.5 hours
```

---

## Phase 1: Make Invisible Visible (Week 1)

**Goal**: Ensure every developer sees prevention BEFORE and WHILE coding

**Total Effort**: 9 hours
**Priority**: P0 (CRITICAL)

### Action 1.1: Transform README.md Structure

**Priority**: P0
**Effort**: 2 hours
**Owner**: Documentation team

Restructure README to lead with warnings:

**New structure**:
1. ⚠️ IMPORTANT: Read This First (what NOT to do)
2. Quick Start (15-line complete example)
3. ⚠️ Common Mistakes (3 anti-patterns with greet_cli reference)
4. Installation (cargo_unilang option first)
5. How It Works
6. Features (approach_ prefix maintained)

**Key addition**: Reference to cargo_unilang throughout

---

### Action 1.2: Add Visible Build Output ⭐

**Priority**: P0 (CRITICAL - KEY INNOVATION)
**Effort**: 2 hours
**Owner**: Core team

Add print function to `/home/user1/pro/lib/wTools/module/core/unilang/build.rs`:

```rust
#[ cfg( feature = "static_registry" ) ]
fn print_build_summary( yaml_files : &[ PathBuf ], command_count : usize )
{
  if yaml_files.is_empty() { return; }

  // Allow suppression
  if std::env::var( "UNILANG_QUIET_BUILD" ).is_ok() { return; }

  eprintln!();
  eprintln!( "╔══════════════════════════════════════════════════════════╗" );
  eprintln!( "║  Unilang: Compile-Time Command Registry                 ║" );
  eprintln!( "╟──────────────────────────────────────────────────────────╢" );
  eprintln!( "║  Found {} YAML file{}", yaml_files.len(),
    if yaml_files.len() == 1 { "" } else { "s" } );

  for file in yaml_files {
    let name = file.file_name().unwrap().to_str().unwrap();
    eprintln!( "║    - {:<50} ║", name );
  }

  eprintln!( "║  Generated PHF map with {} commands", command_count );
  eprintln!( "║  Lookup time: ~80ns (zero runtime overhead)             ║" );
  eprintln!( "║                                                          ║" );
  eprintln!( "║  ✅ You did NOT need to write build.rs                  ║" );
  eprintln!( "║  ✅ YAML parsed at compile time                         ║" );
  eprintln!( "║  ✅ Command registry ready                              ║" );
  eprintln!( "║                                                          ║" );
  eprintln!( "║  Docs: https://docs.rs/unilang                          ║" );
  eprintln!( "╚══════════════════════════════════════════════════════════╝" );
  eprintln!();
}
```

Call from `main()` after generating registry.

---

### Action 1.3: Create Minimal Example

**Priority**: P0
**Effort**: 30 minutes
**Owner**: Examples team

Create `examples/00_minimal.rs` (15 lines) + `00_minimal.commands.yaml`

**Doc comment emphasis**: "What you DON'T need to write"

---

### Action 1.4: Update Package Metadata

**Priority**: P0
**Effort**: 5 minutes
**Owner**: Core team

Update `Cargo.toml`:

```toml
description = "Zero-overhead CLI framework with compile-time YAML processing. No build.rs needed - just add dependency and create YAML files."
keywords = [ "cli", "command", "yaml", "zero-cost", "phf" ]
```

---

### Action 1.5: Clean Deprecated API Usage

**Priority**: P0
**Effort**: 2.5 hours
**Owner**: Core team

Remove ALL `CommandRegistry::new()` from:
- examples/
- readme.md
- src/ doc comments

Replace with `::with_static_commands()`

---

### Action 1.6: Create Quick Start Guide

**Priority**: P0
**Effort**: 3 hours
**Owner**: Documentation team

Create `docs/quick_start.md` with:
- 6 steps (Create → Add Dep → YAML → Code → Build → Run)
- Build step shows visible output
- "What You Did NOT Write" section
- Troubleshooting for each step

---

## Phase 2: Tooling Infrastructure (Week 2-3)

**Goal**: Provide cargo_unilang tool for scaffolding and validation

**Total Effort**: 13.5 hours
**Priority**: P1 (HIGH)

### Action 2.1: Create cargo_unilang Crate Structure

**Priority**: P1
**Effort**: 30 minutes
**Owner**: Tooling team

```bash
cd /home/user1/pro/lib/wTools/module/core/
cargo new --bin cargo_unilang
```

**Directory structure**:
```
cargo_unilang/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, command dispatch
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── new.rs        # Scaffold new project
│   │   └── check.rs      # Health check
│   ├── templates/
│   │   ├── mod.rs
│   │   ├── cargo_toml.rs # Cargo.toml template
│   │   ├── main_rs.rs    # main.rs template
│   │   └── yaml.rs       # .commands.yaml template
│   └── checks/
│       ├── mod.rs
│       ├── build_rs.rs   # Detect custom build.rs
│       ├── deps.rs       # Detect duplicate deps
│       └── api.rs        # Detect deprecated API
├── tests/
│   └── integration_test.rs
└── readme.md
```

**Cargo.toml**:
```toml
[package]
name = "cargo_unilang"
version = "0.1.0"
edition = "2021"
description = "Scaffolding and health check tool for unilang CLI projects"

[dependencies]
serde_yaml = "0.9"

[[bin]]
name = "cargo_unilang"
path = "src/main.rs"
```

---

### Action 2.2: Implement `cargo_unilang .new` Command

**Priority**: P1
**Effort**: 4 hours
**Owner**: Tooling team

**Command**: `cargo_unilang .new project::my-cli`

**CLI Rulebook Compliance**:
- ✅ Dot-prefix command format (`.new`)
- ✅ Parameter format (`project::value`)
- ✅ Uses unilang framework
- ✅ Follows universal CLI standards

**Parameters**:
- `project::` (required) - Project name (must be valid Rust package name)
- `template::` (optional) - Template type: `minimal` (default), `full`
- `author::` (optional) - Author name for Cargo.toml
- `license::` (optional) - License type (MIT, Apache-2.0, etc.)
- `verbosity::` (optional) - Output verbosity level (0-5, default: 2)

**What it creates**:
```
my-cli/
├── Cargo.toml        # unilang = "0.30" + warnings in comments
├── src/main.rs       # 15-line minimal example
└── commands.yaml     # Example command
```

**What it does NOT create**:
- ❌ build.rs

**Output (verbosity::2 - default)**:
```bash
Created project: my-cli/
  ├── Cargo.toml (unilang = "0.30" with warnings)
  ├── src/main.rs (15-line minimal example)
  └── commands.yaml (example command)

✅ You did NOT need to write build.rs
✅ You did NOT need to add serde_yaml, walkdir, or phf

Next steps:
  cd my-cli
  cargo run -- .example name::Alice

⚠️  IMPORTANT:
  - Do NOT create build.rs
  - Do NOT add serde_yaml, walkdir, or phf to Cargo.toml
  - Use CommandRegistry::with_static_commands() (not ::new())
```

**Output (verbosity::1 - single line)**:
```bash
Created my-cli/
```

**Output (verbosity::0 - silent)**:
```bash
(no output, exit code 0 on success)
```

**Output (verbosity::3 - with debug)**:
```bash
[INFO] Creating unilang project: my-cli
[DEBUG] Validating project name...
[DEBUG] Project name 'my-cli' is valid
[DEBUG] Creating directory: my-cli/
[DEBUG] Generating Cargo.toml from template...
[DEBUG] Writing: my-cli/Cargo.toml
[DEBUG] Generating src/main.rs from template...
[DEBUG] Creating directory: my-cli/src/
[DEBUG] Writing: my-cli/src/main.rs
[DEBUG] Generating commands.yaml from template...
[DEBUG] Writing: my-cli/commands.yaml
✅ Created project: my-cli/ (3 files)
```

**Exit Codes**:
- `0`: Project created successfully
- `1`: Failed to create project (I/O error, permissions)
- `2`: Invalid parameters (invalid project name format)
- `3`: Validation failure (project directory already exists)

**Templates**:

`templates/cargo_toml.rs`:
```rust
pub fn cargo_toml( project_name : &str ) -> String
{
  format!(
r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
# Unilang with default features (Approach #2: Multi-YAML Build-Time Static)
unilang = "0.30"

# ⚠️  IMPORTANT: Do NOT add these - unilang already provides them:
# ❌ serde_yaml (via yaml_parser feature)
# ❌ walkdir (via multi_file feature)
# ❌ phf (via static_registry feature)
#
# ⚠️  IMPORTANT: Do NOT create build.rs
# Unilang already provides build.rs that handles everything.
"#,
    project_name = project_name
  )
}
```

**Input Validation** (CLI Security Guidelines Compliance):

Project name validation:
```rust
fn validate_project_name( name : &str ) -> Result< (), String >
{
  // Must be valid Rust package name
  if name.is_empty()
  {
    return Err( "Project name cannot be empty".to_string() );
  }

  // Length: 1-64 characters
  if name.len() > 64
  {
    return Err( "Project name too long (max 64 characters)".to_string() );
  }

  // Security: Prevent path traversal
  if name.contains( ".." ) || name.contains( "/" ) || name.contains( "\\" )
  {
    return Err( "Project name cannot contain path separators".to_string() );
  }

  // Must be valid Rust identifier-like name
  // Allow: alphanumeric, underscore, hyphen
  // Must start with letter or underscore
  let first_char = name.chars().next().unwrap();
  if !first_char.is_ascii_alphabetic() && first_char != '_'
  {
    return Err( "Project name must start with letter or underscore".to_string() );
  }

  for ch in name.chars()
  {
    if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '-'
    {
      return Err( format!( "Invalid character '{}' in project name", ch ) );
    }
  }

  Ok( () )
}
```

Template name validation:
```rust
fn validate_template( template : &str ) -> Result< (), String >
{
  // Whitelist only
  match template
  {
    "minimal" | "full" => Ok( () ),
    _ => Err( format!( "Unknown template '{}'. Valid: minimal, full", template ) )
  }
}
```

Verbosity level validation:
```rust
fn validate_verbosity( level : &str ) -> Result< u8, String >
{
  match level.parse::< u8 >()
  {
    Ok( n ) if n <= 5 => Ok( n ),
    Ok( n ) => Err( format!( "Verbosity must be 0-5, got {}", n ) ),
    Err( _ ) => Err( format!( "Invalid verbosity '{}', must be 0-5", level ) ),
  }
}
```

---

### Action 2.3: Implement `cargo_unilang .check` Command

**Priority**: P1
**Effort**: 6 hours
**Owner**: Tooling team

**Command**: `cargo_unilang .check [path::.] [verbosity::2]`

**CLI Rulebook Compliance**:
- ✅ Dot-prefix command format (`.check`)
- ✅ Parameter format (`path::value`, `verbosity::value`)
- ✅ Uses unilang framework
- ✅ Follows universal CLI standards

**Parameters**:
- `path::` (optional) - Path to project directory (default: `.` current directory)
- `verbosity::` (optional) - Output verbosity level (0-5, default: 2)
- `fix::` (optional) - Auto-fix issues if possible (default: `false`)

**Checks**:
1. Custom build.rs with unilang keywords
2. Duplicate dependencies (serde_yaml, walkdir, phf)
3. Deprecated API usage (CommandRegistry::new())
4. YAML file syntax errors

**Output (verbosity::2 - default, issues found)**:
```
Checking unilang project: .

❌ PROBLEMS DETECTED:

  1. Custom build.rs found
     Location: ./build.rs (220 lines)
     Issue: Duplicates unilang's built-in build system
     Fix: Delete build.rs - unilang provides this automatically
     Docs: https://docs.rs/unilang/.../anti_patterns.html#anti-pattern-1

  2. Duplicate dependencies
     Location: Cargo.toml [build-dependencies]
     Issue: serde_yaml, walkdir already provided by unilang
     Fix: Remove from Cargo.toml
     Docs: https://docs.rs/unilang/.../anti_patterns.html#anti-pattern-3

Summary: 2 issue(s) found
```

**Output (verbosity::2 - default, no issues)**:
```
Checking unilang project: .
✅ All checks passed
```

**Output (verbosity::1 - single line, issues found)**:
```
❌ 2 issue(s) found
```

**Output (verbosity::1 - single line, no issues)**:
```
✅ All checks passed
```

**Output (verbosity::0 - silent)**:
```
(no output, exit code 0 if passed, 1 if issues found)
```

**Output (verbosity::3 - with debug)**:
```
[INFO] Checking unilang project: .
[DEBUG] Reading Cargo.toml...
[DEBUG] Parsing dependencies...
[DEBUG] Checking for build.rs...
[DEBUG] Found build.rs, analyzing content...
[DEBUG] build.rs contains 'serde_yaml' - ISSUE DETECTED
[DEBUG] Checking Cargo.toml for duplicate dependencies...
[DEBUG] Found duplicate: serde_yaml - ISSUE DETECTED
[INFO] Check complete: 2 issue(s) found
```

**Exit Codes**:
- `0`: All checks passed
- `1`: Issues found (custom build.rs, duplicate deps, deprecated API)
- `2`: Invalid parameters (invalid path format)
- `3`: Cannot access path (doesn't exist, no permissions)

**Input Validation** (CLI Security Guidelines Compliance):

Path validation:
```rust
fn validate_path( path : &str ) -> Result< PathBuf, String >
{
  let path_buf = PathBuf::from( path );

  // Security: Canonicalize to prevent path traversal
  let canonical = match path_buf.canonicalize()
  {
    Ok( p ) => p,
    Err( e ) => return Err( format!( "Cannot access path '{}': {}", path, e ) ),
  };

  // Check if path is a directory
  if !canonical.is_dir()
  {
    return Err( format!( "Path '{}' is not a directory", path ) );
  }

  // Check read permissions
  match std::fs::read_dir( &canonical )
  {
    Ok( _ ) => {},
    Err( e ) => return Err( format!( "Cannot read directory '{}': {}", path, e ) ),
  }

  Ok( canonical )
}
```

**Implementation**: 3 check modules (build_rs, deps, api)

---

### Action 2.3.1: Implement Help System (Universal CLI Requirement)

**Priority**: P1
**Effort**: 2 hours
**Owner**: Tooling team

**CLI Rulebook Compliance**:
- ✅ Universal help access via `.` and `.help`
- ✅ Command-specific help via `.command.help`
- ✅ Help commands filtered from listings
- ✅ Consistent help format

**Required Help Commands**:

1. `cargo_unilang .` - General help
2. `cargo_unilang .help` - General help (same as `.`)
3. `cargo_unilang .new.help` - Help for .new command
4. `cargo_unilang .check.help` - Help for .check command

**General Help Output** (`cargo_unilang .` or `cargo_unilang .help`):
```
cargo_unilang - Scaffolding and health check tool for unilang projects

Prevents common mistakes when using unilang CLI framework:
  ❌ Custom build.rs (unilang provides this automatically)
  ❌ Duplicate dependencies (serde_yaml, walkdir, phf)
  ❌ Deprecated API (CommandRegistry::new())

USAGE:
  cargo_unilang .                                # Show this help
  cargo_unilang .help                            # Show this help
  cargo_unilang .new project::<name> [OPTIONS]   # Create new project
  cargo_unilang .check [path::<dir>] [OPTIONS]   # Check existing project

COMMANDS:
  .new       Create new unilang project with correct structure
  .check     Validate existing project for common mistakes

For command-specific help:
  cargo_unilang .new.help
  cargo_unilang .check.help

EXAMPLES:
  # Create minimal project
  cargo_unilang .new project::my-cli

  # Create full-featured project
  cargo_unilang .new project::my-api template::full verbosity::1

  # Check current directory
  cargo_unilang .check

  # Check specific project with debug output
  cargo_unilang .check path::./my-project verbosity::3

DOCUMENTATION:
  https://docs.rs/unilang/
  https://github.com/Wandalen/wTools/tree/master/module/core/unilang

EXIT CODES:
  0  Success
  1  General error (issues found, I/O error)
  2  Invalid parameters
  3  Validation failure (path doesn't exist, project exists)
```

**Command-Specific Help** (`cargo_unilang .new.help`):
```
cargo_unilang .new - Create new unilang project

Creates correct unilang project structure, preventing common mistakes.

USAGE:
  cargo_unilang .new project::<name> [OPTIONS]

PARAMETERS:
  project::<name>      Project name (required)
                       Must be valid Rust package name
                       1-64 characters, alphanumeric/underscore/hyphen
                       Must start with letter or underscore

  template::<type>     Template type (optional, default: minimal)
                       Options: minimal, full

  author::<name>       Author name for Cargo.toml (optional)
                       Format: "Name <email@example.com>"

  license::<type>      License type (optional, default: MIT)
                       Common: MIT, Apache-2.0, GPL-3.0

  verbosity::<level>   Output verbosity (optional, default: 2)
                       0 = silent (exit code only)
                       1 = single line
                       2 = concise (default)
                       3 = with debug info
                       4-5 = maximum debug

WHAT IT CREATES:
  my-cli/
  ├── Cargo.toml       # unilang = "0.30" with warnings
  ├── src/main.rs      # 15-line minimal example
  └── commands.yaml    # Example command

WHAT IT DOES NOT CREATE:
  ❌ build.rs          # unilang provides this automatically

EXAMPLES:
  # Minimal project (default)
  cargo_unilang .new project::my-cli

  # Full-featured project
  cargo_unilang .new project::my-api template::full

  # With all options
  cargo_unilang .new project::my-tool template::full author::"John Doe <john@example.com>" license::Apache-2.0 verbosity::1

EXIT CODES:
  0  Project created successfully
  1  Failed to create (I/O error, permissions)
  2  Invalid parameters (invalid project name)
  3  Project directory already exists
```

**Command-Specific Help** (`cargo_unilang .check.help`):
```
cargo_unilang .check - Validate unilang project

Checks for common mistakes and anti-patterns in unilang projects.

USAGE:
  cargo_unilang .check [path::<dir>] [OPTIONS]

PARAMETERS:
  path::<dir>          Path to project directory (optional, default: .)
                       Must be existing directory

  verbosity::<level>   Output verbosity (optional, default: 2)
                       0 = silent (exit code only)
                       1 = single line summary
                       2 = concise (default)
                       3 = with debug info
                       4-5 = maximum debug

  fix::<bool>          Auto-fix issues if possible (optional, default: false)
                       WARNING: Will modify files

CHECKS PERFORMED:
  1. Custom build.rs with unilang keywords
     Issue: Duplicates unilang's built-in build system
     Fix: Delete build.rs

  2. Duplicate dependencies in Cargo.toml
     Issue: serde_yaml, walkdir, phf already in unilang
     Fix: Remove from [dependencies] or [build-dependencies]

  3. Deprecated API usage
     Issue: CommandRegistry::new() is deprecated
     Fix: Use CommandRegistry::with_static_commands()

  4. YAML syntax errors
     Issue: Invalid YAML in *.commands.yaml files
     Fix: Correct YAML syntax

EXAMPLES:
  # Check current directory
  cargo_unilang .check

  # Check specific project
  cargo_unilang .check path::./my-project

  # Silent check (CI mode)
  cargo_unilang .check verbosity::0

  # Verbose check with debug
  cargo_unilang .check verbosity::3

EXIT CODES:
  0  All checks passed
  1  Issues found
  2  Invalid parameters
  3  Cannot access path
```

**Implementation Notes**:
- Help commands must use unilang's own YAML definitions
- Create `.commands.yaml` with `.`, `.help`, `.new.help`, `.check.help`
- Filter `.*.help` from command listings (never show in `.` output)
- Consistent formatting across all help text

---

### Action 2.4: Add Integration Tests

**Priority**: P1
**Effort**: 3 hours
**Owner**: Tooling team

Tests using assert_cmd, assert_fs, predicates:

**Command Format Tests**:
- `test_new_command_with_dot_prefix` - Verify `.new` works
- `test_new_command_with_param_value_format` - Verify `project::name` works
- `test_check_command_with_dot_prefix` - Verify `.check` works
- `test_invalid_command_without_dot_prefix` - Verify `new` fails with error

**Help System Tests**:
- `test_help_via_dot` - Verify `.` shows general help
- `test_help_via_dot_help` - Verify `.help` shows same help as `.`
- `test_new_help` - Verify `.new.help` shows command help
- `test_check_help` - Verify `.check.help` shows command help
- `test_help_commands_not_listed` - Verify `.*.help` not in command list

**Verbosity Tests**:
- `test_new_verbosity_0_silent` - Verify no output, only exit code
- `test_new_verbosity_1_single_line` - Verify single line output
- `test_new_verbosity_2_default` - Verify default concise output
- `test_new_verbosity_3_debug` - Verify debug output present
- `test_check_verbosity_levels` - Verify all levels for check command

**Exit Code Tests**:
- `test_new_exit_0_on_success` - Verify exit 0 when created
- `test_new_exit_2_invalid_params` - Verify exit 2 for invalid project name
- `test_new_exit_3_already_exists` - Verify exit 3 when dir exists
- `test_check_exit_0_no_issues` - Verify exit 0 when all checks pass
- `test_check_exit_1_issues_found` - Verify exit 1 when issues found
- `test_check_exit_3_invalid_path` - Verify exit 3 for nonexistent path

**Input Validation Tests**:
- `test_new_validates_project_name` - Reject invalid chars, path traversal
- `test_new_validates_template_name` - Reject unknown templates
- `test_new_validates_verbosity` - Reject verbosity > 5
- `test_check_validates_path` - Reject nonexistent paths, check permissions

**Functional Tests**:
- `test_new_creates_correct_structure` - Verify files created
- `test_new_does_not_create_build_rs` - Verify NO build.rs
- `test_new_includes_warnings_in_cargo_toml` - Verify warnings present
- `test_check_detects_custom_build_rs` - Detect custom build.rs
- `test_check_detects_duplicate_dependencies` - Detect serde_yaml, etc.
- `test_check_detects_deprecated_api` - Detect CommandRegistry::new()
- `test_check_passes_on_clean_project` - No issues on correct project

**Example Integration Test**:
```rust
#[test]
fn test_new_command_with_compliant_format()
{
  let temp = assert_fs::TempDir::new().unwrap();

  Command::cargo_bin( "cargo_unilang" ).unwrap()
    .arg( ".new" )  // Dot-prefix format
    .arg( "project::my-cli" )  // param::value format
    .arg( "verbosity::1" )
    .current_dir( &temp )
    .assert()
    .success()
    .code( 0 )
    .stdout( predicate::str::contains( "Created my-cli/" ) );

  // Verify structure
  temp.child( "my-cli/Cargo.toml" ).assert( predicate::path::exists() );
  temp.child( "my-cli/src/main.rs" ).assert( predicate::path::exists() );
  temp.child( "my-cli/commands.yaml" ).assert( predicate::path::exists() );

  // Verify NO build.rs
  temp.child( "my-cli/build.rs" ).assert( predicate::path::missing() );
}

#[test]
fn test_help_system_compliance()
{
  // Test `.` shows help
  Command::cargo_bin( "cargo_unilang" ).unwrap()
    .arg( "." )
    .assert()
    .success()
    .stdout( predicate::str::contains( "cargo_unilang - Scaffolding and health check tool" ) );

  // Test `.help` shows same help
  let help_output = Command::cargo_bin( "cargo_unilang" ).unwrap()
    .arg( ".help" )
    .output()
    .unwrap();

  let dot_output = Command::cargo_bin( "cargo_unilang" ).unwrap()
    .arg( "." )
    .output()
    .unwrap();

  assert_eq!( help_output.stdout, dot_output.stdout );
}

#[test]
fn test_exit_codes()
{
  let temp = assert_fs::TempDir::new().unwrap();

  // Success: exit 0
  Command::cargo_bin( "cargo_unilang" ).unwrap()
    .arg( ".new" )
    .arg( "project::test-project" )
    .current_dir( &temp )
    .assert()
    .code( 0 );

  // Already exists: exit 3
  Command::cargo_bin( "cargo_unilang" ).unwrap()
    .arg( ".new" )
    .arg( "project::test-project" )
    .current_dir( &temp )
    .assert()
    .code( 3 );

  // Invalid name: exit 2
  Command::cargo_bin( "cargo_unilang" ).unwrap()
    .arg( ".new" )
    .arg( "project::../invalid" )
    .current_dir( &temp )
    .assert()
    .code( 2 );
}
```

---

### Action 2.5: Create cargo_unilang Documentation

**Priority**: P1
**Effort**: 1 hour
**Owner**: Documentation team

Create comprehensive `readme.md` for cargo_unilang with:
- Why tool exists
- Installation
- Command documentation
- Common workflows
- Real-world example (greet_cli)

---

### Action 2.6: Publish cargo_unilang to crates.io

**Priority**: P1
**Effort**: 30 minutes
**Owner**: Core team

```bash
cd /home/user1/pro/lib/wTools/module/core/cargo_unilang
cargo publish
```

---

## Phase 3: Polish & Documentation (Week 4)

**Goal**: Complete documentation and add runtime validation

**Total Effort**: 3 hours
**Priority**: P1

### Action 3.1: Add Runtime Misconfiguration Detection

**Priority**: P1
**Effort**: 1 hour
**Owner**: Core team

Add to `src/registry.rs`:

```rust
impl CommandRegistry
{
  pub fn with_static_commands() -> Self
  {
    #[ cfg( debug_assertions ) ]
    {
      Self::check_common_mistakes();
    }

    // ... normal implementation ...
  }

  #[ cfg( debug_assertions ) ]
  fn check_common_mistakes()
  {
    use std::{ fs, path::Path, env };

    if env::var( "UNILANG_IGNORE_CHECKS" ).is_ok() { return; }

    if Path::new( "build.rs" ).exists() {
      if let Ok( content ) = fs::read_to_string( "build.rs" ) {
        if content.contains( "serde_yaml" )
          || content.contains( "phf_codegen" )
          || content.contains( "commands.yaml" )
        {
          eprintln!();
          eprintln!( "╔══════════════════════════════════════════════════════════╗" );
          eprintln!( "║  ⚠️  WARNING: Custom build.rs Detected                   ║" );
          eprintln!( "╟──────────────────────────────────────────────────────────╢" );
          eprintln!( "║  You have a custom build.rs that processes YAML.        ║" );
          eprintln!( "║  Unilang ALREADY provides this!                         ║" );
          eprintln!( "║                                                          ║" );
          eprintln!( "║  FIX: Delete build.rs                                    ║" );
          eprintln!( "║       Run: cargo_unilang .check                          ║" );
          eprintln!( "║                                                          ║" );
          eprintln!( "║  To suppress: UNILANG_IGNORE_CHECKS=1                   ║" );
          eprintln!( "╚══════════════════════════════════════════════════════════╝" );
          eprintln!();
        }
      }
    }
  }
}
```

**Debug-only**: Zero overhead in release builds

---

### Action 3.2: Update Module-Level Documentation

**Priority**: P1
**Effort**: 1 hour
**Owner**: Documentation team

Update `src/lib.rs` with:
- Minimal example upfront
- "What you did NOT write" section
- Features with approach_ prefix maintained
- Links to examples, quick start, cargo_unilang

---

### Action 3.3: Link Minimal Example from All Entry Points

**Priority**: P1
**Effort**: 30 minutes
**Owner**: Documentation team

Add links in:
- spec.md (top)
- docs/index.md (new file)
- readme.md (already in Action 1.1)

---

### Action 3.4: Document Feature Flags (Keep approach_ Prefix)

**Priority**: P1
**Effort**: 30 minutes
**Owner**: Documentation team

Create `docs/features.md` explaining:
- Why approach_ prefix exists
- Default features (use unilang = "0.30")
- When to specify features explicitly (almost never)
- Feature dependencies

**Key message**: "Use defaults, don't specify features"

---

## Success Metrics

### Week 1 (Immediate)

- [ ] README starts with "⚠️ DON'T DO THIS" warning
- [ ] Build output shows what unilang did during cargo build
- [ ] Minimal example in examples/00_minimal.rs
- [ ] Zero deprecated API in examples/readme/docs
- [ ] Package description says "no build.rs needed"

### Week 2-3 (Short-term)

- [ ] cargo_unilang published to crates.io
- [ ] `cargo_unilang .new project::name` creates correct project structure
- [ ] `cargo_unilang .check` detects all 3 issue types
- [ ] Integration tests pass
- [ ] Documentation complete

### Week 4 (Medium-term)

- [ ] Quick Start guide complete and tested
- [ ] Runtime validation in debug builds
- [ ] Compile errors guide to docs
- [ ] Feature documentation clarifies approach_ prefix

### Month 2-3 (Long-term)

- [ ] Zero GitHub issues: "how do I use unilang"
- [ ] Zero PRs with custom build.rs
- [ ] cargo_unilang has >100 downloads
- [ ] Ecosystem projects use correct patterns

---

## Risk Mitigation

### Risk 1: Build Output Too Noisy

**Mitigation**:
- Only show if YAML files found
- Provide UNILANG_QUIET_BUILD env var
- Keep output <10 lines

### Risk 2: cargo_unilang Low Adoption

**Mitigation**:
- Feature prominently in README
- Make Option 1 in installation
- Document in Quick Start

### Risk 3: Documentation Overload

**Mitigation**:
- Progressive disclosure
- Visual warnings (⚠️)
- Repetition of critical points

### Risk 4: Naming Confusion (approach_ prefix)

**Mitigation**:
- Document why prefix exists
- Emphasize "use defaults"
- Provide features.md

---

## Maintenance Plan

### Weekly

- Monitor GitHub issues for misuse patterns
- Update build output if features added
- Review PRs for correct patterns

### Monthly

- Analyze cargo_unilang usage
- Update anti-patterns based on discoveries
- Review docs.rs analytics

### Quarterly

- Survey users about build output
- Measure cargo_unilang adoption
- Update Quick Start based on feedback

---

## Conclusion

This comprehensive plan prevents unilang misuse through **3-layer progressive prevention**:

1. **BEFORE coding**: README warnings, package description, scaffolding tool (cargo_unilang)
2. **WHILE coding**: **Build output visibility** (key innovation), compile errors, clean examples
3. **WHEN running**: Runtime checks, health check tooling (cargo_unilang .check)

**Total effort**: 25.5 hours over 4 weeks

**Key innovation**: **Visible build output** showing what unilang did during cargo build

**Critical naming**: **Maintains approach_ prefix** for all approach features per convention

**Tooling**: **cargo_unilang** (underscore) for consistency with Rust naming conventions

**Expected outcome**: Zero greet_cli-style mistakes within 2 months

**Implementation ready**: All 11 actions detailed with clear steps, acceptance criteria, and verification procedures

**Next step**: Begin Phase 1, Action 1.1 (Transform README structure)
