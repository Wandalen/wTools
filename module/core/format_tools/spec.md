# Specification: format_tools

## Overview

**format_tools** is a comprehensive formatting and serialization crate providing flexible string conversion with fallback mechanisms, table formatting, text wrapping, and field formatting utilities. It integrates with reflect_tools for runtime type information and former for builder patterns, serving as the workspace's primary formatting infrastructure for CLI output, logging, and data serialization.

**Version:** 0.6.0
**Status:** Production
**Category:** Utilities (Formatting/Serialization)
**Dependents:** Unknown (likely CLI tools, build utilities, diagnostic tools)

### Scope

#### Responsibility

Provide comprehensive **pure data transformation** utilities for converting Rust values into formatted strings. This includes fallback-based string conversion, table formatting with multiple output styles, field extraction macros, and text manipulation tools for workspace CLI and diagnostic output.

**Core principle:** format_tools transforms data INTO strings. It does NOT interact with terminals, manage I/O state, or perform system operations.

#### In-Scope

1. **Fallback-Based String Conversion**
   - `to_string_with_fallback!` macro - Convert with primary and fallback formatters
   - `ToStringWithFallback` trait - Flexible string conversion
   - Support for Display → Debug → custom fallbacks
   - Wrapper types: `WithDebug`, `WithDisplay`, `WithRef`

2. **Table Formatting**
   - Multiple output formats: Table, Records, Keys
   - Customizable separators and ASCII styling (NOT terminal colors)
   - Header support
   - Column alignment
   - Raw vector-based table construction

3. **Output Format Styles**
   - **Table Format**: Traditional grid layout with separators
   - **Records Format**: Vertical record-by-record listing
   - **Keys Format**: Key-value pair output
   - Pluggable `TableOutputFormat` trait

4. **Field Formatting Macros**
   - `_field!` macro - Format struct fields with fallbacks
   - `_field_with_key!` macro - Format with custom key names
   - Path-based key extraction from expressions
   - Automatic key naming from field paths

5. **Text Manipulation**
   - Text wrapping utilities
   - Markdown math formatting
   - String utilities

6. **Integration with reflect_tools**
   - `Fields` trait for struct field iteration
   - `IteratorTrait` for reflection-based iteration
   - Runtime type information for formatting

7. **Wrapper Types**
   - Reference wrappers for different formatting strategies
   - `Maybe` adapters for optional formatting
   - Type-safe formatting dispatch

8. **Traditional Module Organization**
   - Nested `format` module structure
   - Standard namespaces: own, orphan, exposed, prelude
   - Re-exports from reflect_tools

9. **Feature Architecture**
   - `enabled` - Master switch (default)
   - Dependencies: reflect_tools, former, collection_tools

#### Out-of-Scope

1. **NOT Procedural Macros**
   - Uses declarative macros only
   - No custom derive for formatting
   - **Rationale:** Simplicity and compile-time performance

2. **NOT Custom Serialization Formats**
   - No JSON/YAML/TOML serialization
   - Focused on string/table formatting only
   - **Rationale:** Use serde for structured serialization

3. **NOT Color/ANSI Styling**
   - No terminal color support
   - Plain text output only
   - **Rationale:** Use dedicated terminal styling crates

4. **NOT Terminal I/O or State Management**
   - No terminal raw mode control
   - No terminal state management (RAII guards, etc.)
   - No cursor positioning or terminal detection
   - No interactive terminal features
   - **Rationale:** format_tools is pure data transformation (string → string), not system I/O. Terminal interaction requires platform-specific code, FFI, and system calls - fundamentally incompatible with formatting scope. Use dedicated terminal libraries (crossterm, termion) or create separate terminal_tools crate.
   - **See also:** Task 001 rejection (format_tools/task/001) for detailed architectural analysis

5. **NOT Binary Serialization**
   - Text-based formats only
   - No binary protocols
   - **Rationale:** Out of scope

6. **NOT Async Formatting**
   - Synchronous string generation only
   - No async writers
   - **Rationale:** Formatting is typically fast enough for sync

7. **NOT Localization**
   - English-only output
   - No internationalization support
   - **Rationale:** Workspace-internal tool

8. **NOT Streaming Output**
   - Buffers entire output before returning
   - No incremental formatting
   - **Rationale:** Simplicity over memory efficiency

