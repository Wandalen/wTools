# Extract Time Formatting Utilities from wplan

**Date**: 2025-11-20
**Priority**: HIGH (originally - reduced due to blocker)
**Category**: API Enhancement - Code Extraction
**Status**: ⛔️ (Blocked)
**Source**: wplan_client/src/cli/formatting.rs
**Task ID**: 001
**Advisability**: 432 (CORRECTED - was 1800, see blocker analysis)

**⛔️ BLOCKED**: This task contains critical factual errors and cannot be implemented as specified.

**Blocker Rationale**: See `-task_001_blocker_analysis.md` for complete analysis. Key issues:
1. Source code mismatch - described code doesn't exist (claims chrono usage, actual uses std::time)
2. Dependency assumption errors - chrono dependency doesn't exist in time_tools or wplan_client
3. Effort estimate mismatch - 4.5h claimed vs 15-20h realistic (NEW development, not extraction)
4. Missing prerequisites - no spec.md exists (violates specification-centric development)
5. Scope misrepresentation - NOT extraction, IS new development

**Resolution Path**: Complete foundation work (tasks 002-005) first, then redefine as new development task with accurate scope. See `-comprehensive_redefinition_plan.md` for detailed roadmap.

**Foundation Tasks Required FIRST**:
- [002](./002_create_specification.md) - Create time_tools specification (3-4h, CRITICAL)
- [003](./003_document_api.md) - Document current API (1-2h, HIGH)
- [004](./004_add_edge_case_tests.md) - Add edge case tests (2-3h, HIGH)
- [005](./005_improve_error_handling.md) - Improve error handling docs (1h, MEDIUM)

---

**⚠️ ORIGINAL TASK DESCRIPTION BELOW (CONTAINS ERRORS)**

**⚠️ CRITICAL**: This task is INCOMPLETE without follow-up adoption. Task will be CANCELED if adoption not implemented.

**Follow-up Adoption Required:**
- [wplan_client/005](../../../../../willbe/module/wplan_client/task/005_adopt_time_formatting_from_time_tools.md) - Replace local time formatting with time_tools

---

## Executive Summary

Extract general-purpose time and duration formatting utilities from the wplan ecosystem to `time_tools`, making them available to all wTools projects. These utilities handle duration formatting (ms/s/m/h/d), Unix timestamp formatting, and duration parsing - all broadly useful for CLI applications, logging, and monitoring tools.

---

## Problem Statement

### Current Location

The wplan codebase contains several time-related utilities that would benefit all wTools projects:

**wplan_client/src/cli/formatting.rs**:
- Lines 203-219: `format_duration_ms()` - Formats milliseconds as human-readable duration ("1.5s", "2m 30s")
- Lines 433-451: `format_duration()` - Formats `std::time::Duration` to human string
- Lines 224-253: `format_timestamp()` - Formats Unix timestamps as "YYYY-MM-DD HH:MM:SS"

### Why Extract to time_tools

1. **General Utility**: Time formatting is needed by every CLI tool with timing/logging
2. **Consistency**: Provides standard time formatting across wTools ecosystem
3. **Extensibility**: Centralized location for time-related utilities
4. **Dependencies**: time_tools already depends on `chrono`, perfect fit
5. **Code Reuse**: willbe, benchkit, unitore, wtest all need time formatting

---

## Detailed Functionality Analysis

### 1. Duration Formatting from Milliseconds

**Current Location**: `wplan_client/src/cli/formatting.rs:203-219`

```rust
pub fn format_duration_ms( duration_ms : u64 ) -> String
{
  if duration_ms < 1000
  {
    format!( "{}ms", duration_ms )
  }
  else if duration_ms < 60_000
  {
    format!( "{:.1}s", duration_ms as f64 / 1000.0 )
  }
  else if duration_ms < 3_600_000
  {
    let minutes = duration_ms / 60_000;
    let seconds = ( duration_ms % 60_000 ) / 1000;
    format!( "{}m {}s", minutes, seconds )
  }
  // ... continues for hours, days
}
```

**Features**:
- Adaptive precision based on magnitude
- Natural units selection (ms, s, m, h, d)
- Human-readable output
- No unnecessary precision (2m 30s vs 2.5m)

**Use Cases**:
- Test execution time
- Benchmark results
- Command duration in logs
- Timeout displays

### 2. Duration Formatting from std::time::Duration

**Current Location**: `wplan_client/src/cli/formatting.rs:433-451`

