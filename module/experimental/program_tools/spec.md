# Specification: program_tools

## Overview

**program_tools** is a dynamic Rust program compilation and execution utility providing structures and builders for defining, compiling, and running Rust programs at runtime. It enables use cases such as code generation testing, dynamic code evaluation, temporary program execution, and build system integration where Rust code needs to be compiled and executed programmatically.

**Version:** 0.1.0
**Status:** Experimental (Work-in-Progress)
**Category:** Development Tools (Program Execution)
**Dependents:** Unknown (likely workspace build tools and test utilities)

### Scope

#### Responsibility

Provide data structures and builders (using Former pattern) for defining Rust program source files, organizing them into programs, and creating execution plans, enabling dynamic compilation and running of Rust code through a structured configuration API.

#### In-Scope

1. **Source Configuration (Source)**
   - `Source` struct - Individual source file definition
   - `file_path` - File path for the source (e.g., "main.rs")
   - `data` - Source code content as String
   - Former derive for builder pattern
   - Multiple sources per program

2. **Program Configuration (Program)**
   - `Program` struct - Collection of source files
   - `source: Vec<Source>` - Multiple source files
   - Subform entry via `#[subform_entry]`
   - Former derive for builder pattern
   - Represents a compilable Rust crate

3. **Execution Plan (Plan)**
   - `Plan` struct - Complete execution configuration
   - `program: Program` - The program to compile/run
   - Subform scalar via `#[subform_scalar]`
   - Former derive for builder pattern
   - Top-level configuration entry point

4. **Builder Pattern (Former Integration)**
   - Fluent builder API
   - `Plan::former()` entry point
   - `.program()` subform access
   - `.source()` subform entry
   - `.end()` completion

5. **mod_interface Integration**
   - Exposed namespace with Source, Program, Plan
   - Prelude namespace with same exports
   - Traditional namespace organization

6. **Feature Architecture**
   - `enabled` - Master switch (default)
   - Dependencies gated on enabled
   - Minimal feature set

#### Out-of-Scope

1. **NOT Compilation Execution (Currently)**
   - No cargo invocation
   - No rustc calls
   - **Rationale:** Work-in-progress, structures first

2. **NOT Process Management**
   - No process spawning
   - No stdout/stderr capture
   - **Rationale:** Use process_tools for execution

3. **NOT Crate Manifest Generation**
   - No Cargo.toml generation
   - No dependency management
   - **Rationale:** Future enhancement

4. **NOT Incremental Compilation**
   - No build caching
   - No artifact reuse
   - **Rationale:** Complexity, out of initial scope

5. **NOT Cross-Compilation**
   - No target specification
   - No toolchain selection
   - **Rationale:** Future enhancement

6. **NOT IDE Integration**
   - No LSP support
   - No diagnostics formatting
   - **Rationale:** Different use case

7. **NOT Async Execution**
   - Synchronous API
   - No async compilation
   - **Rationale:** Simplicity, use async wrappers if needed

8. **NOT Security Sandboxing**
   - No execution isolation
   - No resource limits
   - **Rationale:** Complex, platform-specific

#### Boundaries

- **program_tools vs process_tools**: program_tools for program definition; process_tools for execution
- **program_tools vs cargo**: program_tools wraps cargo; cargo does actual compilation
- **program_tools vs build.rs**: program_tools for runtime; build.rs for build-time

## Architecture

### Dependency Structure

```
program_tools
├── Internal Dependencies (workspace)
│   ├── mod_interface (module organization)
│   ├── former (builder pattern, derive_former)
│   ├── pth (path utilities)
│   ├── error_tools (error handling, error_untyped)
│   └── iter_tools (iterator utilities)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** All dependencies workspace-internal

### Module Organization

```
program_tools
├── lib.rs (top-level, mod_interface)
└── program.rs - Core structures
    ├── Source - Individual source file
    ├── Program - Collection of sources
    └── Plan - Execution configuration

mod_interface layout:
  layer program (program module)
  exposed: Source, Program, Plan
  prelude: Source, Program, Plan
```

### Feature Architecture

```
enabled (master switch, default)
├── mod_interface/enabled
├── former/enabled
├── pth/enabled
├── error_tools/enabled
└── iter_tools/enabled

full = enabled (all features)
```

**Default Features:** `enabled`

### Data Model

```
Plan (execution configuration)
└── program: Program (compilable unit)
    └── source: Vec<Source> (source files)
        ├── file_path: String
        └── data: String

