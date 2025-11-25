# Specification: process_tools

## Overview

**process_tools** is a foundational crate providing robust subprocess execution and CI/CD environment detection. It wraps external process execution with ergonomic builder patterns, comprehensive output capture, and cross-platform shell abstraction, serving as the subprocess foundation for workspace tools like willbe.

**Version:** 0.24.0
**Status:** Production
**Category:** Infrastructure (Subprocess Management)
**Dependents:** ~1 workspace crate (willbe or similar automation tools)

### Scope

#### Responsibility

Provide reliable, type-safe subprocess execution with comprehensive output capture, environment variable management, and CI/CD environment detection for automation tools and build systems.

#### In-Scope

1. **Process Execution**
   - `run()` - Direct binary execution without shell
   - `run_with_shell()` - Cross-platform shell command execution
   - Builder pattern via `Run::former()` for fluent configuration
   - Synchronous process execution with wait semantics

2. **Output Capture**
   - `Report` struct capturing all execution details
   - Separate stdout and stderr streams
   - Optional stream joining (stderr → stdout)
   - Command and working directory tracking
   - UTF-8 output validation and error reporting

3. **Configuration Options**
   - Working directory (`current_path`)
   - Binary path (`bin_path`)
   - Command-line arguments (`args`)
   - Environment variables (`env_variable`)
   - Stream joining mode (`joining_streams`)

4. **Error Handling**
   - Untyped error handling via `error_tools`
   - Exit code validation
   - UTF-8 decoding errors
   - Process spawn failures
   - Detailed error context in Report

5. **Cross-Platform Shell Abstraction**
   - Windows: `cmd /C` for shell commands
   - Unix: `sh -c` for shell commands
   - Automatic platform detection via `cfg!(target_os)`

6. **CI/CD Environment Detection**
   - `is_cicd()` function (feature-gated)
   - Detection of: GitHub Actions, GitLab CI, Travis CI, CircleCI, Jenkins
   - Common CI variable detection (`CI`, `GITHUB_ACTIONS`, etc.)

7. **Integration with duct**
   - Uses duct crate for advanced process execution
   - Stream redirection (stderr_to_stdout)
   - Unchecked execution for custom error handling

8. **Builder Pattern via former**
   - `Run::former()` for fluent API
   - Type-safe configuration
   - Default values for optional fields
   - `.run()` method on builder for immediate execution

9. **Report Display**
   - Formatted output via `Display` trait
   - Command representation
   - Working directory display
   - Indented stdout/stderr output
   - Trimmed whitespace handling

10. **Module Organization**
    - Uses `mod_interface!` pattern
    - Two layers: `process` (execution), `environment` (CI/CD detection)
    - Feature-gated exports

#### Out-of-Scope

1. **NOT Asynchronous Execution**
   - Does not provide async/await APIs
   - No tokio or async-std integration
   - **Rationale:** Synchronous execution sufficient for build tools and automation

2. **NOT Interactive Process Control**
   - Does not support interactive stdin
   - No PTY/terminal emulation
   - No real-time stream reading
   - **Rationale:** Focused on batch execution, not interactive shells

3. **NOT Process Lifetime Management**
   - Does not track child processes
   - No process groups or job control
   - No automatic cleanup of orphaned processes
   - **Rationale:** Simple fire-and-wait model sufficient for target use cases

4. **NOT Signal Handling**
   - Does not send signals to processes
   - No timeout-based termination
   - No graceful shutdown logic
   - **Rationale:** Basic execution model only

5. **NOT Concurrent Execution**
   - Does not run multiple processes in parallel
   - No thread pool or executor
   - **Rationale:** Callers can use rayon or tokio for concurrency

6. **NOT Typed Errors**
   - Uses untyped error_tools, not typed errors
   - Error context is string-based
   - **Rationale:** Flexibility over type safety for diverse error scenarios

7. **NOT Output Streaming**
   - Buffers all output before returning
   - No incremental/streaming output reading
   - **Rationale:** Simplicity over memory efficiency

8. **NOT Process Discovery**
   - Does not enumerate running processes
   - No PID lookup or process tree inspection
   - **Rationale:** Execution-focused, not system monitoring

9. **NOT Shell Parsing**
   - Does not parse shell syntax
   - Does not expand globs or variables
   - **Rationale:** Delegates to system shell via run_with_shell

10. **NOT Cross-Platform Path Translation**
    - Does not convert Unix paths to Windows paths
    - **Rationale:** Caller responsibility via pth crate if needed

#### Boundaries

