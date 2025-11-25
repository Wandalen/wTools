# Extract Terminal and Formatting Utilities from wplan

**Date**: 2025-11-20
**Rejection Date**: 2025-11-22
**Priority**: 0 (REJECTED)
**Category**: API Enhancement - Code Extraction
**Status**: ❌ (Rejected - Specification Violation)
**Source**: wplan_client/src/interactive/terminal.rs, wplan_client/src/decoration/mod.rs
**Task ID**: 001
**Advisability**: 0 (Value: 7, Easiness: 4, Safety: 8, Priority: 0)

---

## ❌ TASK REJECTED - SPECIFICATION VIOLATION

**Rejection Rationale**: This task **CANNOT** be implemented as proposed due to **fundamental architectural incompatibility** with format_tools' specification and design principles.

### Critical Violations

#### 1. Direct Specification Conflict (BLOCKING)

The **format_tools specification** (`spec.md:81-84`) **explicitly prohibits** the core functionality proposed in this task:

```markdown
3. **NOT Color/ANSI Styling**
   - No terminal color support
   - Plain text output only
   - **Rationale:** Use dedicated terminal styling crates
```

**What this task proposes:**
- ❌ **ANSI color codes** (LogsDecoration with color support) - **EXPLICITLY PROHIBITED**
- ❌ **Terminal I/O state management** (TerminalGuard) - **OUT OF SCOPE**
- ⚠️ **Unicode box-drawing** (spec says "could be added" but currently "ASCII Only")

**Impact**: Implementing this task as written would **violate the specification**, which according to Claude Protocol Rule #2 ("The Specification Is The Source of Truth"), requires spec update BEFORE implementation. The specification explicitly directs users to "use dedicated terminal styling crates" - this task contradicts that directive.

#### 2. Scope Mismatch (ARCHITECTURAL)

**format_tools purpose** (from `spec.md:7` and `readme.md:7`):
> Collection of mechanisms for formatting and serialization into string.

**Task 001 proposes:**
- Terminal I/O state management (system interaction, not string formatting)
- Raw mode control (system calls, not serialization)
- Terminal guard (resource management, not formatting)
- ANSI escape sequences (explicitly out of scope)

**Analysis**: format_tools is a **pure data transformation library** (string → formatted string). Task 001 proposes adding **system I/O interaction** (terminal state management). These are fundamentally incompatible responsibilities.

**Scope violation severity**: CRITICAL
- Current: Zero system dependencies, pure Rust, works everywhere
- Proposed: Platform-specific code (libc/windows), unsafe code, FFI complexity

#### 3. Single Consumer - No Demonstrated Value

**Workspace search results:**
- **Terminal I/O usage:** ONLY wplan_client (1 consumer)
- **Potential beneficiaries:** None found in workspace
- **Code reuse:** Zero evidence of need

**Value analysis:**
- Extraction effort: 16-19 hours (realistic, not 7 hours claimed)
- Benefit: Code reuse for 1 consumer (already working)
- ROI: Negative (high effort, zero reuse, architectural compromise)

**Conclusion**: Violates YAGNI principle. Extract when second consumer emerges, not before.

#### 4. Dependency Impact (TECHNICAL DEBT)

**Current format_tools dependencies:**
```toml
[dependencies]
reflect_tools = { workspace = true }
former = { workspace = true }
collection_tools = { workspace = true }
```
- **Zero external dependencies**
- **Zero platform-specific code**
- **Pure Rust**
- **Fast compilation**

**Task 001 would add:**
```toml
libc = { workspace = true }  # Unix termios - FFI, unsafe code
[target.'cfg(windows)'.dependencies]
windows = { ... }  # Windows Console API - massive dependency tree
```
- **OR** crossterm (~50+ transitive dependencies)
- **Platform-specific code paths**
- **Unsafe code** (FFI to libc/windows)
- **Complex testing** (requires PTY/console simulation)

**Impact**: First system dependencies in pure formatting library - violates architectural purity.

### Architectural Analysis

#### format_tools Current Architecture

```
format_tools (PURE DATA TRANSFORMATION)
├── Input: Any type implementing Display/Debug/Fields
├── Processing: String conversion, table formatting, text wrapping
├── Output: Cow<'a, str> (formatted string)
├── Dependencies: reflect_tools, former (workspace, pure Rust)
└── Characteristics: Zero system interaction, pure functions
```

