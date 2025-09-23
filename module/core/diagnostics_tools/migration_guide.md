# Migration Guide

This guide helps you migrate from standard Rust assertions to `diagnostics_tools` for better debugging experience.

## Quick Migration Table

| Standard Rust | Diagnostics Tools | Notes |
|---------------|-------------------|-------|
| `assert!(condition)` | `a_true!(condition)` | Same behavior, better error context |
| `assert!(!condition)` | `a_false!(condition)` | More explicit intent |
| `assert_eq!(a, b)` | `a_id!(a, b)` | Colored diff output |
| `assert_ne!(a, b)` | `a_not_id!(a, b)` | Colored diff output |
| `debug_assert!(condition)` | `a_dbg_true!(condition)` | Always prints values |
| `debug_assert_eq!(a, b)` | `a_dbg_id!(a, b)` | Always prints values |

## Step-by-Step Migration

### 1. Add Dependency

Update your `Cargo.toml`:

```toml
[dependencies]
# Add this line:
diagnostics_tools = "0.11"
```

### 2. Import the Prelude

Add to your source files:

```rust
// At the top of your file:
use diagnostics_tools::*;
```

Or more specifically:
```rust
use diagnostics_tools::{ a_true, a_false, a_id, a_not_id };
```

### 3. Replace Assertions Gradually

**Before:**
```rust
fn test_my_function() 
{
    let result = my_function();
    assert_eq!(result.len(), 3);
    assert!(result.contains("hello"));
    assert_ne!(result[0], "");
}
```

**After:**
```rust
fn test_my_function() 
{
    let result = my_function();
    a_id!(result.len(), 3);           // Better diff on failure
    a_true!(result.contains("hello")); // Better error context  
    a_not_id!(result[0], "");         // Better diff on failure
}
```

## Advanced Migration Scenarios

### Testing Complex Data Structures

**Before:**
```rust
#[test]
fn test_user_data() 
{
    let user = create_user();
    assert_eq!(user.name, "John");
    assert_eq!(user.age, 30);
    assert_eq!(user.emails.len(), 2);
}
```

**After:**
```rust
#[test]
fn test_user_data() 
{
    let user = create_user();
    
    // Get beautiful structured diffs for complex comparisons:
    a_id!(user, User {
        name: "John".to_string(),
        age: 30,
        emails: vec!["john@example.com".to_string(), "j@example.com".to_string()],
    });
}
```

### Adding Compile-Time Checks

**Before:**
```rust
// No equivalent - this was impossible with standard assertions
```

**After:**
```rust
// Validate assumptions at compile time:
cta_true!(cfg(feature = "serde"));
cta_type_same_size!(u32, i32);
cta_type_same_align!(u64, f64);
```

### Development vs Production

**Before:**
```rust
fn validate_input(data: &[u8]) 
{
    debug_assert!(data.len() > 0);
    debug_assert!(data.len() < 1024);
}
```

**After:**
```rust
fn validate_input(data: &[u8]) 
{
    // Debug variants show values even on success during development:
    a_dbg_true!(data.len() > 0);
    a_dbg_true!(data.len() < 1024);
    
    // Or use regular variants that only show output on failure:
    a_true!(data.len() > 0);
    a_true!(data.len() < 1024);
}
```

## Coexistence Strategy

You dont need to migrate everything at once. The crates work together:

```rust
use diagnostics_tools::*;

fn mixed_assertions() 
{
    // Keep existing assertions:
    assert!(some_condition);
    
    // Add enhanced ones where helpful:
    a_id!(complex_struct_a, complex_struct_b); // Better for complex comparisons
    
    // Use compile-time checks for new assumptions:
    cta_true!(cfg(target_pointer_width = "64"));
}
```

## Common Migration Patterns

### 1. Test Suites

Focus on test files first - this is where better error messages provide the most value:

```rust
// tests/integration_test.rs
use diagnostics_tools::*;

#[test]
fn api_response_format() 
{
    let response = call_api();
    
    // Much clearer when JSON structures differ:
    a_id!(response, expected_json_structure());
}
```

### 2. Development Utilities

Use debug variants during active development:

```rust
fn debug_data_processing(input: &Data) -> ProcessedData 
{
    let result = process_data(input);
    
    // Shows values even when assertions pass - helpful during development:
    a_dbg_id!(result.status, Status::Success);
    a_dbg_true!(result.items.len() > 0);
    
    result
}
```

### 3. Library Boundaries

Add compile-time validation for public APIs:

```rust
pub fn new_public_api<T>() -> T 
where 
    T: Default + Clone + Send,
{
    // Validate assumptions about T at compile time:
    cta_type_same_size!(T, T);  // Sanity check
    
    // Runtime validation with better errors:
    let result = T::default();
    a_true!(std::mem::size_of::<T>() > 0);
    
    result
}
```

## Tips for Smooth Migration

1. **Start with Tests**: Migrate test assertions first - you'll see immediate benefits
2. **Use Debug Variants During Development**: They provide extra visibility
3. **Add Compile-Time Checks Gradually**: Look for assumptions that could be validated earlier
4. **Focus on Complex Comparisons**: The biggest wins come from comparing structs, vectors, and other complex data
5. **Keep It Mixed**: You dont need to replace every assertion - focus on where enhanced messages help most

## Rollback Strategy

If you need to rollback temporarily, simply:

1. Remove the `use diagnostics_tools::*;` import
2. Use find-replace to convert back:
   - `a_true!` → `assert!`
   - `a_id!` → `assert_eq!`  
   - `a_not_id!` → `assert_ne!`
   - Remove any compile-time assertions (they have no standard equivalent)

The migration is designed to be low-risk and reversible.