- **process_tools vs duct**: process_tools wraps duct with builder pattern and Report structure; duct provides low-level execution
- **process_tools vs std::process::Command**: process_tools provides fluent API and comprehensive output capture; Command is lower-level
- **process_tools vs willbe**: process_tools provides execution primitives; willbe implements build automation logic

## Architecture

### Dependency Structure

```
process_tools (infrastructure)
├── Internal Dependencies
│   ├── mod_interface (workspace, module organization)
│   ├── former (workspace, builder pattern)
│   ├── pth (workspace, path utilities)
│   ├── error_tools (workspace, untyped errors)
│   └── iter_tools (workspace, Itertools trait)
└── External Dependencies
    └── duct (0.13.7, process execution)
```

### Module Organization

```
process_tools
├── lib.rs (mod_interface! entry point)
├── process (execution layer)
│   ├── run() - Direct execution
│   ├── Run - Configuration struct
│   ├── RunFormer - Builder
│   └── Report - Output capture
└── environment (CI/CD detection layer)
    └── is_cicd() - Environment detection
```

**Pattern:** Uses `mod_interface!` with inline `mod private { ... }` blocks (not `private.rs` files)

### Feature Architecture

```
enabled (master switch)
└── process_environment_is_cicd (CI/CD detection)
```

**Default Features:** `enabled`

**Feature Propagation:**
- All features are local to this crate
- No propagation to dependencies

### Execution Flow

#### Direct Execution Path

```
Run::former()
  .bin_path("rustc")
  .args(vec!["--version"])
  .current_path(".")
  .form()
  ↓
run(options)
  ↓
├─ joining_streams == true
│  ├─ duct::cmd()
│  ├─ .stderr_to_stdout()
│  └─ .stdout_capture()
│
└─ joining_streams == false
   ├─ std::process::Command::new()
   ├─ .stdout(Stdio::piped())
   └─ .stderr(Stdio::piped())
   ↓
Report {
  command: "rustc --version",
  current_path: ".",
  out: "rustc 1.75.0",
  err: "",
  error: Ok(())
}
```

#### Shell Execution Path

```
Run::former()
  .current_path(".")
  .run_with_shell("echo Hello")
  ↓
Detects platform
├─ Windows: cmd /C "echo Hello"
└─ Unix: sh -c "echo Hello"
  ↓
run(options)
  ↓
Report { ... }
```

## Public API

### Execution Functions

```rust
/// Execute process with explicit configuration
pub fn run(options: Run) -> Result<Report, Report>
```

### Configuration Builder

```rust
#[derive(Debug, Former)]
pub struct Run {
  bin_path: PathBuf,
  current_path: PathBuf,
  args: Vec<OsString>,
  #[former(default = false)]
  joining_streams: bool,
  env_variable: HashMap<String, String>,
}

impl RunFormer {
  /// Execute configured process
  pub fn run(self) -> Result<Report, Report>

  /// Execute shell command (cross-platform)
  pub fn run_with_shell(self, exec_path: &str) -> Result<Report, Report>
}
```

### Output Capture

```rust
#[derive(Debug, Clone)]
pub struct Report {
  /// Command that was executed
  pub command: String,
  /// Path where command was executed
  pub current_path: PathBuf,
  /// Standard output
  pub out: String,
  /// Standard error
  pub err: String,
  /// Error if any
  pub error: Result<(), Error>,
}

impl Display for Report {
  // Formats as:
  // > command
  //   @ /working/directory
  //
  //   stdout content
  //   stderr content
}
```

### CI/CD Detection

```rust
#[cfg(feature = "process_environment_is_cicd")]
#[must_use]
pub fn is_cicd() -> bool
```

Detects environment variables:
- `CI` - Common in many CI systems
- `GITHUB_ACTIONS` - GitHub Actions
- `GITLAB_CI` - GitLab CI
- `TRAVIS` - Travis CI
- `CIRCLECI` - CircleCI
- `JENKINS_URL` - Jenkins

## Usage Patterns

### Pattern 1: Direct Binary Execution

```rust
use process_tools::process;

let report = process::Run::former()
  .bin_path("rustc")
  .args(vec!["--version".into()])
  .current_path(".")
  .run()
  .expect("Failed to run rustc");

println!("{}", report.out); // "rustc 1.75.0..."
```

### Pattern 2: Shell Command Execution

```rust
use process_tools::process;

let report = process::Run::former()
  .current_path("/tmp")
  .run_with_shell("ls -la | grep txt")
  .expect("Shell command failed");

println!("Files:\n{}", report.out);
```

### Pattern 3: Stream Joining

