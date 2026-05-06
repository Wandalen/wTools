# Module :: `multiline_input`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=multiline_input)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/multiline_input?color=e3e8f0&logo=docs.rs)](https://docs.rs/multiline_input) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fexperimental%2Fmultiline_input%2Fexamples%2Fwith_config.rs,RUN_POSTFIX=--example%20with_config/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

**Terminal multiline input with rich editing capabilities**

[![Crates.io](https://img.shields.io/crates/v/multiline_input.svg)](https://crates.io/crates/multiline_input)
[![Documentation](https://docs.rs/multiline_input/badge.svg)](https://docs.rs/multiline_input)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

### Scope

**Responsibility:**
- Terminal multiline input widget with rich editing capabilities
- ENTER to submit, CTRL+ENTER for newlines (optimized for AI assistants)
- Raw terminal mode with visual feedback and validation
- Cross-platform support (Linux, macOS, Windows)

**In Scope:**
- Multiline text input widget
- Line editing (cursor movement, backspace, delete)
- Terminal manipulation (raw mode, key event capture)
- Visual feedback (line numbers, status line, colors)
- Validation (min/max length, custom validators)
- Builder API for configuration
- Pre-filled text editing
- Key bindings (ENTER, CTRL+ENTER, ESC, arrows, Home, End)
- Cross-platform terminal support

**Out of Scope:**
- ❌ Wizard framework → delegated to `terminal_wizard` crate
- ❌ CLI applications → individual binaries use this as library
- ❌ Scrolling for large texts → future work
- ❌ Undo/redo → future work
- ❌ Clipboard integration → future work

## Features

- **ENTER** to submit input
- **CTRL+ENTER** to insert newline (multiline support)
- Rich line editing (cursor movement, backspace, delete)
- Visual feedback (line numbers, status line, colors)
- Validation (min/max length, custom validators)
- Cross-platform (Linux, macOS, Windows)
- Zero-config defaults

## Quick Start

```rust
use multiline_input::collect;

fn main() {
  match collect("Enter your message:") {
    Ok(Some(text)) => println!("You entered:\n{}", text),
    Ok(None) => println!("Cancelled"),
    Err(e) => eprintln!("Error: {}", e),
  }
}
```

## Key Bindings

| Key | Action |
|-----|--------|
| **ENTER** | Submit input and return |
| **CTRL+ENTER** | Insert newline |
| **ESC** | Cancel (returns None) |
| **CTRL+C** | Cancel (returns None) |
| **CTRL+D** | Submit (alternative to ENTER) |
| **Backspace** | Delete character before cursor |
| **Delete** | Delete character at cursor |
| **←/→** | Move cursor left/right |
| **↑/↓** | Move cursor up/down (between lines) |
| **Home** | Move to start of current line |
| **End** | Move to end of current line |
| **CTRL+Home** | Move to start of text |
| **CTRL+End** | Move to end of text |

## Advanced Usage

### With Builder Pattern

```rust
use multiline_input::Builder;

let editor = Builder::new()
  .prompt("Enter commit message:")
  .min_length(10)
  .max_length(500)
  .show_line_numbers(true)
  .show_status(true)
  .color(true)
  .build();

match editor.collect() {
  Ok(Some(msg)) => println!("Commit:\n{}", msg),
  Ok(None) => println!("Cancelled"),
  Err(e) => eprintln!("Error: {}", e),
}
```

### With Validation

```rust
use multiline_input::Builder;

let editor = Builder::new()
  .prompt("Enter message:")
  .validator(|text| {
    if text.contains("spam") {
      Err("Message contains prohibited content".to_string())
    } else {
      Ok(())
    }
  })
  .build();

match editor.collect() {
  Ok(Some(text)) => println!("Valid: {}", text),
  Ok(None) => println!("Cancelled"),
  Err(e) => eprintln!("Error: {}", e),
}
```

### Pre-filled Text

```rust
use multiline_input::Builder;

let editor = Builder::new()
  .prompt("Edit TODO:")
  .initial_text("- Task 1\n- Task 2\n- Task 3")
  .show_line_numbers(true)
  .build();

match editor.collect() {
  Ok(Some(text)) => println!("Updated:\n{}", text),
  Ok(None) => println!("Cancelled"),
  Err(e) => eprintln!("Error: {}", e),
}
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `prompt` | `&str` | `""` | Prompt message to display |
| `allow_empty` | `bool` | `true` | Allow empty input |
| `min_length` | `Option<usize>` | `None` | Minimum text length |
| `max_length` | `Option<usize>` | `None` | Maximum text length |
| `validator` | `Fn(&str) -> Result<(), String>` | `None` | Custom validation function |
| `initial_text` | `Option<String>` | `None` | Pre-filled text |
| `placeholder` | `Option<String>` | `None` | Placeholder text when empty |
| `show_line_numbers` | `bool` | `false` | Display line numbers |
| `show_status` | `bool` | `false` | Display status line (line/col/chars) |
| `show_char_count` | `bool` | `false` | Display character count |
| `color` | `bool` | `true` | Enable colored output |

## Examples

See [examples/](examples/) directory for more usage examples:

- `basic_usage.rs` - Simple input collection
- `with_validation.rs` - Custom validation
- `with_config.rs` - Full configuration demo
- `pre_filled.rs` - Edit existing text

Run examples:
```bash
cargo run --example basic_usage
```

## How It Works

1. **Enters raw terminal mode** to capture individual key events
2. **Parses key combinations** (ENTER, CTRL+ENTER, arrows, etc.)
3. **Updates text buffer** with insertions/deletions
4. **Renders to screen** with visual feedback
5. **Returns collected text** on ENTER or None on ESC/CTRL+C

## Platform Support

- ✅ **Linux**: Full support (all terminals)
- ✅ **macOS**: Full support (Terminal.app, iTerm2)
- ⚠️ **Windows**:
  - Windows Terminal: Full support
  - cmd.exe: Limited (CTRL+ENTER may not work)
  - ConEmu: Full support

## Implementation Status

- [x] Core input collection
- [x] Key event handling (ENTER, CTRL+ENTER, ESC, arrows)
- [x] Text buffer with cursor management
- [x] Basic rendering
- [x] Builder API
- [x] Validation (min/max, custom)
- [x] Visual enhancements (line numbers, status, colors)
- [ ] Scrolling for large texts
- [ ] Undo/redo
- [ ] Clipboard integration

## Documentation

For API documentation: `cargo doc --open`

## Testing

```bash
# Run tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

## Contributing

Contributions welcome! Please:

1. Review the API documentation for architecture details
2. Write tests for new features
3. Follow existing code style
4. Update documentation

## License

MIT License - see [LICENSE](LICENSE) for details

## Related Projects

- [dialoguer](https://crates.io/crates/dialoguer) - CLI prompts library
- [inquire](https://crates.io/crates/inquire) - Interactive prompts
- [rustyline](https://crates.io/crates/rustyline) - Readline implementation

**Difference**: `multiline_input` focuses specifically on multiline text collection with ENTER to submit and CTRL+ENTER for newlines, optimized for AI assistant integration and commit message editing.
