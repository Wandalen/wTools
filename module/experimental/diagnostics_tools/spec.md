# Specification: diagnostics_tools

## Overview

**diagnostics_tools** is a comprehensive testing and debugging utilities crate providing enhanced runtime assertions with colorful diffs, compile-time validation macros, and memory layout verification. It serves as a drop-in replacement for standard Rust assertions with significantly better error messages, zero-cost compile-time checks, and low-level memory safety validation for workspace development and testing.

**Version:** 0.11.0
**Status:** Experimental
**Category:** Development Tools (Testing/Diagnostics)
**Dependents:** Unknown (likely all workspace crates for testing)

### Scope

#### Responsibility

Provide enhanced assertion macros for runtime testing with colorful diffs, compile-time validation for feature flags and configurations, and memory layout verification for low-level programming, serving as the workspace's primary diagnostic infrastructure for improved debugging and testing experience.

#### In-Scope

1. **Runtime Assertions (RTA)**
   - `a_true!(condition)` / `a_false!(condition)` - Boolean checks
   - `a_id!(left, right)` / `a_not_id!(left, right)` - Equality with diffs
   - Debug variants: `a_dbg_true!`, `a_dbg_false!`, `a_dbg_id!`, `a_dbg_not_id!`
   - Integration with `pretty_assertions` for colorful output
   - Custom error messages support

2. **Compile-Time Assertions (CTA)**
   - `cta_true!(condition)` - Meta condition validation
   - Feature flag validation at compile-time
   - Target platform verification
   - Zero runtime cost
   - compile_error! integration

3. **Memory Layout Validation**
   - `cta_type_same_size!(T1, T2)` - Type size comparison
   - `cta_type_same_align!(T1, T2)` - Alignment verification
   - `cta_ptr_same_size!(ptr1, ptr2)` - Pointer size checks
   - `cta_mem_same_size!(val1, val2)` - Value memory footprint
   - Const evaluation for zero cost

4. **Feature Architecture**
   - `enabled` - Master switch (default)
   - `diagnostics_runtime_assertions` - RTA macros (default)
   - `diagnostics_compiletime_assertions` - CTA macros (default)
   - `diagnostics_memory_layout` - Layout validation (default)
   - `no_std` / `use_alloc` - Embedded support

5. **Traditional Module Organization**
   - Nested `diag` module with submodules: rta, cta, layout
   - Standard namespaces: own, orphan, exposed, prelude
   - Feature-gated module exposure
   - Macro re-exports

6. **Enhanced Error Messages**
   - Colorful diff output via pretty_assertions
   - Line-by-line comparison for complex structures
   - Context-aware error formatting
   - Better panic messages

7. **Namespace Aliases**
   - `assert_eq` → `a_id` (orphan namespace)
   - `assert_ne` → `a_not_id` (orphan namespace)
   - Drop-in replacement capability

8. **Comprehensive Examples**
   - 6 numbered examples with progressive learning path
   - Real-world usage scenarios
   - Migration guides and documentation

#### Out-of-Scope

1. **NOT Procedural Macros**
   - Uses declarative macros only
   - No custom derive macros
   - **Rationale:** Simplicity and compile-time performance

2. **NOT Custom Test Framework**
   - No test runner replacement
   - Works with standard #[test]
   - **Rationale:** Complement existing tools, not replace

3. **NOT Benchmarking**
   - No performance measurement utilities
   - No timing facilities
   - **Rationale:** Use criterion or other benchmarking crates

4. **NOT Mocking**
   - No mock object generation
   - No stub creation
   - **Rationale:** Use dedicated mocking libraries

5. **NOT Property Testing**
   - No quickcheck/proptest integration
   - No randomized testing
   - **Rationale:** Use dedicated property testing crates

6. **NOT Snapshot Testing**
   - No snapshot generation/comparison
   - No golden file management
   - **Rationale:** Use insta or similar crates

7. **NOT Runtime Performance Profiling**
   - No CPU/memory profiling
   - No allocation tracking
   - **Rationale:** Use perf, valgrind, or similar tools

8. **NOT Custom Panic Handlers**
   - No panic hook modification
   - Uses standard panic mechanism
   - **Rationale:** Avoid interfering with user panic handling

#### Boundaries

- **diagnostics_tools vs assert!**: diagnostics_tools provides better error messages; assert! is standard
- **diagnostics_tools vs pretty_assertions**: diagnostics_tools wraps pretty_assertions with additional features
- **diagnostics_tools vs static_assertions**: diagnostics_tools provides runtime + compile-time; static_assertions is compile-time only

