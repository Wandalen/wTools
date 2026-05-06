# multiline_input - Manual Testing Guide

Manual testing procedures for the multiline_input terminal widget library.

## Overview

This document covers manual testing scenarios for multiline_input that verify functionality not easily covered by automated tests:
- Terminal interaction and key bindings
- Visual rendering and formatting
- Cursor movement and editing behavior
- Validation and error feedback
- Cross-platform terminal compatibility

**Note**: This is a library, not a standalone CLI. Tests use example programs to exercise the library.

## Prerequisites

### 1. Build Examples

```bash
cd /home/user1/pro/lib/willbe/module/multiline_input
cargo build --examples --release
```

### 2. Terminal Requirements

- ANSI color support
- UTF-8 encoding
- Standard terminal size (80x24 minimum)

### 3. Test Terminals

Test on multiple terminal emulators:
- **Linux**: gnome-terminal, konsole, alacritty
- **macOS**: Terminal.app, iTerm2
- **Windows**: Windows Terminal, ConEmu

## Test Scenarios

### Test 1: Basic Text Input

**Objective**: Verify simple text input works

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Manual Steps**:
1. Type: `Hello World`
2. Press ENTER

**Expected Results**:
- ✅ Text appears as typed
- ✅ Cursor follows typing
- ✅ ENTER submits input
- ✅ Program prints: `You entered: Hello World`
- ✅ Exit code 0

**Success Criteria**:
- Text is readable
- Cursor is visible
- Submit works correctly

### Test 2: Multiline Input

**Objective**: Verify CTRL+ENTER creates newlines

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Manual Steps**:
1. Type: `Line 1`
2. Press CTRL+ENTER
3. Type: `Line 2`
4. Press CTRL+ENTER
5. Type: `Line 3`
6. Press ENTER

**Expected Results**:
- ✅ CTRL+ENTER adds newlines (not submits)
- ✅ Cursor moves to next line
- ✅ All lines visible during editing
- ✅ ENTER submits all three lines
- ✅ Output shows all three lines

**Success Criteria**:
- Multiline editing works
- Line breaks preserved
- Submit includes all lines

### Test 3: Cursor Movement - Arrows

**Objective**: Verify arrow key navigation

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Manual Steps**:
1. Type: `ABCDEF`
2. Press LEFT 3 times (cursor at D)
3. Type `X` (should insert, not overwrite)
4. Press RIGHT 2 times
5. Type `Y`
6. Press ENTER

**Expected Result**: `ABXCDEYF`

**Test Vertical Movement**:
1. Type: `Line 1`
2. Press CTRL+ENTER
3. Type: `Line 2`
4. Press UP (should move to Line 1)
5. Press END
6. Type ` end`
7. Press DOWN
8. Type ` end`
9. Press ENTER

**Expected Result**:
```
Line 1 end
Line 2 end
```

**Success Criteria**:
- ✅ LEFT/RIGHT move cursor horizontally
- ✅ UP/DOWN move cursor between lines
- ✅ Cursor position is accurate
- ✅ Text insertion works at cursor position

### Test 4: Home/End Keys

**Objective**: Verify Home/End navigation

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Manual Steps**:
1. Type: `Hello World`
2. Press HOME (cursor to start of line)
3. Type `>> ` (should prepend)
4. Press END (cursor to end of line)
5. Type ` <<` (should append)
6. Press ENTER

**Expected Result**: `>> Hello World <<`

**Test Multi-line**:
1. Type: `Line 1`
2. Press CTRL+ENTER
3. Type: `Line 2`
4. Press CTRL+HOME (to start of all text)
5. Type `START `
6. Press CTRL+END (to end of all text)
7. Type ` END`
8. Press ENTER

**Expected Result**:
```
START Line 1
Line 2 END
```

**Success Criteria**:
- ✅ HOME moves to start of current line
- ✅ END moves to end of current line
- ✅ CTRL+HOME moves to start of all text
- ✅ CTRL+END moves to end of all text