#### Task 001 Proposed Architecture

```
format_tools (MIXED CONCERNS - VIOLATION)
├── Original: String formatting (pure)
├── NEW: Terminal state management (I/O)
├── NEW: System calls (platform-specific)
├── NEW: RAII resource management (not formatting)
└── Dependencies: libc, windows (system FFI)
```

**Problem**: This violates **Single Responsibility Principle** - mixing data transformation with system I/O.

### Alternative Approaches (NOT PURSUED)

Since this task is **rejected**, alternatives are documented for reference only:

#### Option A: Create `terminal_tools` Crate (IF NEEDED IN FUTURE)

**Only pursue if:**
- Second consumer emerges (demonstrates actual need)
- Terminal functionality becomes workspace pattern
- Clear separation of concerns desired

**Structure:**
```
terminal_tools/  # NEW crate, proper scope
├── src/
│   ├── state/        # TerminalGuard, raw mode
│   ├── decoration/   # ANSI colors, box chars
│   └── output/       # Terminal-specific utilities
└── Cargo.toml        # Dependencies: crossterm or libc/windows
```

**Benefit**: Proper naming, clear scope, no architectural violation

#### Option B: Keep in wplan_client (CURRENT RECOMMENDATION)

**Rationale:**
- Working implementation exists
- Single consumer (no demonstrated need for extraction)
- YAGNI principle
- Zero risk, zero effort

**When to reconsider**: When second consumer emerges

#### Option C: Extract ONLY Box-Drawing Characters (MINIMAL SCOPE)

**If box characters are valuable:**
- Extract ONLY static data (BoxChars struct)
- NO terminal I/O
- NO ANSI colors
- NO TerminalGuard
- Requires spec update (adding feature, not violating)

**Effort**: 2-3 hours
**Risk**: Low (pure data, no dependencies)

### Specification References

**From `/home/user1/pro/lib/wTools/module/core/format_tools/spec.md`:**

**Line 7** (Purpose):
> Collection of mechanisms for formatting and serialization into string.

**Lines 16-17** (Responsibility):
> Provide comprehensive formatting and serialization utilities including fallback-based string conversion, table formatting...

**Lines 81-84** (Explicit Prohibition):
> 3. **NOT Color/ANSI Styling**
>    - No terminal color support
>    - Plain text output only
>    - **Rationale:** Use dedicated terminal styling crates

**Lines 555-557** (Current Limitation):
> 5. **ASCII Only**: No Unicode box drawing (could be added)

### Claude Protocol Violations

This task, if implemented as proposed, would violate:

1. **Rule #2: The Specification Is The Source of Truth**
   - Spec explicitly prohibits ANSI colors
   - Cannot implement without spec modification FIRST
   - Current proposal includes prohibited functionality

2. **Rule #4: No Code Duplication, Backups, or Legacy Preservation**
   - Single consumer doesnt justify extraction
   - Would create maintenance burden without demonstrated reuse

### Decision Authority

