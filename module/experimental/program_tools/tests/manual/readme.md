# Manual Testing Plan: program_tools

### Scope

Manual testing plan for the `program_tools` data structure layer (Source, Program, Plan with Former builder pattern) and runner API (run_source, run_file, run_project, CapturedOutput). See `docs/feature/` and `docs/api/` for full specifications.

### Testing Objectives

Validate builder API construction for all three data structures across comprehensive corner cases including edge cases, empty inputs, large data, and special characters.

### Manual Test Cases

#### 1. Source Builder API

**Test Case 1.1: Empty file_path**
```rust
// Execute in Rust playground or test environment
use program_tools::program::*;

let source = Source::former()
  .file_path("")
  .data("fn main() {}")
  .form();

println!("Empty file_path: {:?}", source);
// Expected: Source { file_path: "", data: "fn main() {}" }
// Status: ✅ PASS if constructs without panic
```

**Test Case 1.2: Empty data**
```rust
let source = Source::former()
  .file_path("main.rs")
  .data("")
  .form();

println!("Empty data: {:?}", source);
// Expected: Source { file_path: "main.rs", data: "" }
// Status: ✅ PASS if constructs without panic
```

**Test Case 1.3: Both fields empty**
```rust
let source = Source::former()
  .file_path("")
  .data("")
  .form();

println!("Both empty: {:?}", source);
// Expected: Source { file_path: "", data: "" }
// Status: ✅ PASS if constructs without panic
```

**Test Case 1.4: Large data (1MB string)**
```rust
let large_data = "// comment\n".repeat(50_000); // ~500KB
let source = Source::former()
  .file_path("large.rs")
  .data(&large_data)
  .form();

println!("Large data size: {} bytes", source.data.len());
// Expected: Constructs without panic, len ~500,000
// Status: ✅ PASS if no memory issues
```

**Test Case 1.5: Special characters (Unicode, newlines, tabs)**
```rust
let source = Source::former()
  .file_path("src/模块.rs") // Unicode Chinese
  .data("fn main() {\n\tprintln!(\"🦀 Rust\");\n}")
  .form();

println!("Special chars: {:?}", source);
// Expected: Preserves exact Unicode and escape sequences
// Status: ✅ PASS if exact preservation
```

#### 2. Program Builder API

**Test Case 2.1: Zero sources (empty program)**
```rust
let program = Program::former().form();

println!("Empty program sources: {:?}", program.source);
// Expected: Program { source: [] }
// Status: ✅ PASS if empty Vec
```

**Test Case 2.2: Single source**
```rust
let program = Program::former()
  .source()
    .file_path("main.rs")
    .data("fn main() {}")
    .end()
  .form();

println!("Single source count: {}", program.source.len());
// Expected: 1 source in Vec
// Status: ✅ PASS if len == 1
```

**Test Case 2.3: Multiple sources**
```rust
let program = Program::former()
  .source()
    .file_path("main.rs")
    .data("fn main() {}")
    .end()
  .source()
    .file_path("lib.rs")
    .data("pub fn helper() {}")
    .end()
  .source()
    .file_path("utils.rs")
    .data("pub fn util() {}")
    .end()
  .form();

println!("Multiple sources count: {}", program.source.len());
// Expected: 3 sources in Vec
// Status: ✅ PASS if len == 3
```

**Test Case 2.4: Duplicate file paths**
```rust
let program = Program::former()
  .source()
    .file_path("main.rs")
    .data("// version 1")
    .end()
  .source()
    .file_path("main.rs") // Duplicate path
    .data("// version 2")
    .end()
  .form();

println!("Duplicate paths: {:?}", program.source);
// Expected: Both sources present (no validation)
// Status: ✅ PASS if both exist with same path
```

#### 3. Plan Builder API

**Test Case 3.1: Minimal plan (program with zero sources)**
```rust
let plan = Plan::former()
  .program()
    // No sources added
    .end()
  .form();

println!("Minimal plan: {:?}", plan);
// Expected: Plan with empty program
// Status: ✅ PASS if constructs
```

**Test Case 3.2: Complete nested builder chain**
```rust
let plan = Plan::former()
  .program()
    .source()
      .file_path("main.rs")
      .data("fn main() { println!(\"test\"); }")
      .end()
    .end()
  .form();

println!("Complete chain: {:?}", plan);
// Expected: Fully populated Plan → Program → Source
// Status: ✅ PASS if all fields populated
```

