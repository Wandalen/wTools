# Specification: time_tools

## Overview

**time_tools** is a minimal no_std-compatible crate providing simple functions for obtaining current time from UNIX epoch in multiple units (seconds, milliseconds, nanoseconds). It wraps std::time::SystemTime with ergonomic module-based APIs for different time granularities, serving as a lightweight time utility foundation for workspace tools.

**Version:** 0.2.0
**Status:** Production
**Category:** Utilities (Time/Date)
**Dependents:** Unknown (likely build tools, logging utilities)

### Scope

#### Responsibility

Provide simple, zero-dependency functions for obtaining current time from UNIX epoch in seconds, milliseconds, and nanoseconds, with optional no_std support for embedded environments.

#### In-Scope

1. **Time Acquisition Functions**
   - `now::now()` - Get current time in milliseconds (default)
   - `now::s::now()` - Get current time in seconds
   - `now::ms::now()` - Get current time in milliseconds (explicit)
   - `now::ns::now()` - Get current time in nanoseconds

2. **UNIX Epoch Reference**
   - All times measured from UNIX epoch (January 1, 1970, 00:00:00 UTC)
   - Returns i64 timestamps
   - Handles time conversions internally

3. **Module Organization**
   - `now` module for time functions (feature-gated: `time_now`)
   - Sub-modules for different units: `s`, `ms`, `ns`
   - Consistent `now()` function name across all modules

4. **no_std Compatibility**
   - `#![no_std]` support via feature flag
   - Optional `use_alloc` feature
   - Std-dependent functions behind `not(feature = "no_std")` gates

5. **Traditional Namespace Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Not using mod_interface! (utility crate)
   - Module re-exports via exposed namespace

6. **Feature Architecture**
   - `enabled` - Master switch
   - `time_now` - Enable time functions (default)
   - `no_std` - Embedded support
   - `use_alloc` - Allocation support in no_std
   - `chrono` - Reserved for future chrono integration
   - `time_chrono` - Reserved for future chrono integration

7. **Zero Production Dependencies**
   - Uses only std::time or core
   - No external time libraries required
   - Minimal implementation

8. **Simple API Surface**
   - Four functions total (one per unit + default)
   - All functions return i64
   - No configuration or state

#### Out-of-Scope

1. **NOT Date/Time Formatting**
   - Does not format timestamps to human-readable strings
   - Does not parse date strings
   - **Rationale:** Simple timestamp utility only, use chrono for formatting

2. **NOT Timezone Handling**
   - All times are UTC (UNIX epoch)
   - No timezone conversion
   - No local time support
   - **Rationale:** Keeps implementation simple and portable

3. **NOT Duration Calculations**
   - Does not provide duration types
   - Does not calculate time differences
   - **Rationale:** Users can subtract timestamps manually

4. **NOT High-Precision Timing**
   - Not suitable for benchmarking
   - Not monotonic clock
   - System time can jump backwards
   - **Rationale:** Use std::time::Instant for high-precision timing

5. **NOT Date Arithmetic**
   - No adding days/months/years
   - No calendar operations
   - **Rationale:** Use chrono or time crate for date arithmetic

6. **NOT Timestamp Validation**
   - Does not validate timestamp ranges
   - Does not check for overflow
   - **Rationale:** Trusts system time

7. **NOT Future/Past Time**
   - Only provides current time
   - No time travel functions
   - **Rationale:** Focused scope

8. **NOT Chrono Integration (Yet)**
   - `chrono` and `time_chrono` features exist but unused
   - Reserved for future integration
   - **Rationale:** Not yet implemented

#### Boundaries

- **time_tools vs std::time**: time_tools provides simpler API with multiple units; std::time is lower-level
- **time_tools vs chrono**: time_tools is minimal current-time utility; chrono is full date/time library
- **time_tools vs Instant**: time_tools uses SystemTime (wall clock); Instant is monotonic for benchmarking

## Architecture

### Dependency Structure

