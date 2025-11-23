# Extract Config Extraction Functions to unilang_parser

**Date**: 2025-11-19
**Completed**: 2025-11-22
**Priority**: MEDIUM - Reduces Code Duplication
**Category**: API Enhancement
**Status**: Completed
**Source**: wplan_agent/src/config.rs, wplan_client/src/config.rs
**Actual Location**: unilang/src/config_extraction.rs (moved from unilang_parser due to serde_json dependency)

**⚠️ CRITICAL**: This task is INCOMPLETE without follow-up adoption. Task will be CANCELED if adoption not implemented.

**Follow-up Adoption Required:**
- [wplan_client/002](../../../../../willbe/module/wplan_client/task/002_adopt_config_extraction_from_unilang.md) - Replace local config extraction with unilang
- [wplan_agent/001](../../../../../willbe/module/wplan_agent/task/001_adopt_config_extraction_from_unilang.md) - Replace local config extraction with unilang

---

## Executive Summary

Extract the config value extraction functions from wplan crates to `unilang_parser` as part of the `CliParamsAdvanced` ecosystem. These utilities extract typed values from `HashMap<String, (JsonValue, ConfigSource)>` config structures, which is the standard format for config_hierarchy integration.

---

## Problem Statement

### Current Duplication

**wplan_agent/src/config.rs**:
```rust
pub fn extract_u8( config : &HashMap< String, ( JsonValue, ConfigSource ) >, key : &str ) -> Option< u8 >
pub fn extract_bool( config : &HashMap< String, ( JsonValue, ConfigSource ) >, key : &str ) -> Option< bool >
pub fn extract_string( config : &HashMap< String, ( JsonValue, ConfigSource ) >, key : &str ) -> Option< String >
```

**wplan_client/src/config.rs**:
```rust
// Same functions duplicated
pub fn extract_u8( config : &..., key : &str ) -> Option< u8 >
pub fn extract_bool( config : &..., key : &str ) -> Option< bool >
pub fn extract_string( config : &..., key : &str ) -> Option< String >
```

### Why Extract

1. **Exact Duplication**: Same code in two places
2. **Couples with CliParamsAdvanced**: Used in `apply_defaults()` to get config values
3. **General Utility**: Any CLI using config_hierarchy needs these
4. **Type Safety**: Centralized extraction means consistent error handling

---

## Proposed Solution

### Target Location

```
unilang_parser/src/config/
  mod.rs           # Module exports
  extraction.rs    # Value extraction functions
```

### API Design

```rust
//! Config value extraction utilities.
//!
//! Extracts typed values from config_hierarchy's `HashMap<String, (JsonValue, ConfigSource)>`.
//!
//! # Example
//!
//! ```rust
//! use std::collections::HashMap;
//! use serde_json::json;
//! use config_hierarchy::ConfigSource;
//! use unilang_parser::config::extraction::{ extract_u8, extract_bool, extract_string };
//!
//! let mut config = HashMap::new();
//! config.insert( "verbosity".to_string(), ( json!( 4 ), ConfigSource::Default ) );
//! config.insert( "debug".to_string(), ( json!( true ), ConfigSource::File ) );
//! config.insert( "name".to_string(), ( json!( "test" ), ConfigSource::Env ) );
//!
//! assert_eq!( extract_u8( &config, "verbosity" ), Some( 4 ) );
//! assert_eq!( extract_bool( &config, "debug" ), Some( true ) );
//! assert_eq!( extract_string( &config, "name" ), Some( "test".to_string() ) );
//! ```

use std::collections::HashMap;
use serde_json::Value as JsonValue;
use config_hierarchy::ConfigSource;

/// Type alias for config map.
pub type ConfigMap = HashMap< String, ( JsonValue, ConfigSource ) >;

/// Extract u8 value from config.
#[ inline ]
pub fn extract_u8( config : &ConfigMap, key : &str ) -> Option< u8 >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_u64() )
    .and_then( | n | u8::try_from( n ).ok() )
}

/// Extract u16 value from config.
#[ inline ]
pub fn extract_u16( config : &ConfigMap, key : &str ) -> Option< u16 >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_u64() )
    .and_then( | n | u16::try_from( n ).ok() )
}

/// Extract u32 value from config.
#[ inline ]
pub fn extract_u32( config : &ConfigMap, key : &str ) -> Option< u32 >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_u64() )
    .and_then( | n | u32::try_from( n ).ok() )
}

/// Extract u64 value from config.
#[ inline ]
pub fn extract_u64( config : &ConfigMap, key : &str ) -> Option< u64 >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_u64() )
}