## Architecture

### Dependency Structure

```
diagnostics_tools (testing utilities)
├── Internal Dependencies
│   └── (none - foundational utility)
├── External Dependencies
│   └── pretty_assertions (optional, feature: diagnostics_runtime_assertions)
└── Dev Dependencies
    ├── test_tools (workspace, testing)
    ├── trybuild (crates.io, compile-fail tests)
    ├── strip-ansi-escapes (crates.io, test output parsing)
    └── serde_json (crates.io, test data)
```

**Note:** Single optional production dependency (pretty_assertions)

### Module Organization

```
diagnostics_tools
├── lib.rs (top-level aggregation)
├── diag/ (main diagnostic module)
│   ├── mod.rs - Module aggregation
│   ├── rta.rs - Runtime assertions (feature: diagnostics_runtime_assertions)
│   ├── cta.rs - Compile-time assertions (feature: diagnostics_compiletime_assertions)
│   └── layout.rs - Memory layout (feature: diagnostics_memory_layout)
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Traditional namespace organization with feature-gated submodules

### Feature Architecture

```
enabled (master switch, default)
├── diagnostics_runtime_assertions (default)
│   ├── Enables pretty_assertions dependency
│   ├── a_true!, a_false!, a_id!, a_not_id!
│   └── a_dbg_true!, a_dbg_false!, a_dbg_id!, a_dbg_not_id!
│
├── diagnostics_compiletime_assertions (default)
│   └── cta_true! - Meta condition validation
│
├── diagnostics_memory_layout (default)
│   ├── cta_type_same_size!, cta_type_same_align!
│   └── cta_ptr_same_size!, cta_mem_same_size!
│
full (all features, same as default)
│
no_std (embedded support)
└── use_alloc (allocation in no_std)
```

**Default Features:** `enabled`, `diagnostics_runtime_assertions`, `diagnostics_compiletime_assertions`, `diagnostics_memory_layout`

### Assertion Flow

#### Runtime Assertion Flow

```
a_id!(left, right)
  ↓
pretty_assertions::assert_eq!(left, right)
  ↓
Compare values
  ├─ Equal → Continue execution
  └─ Not Equal → Format diff
      ↓
      Display colorful diff
      ↓
      panic!()
```

#### Compile-Time Assertion Flow

```
cta_true!(condition)
  ↓
#[cfg(not(condition))]
  ↓
compile_error!("Does not hold: condition")
  ↓
Compilation fails with clear message
```

#### Memory Layout Validation Flow

```
cta_type_same_size!(T1, T2)
  ↓
const _: fn() = || {
  let _: [(); size_of::<T1>()] = [(); size_of::<T2>()];
};
  ↓
Const evaluation
  ├─ Same size → Compile succeeds
  └─ Different size → Compilation fails
```

## Public API

### Runtime Assertion Macros

```rust
/// Assert boolean is true
#[macro_export]
macro_rules! a_true {
  () => {};
  ($($Rest: tt)*) => { assert!($($Rest)*); };
}

/// Assert boolean is false
#[macro_export]
macro_rules! a_false {
  () => {};
  ($($Rest: tt)*) => { assert!(! $($Rest)*); };
}

/// Assert equality with diff (uses pretty_assertions)
#[macro_export]
macro_rules! a_id {
  ($left: expr, $right: expr $(,)?) => {
    $crate::dependency::pretty_assertions::assert_eq!($left, $right);
  };
  ($left: expr, $right: expr, $($arg: tt)*) => {
    $crate::dependency::pretty_assertions::assert_eq!($left, $right, $($arg)+);
  };
}

/// Assert inequality with diff
#[macro_export]
macro_rules! a_not_id {
  ($left: expr, $right: expr $(,)?) => {
    $crate::dependency::pretty_assertions::assert_ne!($left, $right);
  };
  ($left: expr, $right: expr, $($arg: tt)*) => {
    $crate::dependency::pretty_assertions::assert_ne!($left, $right, $($arg)+);
  };
}
```

### Debug Assertion Macros

```rust
/// Debug version of a_true! (only in debug builds)
#[macro_export]
macro_rules! a_dbg_true {
  () => {};
  ($($Rest: tt)*) => { debug_assert!($($Rest)*); };
}

/// Debug version of a_false!
#[macro_export]
macro_rules! a_dbg_false {
  () => {};
  ($($Rest: tt)*) => { debug_assert!(! $($Rest)*); };
}