```rust
pub fn format_duration( duration : Duration ) -> String
{
  let total_secs = duration.as_secs();
  if total_secs < 60
  {
    format!( "{:.1}s", duration.as_secs_f64() )
  }
  else if total_secs < 3600
  {
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;
    format!( "{}m {}s", minutes, seconds )
  }
  // ... continues
}
```

**Features**:
- Works with standard library `Duration` type
- Same adaptive formatting as `format_duration_ms()`
- Integrates with std::time APIs

### 3. Unix Timestamp Formatting

**Current Location**: `wplan_client/src/cli/formatting.rs:224-253`

```rust
pub fn format_timestamp( timestamp : i64 ) -> String
{
  if let Some( datetime ) = chrono::DateTime::from_timestamp( timestamp, 0 )
  {
    let local : chrono::DateTime< chrono::Local > = datetime.into();
    local.format( "%Y-%m-%d %H:%M:%S" ).to_string()
  }
  else
  {
    "Invalid timestamp".to_string()
  }
}
```

**Features**:
- Converts Unix timestamps to local time
- Standard ISO-like format (YYYY-MM-DD HH:MM:SS)
- Handles invalid timestamps gracefully
- Uses chrono for timezone conversion

**Use Cases**:
- Log timestamps
- File modification times
- Event timestamps
- Build times

---

## Proposed API Design

### Target Location

```
time_tools/src/format/
  mod.rs           # Module exports
  duration.rs      # Duration formatting
  timestamp.rs     # Timestamp formatting
  parse.rs         # Duration parsing (future)
```

### API Structure

```rust
//! Time formatting utilities for time_tools
//!
//! Provides human-readable formatting for:
//! - Duration (from milliseconds or std::time::Duration)
//! - Unix timestamps (with timezone support)
//! - Relative time (future)
//! - Time ranges (future)

// ============================================================================
// duration.rs - Duration Formatting
// ============================================================================

use std::time::Duration;

/// Duration formatting options.
#[ derive( Debug, Clone ) ]
pub struct DurationFormatOptions
{
  /// Maximum number of units to show (e.g., 2 = "1h 30m", omit seconds).
  pub max_units : usize,
  /// Show fractional seconds for durations <1s.
  pub fractional_seconds : bool,
  /// Compact format ("1h30m" vs "1h 30m").
  pub compact : bool,
}

impl Default for DurationFormatOptions
{
  fn default() -> Self
  {
    Self
    {
      max_units : 2,
      fractional_seconds : true,
      compact : false,
    }
  }
}

/// Format milliseconds as human-readable duration.
///
/// Automatically selects appropriate units (ms, s, m, h, d) based on magnitude.
///
/// # Example
///
/// ```rust
/// use time_tools::format::format_duration_ms;
///
/// assert_eq!( format_duration_ms( 500 ), "500ms" );
/// assert_eq!( format_duration_ms( 1500 ), "1.5s" );
/// assert_eq!( format_duration_ms( 90_000 ), "1m 30s" );
/// assert_eq!( format_duration_ms( 3_600_000 ), "1h 0m" );
/// assert_eq!( format_duration_ms( 86_400_000 ), "1d 0h" );
/// ```
pub fn format_duration_ms( ms : u64 ) -> String;

/// Format duration with custom options.
///
/// # Example
///
/// ```rust
/// use time_tools::format::{ format_duration_ms_with, DurationFormatOptions };
///
/// let opts = DurationFormatOptions
/// {
///   max_units : 1,
///   compact : true,
///   ..Default::default()
/// };
///
/// assert_eq!( format_duration_ms_with( 90_000, &opts ), "1m" );  // Omit seconds
/// ```
pub fn format_duration_ms_with( ms : u64, options : &DurationFormatOptions ) -> String;

/// Format std::time::Duration as human-readable string.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
/// use time_tools::format::format_duration;
///
/// let d = Duration::from_secs( 90 );
/// assert_eq!( format_duration( d ), "1m 30s" );
/// ```
pub fn format_duration( duration : Duration ) -> String
{
  format_duration_ms( duration.as_millis() as u64 )
}

/// Format duration with custom options.
pub fn format_duration_with( duration : Duration, options : &DurationFormatOptions ) -> String
{
  format_duration_ms_with( duration.as_millis() as u64, options )
}

// ============================================================================
// timestamp.rs - Timestamp Formatting
// ============================================================================