### Test 5: Backspace and Delete

**Objective**: Verify character deletion

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Test Backspace**:
1. Type: `Hello World`
2. Press BACKSPACE 5 times
3. Press ENTER

**Expected Result**: `Hello `

**Test Delete**:
1. Type: `Hello World`
2. Press HOME
3. Press DELETE 6 times
4. Press ENTER

**Expected Result**: `World`

**Test at Line Boundaries**:
1. Type: `Line 1`
2. Press CTRL+ENTER
3. Type: `Line 2`
4. Press UP, END
5. Press DELETE (should merge lines)
6. Press ENTER

**Expected Result**: `Line 1Line 2`

**Success Criteria**:
- ✅ BACKSPACE deletes before cursor
- ✅ DELETE deletes at cursor
- ✅ Line merging works correctly
- ✅ No crashes at text boundaries

### Test 6: Cancellation (ESC/CTRL+C)

**Objective**: Verify cancellation returns None

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Test ESC**:
1. Type: `Some text`
2. Press ESC

**Expected Results**:
- ✅ Program exits immediately
- ✅ Prints: `Cancelled`
- ✅ Exit code 0

**Test CTRL+C**:
1. Run example again
2. Type: `Some text`
3. Press CTRL+C

**Expected Results**:
- ✅ Program exits immediately
- ✅ Prints: `Cancelled`
- ✅ Exit code 0

**Success Criteria**:
- Both cancellation methods work
- Clean exit with no errors
- Typed text discarded

### Test 7: Visual Formatting with Line Numbers

**Objective**: Verify line numbers display correctly

**Execute**:
```bash
./target/release/examples/with_config
# (or modify basic_usage to enable line numbers)
```

**Manual Steps**:
1. Type: `Line 1`
2. Press CTRL+ENTER
3. Type: `Line 2`
4. Press CTRL+ENTER
5. Type: `Line 3`
6. Press ENTER

**Expected Display**:
```
1 │ Line 1
2 │ Line 2
3 │ Line 3
```

**Success Criteria**:
- ✅ Line numbers increment correctly
- ✅ Separator (│) is visible
- ✅ Alignment is clean
- ✅ Numbers update on line add/delete

### Test 8: Status Line

**Objective**: Verify status line shows position

**Execute**:
```bash
./target/release/examples/with_config
# (ensure show_status is enabled)
```

**Manual Steps**:
1. Type: `Hello World`
2. Observe status line

**Expected Status**: `Line 1, Col 12, 11 chars`

3. Press CTRL+ENTER
4. Type: `More text`

**Expected Status**: `Line 2, Col 10, 21 chars`

**Success Criteria**:
- ✅ Line number accurate
- ✅ Column number accurate
- ✅ Character count accurate
- ✅ Updates in real-time as you type

### Test 9: Validation - Minimum Length

**Objective**: Verify min length validation

**Execute**:
```bash
./target/release/examples/with_validation
# (ensure min_length is configured, e.g., 10)
```

**Manual Steps**:
1. Type: `Short`
2. Press ENTER

**Expected Results**:
- ✅ Validation error displayed
- ✅ Message: "Text must be at least 10 characters"
- ✅ Does NOT submit (stays in editor)
- ✅ Can continue editing

3. Type more text to meet minimum
4. Press ENTER

**Expected Results**:
- ✅ Validation passes
- ✅ Text submitted successfully

**Success Criteria**:
- Validation enforced on submit
- Clear error messages
- Can retry after validation failure

### Test 10: Validation - Maximum Length

**Objective**: Verify max length validation

**Execute**:
```bash
./target/release/examples/with_validation
# (ensure max_length is configured, e.g., 50)
```

**Manual Steps**:
1. Type 60 characters of text
2. Press ENTER