/// Debug version of a_id! (calls a_id! if debug_assertions)
#[macro_export]
macro_rules! a_dbg_id {
  ($($arg: tt)*) => {
    if cfg!(debug_assertions) {
      $crate::a_id!($($arg)*);
    }
  };
}

/// Debug version of a_not_id!
#[macro_export]
macro_rules! a_dbg_not_id {
  ($($arg: tt)*) => {
    if cfg!(debug_assertions) {
      $crate::a_not_id!($($arg)*);
    }
  };
}
```

### Compile-Time Assertion Macros

```rust
/// Compile-time boolean assertion
#[macro_export]
macro_rules! cta_true {
  () => {};
  ($($Cond: meta)+, $Msg: expr $(,)?) => {
    #[cfg(not($($Cond)+))]
    core::compile_error!($Msg);
  };
  ($($Cond: tt)*) => {
    #[cfg(not($($Cond)*))]
    core::compile_error!(
      concat!(
        "Does not hold: \n  ",
        stringify!($($Cond)*),
      )
    );
  };
}
```

### Memory Layout Macros

```rust
/// Assert two types have same size
#[macro_export]
macro_rules! cta_type_same_size {
  ($Type1: ty, $Type2: ty $(,)?) => {{
    const _: fn() = || {
      let _: [(); core::mem::size_of::<$Type1>()] =
             [(); core::mem::size_of::<$Type2>()];
    };
    true
  }};
}

/// Assert two types have same alignment
#[macro_export]
macro_rules! cta_type_same_align {
  ($Type1: ty, $Type2: ty $(,)?) => {{
    const _: fn() = || {
      let _: [(); core::mem::align_of::<$Type1>()] =
             [(); core::mem::align_of::<$Type2>()];
    };
    true
  }};
}

/// Assert two pointers reference same-sized memory
#[macro_export]
macro_rules! cta_ptr_same_size {
  ($Ins1: expr, $Ins2: expr $(,)?) => {{
    #[allow(unsafe_code, unknown_lints, forget_copy, forget_non_drop, useless_transmute)]
    let _ = || unsafe {
      let mut ins1 = core::ptr::read($Ins1);
      core::ptr::write(&mut ins1, core::mem::transmute(core::ptr::read($Ins2)));
      core::mem::forget(ins1);
    };
    true
  }};
}

/// Assert two values have same memory size
#[macro_export]
macro_rules! cta_mem_same_size {
  ($Ins1: expr, $Ins2: expr $(,)?) => {{
    $crate::cta_ptr_same_size!(&$Ins1, &$Ins2)
  }};
}
```

### Namespace Aliases

```rust
// In orphan namespace:
pub use a_id as assert_eq;
pub use a_not_id as assert_ne;
```

## Usage Patterns

### Pattern 1: Basic Runtime Assertions

```rust
use diagnostics_tools::*;

fn main() {
  let number = 42;

  // Boolean assertions
  a_true!(number > 0, "Expected positive number");
  a_false!(number < 0, "Number should not be negative");

  // Works without custom messages too
  a_true!(number % 2 == 0);
}
```

### Pattern 2: Equality Assertions with Diffs

```rust
use diagnostics_tools::*;

fn test_vectors() {
  let expected = vec![1, 2, 3];
  let actual = vec![1, 2, 4];

  // This will show a beautiful colorful diff:
  // [
  //     1,
  //     2,
  // <   3,
  // >   4,
  // ]
  a_id!(expected, actual);
}
```

### Pattern 3: Debug-Only Assertions

```rust
use diagnostics_tools::*;

fn expensive_validation(data: &Vec<i32>) {
  // Only runs in debug builds
  a_dbg_true!(data.len() < 1000, "Debug build: data too large");
  a_dbg_id!(data.first(), &Some(&42));
}
```

### Pattern 4: Compile-Time Feature Validation

```rust
use diagnostics_tools::*;

// Ensure we're on 64-bit platform
cta_true!(target_pointer_width = "64");

// Validate feature flags
cta_true!(feature = "enabled");

// Check OS compatibility
cta_true!(any(
  target_os = "linux",
  target_os = "windows",
  target_os = "macos"
));
```

### Pattern 5: Memory Layout Validation

```rust
use diagnostics_tools::*;

// Ensure types have same size (for unsafe transmute)
fn validate_layout() {
  // Compile-time check
  cta_type_same_size!(u32, i32);
  cta_type_same_align!(u32, i32);

  // Now safe to transmute
  let unsigned: u32 = 42;
  let signed: i32 = unsafe { std::mem::transmute(unsigned) };
}
```

### Pattern 6: Drop-In Replacement for std::assert

```rust
use diagnostics_tools::*;

