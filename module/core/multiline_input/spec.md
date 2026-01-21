# Multiline Input Crate - Comprehensive Specification

## Executive Summary

**Purpose**: General-purpose Rust crate for collecting multiline text input from terminal with rich editing capabilities.

**Key Features**:
- **ENTER** to submit input
- **CTRL+ENTER** or **SHIFT+ENTER** to insert newline (multiline support)
- Line editing (cursor movement, backspace, delete)
- Visual feedback (cursor, line numbers optional)
- Graceful cancellation (ESC or CTRL+C)
- Cross-platform (Linux, macOS, Windows)

**Design Philosophy**: Simple API, rich UX, zero-config defaults, extensive customization.

---

## Requirements

### Functional Requirements

#### FR1: Basic Input Collection
- **FR1.1**: Collect multiline text from terminal
- **FR1.2**: Return collected text as String
- **FR1.3**: Support UTF-8 text including emoji and Unicode

#### FR2: Key Bindings (Non-negotiable)
- **FR2.1**: **ENTER** → Submit and return collected text
- **FR2.2**: **CTRL+ENTER** or **SHIFT+ENTER** → Insert newline character (`\n`)
- **FR2.3**: **ESC** → Cancel input, return None
- **FR2.4**: **CTRL+C** → Cancel input, return None
- **FR2.5**: **CTRL+D** → Submit (alternative to ENTER)
- **FR2.6**: **Backspace** → Delete character before cursor
- **FR2.7**: **Delete** → Delete character at cursor
- **FR2.8**: **Left Arrow** → Move cursor left
- **FR2.9**: **Right Arrow** → Move cursor right
- **FR2.10**: **Up Arrow** → Move cursor up one line
- **FR2.11**: **Down Arrow** → Move cursor down one line
- **FR2.12**: **Home** → Move cursor to start of current line
- **FR2.13**: **End** → Move cursor to end of current line
- **FR2.14**: **CTRL+Home** → Move cursor to start of text
- **FR2.15**: **CTRL+End** → Move cursor to end of text

#### FR3: Visual Feedback
- **FR3.1**: Display prompt message
- **FR3.2**: Show cursor position
- **FR3.3**: Display entered text in real-time
- **FR3.4**: Optional line numbers
- **FR3.5**: Optional status line showing current line/column
- **FR3.6**: Optional character count

#### FR4: Validation & Constraints
- **FR4.1**: Optional minimum length validation
- **FR4.2**: Optional maximum length validation
- **FR4.3**: Optional custom validator function
- **FR4.4**: Allow/disallow empty input

#### FR5: Customization
- **FR5.1**: Custom prompt text
- **FR5.2**: Custom submit/cancel key bindings (advanced)
- **FR5.3**: Custom colors/styles (if terminal supports)
- **FR5.4**: Initial text (pre-filled input)
- **FR5.5**: Placeholder text (when empty)

### Non-Functional Requirements

#### NFR1: Performance
- **NFR1.1**: Instant key response (<10ms latency)
- **NFR1.2**: Support text up to 1MB without lag
- **NFR1.3**: Minimal memory overhead (<10KB base)

#### NFR2: Compatibility
- **NFR2.1**: Linux (ANSI terminals)
- **NFR2.2**: macOS (Terminal.app, iTerm2)
- **NFR2.3**: Windows (Windows Terminal, ConEmu, cmd.exe with limitations)
- **NFR2.4**: SSH sessions
- **NFR2.5**: tmux/screen multiplexers

#### NFR3: Reliability
- **NFR3.1**: No panics on invalid input
- **NFR3.2**: Graceful degradation when terminal features unavailable
- **NFR3.3**: Proper cleanup of terminal state (raw mode, cursor, etc.)

#### NFR4: Usability
- **NFR4.1**: Zero-config default that "just works"
- **NFR4.2**: Clear error messages
- **NFR4.3**: Intuitive key bindings (match common editors)

#### NFR6: Testing Requirements
- **NFR6.1**: All tests must use explicit parameters (no reliance on default values)
- **NFR6.2**: Tests must be non-fragile (no environment dependencies like TTY)
- **NFR6.3**: Tests must be deterministic (same input produces same output)
- **NFR6.4**: Error paths must have explicit test coverage (100% for critical errors)
- **NFR6.5**: Integration tests must use MockTerminal, not real terminal I/O

#### NFR5: Terminal Requirements
- **NFR5.1**: Minimum terminal width: 20 characters (enough for prompt + minimal text)
- **NFR5.2**: Minimum terminal height: 3 rows (prompt line + text line + cursor/status)
- **NFR5.3**: Returns `Error::TerminalTooSmall` with current and required dimensions if below minimum
- **NFR5.4**: Error message must include both current size and minimum required size

---

## API Design

### Core API