Builder Flow:
Plan::former()
  → .program() → ProgramFormer
    → .source() → SourceFormer
      → .file_path("main.rs")
      → .data("fn main() {}")
      → .end() → ProgramFormer
    → .end() → PlanFormer
  → .end() → Plan
```

## Public API

### Source Struct

```rust
/// Source configuration for a program.
///
/// Represents a single source file with its path and content.
#[derive(Debug, Former)]
pub struct Source {
  /// File path relative to crate root (e.g., "src/main.rs")
  pub file_path: String,
  /// Source code content
  pub data: String,
}
```

### Program Struct

```rust
/// Program configuration.
///
/// Represents a Rust program as a collection of source files.
#[derive(Debug, Former)]
pub struct Program {
  /// Collection of source files
  #[subform_entry]
  pub source: Vec<Source>,
}
```

### Plan Struct

```rust
/// Plan for compiling and running a Rust program.
///
/// Top-level configuration containing the program to execute.
#[derive(Debug, Former)]
pub struct Plan {
  /// Program configuration
  #[subform_scalar]
  pub program: Program,
}
```

### Builder API (via Former)

```rust
// Entry point
impl Plan {
  pub fn former() -> PlanFormer { /* ... */ }
}

// PlanFormer methods
impl PlanFormer {
  /// Access program subform
  pub fn program(self) -> ProgramFormer { /* ... */ }

  /// Complete and build Plan
  pub fn end(self) -> Plan { /* ... */ }
}

// ProgramFormer methods
impl ProgramFormer {
  /// Add a source file entry
  pub fn source(self) -> SourceFormer { /* ... */ }

  /// Complete and return to parent
  pub fn end(self) -> PlanFormer { /* ... */ }
}

// SourceFormer methods
impl SourceFormer {
  /// Set file path
  pub fn file_path(self, path: impl Into<String>) -> Self { /* ... */ }

  /// Set source data
  pub fn data(self, content: impl Into<String>) -> Self { /* ... */ }

  /// Complete and return to parent
  pub fn end(self) -> ProgramFormer { /* ... */ }
}
```

## Usage Patterns

### Pattern 1: Basic Single-File Program

```rust
use program_tools::program;

let plan = program::Plan::former()
  .program()
    .source()
      .file_path("main.rs")
      .data("fn main() { println!(\"Hello, world!\"); }")
      .end()
    .end()
  .end();

println!("{:?}", plan);
```

### Pattern 2: Multi-File Program

```rust
use program_tools::program;