9. **NOT Pretty Printing**
   - Basic formatting only
   - No syntax-aware pretty printing
   - **Rationale:** Use dedicated pretty-printing crates

#### Boundaries

- **format_tools vs std::fmt**: format_tools provides higher-level utilities; std::fmt is lower-level trait
- **format_tools vs serde**: format_tools is display formatting; serde is structured serialization
- **format_tools vs prettytable-rs**: format_tools is workspace-integrated; prettytable is standalone
- **format_tools vs terminal libraries (crossterm/termion)**: format_tools is string formatting (pure data); terminal libraries are system I/O (interactive control)
- **format_tools vs diagnostics_tools**: format_tools is general formatting; diagnostics_tools is testing/debugging assertions with colorful diffs

## Architecture

### Dependency Structure

```
format_tools (formatting utilities)
├── Internal Dependencies
│   ├── reflect_tools (workspace, type reflection)
│   ├── former (workspace, builder patterns)
│   └── collection_tools (workspace, collections)
└── Dev Dependencies
    ├── test_tools (workspace, testing)
    └── collection_tools (workspace, with constructors feature)
```

### Module Organization

```
format_tools
├── lib.rs (top-level aggregation)
├── format/ (main formatting module)
│   ├── to_string_with_fallback.rs - Fallback conversion
│   ├── to_string/ - ToString utilities
│   │   └── aref.rs - Reference adapters
│   ├── wrapper/ - Wrapper types
│   │   ├── aref.rs - Reference wrappers
│   │   └── maybe_as.rs - Optional wrappers
│   ├── output_format/ - Table output
│   │   ├── table.rs - Grid table format
│   │   ├── records.rs - Vertical records format
│   │   └── keys.rs - Key-value format
│   ├── as_table.rs - AsTable trait
│   ├── table.rs - Table utilities
│   ├── print.rs - Printing utilities
│   ├── filter.rs - Output filtering
│   ├── string.rs - String utilities
│   ├── text_wrap.rs - Text wrapping
│   └── md_math.rs - Markdown math
└── Standard namespaces: own, orphan, exposed, prelude
```

### Feature Architecture

```
enabled (master switch, default)
├── Enables reflect_tools/enabled
├── Enables former/enabled
│
full (all features, same as enabled)
```

**Default Features:** `enabled`

### Formatting Flow

#### Fallback Conversion Flow

```
to_string_with_fallback!(WithDisplay, WithDebug, value)
  ↓
Try WithDisplay (primary)
  ├─ Implements Display? → Use Display::fmt()
  │
  └─ No Display → Try WithDebug (fallback1)
      ├─ Implements Debug? → Use Debug::fmt()
      │
      └─ No Debug → Try fallback2 (if provided)
```

#### Table Formatting Flow

```
Data Structure (with Fields trait)
  ↓
Extract field names and values
  ↓
InputExtract { column_names, rows }
  ↓
Select TableOutputFormat
  ├─ Table (grid layout)
  ├─ Records (vertical)
  └─ Keys (key-value)
  ↓
Format into Context buffer
  ↓
String output
```

## Public API

### Core Macros

```rust
/// Convert to string with fallback formatters
#[macro_export]
macro_rules! to_string_with_fallback {
  ($how:ty, $fallback1:ty, $fallback2:ty, $src:expr) => { ... };
  ($how:ty, $fallback1:ty, $src:expr) => { ... };
}

/// Format struct field with fallbacks
#[macro_export]
macro_rules! _field {
  (($expr:expr), $how:ty, $fallback1:ty, $fallback2:ty) => { ... };
}

/// Format struct field with custom key
#[macro_export]
macro_rules! _field_with_key {
  ($path:expr, $key:ident, $how:ty, $fallback1:ty, $fallback2:ty) => { ... };
}
```

### Core Traits

```rust
/// Flexible ToString with fallback formatting
pub trait ToStringWithFallback<'a, How, Fallback1, Fallback2> {
  fn to_string_with_fallback(self) -> Cow<'a, str>;
}

/// Table output formatting strategy
pub trait TableOutputFormat {
  fn extract_write<'buf, 'data>(
    &self,
    x: &InputExtract<'data>,
    c: &mut Context<'buf>,
  ) -> fmt::Result;
}

/// Trait for types that can be displayed as tables
pub trait AsTable {
  // ... table conversion methods
}
```

### Wrapper Types