```rust
/// Collect multiline input from terminal
///
/// # Example
/// ```
/// use multiline_input::collect;
///
/// match collect("Enter your message:") {
///   Ok(Some(text)) => println!("You entered: {}", text),
///   Ok(None) => println!("Cancelled"),
///   Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn collect(prompt: &str) -> Result<Option<String>, Error>
{
  Builder::new()
    .prompt(prompt)
    .build()
    .collect()
}

/// Collect with validation
pub fn collect_validated<F>(
  prompt: &str,
  validator: F,
) -> Result<Option<String>, Error>
where
  F: Fn(&str) -> Result<(), String>,
{
  Builder::new()
    .prompt(prompt)
    .validator(validator)
    .build()
    .collect()
}
```

### Builder API

```rust
/// Builder for configuring multiline input
pub struct Builder {
  prompt: String,
  allow_empty: bool,
  min_length: Option<usize>,
  max_length: Option<usize>,
  validator: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
  initial_text: Option<String>,
  placeholder: Option<String>,
  show_line_numbers: bool,
  show_status: bool,
  show_char_count: bool,
  color: bool,
}

impl Builder {
  pub fn new() -> Self;
  pub fn prompt(mut self, prompt: &str) -> Self;
  pub fn allow_empty(mut self, allow: bool) -> Self;
  pub fn min_length(mut self, len: usize) -> Self;
  pub fn max_length(mut self, len: usize) -> Self;
  pub fn validator<F>(mut self, f: F) -> Self where F: Fn(&str) -> Result<(), String> + 'static;
  pub fn initial_text(mut self, text: &str) -> Self;
  pub fn placeholder(mut self, text: &str) -> Self;
  pub fn show_line_numbers(mut self, show: bool) -> Self;
  pub fn show_status(mut self, show: bool) -> Self;
  pub fn show_char_count(mut self, show: bool) -> Self;
  pub fn color(mut self, enable: bool) -> Self;
  pub fn build(self) -> Editor;
}
```

### Editor API

```rust
/// Multiline input editor
pub struct Editor {
  config: EditorConfig,
}

impl Editor {
  /// Collect input from user
  pub fn collect(&self) -> Result<Option<String>, Error>;

  /// Collect input with custom event handler
  pub fn collect_with_handler<F>(&self, handler: F) -> Result<Option<String>, Error>
  where
    F: FnMut(&mut EditorState, Event) -> EventResult;
}
```

### Error Types

**Note**: This crate uses `error_tools` for error handling (mandatory per PRO rulebook). `error_tools` provides a unified facade over `thiserror` and `anyhow`.

```rust
use error_tools::typed::Error;
use error_tools::dependency::thiserror;

/// Errors that can occur during input collection
#[ derive( Debug, Error ) ]
pub enum Error
{
  #[ error( "Terminal I/O error: {0}" ) ]
  Io( #[ from ] std::io::Error ),

  #[ error( "Not running in a terminal (no TTY)" ) ]
  NoTty,

  #[ error( "Terminal too small ({width}x{height}, need at least {min_width}x{min_height})" ) ]
  TerminalTooSmall
  {
    width: u16,
    height: u16,
    min_width: u16,
    min_height: u16,
  },

  #[ error( "Terminal does not support required features" ) ]
  UnsupportedTerminal,

  #[ error( "Validation failed: {0}" ) ]
  ValidationFailed( String ),

  #[ error( "Input too short (minimum: {min}, got: {got})" ) ]
  TooShort { min: usize, got: usize },

  #[ error( "Input too long (maximum: {max}, got: {got})" ) ]
  TooLong { max: usize, got: usize },
}
```

---

## Architecture

### Module Structure

```
multiline_input/
├── lib.rs           # Public API, re-exports
├── builder.rs       # Builder pattern implementation
├── editor.rs        # Core Editor and EditorState
├── terminal.rs      # Terminal abstraction (TerminalOps trait, RealTerminal)
├── keys.rs          # Key event handling
├── buffer.rs        # Text buffer management (lines, cursor position)
├── render.rs        # Screen rendering logic
└── error.rs         # Error types
```

### Trait-Based Dependency Injection

**Design**: The crate uses trait-based dependency injection to enable testability without environment dependencies.

**TerminalOps Trait**:
```rust
pub trait TerminalOps: std::io::Write
{
  fn is_tty( &self ) -> bool;
  fn size( &self ) -> io::Result< ( u16, u16 ) >;
  fn enable_raw_mode( &mut self ) -> Result< (), Error >;
  fn disable_raw_mode( &mut self ) -> Result< (), Error >;
  fn clear_screen( &mut self ) -> io::Result< () >;
  fn clear_line( &mut self ) -> io::Result< () >;
  fn move_cursor( &mut self, col: u16, row: u16 ) -> io::Result< () >;
  fn hide_cursor( &mut self ) -> io::Result< () >;
  fn show_cursor( &mut self ) -> io::Result< () >;
  fn write_str( &mut self, text: &str ) -> io::Result< () >;
  fn read_key( &mut self, timeout: Option< Duration > ) -> io::Result< KeyEvent >;
}
```

**Implementations**:
- `RealTerminal`: Production implementation using crossterm for actual terminal I/O
- `MockTerminal` (in tests/): Test double with programmable behavior for integration testing

**Editor Generics**:
```rust
pub struct Editor< T = RealTerminal >
where
  T: TerminalOps,
{
  // ... fields ...
  pub terminal: T,
}
```

**Builder Pattern**:
```rust
impl Builder
{
  // Default: creates RealTerminal
  pub fn build( self ) -> Editor< RealTerminal >

