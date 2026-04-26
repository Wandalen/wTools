# Manual Testing Plan: program_tools

### Scope

This crate implements **data structures only** (Source, Program, Plan) with Former builder pattern integration. No compilation or execution functionality is currently implemented.

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

Since this crate has no executable examples and no execution functionality, manual testing requires:

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
| 1.1 | Empty file_path | ⏳ Pending | |
| 1.2 | Empty data | ⏳ Pending | |
| 1.3 | Both empty | ⏳ Pending | |
| 1.4 | Large data | ⏳ Pending | |
| 1.5 | Special chars | ⏳ Pending | |
| 2.1 | Zero sources | ⏳ Pending | |
| 2.2 | Single source | ⏳ Pending | |
| 2.3 | Multiple sources | ⏳ Pending | Partially covered by tests/inc/basic.rs |
| 2.4 | Duplicate paths | ⏳ Pending | |
| 3.1 | Minimal plan | ⏳ Pending | |
| 3.2 | Complete chain | ✅ Covered | tests/inc/basic.rs |
| 3.3 | Nested end() | ✅ Covered | Verified by type system |
| 4.1 | Debug trait | ⏳ Pending | |
| 5.1 | Exposed namespace | ✅ Covered | tests/inc/basic.rs uses the_module |
| 5.2 | Prelude namespace | ⏳ Pending | |

### Known Limitations

The following are **intentionally NOT implemented** at this stage:

- Compilation execution (no cargo/rustc)
- Process management (no spawning/capture)
- Cargo.toml generation
- Incremental compilation
- Cross-compilation
- IDE integration
- Async execution
- Security sandboxing

Manual testing focuses ONLY on data structure construction via Former builders.

### Conclusion

This manual testing plan validates the builder API for program_tools' data structures. Most corner cases should be converted to automated tests in tests/inc/ to prevent regressions. Manual testing serves to verify behavior before adding automated coverage.
