# Task 001: Fix MarkdownUpdater Section Matching Bug

## Priority: CRITICAL
## Status: Open
## Severity: High - Breaks documentation generation

## Problem Summary

The `MarkdownUpdater.replace_section_content()` method in `src/reporting.rs:56` uses substring matching (`line.contains()`) instead of exact matching for section headers. This causes severe section duplication when section names share common substrings.

## Root Cause

**File:** `src/reporting.rs:56`  
**Buggy Code:**
```rust
if line.contains(self.section_marker.trim_start_matches("## ")) {
```

**Issue:** When updating section "Performance Benchmarks", it incorrectly matches ANY section containing "Performance", including:
- `## Performance Benchmarks` ✓ (intended)
- `## Language Operations Performance` ✓ (unintended - contains "Performance")
- `## Realistic Scenarios Performance` ✓ (unintended - contains "Performance")

## Impact

### Real-World Evidence
- wflow project: readme.md grew from 5,865 to 7,751 lines (+1,886 lines) in one benchmark run
- 37 duplicate "Performance Benchmarks" sections created
- 201 duplicate table headers generated
- Documentation becomes unusable and contradictory

### User Experience Impact
- **Critical**: Documentation generation completely breaks
- **High**: Exponential file growth with each benchmark run  
- **Medium**: Confusing/contradictory performance claims
- **Low**: Repository bloat affects git performance

## Reproduction Steps

### MRE (Minimal Reproduction Example)

```rust
// Create file with overlapping section names:
let content = r#"
## Performance Benchmarks
Old data
## Language Operations Performance  
Old data
"#;

// Try to update "Performance Benchmarks" section:
let updater = MarkdownUpdater::new("test.md", "Performance Benchmarks");
updater.update_section("New data");

// BUG: Content gets inserted into BOTH sections because 
// both contain substring "Performance"
```

### Complete MRE File
A complete reproduction example is available at: `../../../llm_tools/module/wflow/final_mre.rs`

Run with: `rustc final_mre.rs && ./final_mre`

## Technical Fix Required

### Change Location
**File:** `src/reporting.rs`  
**Line:** 56

### Current (Buggy) Implementation
```rust
if line.contains(self.section_marker.trim_start_matches("## ")) {
```

### Required Fix
```rust
if line.trim() == self.section_marker.trim() {
```

### Alternative Fix (More Robust)
```rust
if line.trim_start().starts_with("## ") && line.trim() == self.section_marker.trim() {
```

## Testing Requirements

### Test Cases to Add
1. **Overlapping section names**: "Performance" and "Performance Benchmarks"
2. **Substring matching edge cases**: "API" and "API Documentation" 
3. **Exact matching validation**: Ensure only exact sections are updated
4. **Multiple updates**: Verify no duplication after repeated runs
5. **Case sensitivity**: Test various whitespace/formatting scenarios

### Regression Test
```rust
#[test]
fn test_no_section_duplication_with_overlapping_names() {
    let initial = r#"## Performance Benchmarks
Old data
## Language Operations Performance
Old data"#;
    
    let updater = MarkdownUpdater::new("test.md", "Performance Benchmarks");
    let result = updater.replace_section_content(initial, "New data");
    
    // Should have exactly 2 sections, not 3 or more
    assert_eq!(result.matches("## ").count(), 2);
    assert_eq!(result.matches("## Performance Benchmarks").count(), 1);
    assert!(result.contains("New data"));
}
```

## Verification Checklist

- [ ] Fix applied to `src/reporting.rs:56`
- [ ] Regression tests added and passing
- [ ] No existing functionality broken
- [ ] Documentation updated if needed
- [ ] Integration test with real markdown files
- [ ] Performance impact assessed (should be minimal)

## Additional Context

This bug was discovered during wflow project benchmarking where multiple benchmark functions used overlapping section names:
- `benchmark_line_counting_performance()` → "Performance Benchmarks"
- `benchmark_language_operations()` → "Language Operations Performance"  
- `benchmark_realistic_scenarios()` → "Realistic Scenarios Performance"

Each benchmark run multiplied the duplicate sections, making documentation maintenance impossible.

## Related Issues
- None currently, but this affects any user with overlapping section names
- Could impact other projects using benchkit for documentation generation

---
**Created by:** wflow development team  
**Date:** Current  
**Assignee:** TBD  
**Labels:** bug, critical, documentation, markdown