```
time_tools (utilities, zero dependencies)
├── Internal Dependencies
│   └── (none - foundational utility)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** Zero production dependencies for maximum portability.

### Module Organization

```
time_tools
├── lib.rs (traditional namespaces)
├── now.rs (time functions module, feature-gated)
│   ├── now() - Milliseconds (default)
│   ├── s::now() - Seconds
│   ├── ms::now() - Milliseconds (explicit)
│   └── ns::now() - Nanoseconds
└── Standard namespaces: own, orphan, exposed, prelude
```

**Pattern:** Traditional namespace organization, not mod_interface! (utility crate convention)

### Feature Architecture

```
enabled (master switch)
├── time_now (default, enable time functions)
│   ├── now() - Default milliseconds
│   ├── s::now() - Seconds
│   ├── ms::now() - Milliseconds
│   └── ns::now() - Nanoseconds
│
no_std (embedded support)
└── use_alloc (allocation in no_std)
│
full (all features: enabled + use_alloc + time_now)
│
chrono (reserved, not implemented)
time_chrono (reserved, not implemented)
```

**Default Features:** `enabled`, `time_now`

### Time Unit Relationships

```
UNIX Epoch (1970-01-01 00:00:00 UTC)
    ↓
SystemTime::now()
    ↓
.duration_since(UNIX_EPOCH)
    ↓
├─ .as_secs() → s::now() (seconds)
├─ .as_millis() → now() / ms::now() (milliseconds)
└─ .as_nanos() → ns::now() (nanoseconds)

Conversion:
  1 second = 1,000 milliseconds
  1 second = 1,000,000,000 nanoseconds
  now_ms == now_ns / 1,000,000
  now_ms / 1,000 == now_s
```

## Public API

### Functions

```rust
/// Get current time in milliseconds (default)
#[cfg(not(feature = "no_std"))]
pub fn now() -> i64

/// Seconds module
pub mod s {
  /// Get current time in seconds
  #[cfg(not(feature = "no_std"))]
  pub fn now() -> i64
}

/// Milliseconds module (explicit)
pub mod ms {
  /// Get current time in milliseconds
  #[cfg(not(feature = "no_std"))]
  pub fn now() -> i64
}

/// Nanoseconds module
pub mod ns {
  /// Get current time in nanoseconds
  #[cfg(not(feature = "no_std"))]
  pub fn now() -> i64
}
```

### Return Type

All functions return `i64` representing time since UNIX epoch in their respective units.

## Usage Patterns

### Pattern 1: Get Current Time (Default Units)

```rust
use time_tools::*;

// Default: milliseconds
let now_ms = now::now();
println!("Current time: {} ms since epoch", now_ms);
```

### Pattern 2: All Time Units

```rust
use time_tools::*;

// Milliseconds (default)
let now_ms = now::now();
println!("Milliseconds: {}", now_ms);

// Seconds
let now_s = now::s::now();
println!("Seconds: {}", now_s);

// Nanoseconds
let now_ns = now::ns::now();
println!("Nanoseconds: {}", now_ns);
```

### Pattern 3: Unit Conversion Verification

```rust
use time_tools::*;

let now_ms = now::now();
let now_ns = now::ns::now();
let now_s = now::s::now();

// Verify conversions (with tolerance for execution time)
assert_eq!(now_ms, now_ns / 1_000_000);
assert_eq!(now_ms / 1000, now_s);
```

### Pattern 4: Measuring Elapsed Time

```rust
use time_tools::*;

let start = now::now();

// Do some work...
std::thread::sleep(std::time::Duration::from_millis(100));

let end = now::now();
let elapsed_ms = end - start;

println!("Operation took {} ms", elapsed_ms);
```

### Pattern 5: High-Precision Timing

```rust
use time_tools::*;

let start_ns = now::ns::now();

// Precise operation...

let end_ns = now::ns::now();
let elapsed_ns = end_ns - start_ns;

println!("Operation took {} nanoseconds", elapsed_ns);
```

### Pattern 6: Logging with Timestamps

```rust
use time_tools::*;

fn log_event(message: &str) {
  let timestamp = now::now();
  println!("[{}] {}", timestamp, message);
}

log_event("Application started");
log_event("Processing data");
log_event("Application finished");
```

### Pattern 7: Rate Limiting

```rust
use time_tools::*;

struct RateLimiter {
  last_call: i64,
  min_interval_ms: i64,
}

impl RateLimiter {
  fn new(min_interval_ms: i64) -> Self {
    Self {
      last_call: 0,
      min_interval_ms,
    }
  }

  fn can_proceed(&mut self) -> bool {
    let now = now::now();
    if now - self.last_call >= self.min_interval_ms {
      self.last_call = now;
      true
    } else {
      false
    }
  }
}
```

### Pattern 8: Timeout Checking

```rust
use time_tools::*;

