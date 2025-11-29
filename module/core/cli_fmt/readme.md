# cli_fmt

CLI output formatting utilities for command-line applications.

## Purpose

This crate provides utilities specifically designed for building command-line applications,
including output processing, formatting, and other CLI-specific helpers.

## Distinction from strs_tools

- **strs_tools**: General-purpose string and ANSI manipulation (any application)
- **cli_fmt**: CLI-application-specific functionality (command-line tools only)

### Scope

**Responsibilities:**
Provides CLI-application-specific output processing utilities for command-line tools. Focuses on stream selection, line filtering (head/tail), and ANSI-aware width truncation. Delegates to strs_tools for general-purpose string and ANSI manipulation. Designed for command-line applications requiring output formatting and processing.

**In Scope:**
- CLI output processing with stream selection (stdout, stderr, both)
- Head/tail line filtering for limiting output volume
- ANSI-aware width truncation with configurable suffixes
- Stream merging with stderr-before-stdout ordering convention
- Builder pattern configuration API for ergonomic usage
- Integration with strs_tools for underlying string operations

**Out of Scope:**
- General-purpose string manipulation (see strs_tools crate)
- ANSI escape code generation (see strs_tools crate)
- Progress bars, tables, or interactive prompts (see indicatif, tabled crates)
- Terminal control or cursor manipulation (see crossterm crate)
- Configuration file parsing (see config, serde crates)
- Command-line argument parsing (see clap crate)

## Modules

- `output` - Process command output with head/tail filtering, width truncation, and stream merging

## Usage

```rust
use cli_fmt::output::*;

let config = OutputConfig::default()
  .with_head(10)
  .with_width(80);

let result = process_output(stdout_str, stderr_str, &config);
println!("{}", result.content);
```

## Features

- `output` - Output processing module (enabled by default)

## License

MIT
