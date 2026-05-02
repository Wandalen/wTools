<!-- {{# generate.module_header{} #}} -->

# Module :: program_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=program_tools)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/program_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/program_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Builder types for in-memory Rust program representation: Source, Program, and Plan.

### To add to your project

```bash
cargo add program_tools
```

### Quick Start

```rust
use program_tools::*;

let plan = Plan::former()
  .program()
    .source()
      .file_path( "main.rs" )
      .data( "fn main() { println!(\"hello\"); }" )
      .end()
    .end()
  .form();

println!( "{plan:?}" );
```