use chrono::{ DateTime, Local, Utc, TimeZone };

/// Timestamp formatting options.
#[ derive( Debug, Clone ) ]
pub struct TimestampFormatOptions
{
  /// Format string (chrono format syntax).
  pub format : String,
  /// Use UTC instead of local timezone.
  pub utc : bool,
}

impl Default for TimestampFormatOptions
{
  fn default() -> Self
  {
    Self
    {
      format : "%Y-%m-%d %H:%M:%S".to_string(),
      utc : false,
    }
  }
}

/// Format Unix timestamp (seconds since epoch) as local time string.
///
/// # Example
///
/// ```rust
/// use time_tools::format::format_timestamp;
///
/// // 2025-01-15 14:30:00 UTC
/// let ts = 1737821400;
/// let formatted = format_timestamp( ts );
/// // Result depends on local timezone, e.g., "2025-01-15 09:30:00" (EST)
/// ```
pub fn format_timestamp( timestamp : i64 ) -> String;

/// Format Unix timestamp with custom options.
///
/// # Example
///
/// ```rust
/// use time_tools::format::{ format_timestamp_with, TimestampFormatOptions };
///
/// let opts = TimestampFormatOptions
/// {
///   format : "%Y-%m-%d".to_string(),
///   utc : true,
///   ..Default::default()
/// };
///
/// let ts = 1737821400;
/// assert_eq!( format_timestamp_with( ts, &opts ), "2025-01-15" );
/// ```
pub fn format_timestamp_with( timestamp : i64, options : &TimestampFormatOptions ) -> String;

/// Format Unix timestamp (milliseconds) as local time string.
pub fn format_timestamp_ms( timestamp_ms : i64 ) -> String
{
  format_timestamp( timestamp_ms / 1000 )
}

/// Format DateTime as string with default format.
pub fn format_datetime< Tz : TimeZone >( dt : &DateTime< Tz > ) -> String
where
  Tz::Offset : std::fmt::Display
{
  dt.format( "%Y-%m-%d %H:%M:%S" ).to_string()
}

// ============================================================================
// parse.rs - Duration Parsing (Future Extension)
// ============================================================================

/// Parse human-readable duration string to milliseconds.
///
/// Supports formats like:
/// - "500ms"
/// - "1.5s"
/// - "2m 30s"
/// - "1h 15m"
/// - "1d 2h"
///
/// # Example
///
/// ```rust
/// use time_tools::format::parse_duration_ms;
///
/// assert_eq!( parse_duration_ms( "500ms" )?, 500 );
/// assert_eq!( parse_duration_ms( "1.5s" )?, 1500 );
/// assert_eq!( parse_duration_ms( "2m 30s" )?, 150_000 );
/// ```
pub fn parse_duration_ms( s : &str ) -> Result< u64, ParseDurationError >;

