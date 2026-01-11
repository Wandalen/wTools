<!-- {{# generate.module_header{} #}} -->

# Module :: proper_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_proper_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_proper_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/proper_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/proper_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.

## Status

⚠️ **EXPERIMENTAL - PLACEHOLDER MODULE**

This module is currently a minimal placeholder awaiting definition and implementation. It provides only a skeleton structure with a single placeholder function `f1()`.

See [`spec.md`](./spec.md) for detailed status, roadmap, and design decisions.

### Current Functionality

```rust
use proper_tools::*;

fn main()
{
  // Only placeholder function available
  f1();
}
```

### To add to your project

```bash
cargo add proper_tools
```

**Note:** This crate currently provides minimal functionality. Review the [specification](./spec.md) for planned features and implementation status.

### Try out from the repository

```shell
git clone https://github.com/Wandalen/wTools
cd wTools/module/alias/proper_tools
cargo run --example proper_tools_trivial
```

### Examples

- [`proper_tools_trivial`](./examples/proper_tools_trivial.rs) - Minimal usage demonstration

### Documentation

- [Specification](./spec.md) - Complete specification including roadmap and open questions
- [API Documentation](https://docs.rs/proper_tools) - Generated API docs on docs.rs
