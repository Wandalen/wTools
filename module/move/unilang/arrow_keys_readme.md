# Arrow Key Command History in Unilang REPL

## Overview

The Unilang REPL provides full arrow key support for command history navigation using the `rustyline` library. Arrow key support is **enabled by default** and allows users to efficiently recall and modify previously entered commands.

## How to Use Arrow Keys

Arrow key support is available by default via the `enhanced_repl` feature:

```bash
# Run REPL with arrow keys (default behavior - includes enhanced_repl)
cargo run --example 15_interactive_repl_mode
```

## Feature Levels

The REPL has two feature levels:

- **`repl`**: Base REPL functionality (standard input/output, no arrow keys)
- **`enhanced_repl`**: Advanced REPL (arrow keys, command history, tab completion)

```bash
# Enhanced REPL (default - arrow keys work)
cargo run --example 15_interactive_repl_mode

# Basic REPL only (no arrow keys)
cargo run --example 15_interactive_repl_mode --no-default-features --features enabled,repl

# No REPL (shows error message)
cargo run --example 15_interactive_repl_mode --no-default-features --features enabled
```

## Arrow Key Functionality

### ↑ (Up Arrow)
- Navigates **backward** through command history
- Shows the most recently entered command first
- Continues to older commands with each press
- Command appears on the current line, ready for editing

### ↓ (Down Arrow)  
- Navigates **forward** through command history
- Moves from older commands to newer commands
- Returns to empty prompt after the newest command
- Allows moving forward after going back with ↑

### Additional Features
- **Edit before execution**: Recalled commands can be modified before pressing Enter
- **Persistent history**: Commands remain in history throughout the session
- **Ctrl+C**: Graceful exit
- **Tab completion**: Basic tab completion support

## How It Works

1. **Enter Commands**: Type some commands first to build history
   ```
   unilang[0]> .system.info
   unilang[1]> .auth.login username::test
   unilang[2]> help
   ```

2. **Use Arrow Keys**: Press ↑ to recall previous commands
   ```
   unilang[3]> help                    ← (↑ pressed once)
   unilang[3]> .auth.login username::test  ← (↑ pressed twice) 
   unilang[3]> .system.info            ← (↑ pressed three times)
   ```

3. **Edit and Execute**: Modify the recalled command if needed, then press Enter

## Important Notes

### When Arrow Keys Work
✅ **Interactive Terminal**: Running directly in terminal  
✅ **TTY Environment**: Standard terminal emulators  
✅ **SSH Sessions**: Remote terminal sessions  

### When Arrow Keys Don't Work  
❌ **Piped Input**: `echo "commands" | program`  
❌ **Non-TTY**: Redirected stdin/stdout  
❌ **CI/CD Environments**: Automated test environments  

The REPL automatically detects the environment and shows appropriate messages.

## Testing Arrow Keys

### Manual Test
```bash
# 1. Start REPL (arrow keys enabled by default)
cargo run --example 15_interactive_repl_mode

# 2. Enter some test commands
.system.info
help  
.auth.login username::demo

# 3. Press ↑ arrow key multiple times
# You should see previous commands appear

# 4. Press ↓ to navigate forward
# 5. Edit any recalled command and press Enter
```

### Demo Script
```bash
# Run the demo script for guided testing
./demo_arrow_keys.sh
```

## Implementation Details

The arrow key functionality is implemented using:
- **rustyline**: Professional readline library with full terminal support
- **Command History**: Automatic history management
- **TTY Detection**: Environment detection using `atty` crate
- **Error Handling**: Graceful fallback for non-interactive environments

## Comparison: Basic vs Enhanced REPL

| Feature | Basic REPL | Enhanced REPL |
|---------|------------|---------------|
| Arrow Keys | ❌ Shows `^[[A` | ✅ Full navigation |
| Command History | ✅ `history` command | ✅ Arrow keys + `history` |
| Tab Completion | ❌ | ✅ Basic support |
| Line Editing | ❌ | ✅ Full editing |
| Ctrl+C Handling | ✅ Basic | ✅ Graceful |

## Troubleshooting

### Problem: Arrow keys show `^[[A` instead of working
**Solution**: This shouldn't happen with the current version as arrow keys are enabled by default. If you see this, there may be a compilation issue:
```bash
# Try rebuilding the example
cargo build --example 15_interactive_repl_mode
cargo run --example 15_interactive_repl_mode
```

### Problem: "Arrow keys only work in interactive terminals" message
**Solution**: Run directly in terminal, not with piped input:
```bash
# ❌ Won't work
echo ".system.info" | cargo run --example 15_interactive_repl_mode

# ✅ Will work  
cargo run --example 15_interactive_repl_mode
```

### Problem: Commands not appearing in history for arrow keys
**Solution**: The REPL only adds actual commands to history, not meta-commands like `help`, `history`, `clear`, or `quit`.

## Advanced Usage

### History Management
- History persists throughout the session
- Meta-commands (`help`, `quit`, etc.) are not added to history  
- Real commands are added immediately upon entry
- Use `history` command to see all stored commands

### Key Bindings
- **↑/↓**: Navigate command history
- **←/→**: Move cursor within current line  
- **Home/End**: Jump to beginning/end of line
- **Ctrl+A/E**: Jump to beginning/end of line (emacs-style)
- **Ctrl+C**: Interrupt and exit
- **Ctrl+D**: EOF and exit
- **Tab**: Basic completion (when available)

The enhanced REPL provides a professional command-line experience comparable to bash, zsh, or other modern shells.