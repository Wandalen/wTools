# Extract Verbosity Logging to unilang

**Date**: 2025-11-19
**Priority**: LOW - Nice to Have
**Category**: API Enhancement
**Status**: Rejected
**Rejection Date**: 2025-11-22
**Source**: wplan_client/src/cli/verbosity.rs

## Rejection Rationale

**Decision**: REJECTED in favor of `tracing` crate ecosystem.

**Reasons**:
1. **Industry Standard**: `tracing` is the de-facto Rust logging/instrumentation standard
2. **Ecosystem Integration**: Works with tokio, async runtimes, OpenTelemetry
3. **Structured Logging**: Supports spans, fields, and structured output
4. **Filtering**: `tracing-subscriber` provides sophisticated level filtering
5. **Maintenance**: Custom logging adds tech debt without clear benefit
6. **Scope**: Thread-local verbosity macros are niche compared to `tracing`

**Recommended Alternative**:
```rust
// Instead of custom vprintln!(level, ...), use:
use tracing::{info, debug, trace, warn, error};

// Map verbosity levels to tracing:
// 0=Silent → ERROR only
// 1=Quiet  → WARN+
// 2=Normal → INFO+ (default)
// 3=Verbose → DEBUG+
// 4-5=Trace → TRACE

// Configure with tracing-subscriber:
tracing_subscriber::fmt()
  .with_max_level(tracing::Level::DEBUG)
  .init();
```

**Migration Path for wplan_client**:
1. Add `tracing` and `tracing-subscriber` dependencies
2. Replace `vprintln!(N, ...)` with appropriate `tracing` macro
3. Configure subscriber based on verbosity parameter
4. Delete `src/cli/verbosity.rs`

---

## Original Proposal (Archived)

Extract the verbosity-based logging utilities from `wplan_client` to `unilang` as a standard CLI logging solution. This provides thread-local verbosity control with convenience macros, suitable for any CLI application that needs verbosity levels.

---

## Problem Statement

### Current Location

**File**: `wplan_client/src/cli/verbosity.rs`
**Functionality**:
- Thread-local verbosity level storage
- `set_verbosity()` / `get_verbosity()` functions
- `vprint!()` / `vprintln!()` macros for conditional output
- Level-based filtering (0-5 typically)

### Why Extract

1. **Common Pattern**: Verbosity levels are standard in CLI tools
2. **Couples with CliParamsAdvanced**: Often parsed via `verbosity` or `v` parameter
3. **Thread Safety**: Thread-local storage is tricky to get right
4. **Consistency**: Standard macros across unilang-based CLIs

---

## Proposed Solution

### Target Location

```
unilang/src/logging/
  mod.rs           # Module exports
  verbosity.rs     # Verbosity system
```

### API Design

```rust
//! Verbosity-based logging for CLI applications.
//!
//! Provides thread-local verbosity control with convenience macros.
//!
//! # Example
//!
//! ```rust
//! use unilang::logging::verbosity::{ set_verbosity, get_verbosity };
//! use unilang::{ vprint, vprintln };
//!
//! set_verbosity( 2 );
//!
//! // Will print (level 1 <= verbosity 2)
//! vprintln!( 1, "Info message" );
//!
//! // Will NOT print (level 3 > verbosity 2)
//! vprintln!( 3, "Debug message" );
//!
//! // Format arguments work
//! vprintln!( 2, "Value: {}", 42 );
//! ```

use std::cell::Cell;

thread_local!
{
  static VERBOSITY : Cell< u8 > = const { Cell::new( 2 ) };
}

/// Set the verbosity level for the current thread.
///
/// Level meanings (convention):
/// - 0: Silent (errors only)
/// - 1: Quiet (important info)
/// - 2: Normal (default)
/// - 3: Verbose (detailed info)
/// - 4: Debug (debug info)
/// - 5: Trace (all output)
#[ inline ]
pub fn set_verbosity( level : u8 )
{
  VERBOSITY.with( | v | v.set( level ) );
}

/// Get the current verbosity level for the current thread.
#[ inline ]
pub fn get_verbosity() -> u8
{
  VERBOSITY.with( | v | v.get() )
}

/// Check if a message at the given level should be printed.
#[ inline ]
pub fn should_print( level : u8 ) -> bool
{
  level <= get_verbosity()
}

/// Print a message if the verbosity level allows.
#[ macro_export ]
macro_rules! vprint
{
  ( $level : expr, $( $arg : tt )* ) =>
  {
    if $crate::logging::verbosity::should_print( $level )
    {
      print!( $( $arg )* );
    }
  };
}

/// Print a message with newline if the verbosity level allows.
#[ macro_export ]
macro_rules! vprintln
{
  ( $level : expr ) =>
  {
    if $crate::logging::verbosity::should_print( $level )
    {
      println!();
    }
  };
  ( $level : expr, $( $arg : tt )* ) =>
  {
    if $crate::logging::verbosity::should_print( $level )
    {
      println!( $( $arg )* );
    }
  };
}

