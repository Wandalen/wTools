# Test Suite Organization

This directory contains all unit tests for the multiline_input crate.

### Responsibility Table

| Test Suite | Responsibility | In Scope | Out of Scope (See) |
|------------|----------------|----------|-------------------|
| `buffer_operations_test.rs` | Text buffer operations | Text insertion/deletion, cursor movement, multiline editing, Unicode handling | Validation (→ validation tests), Key handling (→ key_handling tests) |
| `key_handling_test.rs` | Key event handling | Key event parsing, action mapping (submit, cancel, edit, movement), special key combinations (CTRL+ENTER, SHIFT+ENTER) | Buffer operations (→ buffer_operations tests), Integration (→ integration_workflows tests) |
| `integration_workflows_test.rs` | End-to-end workflows | Complete workflows using MockTerminal (submit, cancel, multiline, validation, SHIFT+ENTER) | Buffer operations (→ buffer_operations tests), Key handling (→ key_handling tests) |
| `error_paths_test.rs` | Error handling and failure modes | Error injection with MockTerminal (NoTty, TerminalTooSmall, raw mode failures), error message formatting, boundary conditions | Integration workflows (→ integration_workflows tests) |
| `validation_test.rs` | Input validation | Empty input validation, min/max length validation, initial text handling | Buffer operations (→ buffer_operations tests), Integration (→ integration_workflows tests) |
| `builder_config_test.rs` | Builder pattern and configuration | Builder defaults, method chaining, configuration propagation | Validation (→ validation tests), Integration (→ integration_workflows tests) |
| `render_config_test.rs` | Rendering configuration | Render config defaults, custom config creation | Integration (→ integration_workflows tests) |
| `terminal_basic_test.rs` | Terminal abstraction | Terminal creation, size querying | Error paths (→ error_paths tests), Integration (→ integration_workflows tests) |
| `api_surface_test.rs` | Public API | API types exposure verification | All functional tests (→ domain-specific suites) |

## Organization (9 test files)

Tests organized by functional domain (see Responsibility Table above).

### Scope

This test suite covers the multiline_input crate's multiline terminal editor functionality:

**In Scope:**
- Text buffer operations (insertion, deletion, cursor movement, Unicode)
- Key event handling and action mapping
- Input validation (empty, min/max length)
- Builder pattern and configuration
- Rendering configuration
- Terminal abstraction with trait-based dependency injection
- End-to-end workflows with MockTerminal
- Error handling (NoTty, TerminalTooSmall, raw mode failures)
- Public API surface verification

**Out of Scope:**
- Actual terminal I/O (uses MockTerminal for deterministic testing)
- Performance benchmarks (no benches/ yet)
- Advanced Unicode edge cases (emoji, RTL, combining chars) - identified gaps
- Dynamic terminal resize handling - identified gap
- Line/screen overflow scenarios - identified gap

**Test Quality**: 44 deterministic tests using MockTerminal for non-fragile, environment-independent testing. All tests fail loudly with clear error messages.

## Test Files

### buffer_operations_test.rs (8 tests)
**Domain**: Text buffer operations

Tests for TextBuffer functionality:
- Text insertion and deletion
- Cursor movement and positioning
- Multiline editing operations
- Unicode character handling

**Key test cases**:
- `test_new_buffer` - Buffer initialization
- `test_insert_char` - Character insertion
- `test_insert_newline` - Newline insertion
- `test_delete_char_before` - Backspace deletion
- `test_delete_newline` - Newline deletion
- `test_cursor_movement` - Cursor navigation
- `test_unicode_handling` - Unicode characters

### builder_config_test.rs (3 tests)
**Domain**: Builder pattern and configuration

Tests for Builder pattern and editor configuration:
- Default configuration values
- Builder method chaining
- Configuration propagation

**Key test cases**:
- `test_builder_default` - Default values
- `test_builder_configuration` - Method chaining
- `test_builder_builds_editor` - Editor creation

### validation_test.rs (5 tests)
**Domain**: Input validation

