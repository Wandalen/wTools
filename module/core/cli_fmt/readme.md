# cli_fmt

CLI output formatting utilities for command-line applications.

## Purpose

Provides utilities specifically designed for building command-line applications: output
processing with head/tail filtering, ANSI-aware width truncation, and stream merging.

See [docs/invariant/001_architectural_boundary.md](docs/invariant/001_architectural_boundary.md)
for the `cli_fmt` vs `strs_tools` design boundary.

## Modules

- `output` — Process command output with head/tail filtering, width truncation, and stream merging

## Usage

```rust
use cli_fmt::output::*;

let config = OutputConfig::default()
  .with_head( 10 )
  .with_width( 80 );

let result = process_output( stdout_str, stderr_str, &config );
println!( "{}", result.content );
```

## Features

- `enabled` — master switch; activates core dependencies
- `output` — output processing module (requires `enabled`)
- `full` — enables all functionality (`enabled` + `output` + `ansi_unicode`)
- `ansi_unicode` — grapheme-based Unicode width support (opt-in)

## License

MIT
