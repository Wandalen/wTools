# Implement Benchkit Standard Setup Protocol

## Description

**CRITICAL VIOLATION**: Usage.md states "NON-NEGOTIABLE REQUIREMENT" - ALL implementations MUST begin with standardized setup protocol. Current benchmarks lack the required setup pattern.

**Required Setup Pattern** (from usage.md):
```rust
use benchkit::prelude::*;

fn main() {
    let mut suite = BenchmarkSuite::new("Getting Started");
    suite.benchmark("basic_function", || your_function_here());
    let results = suite.run_all();
    
    // MANDATORY: Update README.md automatically
    let updater = MarkdownUpdater::new("README.md", "Performance").unwrap();
    updater.update_section(&results.generate_markdown_report()).unwrap();
}
```

**Current State**: No evidence of this standardized setup protocol in any benchmark file.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must follow benchkit usage.md "Required Implementation Protocols" section
-   Related to Task 028 (directory structure fix) and Task 030 (CV analysis)

## Acceptance Criteria

-   [ ] All benchmark files implement the standardized setup protocol
-   [ ] All benchmarks use `BenchmarkSuite::new()` for initialization
-   [ ] All benchmarks call `suite.run_all()` for execution
-   [ ] Automatic documentation updates implemented with `MarkdownUpdater`
-   [ ] `cargo bench` workflow confirmed as primary interface
-   [ ] No custom benchmark runner scripts remaining