  // Dependency injection: accepts any TerminalOps implementation
  pub fn build_with< T >( self, terminal: T ) -> Editor< T >
  where
    T: TerminalOps,
}
```

**Benefits**:
- Non-fragile testing: Tests use MockTerminal, no TTY required
- Deterministic: Test behavior fully controlled by test code
- Fast: No actual terminal I/O in tests
- Complete coverage: Can test error paths (NoTty, I/O failures) without environment setup

### Data Flow

```
User types key
    ↓
Terminal (raw mode) captures key event
    ↓
keys.rs: Parse key event (Enter, Ctrl+Enter, etc.)
    ↓
editor.rs: Handle event (insert char, move cursor, etc.)
    ↓
buffer.rs: Update text buffer
    ↓
render.rs: Render updated buffer to screen
    ↓
Repeat until ENTER (submit) or ESC (cancel)
    ↓
Return Result<Option<String>, Error>
```

### State Machine

```
┌─────────────┐
│  Inactive   │
└──────┬──────┘
       │ collect()
       ↓
┌─────────────┐
│  Editing    │ ←────┐
└──────┬──────┘      │
       │             │ key event
       │ ENTER       │ (not submit/cancel)
       │ or          │
       │ CTRL+D      │
       ↓             │
┌─────────────┐      │
│ Validating  │──────┤
└──────┬──────┘      │
       │             │
       │ valid       │ invalid
       ↓             │
┌─────────────┐      │
│  Submitted  │      │
└─────────────┘      │
                     │
       ESC or        │
       CTRL+C        │
       ↓             │
┌─────────────┐      │
│  Cancelled  │──────┘
└─────────────┘
```

---

## Implementation Details

### Terminal Raw Mode

**Why**: Capture individual key events (including Ctrl+Enter) before shell processes them.

**How**: Use `crossterm::terminal::enable_raw_mode()`.

**Critical**: Always restore terminal state in Drop or on panic.

```rust
struct RawModeGuard;

impl RawModeGuard {
  fn new() -> Result<Self, Error> {
    crossterm::terminal::enable_raw_mode()?;
    Ok(Self)
  }
}

impl Drop for RawModeGuard {
  fn drop(&mut self) {
    let _ = crossterm::terminal::disable_raw_mode();
  }
}
```

### Key Event Handling

**Crossterm Events**:
```rust
use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEvent};