fn wait_with_timeout(timeout_ms: i64) -> bool {
  let start = now::now();

  loop {
    // Check condition...
    let condition_met = false; // Your logic here

    if condition_met {
      return true;
    }

    if now::now() - start > timeout_ms {
      return false; // Timeout
    }

    std::thread::sleep(std::time::Duration::from_millis(10));
  }
}
```

## Dependencies and Consumers

### Direct Dependencies

**Production:** (none - zero dependencies)

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- Build automation tools (willbe, etc.)
- Logging utilities
- Performance monitoring
- Test utilities

**Usage Pattern:** Workspace tools use time_tools for simple timestamp acquisition in logs, performance measurements, and timeout implementations.

## Design Rationale

### Why Module-Based Units?

Uses modules (s, ms, ns) instead of function suffixes:

```rust
now::s::now()   // ✓ Module-based
now_s()         // ✗ Function suffix
```

**Benefits:**
1. **Consistent API**: All use `now()` function name
2. **Namespacing**: Units are namespaced, not global
3. **Clarity**: `now::ns::now()` is self-documenting
4. **Ergonomics**: Can import specific module: `use time_tools::now::s;`

**Tradeoff:** Slightly more verbose for simplicity and clarity

### Why i64, Not u64?

Returns signed `i64` instead of unsigned `u64`:

**Rationale:**
1. **Arithmetic**: Allows negative results when subtracting timestamps
2. **Compatibility**: Many APIs expect signed integers
3. **Range**: i64::MAX in nanoseconds is ~292 years (sufficient)

**Tradeoff:** Half the positive range for signed arithmetic convenience

### Why Default to Milliseconds?

`now::now()` returns milliseconds by default:

**Rationale:**
1. **Common Case**: Most applications use millisecond precision
2. **Balance**: Not too coarse (seconds), not too fine (nanoseconds)
3. **Compatibility**: Common in web/network protocols

**Alternatives Available:** Use `s::now()` or `ns::now()` for other units

### Why No std::time Re-exports?

Doesn't re-export std::time types like Duration or Instant:

**Rationale:**
1. **Simplicity**: Minimal API surface
2. **Focused Scope**: Only current time acquisition
3. **No Confusion**: Users import std::time directly when needed

**Benefit:** Clear separation of concerns

### Why No Formatting?

Doesn't format timestamps to strings:

**Rationale:**
1. **Dependencies**: Would require chrono or time crate
2. **Scope Creep**: Formatting is complex (locales, timezones)
3. **User Choice**: Let users choose their formatting library

**Workaround:** Use chrono or time crate for formatting

### Why Reserved chrono Features?

Has `chrono` and `time_chrono` features that do nothing:

**Rationale:**
1. **Future Integration**: Placeholder for future chrono support
2. **API Stability**: Won't break existing feature combinations
3. **Planning**: Indicates intended direction

**Current State:** Not yet implemented

### Why no_std Support?

Supports `#![no_std]` via feature flag:

**Rationale:**
1. **Embedded**: Works in embedded environments (when std available)
2. **Portability**: Maximum compatibility
3. **Flexibility**: Users can opt out of std

**Limitation:** Time functions require std::time, so no_std mode disables them (no alternative implementation)

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- No circular dependency issues

### Test Files

```
tests/
├── smoke_test.rs - Basic functionality
└── time_tests.rs - Comprehensive tests
```

### Test Focus

1. **Basic Functionality**: Verify all functions return values
2. **Unit Conversions**: Verify ms/ns/s relationships
3. **Non-Regression**: Ensure time progresses forward
4. **Consistency**: Multiple calls return increasing or equal values

### Known Test Limitations

1. **System Time Dependency**: Tests rely on system clock
2. **Non-Deterministic**: Cannot test exact values (time changes)
3. **No Backwards Check**: System time can jump backwards (NTP, DST)

## Future Considerations

### Potential Enhancements

1. **Chrono Integration**: Implement `chrono` and `time_chrono` features
2. **Formatting Functions**: Add simple timestamp formatting
3. **Monotonic Clock**: Add Instant-based monotonic timing
4. **Duration Helpers**: Add duration calculation utilities
5. **no_std Clock**: Alternative time source for embedded (requires platform support)

### Breaking Changes to Consider

1. **Return Type**: Change to u64 (lose signed arithmetic)
2. **Module Naming**: Shorter names (sec, milli, nano)
3. **Error Handling**: Return Result instead of unwrapping

### Known Limitations

1. **System Time Only**: Not monotonic, can jump backwards
2. **No Formatting**: Cannot convert to human-readable strings
3. **No Timezone**: Always UTC, no local time
4. **no_std Incomplete**: Time functions still require std
5. **Truncation Risk**: i64 as nanoseconds overflows ~292 years

## Adoption Guidelines

### When to Use time_tools