let plan = program::Plan::former()
  .program()
    .source()
      .file_path("main.rs")
      .data(r#"
        mod lib;
        fn main() { lib::greet(); }
      "#)
      .end()
    .source()
      .file_path("lib.rs")
      .data(r#"
        pub fn greet() { println!("Hello from lib!"); }
      "#)
      .end()
    .end()
  .end();
```

### Pattern 3: Test Case Generation

```rust
use program_tools::program;

fn create_test_program(test_code: &str) -> program::Plan {
  program::Plan::former()
    .program()
      .source()
        .file_path("main.rs")
        .data(format!(
          r#"
          fn main() {{
            {}
            println!("Test passed!");
          }}
          "#,
          test_code
        ))
        .end()
      .end()
    .end()
}

let plan = create_test_program("assert_eq!(2 + 2, 4);");
```

### Pattern 4: Dynamic Code Evaluation Setup

```rust
use program_tools::program;

fn setup_evaluation(expression: &str) -> program::Plan {
  program::Plan::former()
    .program()
      .source()
        .file_path("main.rs")
        .data(format!(
          r#"
          fn main() {{
            let result = {};
            println!("{{:?}}", result);
          }}
          "#,
          expression
        ))
        .end()
      .end()
    .end()
}

let plan = setup_evaluation("vec![1, 2, 3].iter().sum::<i32>()");
```

### Pattern 5: With External Crate (Future)

```rust
use program_tools::program;

// Note: Cargo.toml generation not yet implemented
let plan = program::Plan::former()
  .program()
    .source()
      .file_path("main.rs")
      .data(r#"
        use serde_json::Value;
        fn main() {
          let v: Value = serde_json::from_str("{}").unwrap();
          println!("{:?}", v);
        }
      "#)
      .end()
    // Future: .manifest() for Cargo.toml
    .end()
  .end();
```

### Pattern 6: Build Script Testing

```rust
use program_tools::program;

fn create_build_test(build_rs_code: &str, main_code: &str) -> program::Plan {
  program::Plan::former()
    .program()
      .source()
        .file_path("build.rs")
        .data(build_rs_code)
        .end()
      .source()
        .file_path("src/main.rs")
        .data(main_code)
        .end()
      .end()
    .end()
}
```

### Pattern 7: Minimal Program Definition

```rust
use program_tools::prelude::*;

// Using prelude for convenient access
let source = Source {
  file_path: "main.rs".to_string(),
  data: "fn main() {}".to_string(),
};

let program = Program {
  source: vec![source],
};

let plan = Plan {
  program,
};
```

## Dependencies and Consumers

### Direct Dependencies

**Workspace:**
- `mod_interface` (enabled) - Module organization
- `former` (enabled, derive_former) - Builder pattern
- `pth` (enabled) - Path utilities
- `error_tools` (enabled, error_untyped) - Error handling
- `iter_tools` (enabled) - Iterator utilities

**Dev:**
- `test_tools` (workspace, full) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- Test frameworks (compile and run test programs)
- Code generation tools (test generated code)
- Documentation systems (run examples)
- Build tools (dynamic compilation)
- Evaluation systems (code snippets)
- CI/CD pipelines (temporary program execution)

**Usage Pattern:** Workspace tools use program_tools to define and prepare Rust programs for compilation and execution during testing or code generation workflows.

## Design Rationale

### Why Former Pattern?

Builder API via derive:

**Rationale:**
1. **Ergonomics**: Fluent, readable API
2. **Nested Structures**: Subform for program/source
3. **Type Safety**: Compile-time validation
4. **Consistency**: Matches workspace patterns
5. **Extensibility**: Easy to add fields

**Pattern:** Standard workspace builder approach

### Why Separate Source/Program/Plan?

Three-level hierarchy:

**Rationale:**
1. **Granularity**: Source is atomic unit
2. **Composition**: Program groups sources
3. **Execution**: Plan adds execution config
4. **Future Extension**: Each level can grow
5. **Clarity**: Clear responsibility separation

**Model:** Mirrors real Rust project structure

### Why String for Data?

Source code as String:

**Rationale:**
1. **Simplicity**: Easy to construct
2. **Flexibility**: Any source content
3. **Generation**: Easy to format!/concat
4. **Inspection**: Can debug print
5. **Ownership**: Clear ownership model

**Alternative:** Could use Cow<str> for efficiency

### Why mod_interface?

Uses mod_interface for organization:

**Rationale:**
1. **Consistency**: Workspace pattern
2. **Namespaces**: exposed/prelude structure
3. **Clean Exports**: Single declaration
4. **Maintainability**: Centralized exports

**Pattern:** Standard workspace module organization

### Why Current Minimal Implementation?

Only structures, no execution:

**Rationale:**
1. **Incremental**: Build structures first
2. **Validation**: Test data model
3. **Flexibility**: Execution strategy TBD
4. **Dependencies**: Reduce initial deps
5. **WIP**: Marked experimental

**Future:** Will add compilation and execution

### Why Dependencies like pth, error_tools?

Included but not heavily used yet:

**Rationale:**
1. **Preparation**: Will need for execution
2. **Path Handling**: pth for temp directories
3. **Error Handling**: error_tools for results
4. **Iteration**: iter_tools for source processing

**Status:** Infrastructure for future features

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for testing
- Former integration tests
- Structure validation

### Test Focus

1. **Builder API**: Plan/Program/Source construction
2. **Nested Building**: Subform entry/scalar
3. **Multiple Sources**: Programs with many files
4. **Data Validation**: Source content handling
5. **Edge Cases**: Empty programs, long content
6. **Former Integration**: All builder methods work

### Test Example

```rust
#[test]
fn basic() {
  use program_tools::program;

  let _plan = program::Plan::former()
    .program()
      .source()
        .file_path("main.rs")
        .data("fn main() { println!(\"hello!\") }")
        .end()
      .end()
    .end();
}
```

### Known Test Limitations

1. **No Execution Testing**: Can't test compilation yet
2. **No Integration Tests**: Need cargo for real tests
3. **Platform Testing**: Execution platform-dependent
4. **Timeout Testing**: Long compilations need timeouts

## Future Considerations

### Potential Enhancements

1. **Compilation Execution**: Actually run cargo/rustc
2. **Cargo.toml Generation**: Dependencies, edition, etc.
3. **Temp Directory Management**: Auto-cleanup
4. **Output Capture**: stdout/stderr handling
5. **Error Reporting**: Compilation error parsing
6. **Incremental Builds**: Caching for speed
7. **Cross-Compilation**: Target specification
8. **Workspace Support**: Multi-crate programs
9. **Async API**: Non-blocking compilation
10. **Sandbox Mode**: Isolated execution

### Planned Features

Based on dependencies and structure:

```rust
// Future API concept
let result = program::Plan::former()
  .program()
    .source().file_path("main.rs").data(code).end()
    .end()
  .working_dir("/tmp/my_program")
  .timeout(Duration::from_secs(60))
  .end()
  .compile()? // Uses cargo
  .run()?;    // Executes binary

println!("stdout: {}", result.stdout);
println!("stderr: {}", result.stderr);
println!("exit_code: {}", result.exit_code);
```

### Breaking Changes to Consider

1. **Source Data Type**: Change from String
2. **Plan Structure**: Add more configuration
3. **API Shape**: Execution methods
4. **Error Types**: Structured errors
5. **Result Types**: Execution results

### Known Limitations

1. **No Execution**: Currently just data structures
2. **No Manifest**: No Cargo.toml support
3. **No Dependencies**: Can't specify crate deps
4. **No Edition**: Rust edition not configurable
5. **No Features**: Cargo features not supported
6. **No Workspaces**: Single-crate only
7. **No Release Mode**: Debug only (assumed)

## Adoption Guidelines

### When to Use program_tools

**Good Candidates:**
- Testing code generators
- Running example code from docs
- Compile-time verification tools
- Build system integration
- Dynamic code evaluation
- Test harness construction

**Poor Candidates:**
- Production compilation (use cargo directly)
- IDE integration (use rust-analyzer)
- Long-running processes (use proper orchestration)
- Security-sensitive execution (no sandboxing)

### Current Usage (Structures Only)

```rust
use program_tools::program;

// Define a program
let plan = program::Plan::former()
  .program()
    .source()
      .file_path("main.rs")
      .data("fn main() {}")
      .end()
    .end()
  .end();

// For now, manual handling required for execution
// Future: plan.compile().run()
```

### Best Practices

1. **Use Builder**: Prefer Former over direct construction
2. **Validate Paths**: Ensure file_path is valid
3. **Check Syntax**: Validate Rust code before setting
4. **Handle Errors**: Prepare for compilation failures
5. **Clean Up**: Remove temp files after execution
6. **Timeout**: Set reasonable compilation timeouts
7. **Log Output**: Capture stdout/stderr

### Integration Example

```rust
// Integration with process_tools (conceptual)
use program_tools::program;
use process_tools::process; // Assuming this exists

fn run_program(code: &str) -> Result<String, Error> {
  let plan = program::Plan::former()
    .program()
      .source().file_path("main.rs").data(code).end()
      .end()
    .end();

  // Write sources to temp dir
  let temp_dir = create_temp_project(&plan)?;

  // Compile with cargo
  let compile = process::Command::new("cargo")
    .arg("build")
    .current_dir(&temp_dir)
    .run()?;

  if !compile.success() {
    return Err(compile.stderr);
  }

  // Run the binary
  let run = process::Command::new("./target/debug/program")
    .current_dir(&temp_dir)
    .run()?;

  Ok(run.stdout)
}
```

## Related Crates

**Dependencies:**
- **mod_interface**: Module organization (workspace)
- **former**: Builder pattern (workspace)
- **pth**: Path utilities (workspace)
- **error_tools**: Error handling (workspace)
- **iter_tools**: Iterator utilities (workspace)

**Related:**
- **process_tools**: Process execution (workspace)
- **crates_tools**: Crate management (workspace)
- **workspace_tools**: Workspace utilities (workspace)

**Alternatives:**
- **cargo**: Direct cargo invocation
- **rustc**: Direct compiler invocation
- **duct**: Process orchestration (external)

## References

- [API Documentation](https://docs.rs/program_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/program_tools)
- [Former Documentation](https://docs.rs/former)
- [readme.md](./readme.md)
- [process_tools](../process_tools/readme.md) - Process execution