match crossterm::event::read()? {
  Event::Key(KeyEvent { code, modifiers, .. }) => {
    match (code, modifiers) {
      // ENTER → Submit
      (KeyCode::Enter, KeyModifiers::NONE) => return Ok(Some(buffer.text())),

      // CTRL+ENTER → Newline
      (KeyCode::Enter, KeyModifiers::CONTROL) => buffer.insert_char('\n'),

      // ESC → Cancel
      (KeyCode::Esc, _) => return Ok(None),

      // CTRL+C → Cancel
      (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(None),

      // CTRL+D → Submit (alternative)
      (KeyCode::Char('d'), KeyModifiers::CONTROL) => return Ok(Some(buffer.text())),

      // Backspace
      (KeyCode::Backspace, _) => buffer.delete_before_cursor(),

      // Delete
      (KeyCode::Delete, _) => buffer.delete_at_cursor(),

      // Arrow keys
      (KeyCode::Left, _) => buffer.move_cursor_left(),
      (KeyCode::Right, _) => buffer.move_cursor_right(),
      (KeyCode::Up, _) => buffer.move_cursor_up(),
      (KeyCode::Down, _) => buffer.move_cursor_down(),

      // Home/End
      (KeyCode::Home, KeyModifiers::NONE) => buffer.move_to_line_start(),
      (KeyCode::End, KeyModifiers::NONE) => buffer.move_to_line_end(),
      (KeyCode::Home, KeyModifiers::CONTROL) => buffer.move_to_start(),
      (KeyCode::End, KeyModifiers::CONTROL) => buffer.move_to_end(),

      // Regular character
      (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
        buffer.insert_char(c);
      }

      _ => {} // Ignore other keys
    }
  }
  _ => {} // Ignore non-key events
}
```

### Text Buffer Management

**Data Structure**:
```rust
struct TextBuffer {
  lines: Vec<String>,       // Lines of text
  cursor_line: usize,       // Current line (0-indexed)
  cursor_col: usize,        // Current column (0-indexed, grapheme clusters)
}

impl TextBuffer {
  fn new() -> Self {
    Self {
      lines: vec![String::new()],
      cursor_line: 0,
      cursor_col: 0,
    }
  }

  fn insert_char(&mut self, c: char) {
    if c == '\n' {
      self.insert_newline();
    } else {
      let line = &mut self.lines[self.cursor_line];
      // Insert at cursor position (handle grapheme clusters)
      let byte_pos = self.grapheme_col_to_byte_pos(line, self.cursor_col);
      line.insert(byte_pos, c);
      self.cursor_col += 1;
    }
  }

  fn insert_newline(&mut self) {
    let line = &self.lines[self.cursor_line];
    let split_pos = self.grapheme_col_to_byte_pos(line, self.cursor_col);

    let remainder = line[split_pos..].to_string();
    self.lines[self.cursor_line].truncate(split_pos);

    self.cursor_line += 1;
    self.cursor_col = 0;
    self.lines.insert(self.cursor_line, remainder);
  }

  fn delete_before_cursor(&mut self) {
    if self.cursor_col > 0 {
      // Delete character before cursor on current line
      let line = &mut self.lines[self.cursor_line];
      let byte_pos = self.grapheme_col_to_byte_pos(line, self.cursor_col - 1);
      let next_pos = self.grapheme_col_to_byte_pos(line, self.cursor_col);
      line.drain(byte_pos..next_pos);
      self.cursor_col -= 1;
    } else if self.cursor_line > 0 {
      // Join with previous line
      let current_line = self.lines.remove(self.cursor_line);
      self.cursor_line -= 1;
      self.cursor_col = self.grapheme_len(&self.lines[self.cursor_line]);
      self.lines[self.cursor_line].push_str(&current_line);
    }
  }

  fn text(&self) -> String {
    self.lines.join("\n")
  }

  // Helper: Convert grapheme column to byte position
  fn grapheme_col_to_byte_pos(&self, line: &str, col: usize) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    line.grapheme_indices(true)
      .nth(col)
      .map(|(pos, _)| pos)
      .unwrap_or(line.len())
  }

  // Helper: Count grapheme clusters
  fn grapheme_len(&self, s: &str) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    s.graphemes(true).count()
  }
}
```

### Screen Rendering

**Strategy**: Clear and redraw on each key event (simple, works well for typical input sizes).

**Advanced**: Delta rendering (only redraw changed regions) for large texts.

```rust
use crossterm::{
  cursor,
  style::{Color, Print, SetForegroundColor, ResetColor},
  terminal::{Clear, ClearType},
  ExecutableCommand,
};
use std::io::{Write, stdout};

fn render(config: &EditorConfig, buffer: &TextBuffer) -> Result<(), Error> {
  let mut out = stdout();

  // Clear screen
  out.execute(Clear(ClearType::All))?;
  out.execute(cursor::MoveTo(0, 0))?;

  // Print prompt
  if config.color {
    out.execute(SetForegroundColor(Color::Cyan))?;
  }
  out.execute(Print(&config.prompt))?;
  if config.color {
    out.execute(ResetColor)?;
  }
  out.execute(Print("\n"))?;

  // Print text buffer with optional line numbers
  for (i, line) in buffer.lines.iter().enumerate() {
    if config.show_line_numbers {
      if config.color {
        out.execute(SetForegroundColor(Color::DarkGrey))?;
      }
      out.execute(Print(format!("{:3} │ ", i + 1)))?;
      if config.color {
        out.execute(ResetColor)?;
      }
    }
    out.execute(Print(line))?;
    out.execute(Print("\n"))?;
  }

  // Print status line (optional)
  if config.show_status {
    if config.color {
      out.execute(SetForegroundColor(Color::DarkGrey))?;
    }
    let char_count = buffer.text().chars().count();
    out.execute(Print(format!(
      "\nLine {}/{} | Col {} | {} chars | ENTER=submit, CTRL+ENTER=newline, ESC=cancel",
      buffer.cursor_line + 1,
      buffer.lines.len(),
      buffer.cursor_col + 1,
      char_count,
    )))?;
    if config.color {
      out.execute(ResetColor)?;
    }
  }

  // Position cursor
  let visual_row = 1 + buffer.cursor_line + if config.prompt.is_empty() { 0 } else { 1 };
  let visual_col = buffer.cursor_col + if config.show_line_numbers { 6 } else { 0 };
  out.execute(cursor::MoveTo(visual_col as u16, visual_row as u16))?;

  out.flush()?;
  Ok(())
}
```

---

## Usage Examples

### Example 1: Basic Usage

```rust
use multiline_input::collect;