**Good Candidates:**
- Simple timestamp logging
- Basic elapsed time measurement
- Rate limiting implementations
- Timeout checking
- Cache expiration tracking
- Simple performance monitoring

**Poor Candidates:**
- High-precision benchmarking (use std::time::Instant)
- Date arithmetic (use chrono/time crate)
- Timezone conversions (use chrono/time crate)
- Formatted date display (use chrono/time crate)
- Monotonic timing requirements (use std::time::Instant)

### Migration from std::time

```rust
// Before: Using std::time directly
use std::time::{SystemTime, UNIX_EPOCH};

let now_ms = SystemTime::now()
  .duration_since(UNIX_EPOCH)
  .unwrap()
  .as_millis() as i64;

// After: Using time_tools
use time_tools::*;

let now_ms = now::now();
```

### Best Practices

1. **Choose Right Unit**: Use seconds for coarse timing, nanoseconds for precision
2. **Not for Benchmarking**: Use std::time::Instant for benchmarks (monotonic)
3. **Handle Backwards Time**: System time can go backwards (NTP adjustments)
4. **Cache When Possible**: Don't call `now()` in tight loops if precision not needed
5. **Consider Overflow**: i64 nanoseconds overflow in ~292 years

## Related Crates

- **std::time**: Standard library time facilities (lower-level)
- **chrono**: Full-featured date/time library (more features)
- **time**: Alternative date/time library (more features)
- **instant**: Cross-platform Instant (WASM support)

## Development History

### Task 001 Blocking and Foundation-First Approach (2025-11-22)

**Context**: Original task 001 proposed "extracting" time formatting utilities from wplan_client to time_tools.

**Critical Issues Discovered**:

1. **Source Code Mismatch**: Task described chrono-based formatting code that didn't exist. Actual wplan_client code used manual std::time calculations.

2. **Dependency Assumption Errors**: Task claimed chrono dependency existed in both time_tools and wplan_client. Neither crate had chrono dependency.

3. **Effort Underestimation**: Task estimated 4.5h for "extraction" but actual scope required 15-20h for NEW development (not extraction).

4. **Missing Prerequisites**: No spec.md existed, violating specification-centric development principle (CLAUDE.md requirement).

5. **Scope Misrepresentation**: Task framed as "extraction" (implying code exists) when reality was new development from scratch.

**Resolution**: Task 001 blocked. Foundation-first approach adopted.

**Foundation Phase Requirements** (tasks 002-006):
1. Task 002: Create specification (this document)
2. Task 003: Document API in readme.md
3. Task 004: Add comprehensive edge case tests (>90% coverage)
4. Task 005: Document panic conditions, improve panic messages
5. Task 006: Infrastructure cleanup (.gitignore, version decision)

**Lessons Learned**:

- **Always verify code exists** before planning extraction tasks. Don't assume based on descriptions.
- **Distinguish extraction vs new development**. Extraction = code exists and needs moving. New development = code must be written from scratch.
- **Specification is mandatory** before development (CLAUDE.md compliance). Cannot validate work without spec as source of truth.
- **Accurate effort estimation** requires understanding actual scope. Extracting existing code (hours) vs writing new code (days) are vastly different.
- **Factual accuracy matters** in task descriptions. Incorrect assumptions cascade into blocked work.

### Foundation Phase Completion (2025-11-22)

**Status**: All foundation tasks (002-006) completed successfully.

**Completed Work**:
1. Task 002: Created comprehensive specification (this document)
2. Task 003: Documented API in readme.md with examples, panic conditions, performance characteristics
3. Task 004: Added edge case tests (9 integration tests + 11 doc tests)
4. Task 005: Documented panic conditions, improved panic messages in source
5. Task 006: Infrastructure cleanup, version recommendation (v0.2.1)

**Test Coverage**: 20 tests total (9 integration + 11 doc tests), all passing

**Path Decision**: Deferred chrono integration (tasks 007-010). time_tools remains minimal timestamp utility. Formatting functionality stays in consuming crates until explicit need arises.

**Rationale for Defer**:
- No immediate business requirement for formatting in time_tools
- Consuming crates (wplan, etc.) can implement formatting locally
- Keeps time_tools focused and dependency-free
- chrono integration can be revisited when explicit need arises

---

## References

- [API Documentation](https://docs.rs/time_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/time_tools)
- [Example](./examples/time_tools_trivial/src/main.rs)
- [readme.md](./readme.md)
- [std::time::SystemTime](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
- [UNIX Epoch](https://en.wikipedia.org/wiki/Unix_time)