**Expected Results**:
- ✅ Validation error displayed
- ✅ Message: "Text must be at most 50 characters"
- ✅ Does NOT submit
- ✅ Can delete characters and retry

**Success Criteria**:
- Max length enforced
- Error message clear
- Can correct and resubmit

### Test 11: Custom Validation

**Objective**: Verify custom validator works

**Execute**:
```bash
./target/release/examples/with_validation
```

**Manual Steps**:
1. Type: `This contains spam word`
2. Press ENTER

**Expected Results**:
- ✅ Custom validation error displayed
- ✅ Message: "Message contains prohibited content"
- ✅ Does NOT submit

3. Delete "spam" and retype without it
4. Press ENTER

**Expected Results**:
- ✅ Validation passes
- ✅ Text submitted

**Success Criteria**:
- Custom validators work
- Error messages are custom
- Validation logic is respected

### Test 12: Pre-filled Text Editing

**Objective**: Verify editing pre-filled text works

**Execute**:
```bash
./target/release/examples/pre_filled
```

**Expected Initial State**:
```
- Task 1
- Task 2
- Task 3
```

**Manual Steps**:
1. Press END (move to end of line 1)
2. Type ` (edited)`
3. Press DOWN, DOWN
4. Press HOME, DELETE (delete "- ")
5. Press ENTER

**Expected Result**:
```
- Task 1 (edited)
- Task 2
Task 3
```

**Success Criteria**:
- ✅ Pre-filled text appears immediately
- ✅ Cursor starts at beginning
- ✅ All editing operations work
- ✅ Can modify pre-filled content

### Test 13: Color Output

**Objective**: Verify color rendering

**Execute**:
```bash
./target/release/examples/with_config
# (ensure color is enabled)
```

**Visual Inspection**:
- ✅ Prompt is colored (if configured)
- ✅ Status line is colored differently from text
- ✅ Line numbers are colored differently
- ✅ Colors are readable on both light and dark terminals

**Test Color Disabled**:
```bash
./target/release/examples/with_config
# (set color::false)
```

**Visual Inspection**:
- ✅ No color codes visible
- ✅ Still readable
- ✅ Layout unchanged

**Success Criteria**:
- Colors enhance readability
- Graceful degradation when disabled
- Works on various terminal themes

### Test 14: Terminal Size Changes

**Objective**: Verify handling of terminal resize

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Manual Steps**:
1. Type several lines of text
2. Resize terminal window (make narrower)
3. Continue typing
4. Resize terminal window (make wider)
5. Press ENTER

**Expected Results**:
- ✅ Text reflows on resize (if supported)
- ✅ No crashes on resize
- ✅ Cursor remains visible
- ✅ Can continue editing after resize

**Success Criteria**:
- Graceful handling of resize
- No data loss
- Editing continues normally

### Test 15: Unicode and Special Characters

**Objective**: Verify unicode support

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Test Cases**:
1. Type emoji: `Hello 👋 World 🌍`
2. Type accents: `Café résumé naïve`
3. Type CJK: `你好世界 こんにちは 안녕하세요`
4. Type symbols: `→ ≈ © ™ ± ≠`
5. Press ENTER for each

**Expected Results**:
- ✅ All characters display correctly
- ✅ Cursor position accurate
- ✅ Character count correct
- ✅ Backspace/Delete work properly with unicode

**Success Criteria**:
- Full unicode support
- Accurate character counting
- No rendering glitches

### Test 16: Long Lines

**Objective**: Verify handling of very long single lines

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Manual Steps**:
1. Type 200+ characters on a single line (no CTRL+ENTER)
2. Press LEFT repeatedly to navigate back
3. Press RIGHT to navigate forward
4. Press ENTER

**Expected Results**:
- ✅ Long line handles correctly
- ✅ Horizontal scrolling works (or line wraps)
- ✅ Cursor navigation works throughout
- ✅ Full text captured

**Success Criteria**:
- No truncation
- Navigation works
- Text captured completely

