# cli_tools

CLI application building blocks for command-line tools.

## Purpose

This crate provides utilities specifically designed for building command-line applications,
including output processing, formatting, and other CLI-specific helpers.

## Distinction from strs_tools

- **strs_tools**: General-purpose string and ANSI manipulation (any application)
- **cli_tools**: CLI-application-specific functionality (command-line tools only)

## Modules

- `cli_output` - Process command output with head/tail filtering, width truncation, and stream merging

## Usage

```rust
use cli_tools::cli_output::*;

let config = OutputConfig::default()
  .with_head(10)
  .with_width(80);

let result = process_output(stdout_str, stderr_str, &config);
println!("{}", result.content);
```

## Features

- `cli_output` - Output processing module (enabled by default)

## License

MIT
