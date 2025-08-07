# Task: Comprehensive Test Plan for Lifetime-Only Structs

## Test Categories

### 1. Basic Lifetime Tests

#### Test: Simple Single Lifetime
```rust
#[derive(Former)]
struct Simple<'a> {
    data: &'a str,
}

#[test]
fn test_simple_lifetime() {
    let data = "hello";
    let s = Simple::former()
        .data(data)
        .form();
    assert_eq!(s.data, "hello");
}
```

#### Test: Multiple Lifetimes
```rust
#[derive(Former)]
struct MultiLifetime<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

#[test]
fn test_multi_lifetime() {
    let data1 = "hello";
    let data2 = "world";
    let s = MultiLifetime::former()
        .first(data1)
        .second(data2)
        .form();
    assert_eq!(s.first, "hello");
    assert_eq!(s.second, "world");
}
```

### 2. Complex Lifetime Tests

#### Test: Lifetime Bounds
```rust
#[derive(Former)]
struct WithBounds<'a: 'b, 'b> {
    long_lived: &'a str,
    short_lived: &'b str,
}
```

#### Test: Lifetime in Complex Types
```rust
#[derive(Former)]
struct ComplexLifetime<'a> {
    data: &'a str,
    vec_ref: &'a Vec<String>,
    optional: Option<&'a str>,
}
```

### 3. Mixed Generic Tests (Regression)

#### Test: Lifetime + Type Parameter
```rust
#[derive(Former)]
struct Mixed<'a, T> {
    data: &'a str,
    value: T,
}
```

#### Test: Multiple of Each
```rust
#[derive(Former)]
struct Complex<'a, 'b, T, U> {
    ref1: &'a str,
    ref2: &'b str,
    val1: T,
    val2: U,
}
```

### 4. Edge Cases

#### Test: Empty Struct with Lifetime
```rust
#[derive(Former)]
struct Empty<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
```

#### Test: Const Generics with Lifetimes
```rust
#[derive(Former)]
struct ConstGeneric<'a, const N: usize> {
    data: &'a [u8; N],
}
```

### 5. Generated Code Validation Tests

These tests should verify the generated code is correct:

#### Test: Check Former Struct Signature
- Verify `SimpleFormer<'a, Definition>` is generated correctly
- No trailing commas in generic parameters
- Proper where clauses

#### Test: Check Impl Blocks
- EntityToFormer impl has correct generics
- EntityToDefinition impl works
- All associated types resolve correctly

### 6. Compilation Error Tests

These should be in a separate `compile_fail` directory:

#### Test: Lifetime Mismatch
```rust
#[derive(Former)]
struct Test<'a> {
    data: &'a str,
}

fn bad_usage() {
    let s = Test::former()
        .data(&String::from("temp")) // Error: temporary value
        .form();
}
```

### 7. Integration Tests

#### Test: Nested Structs with Lifetimes
```rust
#[derive(Former)]
struct Inner<'a> {
    data: &'a str,
}

#[derive(Former)]
struct Outer<'a> {
    inner: Inner<'a>,
}
```

#### Test: With Collections
```rust
#[derive(Former)]
struct WithVec<'a> {
    items: Vec<&'a str>,
}
```

## Test File Organization

```
tests/inc/struct_tests/
├── lifetime_only_basic.rs          # Basic single/multi lifetime tests
├── lifetime_only_complex.rs        # Complex bounds and edge cases  
├── lifetime_only_mixed.rs          # Mixed generic regression tests
├── lifetime_only_integration.rs    # Integration with other features
└── lifetime_only_compile_fail/     # Compilation error tests
    └── lifetime_mismatch.rs
```

## Test Execution Plan

1. **Phase 1**: Implement basic lifetime tests
   - Start with simplest case (single lifetime)
   - Verify generated code with `#[debug]`
   
2. **Phase 2**: Add complex cases
   - Multiple lifetimes
   - Lifetime bounds
   - Mixed generics
   
3. **Phase 3**: Edge cases and error scenarios
   - Empty structs
   - Const generics
   - Compilation errors
   
4. **Phase 4**: Integration tests
   - Nested structs
   - Collections
   - Subformers

## Success Metrics

1. All tests pass
2. No regression in existing tests
3. Generated code is syntactically correct
4. Compilation errors are clear and helpful
5. Performance is not degraded

## Debugging Strategy

For failing tests:
1. Enable `#[debug]` attribute to see generated code
2. Check for trailing commas in generics
3. Verify impl block generic parameters
4. Look for lifetime position errors
5. Use `cargo expand` for detailed view