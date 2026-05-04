# Manual Testing Plan for impls_index_meta

## Overview

Manual testing plan for the `impls_index_meta` procedural macro crate. This crate provides the `impls3!` macro for function indexing.

## Prerequisites

- Rust toolchain with proc-macro support installed
- `cargo nextest` available for running automated test equivalents
- Project built with `cargo build --all-features`

## Test Categories

### 1. Function Variants

#### 1.1 Functions with Lifetimes
```rust
impls3!
{
  fn with_lifetime<'a>(s: &'a str) -> &'a str { s }
}
```
**Expected**: Compiles successfully, generates macro with lifetime parameters preserved

#### 1.2 Functions with Where Clauses
```rust
impls3!
{
  fn with_where<T>(x: T) -> T
  where
    T: Clone + Send
  {
    x.clone()
  }
}
```
**Expected**: Compiles successfully, preserves where clause in generated macro

#### 1.3 Async Functions
```rust
impls3!
{
  async fn async_operation() -> i32
  {
    42
  }
}
```
**Expected**: Compiles successfully, preserves async keyword

#### 1.4 Const Functions
```rust
impls3!
{
  const fn const_operation() -> i32
  {
    42
  }
}
```
**Expected**: Compiles successfully, preserves const keyword

#### 1.5 Unsafe Functions
```rust
impls3!
{
  unsafe fn unsafe_operation()
  {
    // unsafe code
  }
}
```
**Expected**: Compiles successfully, preserves unsafe keyword

### 2. Attribute Handling

#### 2.1 Functions with Inline Attribute
```rust
impls3!
{
  #[inline]
  fn inline_fn() -> i32 { 42 }
}
```
**Expected**: Compiles successfully, preserves attributes

#### 2.2 Functions with Doc Comments
```rust
impls3!
{
  /// Documentation for this function
  fn documented() -> i32 { 42 }
}
```
**Expected**: Compiles successfully, preserves doc comments

#### 2.3 Functions with Conditional Compilation
```rust
impls3!
{
  #[cfg(test)]
  fn test_only() -> i32 { 42 }
}
```
**Expected**: Compiles successfully, preserves cfg attributes

### 3. Edge Cases

#### 3.1 Empty impls3! Block
```rust
impls3! {}
```
**Expected**: Compiles successfully (empty expansion)

#### 3.2 Mixed Optional and Non-Optional Functions
```rust
impls3!
{
  fn required() -> i32 { 1 }

  ? fn optional() -> i32 { 2 }
}
```
**Expected**: Compiles successfully, non-optional macro must be used

#### 3.3 Complex Parameter Types
```rust
impls3!
{
  fn complex(
    a: Vec<(i32, String)>,
    b: Option<Result<i32, String>>,
  ) -> i32
  {
    42
  }
}
```
**Expected**: Compiles successfully, preserves complex types

#### 3.4 Functions Returning impl Trait
```rust
impls3!
{
  fn returns_impl() -> impl Iterator<Item = i32>
  {
    std::iter::once(42)
  }
}
```
**Expected**: Compiles successfully, preserves impl Trait return type

#### 3.5 Functions with Default Type Parameters
```rust
impls3!
{
  fn with_default<T = i32>(x: T) -> T { x }
}
```
**Expected**: Compiles successfully, preserves default type parameters

### 4. Error Cases

#### 4.1 Non-Function Item (Struct)
```rust
impls3!
{
  struct NotAFunction {}
}
```
**Expected**: Compilation error with message "Expected a function item"

#### 4.2 Non-Function Item (Const)
```rust
impls3!
{
  const NOT_A_FN: i32 = 42;
}
```
**Expected**: Compilation error with message "Expected a function item"

### 5. Integration Scenarios

#### 5.1 Multiple Functions with Various Features
```rust
impls3!
{
  /// First function
  #[inline]
  fn first<T: Clone>(x: T) -> T
  {
    x.clone()
  }

  ? async fn second() -> i32
  {
    42
  }

  const fn third() -> bool
  {
    true
  }

  ? fn fourth<'a>(s: &'a str) -> &'a str
  {
    s
  }
}
```
**Expected**: All functions compile successfully with features preserved

## Testing Status

| Category | Test Case | Status | Issues |
|----------|-----------|--------|--------|
| Function Variants | Lifetimes | ⏳ Pending | - |
| Function Variants | Where Clauses | ⏳ Pending | - |
| Function Variants | Async Functions | ⏳ Pending | - |
| Function Variants | Const Functions | ⏳ Pending | - |
| Function Variants | Unsafe Functions | ⏳ Pending | - |
| Attribute Handling | Inline Attribute | ⏳ Pending | - |
| Attribute Handling | Doc Comments | ⏳ Pending | - |
| Attribute Handling | Conditional Compilation | ⏳ Pending | - |
| Edge Cases | Empty Block | ⏳ Pending | - |
| Edge Cases | Mixed Optional/Required | ⏳ Pending | - |
| Edge Cases | Complex Parameter Types | ⏳ Pending | - |
| Edge Cases | impl Trait Return | ⏳ Pending | - |
| Edge Cases | Default Type Parameters | ⏳ Pending | - |
| Error Cases | Non-Function (Struct) | ⏳ Pending | - |
| Error Cases | Non-Function (Const) | ⏳ Pending | - |
| Integration | Multiple Mixed Features | ⏳ Pending | - |

## Execution Notes

- All tests should be executed as compilation tests
- Use `? fn` prefix for optional functions to avoid unused macro warnings
- Tests are primarily for this proc-macro crate; behavioral tests are in parent `impls_index` crate

## Known Issues

None currently known.