```rust
use process_tools::process;

// Capture both stdout and stderr in single stream (preserves order)
let report = process::Run::former()
  .bin_path("cargo")
  .args(vec!["build".into()])
  .current_path("./my_project")
  .joining_streams(true) // stderr → stdout
  .run()
  .expect("Build failed");

// report.out contains interleaved stdout/stderr
// report.err is empty
```

### Pattern 4: Environment Variables

```rust
use process_tools::process;
use std::collections::HashMap;

let mut env = HashMap::new();
env.insert("RUST_BACKTRACE".to_string(), "1".to_string());

let report = process::Run::former()
  .bin_path("./my_app")
  .current_path(".")
  .env_variable(env)
  .run()
  .expect("App failed");
```

### Pattern 5: Error Handling

```rust
use process_tools::process;

match process::Run::former()
  .bin_path("cargo")
  .args(vec!["test".into()])
  .current_path(".")
  .run()
{
  Ok(report) => {
    println!("Tests passed!");
    println!("{}", report.out);
  },
  Err(report) => {
    eprintln!("Tests failed!");
    eprintln!("stdout: {}", report.out);
    eprintln!("stderr: {}", report.err);
    eprintln!("error: {:?}", report.error);
  }
}
```

### Pattern 6: CI/CD Detection

```rust
#[cfg(feature = "process_environment_is_cicd")]
use process_tools::environment;

if environment::is_cicd() {
  println!("Running in CI/CD environment");
  // Use different settings for CI
} else {
  println!("Running locally");
}
```

### Pattern 7: Report Display

```rust
use process_tools::process;

let report = process::Run::former()
  .bin_path("cargo")
  .args(vec!["build".into()])
  .current_path(".")
  .run()
  .expect("Build failed");

// Formatted output:
println!("{}", report);
// > cargo build
//   @ /home/user/project
//
//   Compiling my_crate v0.1.0
//   Finished dev [unoptimized] target(s)
```

## Dependencies and Consumers

### Direct Dependencies

**Internal (workspace):**
- `mod_interface` - Module organization pattern
- `former` - Builder pattern implementation
- `pth` - Path utilities
- `error_tools` - Untyped error handling (features: `error_untyped`)
- `iter_tools` - Iterator utilities (Itertools trait)

**External:**
- `duct` (0.13.7) - Advanced process execution with stream control

### Consumers (~1 workspace crate)

**Identified:**
- Likely used by `willbe` or similar automation tools for running build commands

**Usage Pattern:** Automation tools use process_tools to execute cargo commands, run tests, compile code, and detect CI/CD environments for conditional behavior.

## Design Rationale

### Why Wrap duct?

**Problem:** duct provides low-level process execution, but lacks:
- Fluent builder API
- Comprehensive output capture structure
- Error handling integration with workspace patterns

**Solution:** Wrap duct with Run/RunFormer/Report abstraction

**Benefits:**
1. **Ergonomics**: Fluent API via former
2. **Consistency**: Matches workspace patterns (error_tools, former)
3. **Output Capture**: Report struct is reusable across callers
4. **Flexibility**: Can use either duct or std::process::Command internally

### Why Result<Report, Report>?

The `run()` function returns `Result<Report, Report>` instead of `Result<Report, Error>` because:

1. **Complete Information**: Even on failure, caller needs stdout/stderr/command/path
2. **Error Context**: The Report.error field contains the actual error
3. **Display Formatting**: Both success and failure can be formatted the same way
4. **Debugging**: Full execution context available in both cases

**Example:**
```rust
match run(options) {
  Ok(report) => println!("Success: {}", report),
  Err(report) => eprintln!("Failure: {} - {:?}", report, report.error),
}
```

### Why Two Execution Modes?

The crate supports both duct-based and Command-based execution:

1. **Stream Joining (duct)**: When `joining_streams == true`, uses duct to preserve stdout/stderr interleaving
2. **Separate Streams (Command)**: When `joining_streams == false`, uses std::process::Command for separate capture

**Tradeoff:** Code complexity for flexibility in output handling

### Why Untyped Errors?

Uses `error_tools::untyped` instead of typed errors because:

1. **Error Diversity**: Process execution can fail in many ways (spawn, wait, UTF-8, exit code)
2. **Context Flexibility**: String-based context is easier to compose
3. **Ergonomics**: Simpler than defining comprehensive error enum

**Tradeoff:** Type safety for ergonomics and flexibility

### Why Feature-Gate is_cicd?

The `is_cicd()` function is behind `process_environment_is_cicd` feature because:

1. **Niche Use Case**: Not all process execution needs CI/CD detection
2. **Zero Dependencies**: Feature has no additional dependencies, but allows granular control
3. **Explicit Intent**: Users opt-in to environment inspection

### Why No Async Support?