```rust
/// Format using Display trait
pub struct WithDisplay;

/// Format using Debug trait
pub struct WithDebug;

/// Format using reference
pub struct WithRef;

// Reference adapters for fallback chain
pub struct Ref<'a, T, How, Fallback1, Fallback2>(...);
pub struct Ref2<'a, T, How, Fallback1, Fallback2>(...);
pub struct Ref3<'a, T, How, Fallback1, Fallback2>(...);
```

### Re-exports from reflect_tools

```rust
pub use reflect_tools::{
  Fields,              // Field iteration trait
  IteratorTrait,       // Iterator trait for reflection
  _IteratorTrait,      // Internal iterator trait
};
```

## Usage Patterns

### Pattern 1: Basic Fallback Conversion

```rust
use format_tools::*;
use core::fmt;

struct Both;

impl fmt::Debug for Both {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "This is debug")
  }
}

impl fmt::Display for Both {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "This is display")
  }
}

let src = Both;
// Primary: Display, Fallback: Debug
let got = to_string_with_fallback!(WithDisplay, WithDebug, &src);
assert_eq!(got, "This is display");
```

### Pattern 2: Fallback to Debug

```rust
use format_tools::*;
use core::fmt;

struct OnlyDebug;

impl fmt::Debug for OnlyDebug {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "This is debug")
  }
}

let src = OnlyDebug;
// Display not available, falls back to Debug
let got = to_string_with_fallback!(WithDisplay, WithDebug, &src);
assert_eq!(got, "This is debug");
```

### Pattern 3: Table Formatting (Grid Layout)

```rust
use format_tools::*;

// Example table output:
//  sid | sname | gap
// -----+-------+-----
//    3 | Alice |   5
//    6 | Joe   |   1
//   10 | Boris |   5
```

### Pattern 4: Records Formatting (Vertical)

```rust
use format_tools::*;

// Example records output:
// -[ RECORD 1 ]
// sid   | 3
// sname | Alice
// gap   | 5
// -[ RECORD 2 ]
// sid   | 6
// sname | Joe
// gap   | 1
```

### Pattern 5: Field Formatting Macro

```rust
use format_tools::*;

struct Person {
  name: String,
  age: u32,
}

let person = Person {
  name: "Alice".to_string(),
  age: 30,
};

// Format fields with fallbacks
let name_field = _field!((person.name), WithDisplay, WithDebug, WithDebug);
let age_field = _field!((person.age), WithDisplay, WithDebug, WithDebug);
```

### Pattern 6: Raw Vector Table

```rust
use format_tools::*;
use std::borrow::Cow;

let column_names = vec![
  Cow::from("Name"),
  Cow::from("Age"),
];

let rows = vec![
  vec![Cow::from("Alice"), Cow::from("30")],
  vec![Cow::from("Bob"), Cow::from("25")],
];

// Construct and format table from raw vectors
// vector_table_write(column_names, has_header, rows, context)?;
```

## Dependencies and Consumers

### Direct Dependencies

**Internal (workspace):**
- `reflect_tools` - Type reflection (features: `reflect_types`)
- `former` - Builder patterns (features: `derive_former`)
- `collection_tools` - Collections (dev: `collection_constructors`)

**External:** (none)

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- CLI tools (willbe, etc.)
- Diagnostic utilities
- Build output formatters
- Logging systems
- Test reporters

**Usage Pattern:** Workspace tools use format_tools for pretty-printing structured data, formatting CLI output tables, and flexible string conversion with fallbacks.

## Design Rationale

### Why Fallback Mechanism?

Provides primary → fallback → fallback2 formatting chain:

**Problem:** Want Display if available, Debug as fallback, custom as last resort

**Solution:** Type-based dispatch with multiple fallback levels

**Benefits:**
1. **Flexibility**: Try preferred formatter first
2. **Robustness**: Fallback ensures something always works
3. **Type Safety**: Compile-time verification of formatter availability

**Tradeoff:** Complex trait bounds for convenience

### Why Multiple Table Formats?

Supports Table, Records, and Keys formats:

**Rationale:**
1. **Table**: Best for many rows, few columns
2. **Records**: Best for few rows, many columns
3. **Keys**: Best for single record display

**Benefit:** Choose format based on data shape

### Why Field Macros?

`_field!` and `_field_with_key!` macros for field formatting:

**Problem:** Repetitive field extraction and formatting

