# REPL Feature Specification

## Overview

The Unilang REPL functionality is organized into two feature levels:

1. **`repl`** - Base REPL functionality with standard input/output
2. **`enhanced_repl`** - Advanced REPL with arrow keys, command history, and tab completion

## Feature Dependencies

```
enhanced_repl
├── repl (base REPL functionality)
├── rustyline (readline library for advanced features)
└── atty (TTY detection)

repl
└── (no dependencies - uses std::io only)
```

## Feature Combinations & Behavior

| Features Enabled | Behavior | Arrow Keys | Command History | Tab Completion |
|------------------|----------|------------|-----------------|----------------|
| `enhanced_repl` | Enhanced REPL | ✅ Full support | ✅ Up/Down arrows + `history` | ✅ Basic |
| `repl` only | Basic REPL | ❌ Shows `^[[A` | ✅ `history` command only | ❌ |
| Neither | Error message | ❌ N/A | ❌ N/A | ❌ N/A |

### Important Notes:
- **`enhanced_repl` automatically enables `repl`** (dependency relationship)
- **`enhanced_repl` without `repl`** is equivalent to **neither feature enabled** (shows error)
- **Default configuration** includes both `repl` and `enhanced_repl`

## Default Features

```toml
default = [ "enabled", "simd", "repl", "enhanced_repl" ]
```

This means running without explicit features gets the full enhanced experience:
```bash
cargo run --example 15_interactive_repl_mode  # Uses enhanced REPL by default
```

## Usage Examples

### 1. Enhanced REPL (Default)
```bash
# All these are equivalent and provide enhanced REPL:
cargo run --example 15_interactive_repl_mode
cargo run --example 15_interactive_repl_mode --features enhanced_repl
cargo run --example 15_interactive_repl_mode --features repl,enhanced_repl
```

**Features:**
- ✅ Arrow key navigation (↑/↓) through command history
- ✅ Line editing (←/→, Home/End, Ctrl+A/E)
- ✅ Tab completion (basic)
- ✅ Ctrl+C/Ctrl+D handling
- ✅ `history` command
- ✅ TTY detection with user guidance

### 2. Basic REPL Only
```bash
# Basic REPL without arrow keys:
cargo run --example 15_interactive_repl_mode --no-default-features --features enabled,repl
```

**Features:**
- ❌ No arrow key support (shows `^[[A`)
- ✅ `history` command (with manual list)
- ✅ All other REPL functionality
- ✅ Standard input/output handling

### 3. No REPL Features
```bash
# Shows helpful error message:
cargo run --example 15_interactive_repl_mode --no-default-features --features enabled
```

**Result:**
```
❌ REPL functionality is not enabled.
This example requires the 'repl' feature to be enabled.

Available options:
  cargo run --example 15_interactive_repl_mode --features repl
  cargo run --example 15_interactive_repl_mode --features enhanced_repl
  cargo run --example 15_interactive_repl_mode  (default includes repl)

💡 The 'repl' feature provides basic REPL functionality
💡 The 'enhanced_repl' feature adds arrow keys, history, and tab completion
```

## Implementation Details

### Conditional Compilation

The example uses conditional compilation to handle different feature combinations:

```rust
#[cfg(feature = "repl")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // REPL functionality when repl feature is enabled
    
    #[cfg(feature = "enhanced_repl")]
    run_enhanced_repl(&pipeline)?;
    
    #[cfg(all(feature = "repl", not(feature = "enhanced_repl")))]
    run_basic_repl(&pipeline)?;
}

#[cfg(not(feature = "repl"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Error message when repl feature is disabled
}
```

### Function Feature Gates

- **`register_interactive_commands`**: `#[cfg(feature = "repl")]`
- **`run_enhanced_repl`**: `#[cfg(feature = "enhanced_repl")]`
- **`run_basic_repl`**: `#[cfg(all(feature = "repl", not(feature = "enhanced_repl")))]`
- **`display_repl_help`**: `#[cfg(feature = "repl")]`
- **`display_command_history`**: `#[cfg(all(feature = "repl", not(feature = "enhanced_repl")))]`

### Dependency Management

#### Enhanced REPL Dependencies
```toml
rustyline = { version = "14.0", optional = true }
atty = { version = "0.2", optional = true }
```

#### Feature Definitions
```toml
repl = []  # Base feature, no dependencies
enhanced_repl = [ "repl", "dep:rustyline", "dep:atty" ]
```

## Arrow Key Functionality

### How Arrow Keys Work

When **`enhanced_repl`** feature is enabled:

1. **↑ (Up Arrow)**: Navigate backward through command history
   - Most recent command appears first
   - Continues to older commands with each press
   - Command appears on current line, ready for editing

2. **↓ (Down Arrow)**: Navigate forward through command history
   - Moves from older to newer commands
   - Returns to empty prompt after newest command

3. **Enter**: Execute the currently displayed command

4. **Edit**: Recalled commands can be modified before execution

### When Arrow Keys Work

✅ **Interactive Terminal Sessions**
- Direct terminal execution
- SSH sessions
- Standard terminal emulators

❌ **Non-Interactive Sessions**
- Piped input: `echo "cmd" | program`
- Redirected stdin/stdout
- CI/CD environments
- Automated scripts

The REPL automatically detects the environment and provides appropriate guidance.

### TTY Detection

```rust
let is_tty = atty::is(atty::Stream::Stdin);

if is_tty {
    println!("💡 Arrow Key Usage:");
    println!("  • Enter some commands first");
    println!("  • Then use ↑ to go back through history");
    // ...
} else {
    println!("⚠️  Note: Arrow keys only work in interactive terminals");
    println!("   Current session: Non-interactive (piped input detected)");
    println!("   For arrow key support, run directly in terminal");
}
```

## History Management

### Enhanced REPL History
- **Storage**: Handled by `rustyline` internally
- **Navigation**: ↑/↓ arrow keys
- **Persistence**: Session-only (not saved to file)
- **Filtering**: Only actual commands added (not meta-commands like `help`, `quit`)

### Basic REPL History
- **Storage**: Manual `Vec<String>` storage
- **Access**: `history` command only
- **Display**: Numbered list format

### Commands Not Added to History
- `help`, `h`
- `history`
- `clear`
- `quit`, `exit`, `q`
- Empty input

## Error Handling

### Feature-Specific Error Handling

1. **No REPL Features**: Shows instructional error message with usage options
2. **Basic REPL**: Standard error messages with tips to use `help`
3. **Enhanced REPL**: Advanced error handling with context-aware suggestions

### Interactive Argument Handling

All REPL modes support interactive argument detection and secure input prompting:

```rust
if error.contains("UNILANG_ARGUMENT_INTERACTIVE_REQUIRED") || 
   error.contains("Interactive Argument Required") {
    // Handle secure input prompting
}
```

## REPL Implementation Performance Analysis

### Enhanced REPL
- **Memory**: Higher due to rustyline dependencies
- **Startup**: Slightly slower due to terminal initialization
- **Runtime**: Negligible performance difference
- **User Experience**: Significantly better

### Basic REPL
- **Memory**: Lower (standard library only)
- **Startup**: Faster
- **Runtime**: Minimal overhead
- **User Experience**: Functional but basic

## Testing

### Feature Combination Tests

```bash
# Test 1: Default (enhanced)
cargo run --example 15_interactive_repl_mode

# Test 2: Basic only
cargo run --example 15_interactive_repl_mode --no-default-features --features enabled,repl

# Test 3: Enhanced explicit
cargo run --example 15_interactive_repl_mode --no-default-features --features enabled,enhanced_repl

# Test 4: No REPL
cargo run --example 15_interactive_repl_mode --no-default-features --features enabled
```

### Arrow Key Testing

Arrow keys can only be tested interactively:

```bash
# Start REPL in terminal
cargo run --example 15_interactive_repl_mode

# Enter commands:
.system.info
help
.auth.login username::test

# Test arrows:
# Press ↑ to see "help"
# Press ↑ again to see ".system.info"
# Press ↓ to go forward
# Edit and press Enter to execute
```

## Migration Guide

### From Old Implementation
If you have existing code using the old feature structure:

**Before:**
```bash
cargo run --example 15_interactive_repl_mode --features enhanced_repl
```

**After:**
```bash
cargo run --example 15_interactive_repl_mode  # Default now includes enhanced REPL
```

### Minimal Builds
For environments where enhanced features aren't needed:

```bash
cargo build --example 15_interactive_repl_mode --no-default-features --features enabled,repl
```

## Future Enhancements

Possible future improvements:

1. **Persistent History**: Save command history to file
2. **Advanced Tab Completion**: Context-aware command and argument completion
3. **Command Aliases**: User-definable command shortcuts
4. **Syntax Highlighting**: Real-time command syntax highlighting
5. **Multi-line Input**: Support for complex multi-line commands

## Summary

The REPL feature system provides a clean separation between basic functionality (`repl`) and enhanced user experience (`enhanced_repl`), with sensible defaults that provide the best experience while allowing minimal configurations when needed.