Tests for editor input validation logic:
- Empty input validation
- Minimum length validation
- Maximum length validation
- Initial text handling

**Key test cases**:
- `test_editor_creation` - Editor with validation
- `test_validation_empty` - Empty input rules
- `test_validation_min_length` - Minimum length enforcement
- `test_validation_max_length` - Maximum length enforcement
- `test_initial_text` - Initial text support

### key_handling_test.rs (9 tests)
**Domain**: Key event handling

Tests for key event parsing and action mapping:
- Submit actions (ENTER, CTRL+D)
- Cancel actions (ESC, CTRL+C)
- Text editing (character insertion, backspace)
- Cursor movement (arrows)
- Special key combinations (CTRL+ENTER, SHIFT+ENTER)

**Key test cases**:
- `test_submit_on_enter` - Submit trigger
- `test_newline_on_ctrl_enter` - CTRL+ENTER newline insertion
- `test_newline_on_shift_enter` - SHIFT+ENTER newline insertion
- `test_newline_on_ctrl_shift_enter` - CTRL+SHIFT+ENTER newline insertion
- `test_cancel_on_esc` - Cancel trigger
- `test_cancel_on_ctrl_c` - Alternative cancel
- `test_char_insertion` - Character input
- `test_backspace` - Backspace handling
- `test_cursor_movement` - Arrow key movement

### render_config_test.rs (2 tests)
**Domain**: Rendering configuration

Tests for render configuration:
- Default render config values
- Custom config creation

**Key test cases**:
- `test_render_config_default` - Default values
- `test_render_config_creation` - Custom configuration

### terminal_basic_test.rs (2 tests)
**Domain**: Terminal abstraction

Tests for terminal abstraction layer:
- Terminal creation
- Size querying

**Key test cases**:
- `test_terminal_creation` - Terminal initialization
- `test_size_query` - Terminal size detection

### api_surface_test.rs (1 test)
**Domain**: Public API

Tests for public API surface:
- API types are exposed correctly

**Key test cases**:
- `test_api_exists` - API availability

### integration_workflows_test.rs (5 tests)
**Domain**: End-to-end workflows

Integration tests using MockTerminal for deterministic testing:
- Complete input submission workflows
- Cancel workflows
- Multiline editing workflows
- Validation workflows
- SHIFT+ENTER newline insertion

**Key test cases**:
- `test_submit_single_line_workflow` - Complete submit flow
- `test_cancel_on_esc_workflow` - Cancel flow
- `test_multiline_editing_workflow` - CTRL+ENTER multiline
- `test_validation_empty_workflow` - Validation rejection + retry
- `test_shift_enter_inserts_newline_workflow` - SHIFT+ENTER newline

### error_paths_test.rs (10 tests)
**Domain**: Error handling and failure modes

Error path tests using MockTerminal for deterministic error injection:
- NoTty errors (stdin not a TTY)
- TerminalTooSmall errors (dimensions below minimum)
- Raw mode enable failures
- Error message formatting
- Boundary conditions (exact minimum size)

**Key test cases**:
- `test_error_no_tty` - NoTty error when is_tty=false
- `test_error_no_tty_message` - NoTty error message format
- `test_error_terminal_too_narrow` - Width below minimum (19 < 20)
- `test_error_terminal_too_short` - Height below minimum (2 < 3)
- `test_error_terminal_too_small_both` - Both dimensions below minimum
- `test_terminal_exactly_minimum_size` - Boundary test (20x3 works)
- `test_error_terminal_too_small_message` - Error message includes dimensions
- `test_error_raw_mode_enable_non_tty` - Raw mode fails on non-TTY
- `test_error_display_all_variants` - All error variants display correctly
- `test_error_source` - Error::source() implementation

### common/mock_terminal.rs (Test Utilities)
**Purpose**: Test double for TerminalOps trait

Provides MockTerminal for non-fragile integration testing:
- Implements TerminalOps with programmable behavior
- Allows pre-programming key event sequences
- Captures output for verification
- Fully deterministic, no environment dependencies

