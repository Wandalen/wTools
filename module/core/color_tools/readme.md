# `color_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=color_tools)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/color_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/color_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fcolor_tools%2Fexamples%2Fbasic.rs,RUN_POSTFIX=--example%20basic/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Typed text-with-optional-ANSI-color abstraction.

## Overview

`color_tools` provides [`DecoratedText`] — a plain `String` wrapper with an optional ANSI escape color prefix. It is designed for contexts where most text is plain but some strings need per-instance terminal coloring without requiring a global color configuration.

Key properties:
- `From<String>` and `From<&str>` are transparent (no color, no overhead)
- `.with_color( "\x1b[33m" )` attaches a color prefix via builder
- `.render()` appends `"\x1b[0m"` (ANSI reset) only when colored
- `From<DecoratedText> for String` delegates to `.render()`

## Quick Start

```rust
use color_tools::DecoratedText;

// Plain — no color injected
let plain : DecoratedText = "status: ok".into();
assert_eq!( plain.render(), "status: ok" );

// Colored — yellow with reset
let warn = DecoratedText::from( "status: warn" ).with_color( "\x1b[33m" );
println!( "{warn}" );  // prints yellow text to terminal
```

## Features

| Feature | Enabled by default | Purpose |
|---|---|---|
| `enabled` | no | Compile the crate |
| `serde_support` | no | Derive `Serialize`/`Deserialize` for `DecoratedText` |
| `html_support`  | no | Add `render_html()` to `DecoratedText` and `to_css()` to `Color` |

## Directory Structure

| Directory | Responsibility |
|-----------|----------------|
| `src/` | Crate source: `DecoratedText` struct and impls |
| `tests/` | Unit and integration tests |
| `examples/` | Manual color verification binary |
| `docs/` | Design and API documentation |
| `task/` | Implementation task history |