// Before: Standard assertions
assert_eq!(vec![1, 2], vec![1, 3]); // Cryptic error

// After: Enhanced assertions
a_id!(vec![1, 2], vec![1, 3]); // Beautiful diff

// Or use namespace alias:
use diagnostics_tools::orphan::*;
assert_eq!(vec![1, 2], vec![1, 3]); // Now uses a_id!
```

### Pattern 7: Type Size Assumptions

```rust
use diagnostics_tools::*;

// Document and verify size assumptions
fn optimized_storage() {
  // Ensure Option<&T> has same size as *const T (null optimization)
  cta_type_same_size!(Option<&u32>, *const u32);

  // Verify enum layout assumptions
  cta_type_same_size!(Result<(), ()>, bool);
}
```

### Pattern 8: Platform-Specific Validation

```rust
use diagnostics_tools::*;

#[cfg(target_os = "linux")]
fn linux_specific() {
  // Ensure we're actually on Linux
  cta_true!(target_os = "linux");

  // Verify architecture
  cta_true!(target_arch = "x86_64");
}
```

## Dependencies and Consumers

### Direct Dependencies

**External:**
- `pretty_assertions` (optional, feature: `diagnostics_runtime_assertions`) - Colorful assertion diffs

**Dev:**
- `test_tools` (workspace) - Testing utilities
- `trybuild` (crates.io) - Compile-fail tests
- `strip-ansi-escapes` (crates.io) - ANSI code stripping for tests
- `serde_json` (crates.io) - Test data handling

### Consumers (Unknown)

**Likely used by:**
- All workspace crates for testing
- Development utilities
- Integration test suites
- Documentation examples

**Usage Pattern:** Workspace crates use diagnostics_tools as primary assertion library for enhanced error messages and compile-time validation, replacing standard assert! macros.

## Design Rationale

### Why Wrap Standard Assertions?

Uses macros wrapping assert! and pretty_assertions:

**Benefits:**
1. **Drop-In Replacement**: Minimal code changes
2. **Better Errors**: Colorful diffs for complex structures
3. **Consistent API**: Same interface as standard assertions
4. **Zero Cost**: Compiles to same code

**Tradeoff:** Extra dependency (pretty_assertions) for better UX

### Why Three Feature Categories?

Separate features for runtime, compile-time, and layout:

**Rationale:**
1. **Granular Control**: Enable only what you need
2. **Dependency Management**: pretty_assertions only when needed
3. **Compile-Time Options**: Some users only want CTA
4. **Flexibility**: Can disable runtime checks in release

**Default:** All three enabled for full functionality

### Why Debug Variants?

`a_dbg_*` macros for debug-only assertions:

**Rationale:**
1. **Performance**: Expensive checks only in debug builds
2. **Development**: Catch bugs during development
3. **Release Optimization**: Zero cost in release builds
4. **Standard Practice**: Mirrors std::debug_assert!

**Pattern:** Same as Rust's debug_assert! / assert! split

### Why Declarative Macros Only?

No procedural macros, only macro_rules!:

**Benefits:**
1. **Fast Compilation**: No proc-macro overhead
2. **Simplicity**: Easy to understand and debug
3. **Portability**: Works everywhere Rust works
4. **No Dependencies**: No syn/quote/proc-macro2

**Tradeoff:** Less powerful than proc macros, but sufficient

### Why Memory Layout Macros?

`cta_type_same_size!` and similar:

**Rationale:**
1. **Safety**: Validate unsafe code assumptions
2. **Documentation**: Make requirements explicit
3. **Compile-Time**: Zero runtime cost
4. **Const Evaluation**: Fails at compile time if wrong

**Use Case:** Systems programming, FFI, optimization

### Why Namespace Aliases?

`assert_eq` → `a_id` in orphan namespace:

**Benefits:**
1. **Migration Path**: Easier to switch from std
2. **Drop-In Replacement**: Import orphan namespace
3. **Compatibility**: Works with existing code patterns

**Mechanism:** Re-export with different names

### Why No Custom Test Framework?

Doesn't replace #[test]:

**Rationale:**
1. **Compatibility**: Works with existing test infrastructure
2. **Simplicity**: No test runner complexity
3. **Flexibility**: Use with any test framework
4. **Focus**: Better assertions, not test orchestration

**Benefit:** Complement existing tools

### Why pretty_assertions Dependency?

Uses external crate for colorful output:

**Rationale:**
1. **Proven Solution**: Well-tested library
2. **Maintenance**: Don't reinvent wheel
3. **Features**: Comprehensive diff formatting
4. **Optional**: Can disable with features

**Tradeoff:** External dependency for better UX

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- Integration tests with other workspace crates

### Test Files

```
tests/
├── trybuild.rs - Compile-fail tests (harness = false)
├── runtime_assertion_tests.rs - RTA macro tests
└── trybuild/ - Compile-fail test cases
    └── *.rs - Expected compilation failures