**Key features**:
- `MockTerminal::new(is_tty, size)` - Explicit configuration
- `push_key(event)` - Program key events
- `output()` - Capture terminal output
- `key(code, modifiers)` - Helper for creating key events

## Total Test Coverage

**44 tests** covering:
- Buffer operations (7 tests)
- Builder configuration (3 tests)
- Input validation (5 tests)
- Key handling (9 tests)
- Rendering (2 tests)
- Terminal operations (2 tests)
- Public API (1 test)
- Integration workflows (5 tests)
- **Error paths (10 tests)** ✅ NEW

## Running Tests

Run all tests:
```bash
cargo test
```

Run specific test file:
```bash
cargo test --test buffer_operations_test
```

Run with output:
```bash
cargo test -- --nocapture
```

## Test Quality Standards

All tests in this suite:
- Use explicit parameters (no environment dependencies)
- Are deterministic (same input → same output)
- Fail loudly with clear error messages
- Document their domain and purpose

## Architecture

### Trait-Based Dependency Injection

The crate uses trait-based dependency injection for testability:
- `TerminalOps` trait abstracts all terminal operations
- `RealTerminal` provides production implementation
- `MockTerminal` provides test double for integration tests

This enables non-fragile testing without environment dependencies.

## Usage Examples: MockTerminal

### Example 1: Basic Single-Line Input Test

```rust
use multiline_input::Builder;
use common::mock_terminal::{ MockTerminal, key };
use crossterm::event::{ KeyCode, KeyModifiers };

#[ test ]
fn test_basic_input()
{
  // Step 1: Create MockTerminal with explicit state
  let mut terminal = MockTerminal::new(
    true,      // is_tty = true (terminal connected)
    ( 80, 24 ) // size = 80 cols × 24 rows
  );

  // Step 2: Program the key event sequence
  terminal.push_key( key( KeyCode::Char( 'h' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'i' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) ); // Submit

  // Step 3: Create editor with MockTerminal
  let editor = Builder::new()
    .prompt( "Input:" )
    .build_with( terminal ); // Dependency injection

  // Step 4: Execute and assert
  let result = editor.collect();
  assert_eq!( result.unwrap(), Some( "hi".to_string() ) );
}
```

### Example 2: Multiline Input with CTRL+ENTER

```rust
#[ test ]
fn test_multiline()
{
  let mut terminal = MockTerminal::new( true, ( 80, 24 ) );

  // Program: "line1" → CTRL+ENTER → "line2" → ENTER
  terminal.push_key( key( KeyCode::Char( 'l' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'i' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'n' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'e' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( '1' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::CONTROL ) ); // Newline
  terminal.push_key( key( KeyCode::Char( 'l' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'i' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'n' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'e' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( '2' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Enter, KeyModifiers::NONE ) ); // Submit

  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  assert_eq!( result.unwrap(), Some( "line1\nline2".to_string() ) );
}
```

### Example 3: Testing Error Paths (NoTty)

```rust
#[ test ]
fn test_no_tty_error()
{
  // Configure MockTerminal to simulate non-TTY environment
  let terminal = MockTerminal::new(
    false,     // is_tty = false (stdin redirected)
    ( 80, 24 )
  );

  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Assert error type
  assert!( matches!( result, Err( Error::NoTty ) ) );
}
```

### Example 4: Testing Terminal Size Validation

```rust
#[ test ]
fn test_terminal_too_small()
{
  // Configure terminal below minimum (need 20x3, provide 19x2)
  let mut terminal = MockTerminal::new( true, ( 19, 2 ) );
  terminal.push_key( key( KeyCode::Char( 'x' ), KeyModifiers::NONE ) );

  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Assert error type and diagnostic information
  match result
  {
    Err( Error::TerminalTooSmall { width, height, min_width, min_height } ) =>
    {
      assert_eq!( width, 19 );
      assert_eq!( height, 2 );
      assert_eq!( min_width, 20 );
      assert_eq!( min_height, 3 );
    }
    _ => panic!( "Expected TerminalTooSmall error" ),
  }
}
```

### Example 5: Testing Cancellation