process_tools is synchronous-only because:

1. **Target Use Case**: Build tools and automation typically run sequentially
2. **Simplicity**: No async runtime dependency
3. **Sufficient**: Callers can wrap in tokio::task::spawn_blocking if needed

**Tradeoff:** Simplicity over async flexibility

### Why No Timeout Support?

No timeout mechanism because:

1. **Complexity**: Requires threading or async runtime
2. **Platform Differences**: Timeout behavior varies across platforms
3. **External Control**: Callers can use external timeout mechanisms if needed

**Future Enhancement:** Could add optional timeout feature with std::thread-based implementation

## Testing Strategy

### Test Coverage

- **Basic Tests**: Smoke tests for core functionality
- **Stream Tests**: Separate tests for joined vs separate streams
- **Error Tests**: Tests for various failure modes
- **CI/CD Tests**: Basic test for is_cicd() (commented out due to environment dependency)

### Test Files

```
tests/
├── inc/
│   ├── process_run.rs - Process execution tests
│   ├── environment_is_cicd.rs - CI/CD detection tests
│   └── basic.rs - Smoke tests
├── asset/
│   └── err_out_test/ - Test programs producing stdout/stderr
├── smoke_test.rs - Entry point
└── tests.rs - Test aggregator
```

### Test Focus

1. **Stream Ordering**: Verify joined streams preserve order (err_out_err, out_err_out tests)
2. **Output Capture**: Verify stdout/stderr captured correctly
3. **Exit Codes**: Verify error reporting on non-zero exit
4. **Cross-Platform**: Tests work on Windows and Unix

### Known Test Limitations

1. **CI/CD Tests Commented Out**: Cannot reliably test environment variables without side effects
2. **No Async Tests**: No async execution to test
3. **No Timeout Tests**: No timeout functionality

## Future Considerations

### Potential Enhancements

1. **Timeout Support**: Add optional timeout with thread-based implementation
2. **Async API**: Add async variants (run_async, run_with_shell_async)
3. **Typed Errors**: Define ProcessError enum for better error handling
4. **Streaming Output**: Add incremental output reading via callback
5. **Process Groups**: Support for process group management
6. **Signal Sending**: Ability to send signals to child processes

### Breaking Changes to Consider

1. **Result Type**: Change from `Result<Report, Report>` to `Result<Report, ProcessError>`
2. **Typed Errors**: Replace error_tools::untyped with custom error types
3. **Default Stream Mode**: Make joining_streams default to true for better output ordering

### Known Limitations

1. **No Interactive Stdin**: Cannot interact with processes requiring user input
2. **Memory Buffering**: All output buffered in memory (problematic for large outputs)
3. **No Streaming**: Cannot process output incrementally
4. **Platform Shell Dependency**: Relies on sh/cmd availability
5. **UTF-8 Only**: No support for non-UTF-8 process output

## Adoption Guidelines

### When to Use process_tools

**Good Candidates:**
- Build automation tools
- CI/CD scripts and workflows
- Test runners and code generators
- Command-line tools orchestrating external programs
- Tools needing CI/CD environment detection

**Poor Candidates:**
- Interactive shell applications (use pty crate)
- Long-running processes with streaming output (use tokio::process)
- High-performance parallel execution (use rayon + std::process directly)
- Windows-specific process management (use winapi)

### Migration from std::process::Command

```rust
// Before: std::process::Command
let output = Command::new("cargo")
  .args(&["build"])
  .current_dir(".")
  .output()
  .expect("Failed to run");

println!("{}", String::from_utf8_lossy(&output.stdout));

// After: process_tools
let report = process::Run::former()
  .bin_path("cargo")
  .args(vec!["build".into()])
  .current_path(".")
  .run()
  .expect("Failed to run");

println!("{}", report.out);
```

### Best Practices

1. **Use Builder Pattern**: Always use `Run::former()` for clarity
2. **Handle Both Cases**: Match on `Result<Report, Report>` to handle success and failure
3. **Check Exit Codes**: Verify `output.status.success()` or match on Result
4. **Stream Joining**: Use `joining_streams(true)` when order matters
5. **Environment Variables**: Pass only necessary variables, not entire environment
6. **CI/CD Detection**: Use `is_cicd()` for conditional behavior in tools

## Related Crates

- **duct**: External process execution library (dependency)
- **former**: Builder pattern implementation (dependency)
- **error_tools**: Untyped error handling (dependency)
- **willbe**: Build automation tool (likely consumer)

## References

- [API Documentation](https://docs.rs/process_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/process_tools)
- [duct crate](https://docs.rs/duct) - Underlying process execution library
- [readme.md](./readme.md)
