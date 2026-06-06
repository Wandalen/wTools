# Module :: `cli_fmt`
<!--{ generate.module_header.start() }-->
 [![stable](https://raster.shields.io/static/v1?label=&message=stable&color=green)](https://github.com/emersion/stability-badges#stable) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=cli_fmt)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/cli_fmt?color=e3e8f0&logo=docs.rs)](https://docs.rs/cli_fmt) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

CLI output formatting utilities for command-line applications.

## Purpose

Provides utilities specifically designed for building command-line applications: output
processing with head/tail filtering, ANSI-aware width truncation, and stream merging.

See [docs/invariant/001_architectural_boundary.md](docs/invariant/001_architectural_boundary.md)
for the `cli_fmt` vs `strs_tools` design boundary.

## Modules

- `output` — Process command output with head/tail filtering, width truncation, and stream merging
- `help` — Render structured CLI help text from typed data with configurable style and TTY-conditional ANSI

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
- `output` — CLI output processing (head/tail filtering, width truncation, stream merging)
- `cli_help_template` — typed CLI help renderer (`CliHelpStyle`, `CliHelpData`, `CliHelpTemplate`)
- `ansi_unicode` — grapheme-based Unicode width via `unicode-segmentation` (opt-in)
- `full` — enables all functionality (`enabled` + `output` + `ansi_unicode` + `cli_help_template`)

## License

MIT