```rust
#[ test ]
fn test_cancel_with_esc()
{
  let mut terminal = MockTerminal::new( true, ( 80, 24 ) );

  // Type some text, then press ESC
  terminal.push_key( key( KeyCode::Char( 'a' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Char( 'b' ), KeyModifiers::NONE ) );
  terminal.push_key( key( KeyCode::Esc, KeyModifiers::NONE ) ); // Cancel

  let editor = Builder::new().build_with( terminal );
  let result = editor.collect();

  // Cancelled input returns None
  assert_eq!( result.unwrap(), None );
}
```

### Key Principles for MockTerminal Usage

1. **Explicit Configuration**: Always specify `is_tty` and `size` explicitly
2. **Deterministic Sequences**: Pre-program all key events before calling `collect()`
3. **No Environment Dependencies**: MockTerminal never interacts with actual terminal
4. **Fast Execution**: Tests run instantly, no waiting for user input
5. **Error Injection**: Easy to simulate error conditions (non-TTY, small terminal)

## Coverage Progress

**Completed** (as of 2025-11-16):
- ✅ Error path tests (10 tests) - NoTty, TerminalTooSmall, error messages
- ✅ Basic integration tests (5 tests) - Submit, cancel, multiline workflows
- ✅ Trait-based architecture - Enables non-fragile testing

**Remaining Gaps** (identified 2025-11-15, see docs/architecture.md):
- Unicode edge cases (emoji, RTL, combining chars) - 10 tests planned
- Buffer edge cases (boundaries, cursor clamping) - 12 tests planned
- Key handling edge cases (Home/End, Delete, Tab) - 8 tests planned
- Render edge cases (overflow, wide chars) - 6 tests planned

**Total Coverage**: 44/132 scenarios (33%)

The trait-based architecture is now in place, enabling all planned test additions.

---

## Known Test Gaps (Detailed)

### Unicode Edge Cases (P3 Priority)

**Status**: ⚠️ Assumed correct based on unicode-segmentation crate, but not explicitly tested

**Missing Tests**:
1. **Emoji handling** - Multi-codepoint emoji (👨‍👩‍👧‍👦) should move as single grapheme
2. **Combining characters** - é as (e + combining acute) should be one cursor position
3. **RTL text** - Arabic (مرحبا), Hebrew (שלום) cursor movement
4. **Zero-width joiners** - Flag emojis using ZWJ sequences
5. **Wide characters** - Emoji width affects column calculation (CJK characters)

**Why Not Tested**: Investigation (2025-11-15) validated that `unicode-segmentation` crate handles these correctly, but explicit validation tests remain TODO.

**How to Test**: Use MockTerminal to program Unicode input sequences and assert correct cursor positions.

### Buffer Edge Cases (P2 Priority)

**Missing Tests**:
1. **Empty buffer operations** - Backspace/Delete at (0,0) should be no-op
2. **Cursor clamping** - Move from long line (10 chars) to short line (3 chars)
3. **Boundary deletion** - Delete newline at start/end of buffer
4. **Multi-line boundaries** - Cursor movement at first/last line edges

**Why Not Tested**: Basic functionality covered, but edge cases can reveal off-by-one errors.

### Performance Validation (P3 Priority)

**Missing Tests**:
1. **Large text handling** - 1MB text (spec claims support, NFR1.2)
2. **Long line performance** - 10,000+ char single line
3. **Many line performance** - 10,000+ lines

**Why Not Tested**: No `benches/` directory exists yet. Performance untested but works for typical use (<100KB).

**Proper Fix**: Create `benches/` directory with criterion benchmarks.

### Terminal Size Edge Cases (Partially Tested)

**Tested**:
- ✅ Below minimum (19x2) → TerminalTooSmall error
- ✅ Exact minimum (20x3) → Works correctly

**Missing Tests**:
1. **Dynamic resize** - Terminal resized during input
2. **Line overflow** - Line longer than terminal width
3. **Screen overflow** - More lines than terminal height

**Priority**: P2 (affects real-world usage)
