# Design Rules for Tests Directory

## CRITICAL: Test Organization Rules

This `tests/` directory must follow strict design rules from `$PRO/genai/code/rules/code_design.rulebook.md`.

### ✅ ALLOWED in tests/ directory:
- **Unit tests** - Testing individual functions and modules
- **Integration tests** - Testing public APIs and system integration
- **Functional tests** - Testing complete workflows and features
- **Test utilities** - Helper functions and shared test code

### ❌ PROHIBITED in tests/ directory:
- **Performance benchmarks** - Must use `benchkit` framework separately
- **Custom timing code** - No `std::time::Instant` for performance measurement
- **Benchmark disguised as tests** - No performance measurement in test assertions
- **Speed comparisons** - Belongs in proper benchmark infrastructure

## Required Test File Structure

Every test file MUST include:

```rust
//! ## Test Matrix for [Feature Name]
//!
//! | ID | Test Case | Expected Result |
//! |----|-----------|-----------------|
//! | TC1 | [description] | [expected] |
//! | TC2 | [description] | [expected] |
//!
//! This documentation is MANDATORY per design rules.

/// Test for [specific functionality]
///
/// **Test Combination ID:** TC1
/// **Purpose:** [Clear description of what this test validates]
#[test]
fn test_specific_functionality() {
    // Test implementation
}
```

## Performance Testing

**For performance testing, use `benchkit` framework in separate infrastructure:**

```rust
// ❌ WRONG - Do not put this in tests/
#[test]
fn test_performance() {
    let start = std::time::Instant::now();
    // ... some operation
    let duration = start.elapsed();
    assert!(duration < std::time::Duration::from_millis(100)); // RULE VIOLATION
}

// ✅ CORRECT - Use benchkit framework separately
// (Not in tests/ directory)
```

## Common Rule Violations

### ❌ Violation Examples:
1. **Custom Performance Timing:**
   ```rust
   let start = std::time::Instant::now();
   // operation
   let duration = start.elapsed();
   ```

2. **Speed Assertions in Tests:**
   ```rust
   assert!(ops_per_second > 1000.0); // Performance assertion in unit test
   ```

3. **Missing Test Matrix Documentation:**
   ```rust
   // Missing //! Test Matrix comment
   #[test]
   fn some_test() { /* ... */ }
   ```

### ✅ Correct Approach:
1. **Functional Testing Only:**
   ```rust
   #[test]
   fn test_correctness() {
       let result = function_under_test();
       assert_eq!(result, expected_value); // Correctness, not performance
   }
   ```

2. **Proper Documentation:**
   ```rust
   //! ## Test Matrix for Registry
   //! | TC1 | Register command | Success |
   /// Test command registration functionality
   /// **Test Combination ID:** TC1
   #[test]
   fn test_register_command() { /* ... */ }
   ```

## Remember: Separate Concerns

- **`tests/`** → Correctness, functionality, integration
- **`benchkit`** → Performance, speed, optimization measurement

This separation is enforced by design rules and must be maintained.