/// Extract i32 value from config.
#[ inline ]
pub fn extract_i32( config : &ConfigMap, key : &str ) -> Option< i32 >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_i64() )
    .and_then( | n | i32::try_from( n ).ok() )
}

/// Extract i64 value from config.
#[ inline ]
pub fn extract_i64( config : &ConfigMap, key : &str ) -> Option< i64 >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_i64() )
}

/// Extract bool value from config.
#[ inline ]
pub fn extract_bool( config : &ConfigMap, key : &str ) -> Option< bool >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_bool() )
}

/// Extract String value from config.
#[ inline ]
pub fn extract_string( config : &ConfigMap, key : &str ) -> Option< String >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_str() )
    .map( | s | s.to_string() )
}

/// Extract f64 value from config.
#[ inline ]
pub fn extract_f64( config : &ConfigMap, key : &str ) -> Option< f64 >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_f64() )
}

/// Extract array of strings from config.
#[ inline ]
pub fn extract_string_array( config : &ConfigMap, key : &str ) -> Option< Vec< String > >
{
  config.get( key )
    .and_then( | ( v, _ ) | v.as_array() )
    .map( | arr | arr.iter().filter_map( | v | v.as_str().map( String::from ) ).collect() )
}

/// Get the config source for a key.
#[ inline ]
pub fn get_source( config : &ConfigMap, key : &str ) -> Option< ConfigSource >
{
  config.get( key ).map( | ( _, source ) | source.clone() )
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use serde_json::json;

  fn make_config() -> ConfigMap
  {
    let mut config = HashMap::new();
    config.insert( "u8_val".to_string(), ( json!( 42 ), ConfigSource::Default ) );
    config.insert( "bool_val".to_string(), ( json!( true ), ConfigSource::File ) );
    config.insert( "string_val".to_string(), ( json!( "hello" ), ConfigSource::Env ) );
    config.insert( "float_val".to_string(), ( json!( 3.14 ), ConfigSource::Cli ) );
    config.insert( "array_val".to_string(), ( json!( [ "a", "b", "c" ] ), ConfigSource::Default ) );
    config
  }

  #[ test ]
  fn extract_u8_value()
  {
    let config = make_config();
    assert_eq!( extract_u8( &config, "u8_val" ), Some( 42 ) );
    assert_eq!( extract_u8( &config, "missing" ), None );
  }

  #[ test ]
  fn extract_bool_value()
  {
    let config = make_config();
    assert_eq!( extract_bool( &config, "bool_val" ), Some( true ) );
    assert_eq!( extract_bool( &config, "missing" ), None );
  }

  #[ test ]
  fn extract_string_value()
  {
    let config = make_config();
    assert_eq!( extract_string( &config, "string_val" ), Some( "hello".to_string() ) );
    assert_eq!( extract_string( &config, "missing" ), None );
  }

  #[ test ]
  fn extract_f64_value()
  {
    let config = make_config();
    assert_eq!( extract_f64( &config, "float_val" ), Some( 3.14 ) );
  }

  #[ test ]
  fn extract_string_array_value()
  {
    let config = make_config();
    assert_eq!(
      extract_string_array( &config, "array_val" ),
      Some( vec![ "a".to_string(), "b".to_string(), "c".to_string() ] )
    );
  }

  #[ test ]
  fn get_source_value()
  {
    let config = make_config();
    assert_eq!( get_source( &config, "bool_val" ), Some( ConfigSource::File ) );
    assert_eq!( get_source( &config, "string_val" ), Some( ConfigSource::Env ) );
  }
}
```

### Re-export from unilang_parser

```rust
// In unilang_parser/src/lib.rs
pub mod config;
pub use config::extraction::{ ConfigMap, extract_u8, extract_bool, extract_string };
```

### Re-export from unilang

Already exports unilang_parser as `parser`, so accessible as:
```rust
use unilang::parser::config::extraction::{ extract_u8, extract_bool, extract_string };
```

---

## Implementation Phases

### Phase 1: Implement in unilang_parser (1.5 hours)

1. Create `unilang_parser/src/config/mod.rs`
2. Create `unilang_parser/src/config/extraction.rs`
3. Implement all extraction functions with comprehensive type coverage
4. Add tests for all functions
5. Re-export from lib.rs

### Phase 2: Update wplan_agent (30 minutes)

1. Replace imports: `use unilang::parser::config::extraction::*`
2. Delete local `extract_*` functions from `wplan_agent/src/config.rs`
3. Verify all tests pass

### Phase 3: Update wplan_client (30 minutes)

1. Replace imports: `use unilang::parser::config::extraction::*`
2. Delete local `extract_*` functions from `wplan_client/src/config.rs`
3. Verify all tests pass

---

## Dependencies

```toml
# unilang_parser/Cargo.toml
[dependencies]
serde_json = { workspace = true }
config_hierarchy = { workspace = true }
```

---

## Expected Impact

| Metric | Before | After |
|--------|--------|-------|
| Duplicated extraction functions | 2 copies | 1 source |
| Type coverage | u8, bool, string | u8-u64, i32-i64, bool, string, f64, array |
| ConfigSource access | Not exposed | `get_source()` available |
| Type safety | Per-project | Centralized |

---

## Acceptance Criteria

- [x] `unilang/src/config_extraction.rs` implements all extraction functions (changed from unilang_parser)
- [x] `ConfigMap<S>` generic type alias exported for convenience (generic over source type)
- [x] All numeric types covered (u8, u16, u32, u64, i32, i64, f64)
- [ ] `get_source()` function to access ConfigSource (not needed - source accessible via tuple)
- [ ] All wplan_agent tests pass with new import (pending adoption task)
- [ ] All wplan_client tests pass with new import (pending adoption task)
- [x] `cargo test -p unilang` passes (792 tests, 33 new for config_extraction)
- [ ] No duplication remains in wplan crates (pending adoption tasks)

---

## Verification

```bash
cargo test -p unilang_parser -- config
cargo test -p wplan_agent
cargo test -p wplan_client
```

---

## Notes

- Consider making `ConfigMap` a newtype with methods instead of functions
- Could add `extract_or_default()` variants for convenience
- May want to add validation/range checking helpers

---

## Priority

**MEDIUM** - Reduces duplication but not blocking any features.

## Estimated Effort

2.5 hours total.

---

## RESOLUTION - 2025-11-22

**Status**: ✅ COMPLETED

### Implementation Summary

Task 090 was implemented using Test-Driven Development (TDD) approach.

**Critical Decision**: Target changed from `unilang_parser` to `unilang` because `unilang_parser` lacks `serde_json` dependency, while `unilang` already has it via the `json_parser` feature.

#### Files Created

1. **`src/config_extraction.rs`** (~230 lines) - Core implementation:
   - `ConfigMap<S>` generic type alias for `HashMap<String, (JsonValue, S)>`
   - 10 extraction functions:
     - `extract_u8`, `extract_u16`, `extract_u32`, `extract_u64`
     - `extract_i32`, `extract_i64`
     - `extract_f64`
     - `extract_bool`
     - `extract_string`
     - `extract_string_array`
   - All functions generic over source type `S` (avoids config_hierarchy dependency)
   - Comprehensive doc comments with examples

2. **`tests/config_extraction.rs`** (33 tests) - Comprehensive test coverage:
   - Basic extraction for all types
   - Type mismatches return None
   - Overflow handling (u8 > 255 → None)
   - Null handling (null → None for strings)
   - Empty string handling
   - Generic source type verification
   - Missing key handling

#### Key Design Decisions

1. **Generic Source Type**: `ConfigMap<S>` instead of concrete `ConfigSource` type
   - Avoids requiring `config_hierarchy` dependency
   - Works with any source-tracking type: `()`, `String`, `ConfigSource`, etc.
   - Source accessible via tuple: `config.get(key).map(|(_, source)| source)`

2. **Feature-Gated**: Requires `json_parser` feature
   ```rust
   #[cfg(feature = "json_parser")]
   layer config_extraction;
   ```

3. **Null Handling**: `extract_string` returns `None` for JSON null values (not empty string)

4. **Overflow Safety**: Numeric extraction returns `None` if value exceeds target type range

#### Wiring

```rust
// lib.rs
#[cfg(feature = "json_parser")]
layer config_extraction;
```

### Validation Results

- **33 tests pass** in `tests/config_extraction.rs`
- **792 total tests pass** across unilang crate
- **Clippy clean** - no warnings
- **Doc tests pass**

### Deviation from Original Design

| Aspect | Original Proposal | Actual Implementation |
|--------|-------------------|----------------------|
| Location | `unilang_parser/src/config/` | `unilang/src/config_extraction.rs` |
| ConfigMap type | `HashMap<String, (JsonValue, ConfigSource)>` | `ConfigMap<S> = HashMap<String, (JsonValue, S)>` |
| get_source() | Standalone function | Not needed - tuple access sufficient |
| Feature gate | None | `json_parser` feature required |

### Pending

- **Adoption tasks**: `wplan_client` and `wplan_agent` need to replace local implementations

---

**Resolution Date**: 2025-11-22
**Test Coverage**: 33 new tests
**Files Created**: 2 (config_extraction.rs, tests/config_extraction.rs)