fn main() {
  match collect("Enter your message (ENTER to submit, CTRL+ENTER for newline):") {
    Ok(Some(text)) => {
      println!("\n=== You entered: ===");
      println!("{}", text);
    }
    Ok(None) => {
      println!("Cancelled");
    }
    Err(e) => {
      eprintln!("Error: {}", e);
    }
  }
}
```

### Example 2: With Validation

```rust
use multiline_input::Builder;

fn main() {
  let editor = Builder::new()
    .prompt("Enter commit message:")
    .min_length(10)
    .max_length(500)
    .validator(|text| {
      if text.lines().next().unwrap_or("").len() > 72 {
        Err("First line must be ≤72 characters".to_string())
      } else {
        Ok(())
      }
    })
    .show_line_numbers(true)
    .show_status(true)
    .build();

  match editor.collect() {
    Ok(Some(msg)) => {
      println!("Commit message:\n{}", msg);
    }
    Ok(None) => println!("Cancelled"),
    Err(e) => eprintln!("Error: {}", e),
  }
}
```

### Example 3: Pre-filled Text

```rust
use multiline_input::Builder;

fn main() {
  let initial = "TODO: implement feature\n\n- Step 1\n- Step 2";

  let editor = Builder::new()
    .prompt("Edit TODO list:")
    .initial_text(initial)
    .show_line_numbers(true)
    .color(true)
    .build();

  match editor.collect() {
    Ok(Some(text)) => println!("Updated:\n{}", text),
    Ok(None) => println!("Cancelled"),
    Err(e) => eprintln!("Error: {}", e),
  }
}
```

### Example 4: Integration with `.plan.please`

```rust
// In wplan/src/job_routines.rs

use multiline_input::Builder;

pub fn plan_please_routine(
  cmd: VerifiedCommand,
  _ctx: ExecutionContext,
) -> Result<OutputData, ErrorData> {
  let message = cmd.get_string("command").filter(|s| !s.is_empty());

  // Collect message interactively if not provided
  let message = if message.is_none() {
    let editor = Builder::new()
      .prompt("Enter message for AI assistant:")
      .placeholder("Describe what you want to implement...")
      .show_status(true)
      .color(true)
      .build();

    match editor.collect() {
      Ok(Some(text)) => Some(text),
      Ok(None) => {
        return Ok(OutputData::new("Cancelled", "text"));
      }
      Err(e) => {
        return Err(ErrorData::new(format!("Failed to collect input: {}", e)));
      }
    }
  } else {
    message
  };

  // Rest of implementation...
  let please_cmd = format!(
    "{} .please message::\"{}\" topic::{}",
    wplan_path,
    message.unwrap().replace('"', "\\\""),
    topic_name,
  );

  // Queue via .plan...
}
```

---

## Testing Strategy

### Test Organization

**Architecture**: All tests located in `tests/` directory with domain-based organization.

**Test Suite Stats**: 44 tests across 9 test files (as of Phase 2 completion)
- 9 buffer operation tests
- 10 key handling tests
- 3 builder configuration tests
- 2 render config tests
- 3 validation tests
- 2 terminal basic tests
- 5 integration workflow tests
- 10 error path tests (Phase 2)
- 1 API surface test

**Error Path Coverage**: 100% coverage for critical error scenarios:
- NoTty error (stdin not a TTY) - 2 tests
- TerminalTooSmall error (dimensions below 20x3 minimum) - 5 tests including boundary conditions
- Raw mode failures - 1 test
- Error message quality validation - 2 tests

**Structure**:
```
tests/
├── common/
│   ├── mod.rs
│   └── mock_terminal.rs       # MockTerminal test double
├── buffer_operations_test.rs  # TextBuffer unit tests
├── key_handling_test.rs       # Key event handling tests
├── builder_config_test.rs     # Builder pattern tests
├── validation_test.rs         # Input validation tests
├── render_config_test.rs      # Rendering configuration tests
├── terminal_basic_test.rs     # Terminal abstraction tests
├── integration_workflows_test.rs  # End-to-end workflow tests
├── error_paths_test.rs        # Error path tests (Phase 2)
├── readme.md                  # Test suite documentation
└── api_surface_test.rs        # Public API tests
```

### Unit Tests (Domain-Based)

**File**: `tests/buffer_operations_test.rs`
```rust
use multiline_input::buffer::TextBuffer;

#[ test ]
fn test_insert_char()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( 'a' );
  buffer.insert_char( 'b' );
  assert_eq!( buffer.text(), "ab" );
  assert_eq!( buffer.cursor_col(), 2 );
}