/// Print to stderr if the verbosity level allows.
#[ macro_export ]
macro_rules! veprint
{
  ( $level : expr, $( $arg : tt )* ) =>
  {
    if $crate::logging::verbosity::should_print( $level )
    {
      eprint!( $( $arg )* );
    }
  };
}

/// Print to stderr with newline if the verbosity level allows.
#[ macro_export ]
macro_rules! veprintln
{
  ( $level : expr ) =>
  {
    if $crate::logging::verbosity::should_print( $level )
    {
      eprintln!();
    }
  };
  ( $level : expr, $( $arg : tt )* ) =>
  {
    if $crate::logging::verbosity::should_print( $level )
    {
      eprintln!( $( $arg )* );
    }
  };
}

/// Verbosity level constants.
pub mod levels
{
  /// Silent - only errors.
  pub const SILENT : u8 = 0;
  /// Quiet - important info only.
  pub const QUIET : u8 = 1;
  /// Normal - default level.
  pub const NORMAL : u8 = 2;
  /// Verbose - detailed information.
  pub const VERBOSE : u8 = 3;
  /// Debug - debug information.
  pub const DEBUG : u8 = 4;
  /// Trace - all output.
  pub const TRACE : u8 = 5;
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn default_verbosity()
  {
    // Reset to default for this test
    set_verbosity( 2 );
    assert_eq!( get_verbosity(), 2 );
  }

  #[ test ]
  fn set_and_get_verbosity()
  {
    set_verbosity( 4 );
    assert_eq!( get_verbosity(), 4 );
    set_verbosity( 0 );
    assert_eq!( get_verbosity(), 0 );
  }

  #[ test ]
  fn should_print_levels()
  {
    set_verbosity( 2 );

    assert!( should_print( 0 ) );  // 0 <= 2
    assert!( should_print( 1 ) );  // 1 <= 2
    assert!( should_print( 2 ) );  // 2 <= 2
    assert!( !should_print( 3 ) ); // 3 > 2
    assert!( !should_print( 4 ) ); // 4 > 2
    assert!( !should_print( 5 ) ); // 5 > 2
  }

  #[ test ]
  fn level_constants()
  {
    assert_eq!( levels::SILENT, 0 );
    assert_eq!( levels::QUIET, 1 );
    assert_eq!( levels::NORMAL, 2 );
    assert_eq!( levels::VERBOSE, 3 );
    assert_eq!( levels::DEBUG, 4 );
    assert_eq!( levels::TRACE, 5 );
  }
}
```

### Re-export from unilang

```rust
// In unilang/src/lib.rs
pub mod logging;
pub use logging::verbosity::{ set_verbosity, get_verbosity, should_print };

// Macros are auto-exported via #[macro_export]
```

---

## Implementation Phases

### Phase 1: Implement in unilang (1.5 hours)

1. Create `unilang/src/logging/mod.rs`
2. Create `unilang/src/logging/verbosity.rs`
3. Implement thread-local storage and macros
4. Add level constants
5. Add tests
6. Re-export from lib.rs

### Phase 2: Update wplan_client (30 minutes)

1. Replace imports: `use unilang::{ vprint, vprintln, set_verbosity, get_verbosity }`
2. Delete `wplan_client/src/cli/verbosity.rs`
3. Update module structure
4. Verify all tests pass

### Phase 3: Documentation (30 minutes)

1. Add module-level documentation with examples
2. Document level conventions
3. Add usage patterns for common scenarios

---

## Dependencies

No additional dependencies required.

---

## Expected Impact

| Metric | Before | After |
|--------|--------|-------|
| wplan_client verbosity.rs LOC | ~80 | 0 (deleted) |
| Macro availability | Per-project | Shared |
| Level conventions | Informal | Documented constants |
| Thread safety | Per-project | Centralized |

---

## Acceptance Criteria

- [ ] `unilang/src/logging/verbosity.rs` implements all functionality
- [ ] Thread-local storage works correctly
- [ ] `vprint!`, `vprintln!`, `veprint!`, `veprintln!` macros exported
- [ ] Level constants documented and exported
- [ ] All wplan_client tests pass with new imports
- [ ] `cargo test -p unilang` passes
- [ ] `cargo test -p wplan_client` passes

---

## Verification

```bash
cargo test -p unilang -- logging
cargo test -p wplan_client
```

---

## Notes

- Consider integration with `tracing` or `log` crates as alternative
- May want to add colored output support (e.g., warnings in yellow)
- Could add `vdbg!` macro similar to `dbg!` but verbosity-aware

---

## Priority

**LOW** - Functional but not blocking. Alternative: use `tracing` crate.

## Estimated Effort

2.5 hours total.