**Solution:** Macros that extract field name from expression path

```rust
_field!((person.name), ...)
// Automatically uses "name" as key
```

**Benefit:** DRY principle, less boilerplate

### Why Integrate reflect_tools?

Uses reflect_tools for runtime field iteration:

**Rationale:**
1. **Dynamic Fields**: Iterate struct fields at runtime
2. **Generic Formatting**: Format any type implementing Fields
3. **No Macros**: No proc macros needed for basic cases

**Tradeoff:** Runtime reflection cost for flexibility

### Why Reference Wrappers?

Uses Ref, Ref2, Ref3 wrapper types:

**Rationale:**
1. **Fallback Chain**: Each wrapper level tries different formatter
2. **Type Dispatch**: Rust's trait resolution picks correct wrapper
3. **Zero-Cost**: Wrappers optimize away

**Mechanism:** Ref<T, Display, Debug, Custom> tries Display, falls back to Ref2 which tries Debug, etc.

### Why No Color Support?

Plain text output only:

**Rationale:**
1. **Simplicity**: No terminal detection complexity
2. **Composability**: Users can add color with dedicated crates
3. **Testing**: Plain text easier to test

**Workaround:** Use crates like colored or termcolor on top

### Why Cow<'a, str> Return?

Returns `Cow<'a, str>` instead of `String`:

**Benefits:**
1. **Zero-Copy**: Can return borrowed strings when possible
2. **Flexibility**: Owned or borrowed
3. **Performance**: Avoid allocation for static strings

**Tradeoff:** Slightly more complex API

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- Integration tests with collection_tools

### Test Files

```
tests/
└── (various test files for formatting)
```

### Test Focus

1. **Fallback Chain**: Verify Display → Debug → custom fallback works
2. **Table Formats**: Test all three output formats
3. **Field Macros**: Verify key extraction and formatting
4. **Edge Cases**: Empty tables, single column, no headers
5. **Integration**: Test with reflect_tools and former

## Future Considerations

### Potential Enhancements

1. **Color Support**: Optional ANSI color codes
2. **CSV/TSV Output**: Additional table formats
3. **JSON/YAML**: Structured format output
4. **Streaming**: Incremental table row output
5. **Alignment Options**: Left/right/center column alignment
6. **Truncation**: Automatic column width limiting
7. **Pagination**: Split large tables into pages

### Breaking Changes to Consider

1. **Return String**: Change from Cow to String (simpler)
2. **Rename Macros**: Remove underscore prefix from public macros
3. **Async Support**: Add async formatting variants

### Known Limitations

1. **No Syntax Highlighting**: Plain text only
2. **No Width Detection**: Doesn't auto-detect terminal width
3. **Memory Buffering**: Buffers entire table before output
4. **Limited Alignment**: Basic alignment only
5. **ASCII Only**: No Unicode box drawing (could be added)

## Adoption Guidelines

### When to Use format_tools

**Good Candidates:**
- CLI tool output formatting
- Diagnostic message tables
- Test result formatting
- Log message structuring
- Debug output utilities

**Poor Candidates:**
- Web API responses (use serde_json)
- Binary protocols (use bincode, etc.)
- Performance-critical formatting (use std::fmt directly)
- Rich terminal UI (use tui-rs, ratatui)

### Migration from std::fmt

```rust
// Before: Manual Display/Debug selection
let output = if has_display {
  format!("{}", value)
} else {
  format!("{:?}", value)
};

// After: Automatic fallback
use format_tools::*;
let output = to_string_with_fallback!(WithDisplay, WithDebug, &value);
```

### Best Practices

1. **Choose Right Format**: Table for wide data, Records for deep data
2. **Use Fallbacks**: Always provide Display and Debug fallbacks
3. **Field Macros**: Use for struct formatting to avoid boilerplate
4. **Cow Optimization**: Return borrowed strings when possible
5. **Test Output**: Verify table formatting in tests

## Related Crates

- **prettytable-rs**: Alternative table formatting library
- **tabled**: Modern table formatting
- **comfy-table**: Cross-platform tables with styling
- **reflect_tools**: Type reflection (dependency)
- **former**: Builder patterns (dependency)

## References

- [API Documentation](https://docs.rs/format_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/format_tools)
- [readme.md](./readme.md)
- [reflect_tools](https://docs.rs/reflect_tools)
- [former](https://docs.rs/former)