#[ test ]
fn test_unicode_handling()
{
  let mut buffer = TextBuffer::new();
  buffer.insert_char( '😀' );
  buffer.insert_char( '👍' );
  assert_eq!( buffer.text(), "😀👍" );
  assert_eq!( buffer.cursor_col(), 2 );
}
```

### Integration Tests (MockTerminal-Based)

**File**: `tests/integration_workflows_test.rs`

**Design Principle**: Use MockTerminal for deterministic, non-fragile integration testing.
All terminal state and key events are explicitly programmed by test code.

```rust
use multiline_input::Builder;
use common::mock_terminal::{ MockTerminal, key };
use crossterm::event::{ KeyCode, KeyModifiers };

#[ test ]
fn test_submit_single_line_workflow()
{
  // ARRANGE: Create mock terminal with explicit state
  let mut terminal = MockTerminal::new(
    true,              // is_tty = true (explicit)
    ( 80, 24 )         // size = 80x24 (explicit)
  );

  // Program key sequence: h → e → l → l → o → ENTER
  terminal.push_key( key( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'e' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'l' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'l' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'o' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) );

  let editor = Builder::new()
    .prompt( "Input:" )
    .build_with( terminal );

  // ACT: Collect input
  let result = editor.collect();

  // ASSERT: Verify result
  assert!( result.is_ok() );
  let text = result.unwrap();
  assert_eq!( text, Some( "hello".to_string() ) );
}