```

### Test Focus

1. **Runtime Assertions**: Verify macros expand correctly, error messages
2. **Compile-Time Assertions**: Use trybuild for compile-fail tests
3. **Memory Layout**: Verify size/align checks work
4. **Debug Variants**: Test debug_assertions cfg logic
5. **Feature Combinations**: Test with different feature sets

### Compile-Fail Testing

Uses `trybuild` to verify compile-time assertions fail correctly:

```rust
#[test]
fn test_compile_failures() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/trybuild/*.rs");
}
```

### Known Test Limitations

1. **ANSI Output**: Tests must strip ANSI codes from pretty_assertions
2. **Compile-Fail Fragility**: trybuild tests depend on exact error messages
3. **Platform-Specific**: Some CTA tests only work on specific platforms

## Future Considerations

### Potential Enhancements

1. **More CTA Macros**: `cta_false!`, `cta_id!`, etc.
2. **Custom Diff Formatting**: Configurable diff styles
3. **Integration with test_tools**: Enhanced test utilities
4. **Snapshot Testing**: Optional snapshot comparison
5. **Better Error Context**: More information in panics
6. **Performance Assertions**: Runtime performance checks
7. **Async Assertions**: Special handling for async code

### Breaking Changes to Consider

1. **Rename Macros**: Remove underscore prefix (e.g., `a_id` → `aid`)
2. **Return Values**: Make assertions return Result instead of panic
3. **Proc Macros**: Add derive macros for custom assertions
4. **Default Features**: Change default feature set

### Known Limitations

1. **No Async Support**: Assertions are synchronous only
2. **No Custom Formatters**: Fixed diff format from pretty_assertions
3. **Macro Hygiene**: Some edge cases with macro expansion
4. **Error Messages**: Limited customization of panic messages
5. **No Soft Assertions**: All assertions panic on failure

## Adoption Guidelines

### When to Use diagnostics_tools

**Good Candidates:**
- All test code for better error messages
- Development assertions for debugging
- Compile-time platform validation
- Memory layout verification for unsafe code
- Feature flag validation

**Poor Candidates:**
- Production error handling (use Result)
- Performance-critical paths (minimal overhead but still some)
- no_std without alloc (pretty_assertions needs formatting)

### Migration from std::assert

```rust
// Before: Standard assertions
assert!(condition);
assert_eq!(left, right);
assert_ne!(left, right);

// After: Enhanced assertions
use diagnostics_tools::*;
a_true!(condition);
a_id!(left, right);
a_not_id!(left, right);

// Or use namespace alias for drop-in replacement:
use diagnostics_tools::orphan::*;
// Now assert_eq! and assert_ne! use enhanced versions
```

### Best Practices

1. **Always Use in Tests**: Better error messages save debugging time
2. **Compile-Time First**: Use CTA when possible for zero cost
3. **Document Assumptions**: Use CTA to make requirements explicit
4. **Custom Messages**: Add context to runtime assertions
5. **Debug Variants**: Use for expensive checks
6. **Validate Early**: Add assertions at function entry points

## Related Crates

- **pretty_assertions**: Colorful assertion diffs (dependency)
- **static_assertions**: Compile-time assertions only
- **trybuild**: Compile-fail testing (dev dependency)
- **assert_fs**: Filesystem assertion utilities
- **claim**: Alternative assertion macros
- **test_tools**: wTools testing utilities (workspace)

## References

- [API Documentation](https://docs.rs/diagnostics_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/diagnostics_tools)
- [readme.md](./readme.md)
- [pretty_assertions](https://docs.rs/pretty_assertions)
- [trybuild](https://docs.rs/trybuild)
- [Examples](./examples/)
  - [001_basic_runtime_assertions.rs](./examples/001_basic_runtime_assertions.rs)
  - [002_better_error_messages.rs](./examples/002_better_error_messages.rs)
  - [003_compile_time_checks.rs](./examples/003_compile_time_checks.rs)
  - [004_memory_layout_validation.rs](./examples/004_memory_layout_validation.rs)
  - [005_debug_variants.rs](./examples/005_debug_variants.rs)
  - [006_real_world_usage.rs](./examples/006_real_world_usage.rs)
