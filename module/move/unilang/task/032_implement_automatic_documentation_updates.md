# Implement Automatic Documentation Updates

## Description

**CRITICAL VIOLATION**: Usage.md **BEST PRACTICE** states benchmarks MUST automatically update documentation. No `MarkdownUpdater` usage found in any benchmark file.

**Required Implementation** (from usage.md):
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let results = run_benchmark_suite()?;
    
    // Update multiple documentation files
    let updates = vec![
        ("README.md", "Performance Overview"),
        ("PERFORMANCE.md", "Detailed Results"), 
        ("docs/optimization_guide.md", "Current Benchmarks"),
    ];
    
    for (file, section) in updates {
        let updater = MarkdownUpdater::new(file, section)?;
        updater.update_section(&results.generate_markdown_report())?;
    }
    
    println!("✅ Documentation updated automatically");
    Ok(())
}
```

**Current State**: Manual documentation updates are error-prone and time-consuming. No automation exists.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must implement benchkit usage.md "Automatic Documentation Updates" section
-   Must fix generic section naming (related to Task 033)
-   Related to Task 029 (setup protocol) and Task 031 (measurement context)

## Acceptance Criteria

-   [ ] All benchmarks use `MarkdownUpdater` for automatic documentation updates
-   [ ] Multiple documentation files updated automatically (README.md, PERFORMANCE.md, etc.)
-   [ ] Benchmark results automatically formatted as markdown reports
-   [ ] Section names are specific and descriptive (not generic "Performance")
-   [ ] Documentation stays current with benchmark results
-   [ ] Error handling implemented for documentation update failures
-   [ ] Success confirmation messages displayed after updates