### Test 17: Empty Input

**Objective**: Verify handling of empty/whitespace input

**Execute**:
```bash
./target/release/examples/basic_usage
```

**Test Empty**:
1. Don't type anything
2. Press ENTER

**Expected Results** (if allow_empty is true):
- ✅ Submits empty string
- ✅ Program prints: `You entered: ` (empty)

**Test Whitespace Only**:
1. Type several spaces
2. Press ENTER

**Expected Results**:
- ✅ Whitespace preserved (not trimmed automatically)
- ✅ Output shows spaces

**Success Criteria**:
- Empty handling configurable
- Whitespace preserved when intended

### Test 18: Cross-Platform Compatibility

**Objective**: Verify functionality across platforms

**Test on Each Platform**:
- Linux (primary)
- macOS (if available)
- Windows (if available)

**For Each Platform, Test**:
1. Basic input and submit (ENTER)
2. Multiline (CTRL+ENTER)
3. Cursor movement (arrows)
4. Cancellation (ESC, CTRL+C)
5. Special keys (Home, End, Delete)

**Expected Results**:
- ✅ All tests pass on each platform
- ✅ Key bindings work correctly
- ✅ Terminal rendering is clean
- ✅ No platform-specific bugs

**Known Issues**:
- Windows cmd.exe: CTRL+ENTER may not work
- Windows cmd.exe: Limited ANSI color support

## Verification Checklist

**Basic Input**:
- [ ] Simple text input works
- [ ] Multiline input (CTRL+ENTER) works
- [ ] Submit (ENTER) works
- [ ] Cancellation (ESC, CTRL+C) works

**Navigation**:
- [ ] Arrow keys (LEFT, RIGHT, UP, DOWN) work
- [ ] Home/End keys work
- [ ] CTRL+Home/CTRL+End work
- [ ] Cursor position is accurate

**Editing**:
- [ ] Backspace deletes correctly
- [ ] Delete key works
- [ ] Text insertion at cursor works
- [ ] Line merging works

**Visual Rendering**:
- [ ] Line numbers display correctly (if enabled)
- [ ] Status line shows accurate info (if enabled)
- [ ] Colors render properly (if enabled)
- [ ] Layout is clean and readable

**Validation**:
- [ ] Minimum length enforced
- [ ] Maximum length enforced
- [ ] Custom validators work
- [ ] Error messages are clear

**Advanced Features**:
- [ ] Pre-filled text editing works
- [ ] Unicode characters supported
- [ ] Long lines handled
- [ ] Terminal resize handled
- [ ] Empty input allowed (if configured)

**Cross-Platform**:
- [ ] Linux terminal support verified
- [ ] macOS terminal support verified (if tested)
- [ ] Windows terminal support verified (if tested)

## Known Limitations

1. **Scrolling**: Long texts may not scroll properly (planned feature)
2. **Undo/Redo**: Not yet implemented
3. **Clipboard**: No copy/paste integration yet
4. **Windows cmd.exe**: Limited support, use Windows Terminal instead

## Reporting Issues

When reporting issues from manual testing:

1. **Include terminal type and version**:
   ```bash
   echo $TERM
   # Terminal emulator name and version
   ```

2. **Include example used**:
   ```bash
   ./target/release/examples/basic_usage
   ```

3. **Describe exact key sequence**:
   - "Typed 'Hello', pressed CTRL+ENTER, typed 'World', pressed ENTER"

4. **Expected vs actual behavior**

5. **Screenshot** showing visual issue (if rendering problem)

6. **OS and version**:
   ```bash
   uname -a  # Linux/macOS
   ver  # Windows
   ```

## References

- Terminal control sequences: <https://en.wikipedia.org/wiki/ANSI_escape_code>
- Rust termion crate: <https://docs.rs/termion/>
- Feature doc: `../../docs/feature/001_multiline_input.md`
- Examples: `../../examples/`