**Rejection based on:**
- Specification analysis (`spec.md` explicit prohibition)
- Architectural principles (scope mismatch, SRP violation)
- Value analysis (single consumer, no ROI)
- Risk assessment (dependencies, complexity, technical debt)
- Claude Protocol compliance (Rule #2)

**Decision**: **REJECT**
**Recommended alternative**: Keep in wplan_client until second consumer emerges
**Future consideration**: Create `terminal_tools` crate IF workspace pattern emerges

---

## Original Task Proposal (REJECTED)

**⚠️ NOTE**: Everything below this line represents the **REJECTED** proposal. It is preserved for historical reference and future consideration if requirements change.

---

## Executive Summary

Extract terminal state management and decorative formatting utilities from the wplan ecosystem to `format_tools`, making them available to all wTools projects. These utilities handle terminal raw mode (for interactive applications), box-drawing characters, and decorative output formatting - useful for any TUI or CLI with rich output.

---

## Problem Statement

### Current Location

The wplan codebase contains terminal utilities:

**wplan_client/src/interactive/terminal.rs**:
- Lines 13-76: `TerminalGuard` - RAII guard for terminal raw mode
- Functionality: Safe terminal state management for interactive applications

**wplan_client/src/decoration/mod.rs**:
- Lines 125-200: Box-drawing decorations, colored headers, metadata lines
- Functionality: Decorative output formatting with Unicode box characters

### Why Extract to format_tools

1. **TUI Foundation**: Essential for any terminal user interface
2. **Safety**: RAII terminal guard prevents leaving terminal in broken state
3. **Consistency**: Standardizes decorative output across wTools
4. **Code Reuse**: wtest, benchkit, unitore all benefit from rich CLI output
5. **Unicode Support**: Centralized box-drawing character handling

---

## Detailed Functionality Analysis

### 1. Terminal State Guard (RAII)

**Current Location**: `wplan_client/src/interactive/terminal.rs:13-76`

```rust
pub struct TerminalGuard
{
  // Original terminal state
}

impl TerminalGuard
{
  pub fn new() -> Result< Self, std::io::Error >
  {
    // Enable raw mode
    // Save original state
  }
}

impl Drop for TerminalGuard
{
  fn drop( &mut self )
  {
    // Restore original terminal state
    // Guaranteed cleanup even on panic
  }
}
```

**Features**:
- RAII pattern ensures terminal state restored
- Enables raw mode (no line buffering, no echo)
- Disables mouse, alternate screen, etc.
- Cleanup on panic

**Use Cases**:
- Interactive prompts
- TUI applications
- Password input
- Real-time key handling

**Why Critical**:
- Without RAII, panic leaves terminal unusable
- User must close terminal window to recover
- Professional apps MUST handle this correctly

### 2. Box-Drawing Characters

**Current Location**: `wplan_client/src/decoration/mod.rs:125-200`

```rust
pub struct BoxChars
{
  pub horizontal : char,      // ─
  pub vertical : char,        // │
  pub top_left : char,        // ┌
  pub top_right : char,       // ┐
  pub bottom_left : char,     // └
  pub bottom_right : char,    // ┘
  // ... and more
}

impl BoxChars
{
  pub const LIGHT : Self = Self { /* ... */ };
  pub const HEAVY : Self = Self { /* ... */ };
  pub const DOUBLE : Self = Self { /* ... */ };
  pub const ROUNDED : Self = Self { /* ... */ };
}
```

**Features**:
- Unicode box-drawing character sets
- Multiple styles (light, heavy, double, rounded)
- Helper methods for drawing boxes

**Use Cases**:
- Table borders
- Section separators
- Progress bars
- TUI panels

### 3. Decorative Formatting

**Current Location**: `wplan_client/src/decoration/mod.rs:125-200`

```rust
pub fn format_job_header( job_id : &str, status : &str ) -> String
{
  format!(
    "{}{}{}",
    BoxChars::LIGHT.top_left,
    BoxChars::LIGHT.horizontal.to_string().repeat( 50 ),
    BoxChars::LIGHT.top_right
  )
}
```

**Features**:
- Formatted headers with borders
- Colored status indicators
- Metadata lines with alignment
- ANSI color support

**Use Cases**:
- Test output formatting
- Benchmark results display
- Log entry formatting
- Status reports

---

## Proposed API Design

### Target Location

```
format_tools/src/terminal/
  mod.rs           # Module exports
  guard.rs         # Terminal state guard (RAII)
  mode.rs          # Terminal mode utilities
format_tools/src/decoration/
  mod.rs           # Module exports
  box_chars.rs     # Box-drawing characters
  header.rs        # Header formatting
```

### API Structure

```rust
//! Terminal and formatting utilities for format_tools
//!
//! Provides:
//! - Terminal state management (raw mode, RAII guard)
//! - Box-drawing characters and decorations
//! - Formatted headers and tables

// ============================================================================
// terminal/guard.rs - Terminal State Guard
// ============================================================================

use std::io;

/// RAII guard for terminal state management.
///
/// Enables terminal raw mode and ensures restoration even on panic.
///
/// **CRITICAL**: Use this guard for ALL terminal raw mode operations.
/// Without it, panics leave terminal in broken state.
///
/// # Example
///
/// ```rust,no_run
/// use format_tools::terminal::TerminalGuard;
///
/// {
///   let _guard = TerminalGuard::new()?;
///
///   // Terminal now in raw mode
///   // Read single keypresses, no buffering
///
///   // Panic here? Terminal still restored by Drop
/// }
///
/// // Terminal automatically restored here
/// ```
pub struct TerminalGuard
{
  #[ cfg( unix ) ]
  original : libc::termios,
}

impl TerminalGuard
{
  /// Create guard and enter raw mode.
  ///
  /// Raw mode characteristics:
  /// - No line buffering (read single bytes)
  /// - No echo (keypresses not printed)
  /// - No signal generation (Ctrl+C doesn't send SIGINT)
  /// - No special character handling
  pub fn new() -> io::Result< Self >;

  /// Enter raw mode with custom configuration.
  pub fn with_options( options : &TerminalOptions ) -> io::Result< Self >;
}

impl Drop for TerminalGuard
{
  fn drop( &mut self )
  {
    // MUST restore original terminal state
    // Even if restoration fails, try best effort
  }
}

/// Terminal configuration options.
#[ derive( Debug, Clone ) ]
pub struct TerminalOptions
{
  /// Enable raw mode (no buffering, no echo).
  pub raw_mode : bool,
  /// Enable mouse events.
  pub mouse_capture : bool,
  /// Use alternate screen buffer.
  pub alternate_screen : bool,
}

impl Default for TerminalOptions
{
  fn default() -> Self
  {
    Self
    {
      raw_mode : true,
      mouse_capture : false,
      alternate_screen : false,
    }
  }
}

// ============================================================================
// terminal/mode.rs - Terminal Mode Utilities
// ============================================================================

/// Check if stdout is a terminal.
///
/// # Example
///
/// ```rust
/// use format_tools::terminal::is_terminal;
///
/// if is_terminal()
/// {
///   println!( "Running in terminal - colors enabled" );
/// }
/// else
/// {
///   println!( "Not a terminal - colors disabled" );
/// }
/// ```
pub fn is_terminal() -> bool;

/// Get terminal size (width × height).
///
/// # Returns
///
/// `Some((width, height))` or `None` if not a terminal.
///
/// # Example
///
/// ```rust
/// use format_tools::terminal::terminal_size;
///
/// if let Some( ( width, height ) ) = terminal_size()
/// {
///   println!( "Terminal: {}×{}", width, height );
/// }
/// ```
pub fn terminal_size() -> Option< ( usize, usize ) >;

// ============================================================================
// decoration/box_chars.rs - Box-Drawing Characters
// ============================================================================

/// Box-drawing character set.
#[ derive( Debug, Clone, Copy ) ]
pub struct BoxChars
{
  pub horizontal : char,
  pub vertical : char,
  pub top_left : char,
  pub top_right : char,
  pub bottom_left : char,
  pub bottom_right : char,
  pub left_tee : char,
  pub right_tee : char,
  pub top_tee : char,
  pub bottom_tee : char,
  pub cross : char,
}

impl BoxChars
{
  /// Light box-drawing characters (┌─┐│└┘├┤┬┴┼).
  pub const LIGHT : Self = Self
  {
    horizontal : '─',
    vertical : '│',
    top_left : '┌',
    top_right : '┐',
    bottom_left : '└',
    bottom_right : '┘',
    left_tee : '├',
    right_tee : '┤',
    top_tee : '┬',
    bottom_tee : '┴',
    cross : '┼',
  };

  /// Heavy box-drawing characters (┏━┓┃┗┛┣┫┳┻╋).
  pub const HEAVY : Self = Self
  {
    horizontal : '━',
    vertical : '┃',
    top_left : '┏',
    top_right : '┓',
    bottom_left : '┗',
    bottom_right : '┛',
    left_tee : '┣',
    right_tee : '┫',
    top_tee : '┳',
    bottom_tee : '┻',
    cross : '╋',
  };

  /// Double box-drawing characters (╔═╗║╚╝╠╣╦╩╬).
  pub const DOUBLE : Self = Self
  {
    horizontal : '═',
    vertical : '║',
    top_left : '╔',
    top_right : '╗',
    bottom_left : '╚',
    bottom_right : '╝',
    left_tee : '╠',
    right_tee : '╣',
    top_tee : '╦',
    bottom_tee : '╩',
    cross : '╬',
  };

  /// Rounded box-drawing characters (╭─╮│╰╯).
  pub const ROUNDED : Self = Self
  {
    horizontal : '─',
    vertical : '│',
    top_left : '╭',
    top_right : '╮',
    bottom_left : '╰',
    bottom_right : '╯',
    left_tee : '├',
    right_tee : '┤',
    top_tee : '┬',
    bottom_tee : '┴',
    cross : '┼',
  };

  /// ASCII fallback (+-+|-+|-+-+).
  pub const ASCII : Self = Self
  {
    horizontal : '-',
    vertical : '|',
    top_left : '+',
    top_right : '+',
    bottom_left : '+',
    bottom_right : '+',
    left_tee : '+',
    right_tee : '+',
    top_tee : '+',
    bottom_tee : '+',
    cross : '+',
  };

  /// Draw horizontal line.
  pub fn hline( &self, width : usize ) -> String
  {
    self.horizontal.to_string().repeat( width )
  }

  /// Draw vertical line.
  pub fn vline( &self, height : usize ) -> String
  {
    (0..height).map( | _ | format!( "{}\n", self.vertical ) ).collect()
  }

  /// Draw box with title.
  pub fn box_with_title( &self, width : usize, title : &str ) -> String
  {
    let title_width = title.chars().count();
    let left_width = ( width - title_width - 2 ) / 2;
    let right_width = width - title_width - 2 - left_width;

    format!(
      "{}{} {} {}{}",
      self.top_left,
      self.hline( left_width ),
      title,
      self.hline( right_width ),
      self.top_right
    )
  }
}

// ============================================================================
// decoration/header.rs - Header Formatting
// ============================================================================

/// Format options for headers.
#[ derive( Debug, Clone ) ]
pub struct HeaderOptions
{
  pub box_chars : BoxChars,
  pub width : usize,
  pub color : Option< AnsiColor >,
}

/// ANSI color codes.
#[ derive( Debug, Clone, Copy ) ]
pub enum AnsiColor
{
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  White,
}

impl AnsiColor
{
  pub fn code( &self ) -> &'static str
  {
    match self
    {
      AnsiColor::Red => "\x1b[31m",
      AnsiColor::Green => "\x1b[32m",
      AnsiColor::Yellow => "\x1b[33m",
      AnsiColor::Blue => "\x1b[34m",
      AnsiColor::Magenta => "\x1b[35m",
      AnsiColor::Cyan => "\x1b[36m",
      AnsiColor::White => "\x1b[37m",
    }
  }

  pub const RESET : &'static str = "\x1b[0m";
}

/// Format a header with title.
///
/// # Example
///
/// ```rust
/// use format_tools::decoration::{ format_header, HeaderOptions, BoxChars };
///
/// let opts = HeaderOptions
/// {
///   box_chars : BoxChars::LIGHT,
///   width : 50,
///   color : None,
/// };
///
/// println!( "{}", format_header( "Job #123", &opts ) );
/// // Output:
/// // ┌──────────── Job #123 ────────────┐
/// ```
pub fn format_header( title : &str, options : &HeaderOptions ) -> String;

/// Format a section separator.
pub fn format_separator( width : usize, box_chars : &BoxChars ) -> String;
```

---

## Implementation Phases

### Phase 1: Terminal Guard (2 hours)

**Tasks**:
1. Create `format_tools/src/terminal/guard.rs`
2. Implement `TerminalGuard` with Unix support (termios)
3. Implement `TerminalOptions` configuration
4. Add Drop implementation with panic safety
5. Add tests (difficult - requires pty)
6. Document safety guarantees

**Acceptance Criteria**:
- [ ] TerminalGuard enables raw mode
- [ ] Drop restores terminal state
- [ ] Panic doesn't break terminal
- [ ] Tests verify state restoration
- [ ] Documentation warns about thread safety

### Phase 2: Terminal Mode Utilities (1 hour)

**Tasks**:
1. Create `format_tools/src/terminal/mode.rs`
2. Implement `is_terminal()` check
3. Implement `terminal_size()` query
4. Add platform-specific implementations
5. Add tests

**Acceptance Criteria**:
- [ ] `is_terminal()` detects TTY correctly
- [ ] `terminal_size()` returns accurate dimensions
- [ ] Works on Unix and Windows
- [ ] Tests verify behavior

### Phase 3: Box-Drawing Characters (1.5 hours)

**Tasks**:
1. Create `format_tools/src/decoration/box_chars.rs`
2. Implement `BoxChars` with all styles
3. Implement helper methods (hline, vline, box_with_title)
4. Add ASCII fallback for non-Unicode terminals
5. Add tests

**Acceptance Criteria**:
- [ ] All box character styles defined
- [ ] Helper methods work correctly
- [ ] ASCII fallback available
- [ ] Tests verify all styles

### Phase 4: Header Formatting (1.5 hours)

**Tasks**:
1. Create `format_tools/src/decoration/header.rs`
2. Implement `HeaderOptions` and `AnsiColor`
3. Implement `format_header()` and `format_separator()`
4. Add color support with ANSI codes
5. Add tests

**Acceptance Criteria**:
- [ ] Headers formatted correctly
- [ ] Colors applied properly
- [ ] Width calculation accurate
- [ ] Tests verify output

### Phase 5: Integration and Migration (1 hour)

**Tasks**:
1. Update `format_tools/src/lib.rs` to export modules
2. Migrate wplan_client to use new API
3. Delete old implementations
4. Verify all tests pass

**Acceptance Criteria**:
- [ ] All modules exported
- [ ] wplan_client uses new API
- [ ] Old code deleted
- [ ] Tests pass

---

## Dependencies

```toml
# format_tools/Cargo.toml
[dependencies]
libc = { workspace = true }  # For Unix termios

[target.'cfg(windows)'.dependencies]
windows = { workspace = true, features = ["Win32_System_Console"] }  # For Windows console API
```

---

## Expected Impact

| Metric | Before | After |
|--------|--------|-------|
| wplan_client terminal.rs LOC | ~65 | 0 (deleted) |
| wplan_client decoration.rs LOC | ~150 | ~20 (imports) |
| Terminal safety | Per-project | Centralized RAII |
| Box char consistency | Per-project | Standardized |
| Code duplication | Isolated | Shared |

---

## Testing Strategy

### Unit Tests

**Terminal Guard**:
- Raw mode enabled/disabled
- State restoration on drop
- Panic safety (catch_unwind)

**Box Characters**:
- All styles render correctly
- Helper methods work
- ASCII fallback

**Header Formatting**:
- Width calculation
- Title centering
- Color codes

### Integration Tests

- wplan_client migration
- Interactive prompt example
- TUI demo application

---

## Platform Considerations

### Unix

- Use `termios` for raw mode
- `ioctl` for terminal size
- POSIX compliance

### Windows

- Use Windows Console API
- Handle console mode flags
- VT100 sequence support

---

## Documentation Requirements

1. Module-level documentation
2. Safety guarantees for TerminalGuard
3. Platform-specific behavior
4. Box character visual reference
5. Color code reference
6. Interactive example applications

---

## Acceptance Criteria

- [ ] Terminal guard module complete
- [ ] Terminal mode utilities complete
- [ ] Box-drawing characters complete
- [ ] Header formatting complete
- [ ] Cross-platform support
- [ ] Comprehensive test coverage
- [ ] Documentation complete
- [ ] wplan_client successfully migrated
- [ ] Old implementations deleted
- [ ] `cargo test -p format_tools` passes
- [ ] `cargo test -p wplan_client` passes

---

## References

**Source Files**:
- `/home/user1/pro/lib/willbe/module/wplan_client/src/interactive/terminal.rs:13-76` (TerminalGuard)
- `/home/user1/pro/lib/willbe/module/wplan_client/src/decoration/mod.rs:125-200` (box chars, headers)

**Related Projects**:
- wtest - needs terminal guard for interactive prompts
- benchkit - needs decorative output for results
- unitore - needs terminal detection for colored output

**Dependencies**:
- libc (workspace) - Unix termios
- windows (workspace) - Windows console API

---

## Estimated Effort

- Phase 1: 2 hours (terminal guard)
- Phase 2: 1 hour (mode utilities)
- Phase 3: 1.5 hours (box characters)
- Phase 4: 1.5 hours (header formatting)
- Phase 5: 1 hour (migration)

**Total**: 7 hours

---

## Priority Justification

**MEDIUM Priority** because:
1. **Specialized Use**: Not all projects need interactive terminal features
2. **Platform Complexity**: Significant platform differences to handle
3. **Testing Difficulty**: Terminal tests require PTY, hard to automate
4. **Limited Users**: Primarily benefits interactive/TUI applications
5. **Nice-to-Have**: Decorative output is enhancement, not core functionality

However, for projects that DO need it (wtest, interactive tools), it's CRITICAL for preventing broken terminal states.
