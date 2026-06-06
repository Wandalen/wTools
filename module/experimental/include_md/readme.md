
# Module :: include_md
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=include_md)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/include_md?color=e3e8f0&logo=docs.rs)](https://docs.rs/include_md) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Include markdown file or its section.

### To add to your project

```shell
cargo add include_md
```

## Quick Start

Include a complete markdown file as a compile-time string constant:

```rust
let content = include_md ::include_md!( "../readme.md" );
println!( "{content}" );
```

Include only a named section from a markdown file:

```rust
let section = include_md ::include_md_section!( "readme.md", "## Quick Start" );
println!( "{section}" );
```

Paths for `include_md!` resolve relative to the source file (same as `include_str!`).
Paths for `include_md_section!` resolve relative to `CARGO_MANIFEST_DIR` (crate root).
Both macros reject files larger than 10 MB at compile time.