/// Parse to std::time::Duration.
pub fn parse_duration( s : &str ) -> Result< Duration, ParseDurationError >
{
  Ok( Duration::from_millis( parse_duration_ms( s )? ) )
}
```

---

## Implementation Phases

### Phase 1: Duration Formatting (2 hours)

**Tasks**:
1. Create `time_tools/src/format/duration.rs`
2. Implement `format_duration_ms()` with all unit ranges
3. Implement `DurationFormatOptions` and `format_duration_ms_with()`
4. Implement `format_duration()` wrapper for `std::time::Duration`
5. Add comprehensive tests
6. Document edge cases (0 duration, huge durations)

**Acceptance Criteria**:
- [ ] All unit ranges handled (ms, s, m, h, d)
- [ ] Adaptive precision works correctly
- [ ] Options allow customization (max_units, compact)
- [ ] std::time::Duration integration works
- [ ] Edge cases tested (0, overflow, fractional)
- [ ] Documentation includes examples

### Phase 2: Timestamp Formatting (1.5 hours)

**Tasks**:
1. Create `time_tools/src/format/timestamp.rs`
2. Implement `format_timestamp()` with chrono integration
3. Implement `TimestampFormatOptions` and `format_timestamp_with()`
4. Implement `format_timestamp_ms()` for millisecond timestamps
5. Implement `format_datetime()` for chrono DateTime
6. Add tests for timezone conversion
7. Document format string syntax

**Acceptance Criteria**:
- [ ] Unix timestamps formatted correctly
- [ ] Local timezone conversion works
- [ ] UTC option works
- [ ] Custom format strings supported
- [ ] Invalid timestamps handled gracefully
- [ ] Tests cover timezone edge cases

### Phase 3: Duration Parsing (3 hours - Future)

**Tasks**:
1. Create `time_tools/src/format/parse.rs`
2. Implement parser for duration strings
3. Support all format variants (compact, spaced, fractional)
4. Add comprehensive error handling
5. Round-trip testing (parse → format → parse)

**Acceptance Criteria**:
- [ ] All format variants parseable
- [ ] Errors provide helpful messages
- [ ] Round-trip preserves values
- [ ] Fuzzing tests for malformed input

### Phase 4: Integration and Migration (1 hour)

**Tasks**:
1. Update `time_tools/src/lib.rs` to export format module
2. Migrate wplan_client to use new API
3. Delete old implementations from wplan_client
4. Verify all wplan tests pass
5. Update documentation

**Acceptance Criteria**:
- [ ] All functions exported from `time_tools::format`
- [ ] wplan_client uses `time_tools::format::*`
- [ ] Old code deleted from wplan_client
- [ ] All wplan tests pass
- [ ] Documentation complete

---

## Dependencies

```toml
# time_tools/Cargo.toml
[dependencies]
chrono = { workspace = true }  # Already present
```

No new dependencies required - time_tools already uses chrono.

---

## Expected Impact

| Metric | Before | After |
|--------|--------|-------|
| wplan_client formatting.rs LOC | ~100 | ~10 (imports only) |
| Code duplication | Isolated | Shared across wTools |
| Time formatting consistency | Per-project | Standardized |
| Test coverage | Per-project | Centralized |

---

## Testing Strategy

### Unit Tests

**Duration Formatting**:
- All unit ranges (ms, s, m, h, d)
- Adaptive precision logic
- Edge cases (0, huge values, fractional)
- Options (max_units, compact, fractional_seconds)
- std::time::Duration integration

**Timestamp Formatting**:
- Unix timestamp conversion
- Timezone handling (local vs UTC)
- Custom format strings
- Invalid timestamps
- Edge cases (epoch, negative, overflow)

### Integration Tests

- wplan_client migration tests
- Round-trip parsing tests (when implemented)
- Performance benchmarks

---

## Performance Considerations

**Duration Formatting**:
- O(1) complexity (fixed unit ranges)
- Minimal allocations (single String)
- Target: <500ns per format

**Timestamp Formatting**:
- O(1) complexity
- chrono handles timezone conversion
- Target: <2µs per format

---

## Documentation Requirements

Each module must include:
1. Module-level documentation with overview
2. Function documentation with examples
3. Format string syntax documentation (timestamps)
4. Timezone behavior explanation
5. Edge case handling

---

## Acceptance Criteria

- [ ] Duration formatting module complete
- [ ] Timestamp formatting module complete
- [ ] Comprehensive test coverage (>90%)
- [ ] All edge cases tested and handled
- [ ] Documentation complete with examples
- [ ] wplan_client successfully migrated
- [ ] Old wplan implementations deleted
- [ ] `cargo test -p time_tools` passes
- [ ] `cargo test -p wplan_client` passes

---

## References

**Source Files**:
- `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/formatting.rs:203-219` (format_duration_ms)
- `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/formatting.rs:433-451` (format_duration)
- `/home/user1/pro/lib/willbe/module/wplan_client/src/cli/formatting.rs:224-253` (format_timestamp)

**Related Projects**:
- benchkit - needs duration formatting for benchmark results
- wtest - needs duration formatting for test times
- willbe - needs duration formatting for build times
- unitore - needs timestamp formatting for feeds

**Dependencies**:
- chrono (workspace) - Already present in time_tools

---

## Estimated Effort

- Phase 1: 2 hours (duration formatting)
- Phase 2: 1.5 hours (timestamp formatting)
- Phase 3: 3 hours (duration parsing - future)
- Phase 4: 1 hour (migration)

**Total (Phases 1-2, 4)**: 4.5 hours
**With Phase 3**: 7.5 hours

---

## Priority Justification

**HIGH Priority** because:
1. **Universal Need**: Every CLI tool needs time formatting
2. **Consistency**: Standardizes time display across wTools
3. **Simple Migration**: Straightforward extraction with clear API
4. **Immediate Value**: Eliminates duplication in wplan immediately
5. **Foundation**: Other wTools projects can adopt immediately
6. **Low Risk**: time_tools already has chrono, no new dependencies