**Test Case 3.3: Verify .end() returns correct parent**
```rust
// This is tested by compilation success - type system enforces correctness
let plan = Plan::former()
  .program() // Returns ProgramFormer
    .source() // Returns SourceFormer
      .file_path("test.rs")
      .data("code")
      .end() // MUST return ProgramFormer, not PlanFormer
    .end() // MUST return PlanFormer
  .form();

// Expected: Compiles without type errors
// Status: ✅ PASS if compiles (Former guarantees)
```

#### 4. Debug Trait Validation

**Test Case 4.1: Debug formatting for all structs**
```rust
let source = Source::former()
  .file_path("test.rs")
  .data("code")
  .form();

let program = Program::former()
  .source()
    .file_path("main.rs")
    .data("fn main() {}")
    .end()
  .form();

let plan = Plan::former()
  .program()
    .source()
      .file_path("lib.rs")
      .data("pub fn test() {}")
      .end()
    .end()
  .form();

println!("Source: {:?}", source);
println!("Program: {:?}", program);
println!("Plan: {:?}", plan);

// Expected: All print without panic
// Status: ✅ PASS if no panics
```

#### 5. Namespace Validation

**Test Case 5.1: Exposed namespace**
```rust
use program_tools::*;

let _source: program::Source = program::Source::former().form();
let _program: program::Program = program::Program::former().form();
let _plan: program::Plan = program::Plan::former().form();

// Expected: All types accessible via program:: namespace
// Status: ✅ PASS if compiles
```

**Test Case 5.2: Prelude namespace**
```rust
use program_tools::prelude::*;

// Prelude imports should also work
// Check via documentation what prelude exports

// Expected: Types accessible via prelude
// Status: ℹ️ CHECK documentation for prelude contents
```

### Manual Execution Instructions

All cases are now covered by automated tests. For ad-hoc manual verification:

1. **Create temporary test binary:**
   ```bash
   cd /tmp
   cargo new program_tools_manual_test
   cd program_tools_manual_test
   ```

2. **Add program_tools dependency to Cargo.toml:**
   ```toml
   [dependencies]
   program_tools = { path = "/home/user1/pro/lib/wip_core/wtools/dev/module/experimental/program_tools" }
   ```

3. **Copy test case code to src/main.rs**

4. **Run:**
   ```bash
   cargo run
   ```

5. **Verify output matches expected behavior**

### Test Results Tracking

| Test ID | Description | Status | Notes |
|---------|-------------|--------|-------|
| 1.1 | Empty file_path | Covered | `corner_cases_test::source_empty_file_path` |
| 1.2 | Empty data | Covered | `corner_cases_test::source_empty_data` |
| 1.3 | Both empty | Covered | `corner_cases_test::source_both_fields_empty` |
| 1.4 | Large data | Covered | `corner_cases_test::source_large_data` |
| 1.5 | Special chars | Covered | `corner_cases_test::source_special_characters` |
| 2.1 | Zero sources | Covered | `corner_cases_test::program_zero_sources` |
| 2.2 | Single source | Covered | `corner_cases_test::program_single_source` |
| 2.3 | Multiple sources | Covered | `basic::basic` |
| 2.4 | Duplicate paths | Covered | `corner_cases_test::program_duplicate_file_paths` |
| 3.1 | Minimal plan | Covered | `corner_cases_test::plan_minimal_with_empty_program` |
| 3.2 | Complete chain | Covered | `basic::basic` |
| 3.3 | Nested end() | Covered | Verified by type system |
| 4.1 | Debug trait | Covered | `corner_cases_test::debug_trait_all_structs` |
| 5.1 | Exposed namespace | Covered | `basic::basic` |
| 5.2 | Prelude namespace | Covered | `corner_cases_test::namespace_prelude_imports` |

### Implemented Runner API

The following capabilities are now implemented and covered by automated tests in `tests/inc/runner_test.rs`:

- Script compilation and execution via `run_source`, `run_file`, `run_project` (`api/002`)
- Output capture into `CapturedOutput` with predicate and assertion methods (`feature/002`, `api/003`)
- Isolated temp workspaces with RAII cleanup (`feature/003`)
- Forwarding mode (`capture=false`) via `cmd.status()` / `try_wait()` — output streams to terminal
- Timeout enforcement via `timeout_ms` (`runner_test::run_timeout_fires`, `run_timeout_does_not_fire`)
  - Capture mode: channel + background thread; child not killed on timeout (v0.1.0 limitation)
  - Forwarding mode: polling `try_wait()` + `child.kill()` on expiry

Not yet implemented:

- CLI interface `program_tools run` (`api/004`)
- Async execution

### Conclusion

All data-structure corner cases are automated in `tests/inc/corner_cases_test.rs`. Runner and output API integration tests are in `tests/inc/runner_test.rs`. This plan serves as a reference for test design rationale.
