# `color_tools`

Typed text-with-optional-ANSI-color abstraction.

## Overview

`color_tools` provides [`ColorfulText`] — a plain `String` wrapper with an optional ANSI escape color prefix. It is designed for contexts where most text is plain but some strings need per-instance terminal coloring without requiring a global color configuration.

Key properties:
- `From<String>` and `From<&str>` are transparent (no color, no overhead)
- `.with_color( "\x1b[33m" )` attaches a color prefix via builder
- `.render()` appends `"\x1b[0m"` (ANSI reset) only when colored
- `From<ColorfulText> for String` delegates to `.render()`

## Quick Start

```rust
use color_tools::ColorfulText;

// Plain — no color injected
let plain : ColorfulText = "status: ok".into();
assert_eq!( plain.render(), "status: ok" );

// Colored — yellow with reset
let warn = ColorfulText::from( "status: warn" ).with_color( "\x1b[33m" );
println!( "{warn}" );  // prints yellow text to terminal
```

## Features

| Feature | Enabled by default | Purpose |
|---|---|---|
| `enabled` | yes | Compile the crate |
| `serde_support` | no | Derive `Serialize`/`Deserialize` for `ColorfulText` |