#[ test ]
fn test_multiline_editing_workflow()
{
  let mut terminal = MockTerminal::new( true, ( 80, 24 ) );

  // Program key sequence: h → i → CTRL+ENTER → b → y → ENTER
  terminal.push_key( key( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'i' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::CONTROL ) );
  terminal.push_key( key( KeyCode::Char( 'b' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'y' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) );

  let editor = Builder::new()
    .prompt( "Input:" )
    .build_with( terminal );

  let result = editor.collect();

  assert!( result.is_ok() );
  let text = result.unwrap();
  assert_eq!( text, Some( "hi\nby".to_string() ) );
}
```

### Error Path Tests (Phase 2)

**File**: `tests/error_paths_test.rs`

**Purpose**: Achieve 100% coverage of critical error scenarios with explicit tests.

**Design Principle**: Every error variant in `Error` enum must have:
- Test that triggers the error
- Test that validates error message quality
- Boundary condition tests where applicable

**Example: NoTty Error** (stdin not a TTY):
```rust
/// Test NoTty error when stdin is not a TTY
///
/// ## Root Cause
/// Editor requires interactive terminal (TTY). When stdin is redirected
/// (e.g., `program < file`), is_tty() returns false.
///
/// ## Why Not Caught
/// No previous test verified behavior in non-TTY environment.
///
/// ## Fix Applied
/// MockTerminal allows configuring is_tty=false to simulate stdin redirect.
///
/// ## Prevention
/// All error paths must have explicit test coverage.
///
/// ## Pitfall
/// Forgetting to test non-interactive environments leads to confusing
/// failures when users pipe input.
#[ test ]
fn test_error_no_tty()
{
  // Given: Terminal that is NOT a TTY (simulates stdin redirect)
  let terminal = MockTerminal::new( false, ( 80, 24 ) );

  // When: Attempt to collect input
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should return NoTty error
  assert!(
    matches!( result, Err( Error::NoTty ) ),
    "Expected NoTty error, got {:?}",
    result
  );
}
```

**Example: TerminalTooSmall Error** (dimensions below minimum):
```rust
/// Test TerminalTooSmall error when width below minimum
#[ test ]
fn test_error_terminal_too_narrow()
{
  // Given: Terminal width below minimum (19 < 20)
  let mut terminal = MockTerminal::new( true, ( 19, 24 ) );
  terminal.push_key( key( KeyCode::Char( 'a' ), KeyModifiers::NONE ) );

  // When: Attempt to collect input
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should return TerminalTooSmall error with diagnostic info
  match result
  {
    Err( Error::TerminalTooSmall { width, height, min_width, min_height } ) =>
    {
      assert_eq!( width, 19, "Should report actual width" );
      assert_eq!( height, 24, "Should report actual height" );
      assert_eq!( min_width, 20, "Should report required minimum width" );
      assert_eq!( min_height, 3, "Should report required minimum height" );
    }
    other => panic!( "Expected TerminalTooSmall error, got {:?}", other ),
  }
}
```

**Example: Boundary Condition Test** (exact minimum size):
```rust
/// Test terminal at exact minimum size works (boundary condition)
#[ test ]
fn test_terminal_exactly_minimum_size()
{
  // Given: Terminal at exact minimum (20x3)
  let mut terminal = MockTerminal::new( true, ( 20, 3 ) );
  terminal.push_key( key( KeyCode::Char( 'a' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) );

  // When: Collect input
  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Then: Should succeed (minimum size is sufficient)
  assert!( result.is_ok(), "Minimum size (20x3) should be sufficient" );
  assert_eq!( result.unwrap(), Some( "a".to_string() ) );
}
```

**Test Documentation Format** (5-section format):
Each error path test includes comprehensive doc comments:
1. **Root Cause**: Why this error occurs
2. **Why Not Caught**: Why existing tests didn't catch it
3. **Fix Applied**: How the test was implemented
4. **Prevention**: How to avoid similar gaps
5. **Pitfall**: Lessons learned for future development

### Manual Testing Checklist

```markdown
## Manual Testing Checklist

### Basic Input
- [ ] Type single character, press ENTER → submits
- [ ] Type multiple characters, press ENTER → submits
- [ ] Press ENTER on empty input → submits empty (if allowed)
- [ ] Press ESC → cancels, returns None
- [ ] Press CTRL+C → cancels, returns None

### Multiline
- [ ] Type "line1", press CTRL+ENTER, type "line2", press ENTER → submits "line1\nline2"
- [ ] Create 10 lines → all lines visible
- [ ] Create 100 lines → scrolls properly (future enhancement)

### Editing
- [ ] Type "abc", press Left Arrow twice, type "x" → "axbc"
- [ ] Type "abc", press Home, type "x" → "xabc"
- [ ] Type "abc", press End, type "x" → "abcx"
- [ ] Type "abc", press Backspace → "ab"
- [ ] Type "abc", press Left, press Delete → "ac"

### Cursor Movement
- [ ] Up/Down arrows move between lines correctly
- [ ] Left/Right arrows move within line correctly
- [ ] CTRL+Home → moves to start of text
- [ ] CTRL+End → moves to end of text

### Unicode
- [ ] Type emoji "😀" → displays correctly
- [ ] Type combined characters "é" (e + combining acute) → cursor position correct
- [ ] Type RTL text (Arabic, Hebrew) → displays correctly

### Validation
- [ ] Min length validation shows error on too-short input
- [ ] Max length validation prevents typing beyond limit
- [ ] Custom validator error message displays

### Visual
- [ ] Line numbers display correctly (if enabled)
- [ ] Status line shows correct line/column/char count
- [ ] Colors display in supported terminals
- [ ] Prompt message displays correctly
```

---

## Implementation Phases

### Phase 1: MVP (Core Functionality)
**Estimated**: 8-12 hours

**Deliverables**:
- [ ] Basic `TextBuffer` with insert/delete/cursor movement
- [ ] Raw terminal mode handling
- [ ] Key event parsing (ENTER, CTRL+ENTER, ESC, arrows, backspace)
- [ ] Simple rendering (no colors, no line numbers)
- [ ] `collect()` function that works
- [ ] Basic tests
- [ ] Example: basic_usage.rs

**Success Criteria**:
- Can collect multiline input
- ENTER submits, CTRL+ENTER adds newline
- ESC cancels
- Basic editing works (backspace, cursor movement)

### Phase 2: Polish & Features
**Estimated**: 6-8 hours

**Deliverables**:
- [ ] Builder API
- [ ] Validation (min/max length, custom validator)
- [ ] Line numbers
- [ ] Status line
- [ ] Colors
- [ ] Initial text support
- [ ] Placeholder text
- [ ] Comprehensive tests

**Success Criteria**:
- All Builder options work
- Validation errors display properly
- Visual enhancements improve UX

### Phase 3: Advanced Features (Optional)
**Estimated**: 8-12 hours

**Deliverables**:
- [ ] Scrolling for large texts
- [ ] Search/replace
- [ ] Undo/redo
- [ ] Clipboard integration (paste)
- [ ] History (recall previous inputs)
- [ ] Autocomplete hooks

**Success Criteria**:
- Handles large texts (1000+ lines)
- Advanced editing features work smoothly

### Phase 4: Documentation & Publication
**Estimated**: 4-6 hours

**Deliverables**:
- [ ] Comprehensive README.md
- [ ] API documentation with examples
- [ ] Usage guide
- [ ] CHANGELOG.md
- [ ] Publish to crates.io

---

## Performance Considerations

### Memory

**Current Text**: `O(n)` where n = text length
**Lines**: `O(m)` where m = number of lines
**Total**: `O(n + m)` ≈ `O(n)` for typical texts

**Optimization**: Use rope data structure for very large texts (>100KB).

### Rendering

**Current**: Full screen redraw on every key event
**Cost**: `O(lines * avg_line_length)` per render
**Acceptable**: For texts <1000 lines

**Optimization**: Delta rendering (only redraw changed lines).

### Key Event Processing

**Latency**: <1ms per key event (crossterm overhead + processing)
**Acceptable**: Yes, imperceptible to user

---

## Cross-Platform Considerations

### Linux/macOS
- ✅ ANSI terminal codes fully supported
- ✅ Raw mode works reliably
- ✅ All key combinations supported

### Windows
- ⚠️ Windows Terminal: Full support
- ⚠️ cmd.exe: Limited color support, CTRL+ENTER may not work
- ⚠️ ConEmu: Full support
- **Mitigation**: Detect terminal capabilities, graceful degradation

### SSH/Remote
- ✅ Works over SSH
- ⚠️ Some key combinations may be intercepted by client
- **Mitigation**: Document known limitations, provide alternative key bindings

---

## Security Considerations

### Input Sanitization
- **Risk**: Malicious terminal escape sequences in input
- **Mitigation**: Render with `crossterm` which auto-escapes

### Terminal State
- **Risk**: Raw mode left enabled on panic
- **Mitigation**: Use RAII guard (RawModeGuard)

### Buffer Overflow
- **Risk**: Unbounded input causes OOM
- **Mitigation**: Optional max_length validation

---

## Future Enhancements

**Not in MVP scope, consider for future versions**:

1. **Syntax Highlighting**: Color code input based on language
2. **Modal Editing**: Vim-like keybindings (i/ESC/hjkl)
3. **Mouse Support**: Click to position cursor
4. **Split View**: Edit multiple inputs side-by-side
5. **Diff View**: Show changes vs. initial text
6. **Auto-save**: Periodically save to temp file
7. **Crash Recovery**: Restore from temp file after crash
8. **Templates**: Load common messages from template file
9. **Macros**: Record and replay key sequences
10. **Plugins**: Hook system for extensions

---

## Dependencies

### Direct Dependencies
```toml
crossterm = "0.27"              # Terminal control (raw mode, events, colors)
unicode-width = "0.1"           # Display width of Unicode chars
unicode-segmentation = "1.10"   # Grapheme cluster iteration
```

### Optional Dependencies
```toml
# For syntax highlighting (future)
syntect = "5.0"

# For error handling (mandatory per PRO rulebook)
# Provides unified facade over thiserror/anyhow
error_tools = "0.35.0"
```

### Dev Dependencies
```toml
tempfile = "3.8"  # For integration tests
```

---

## Success Metrics

### Functional
- ✅ Collects multiline input reliably
- ✅ ENTER submits, CTRL+ENTER newline (as specified)
- ✅ All key bindings work
- ✅ Zero panics on invalid input

### Performance
- ✅ <10ms key-to-screen latency
- ✅ Handles 1000-line texts without lag
- ✅ <10KB memory overhead

### Usability
- ✅ Zero-config `collect()` works out of box
- ✅ Clear visual feedback
- ✅ Intuitive key bindings

### Quality
- ✅ >80% test coverage
- ✅ Zero clippy warnings
- ✅ Comprehensive documentation
- ✅ Published to crates.io

---

## Integration Plan for wplan

### Modification to wplan

**File**: `module/wplan/Cargo.toml`
```toml
[dependencies]
multiline_input = { path = "../multiline_input" }
```

**File**: `module/wplan/src/job_routines.rs`
```rust
use multiline_input::Builder;

pub fn plan_please_routine(...) -> Result<...> {
  let message = cmd.get_string("command").filter(|s| !s.is_empty());

  let message = if message.is_none() {
    let editor = Builder::new()
      .prompt("Enter message for AI assistant (ENTER=submit, CTRL+ENTER=newline, ESC=cancel):")
      .show_status(true)
      .color(true)
      .build();

    match editor.collect() {
      Ok(Some(text)) => Some(text),
      Ok(None) => {
        return Ok(OutputData::new("Cancelled", "text"));
      }
      Err(multiline_input::Error::NoTty) => {
        // Fallback to old behavior when no TTY
        None
      }
      Err(e) => {
        return Err(ErrorData::new(format!("Input error: {}", e)));
      }
    }
  } else {
    message
  };

  // Rest of implementation unchanged...
}
```

---

## Conclusion

This specification provides a complete roadmap for implementing a general-purpose multiline input crate with:

- **Simple API**: `collect()` for basic use
- **Rich UX**: ENTER to submit, CTRL+ENTER for newline
- **Extensibility**: Builder pattern for customization
- **Reliability**: Comprehensive error handling, terminal cleanup
- **Performance**: Acceptable for typical CLI use cases
- **Cross-platform**: Works on Linux, macOS, Windows

**Implementation Priority**: MVP first (Phase 1), then polish (Phase 2), defer advanced features (Phase 3) based on user feedback.

**Total Estimated Effort**:
- MVP: 8-12 hours
- Polish: 6-8 hours
- Total: 14-20 hours for production-ready v0.1.0
