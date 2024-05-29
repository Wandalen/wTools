<!-- {{# generate.module_header{} #}} -->

# Module :: crates_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_crates_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_crates_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/crates_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/crates_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fmove%2Fcrates_tools%2Fexamples%2Fcrates_tools_trivial.rs,RUN_POSTFIX=--example%20crates_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools to analyse crate files.

A crate file is a package of Rust source code that can be downloaded from crates.io, the official Rust package registry. A crate file has the extension `.crate` and contains a compressed archive of the source code and other files needed to compile and run the crate.

`crate_tools` allows you to download and read and decode the `.crate` files. You can then use the `CrateArchive` struct to list and access the contents of the file as bytes.

This crate is useful for developers who want to inspect and analyze Rust crates.
Some possible use cases are:

- Compare the source code of different versions of a crate to see what has changed;
- Search for leftover confidential data before publishing;
- Analyze the size of packed files.

## Sample  :: show crate content

<!-- {{# generate.module{} #}} -->

```rust
use crates_tools::*;

fn main()
{
  #[ cfg( feature = "enabled" ) ]
  {
    // download a package with specific version from `crates.io`
    let crate_archive = CrateArchive::download_crates_io( "test_experimental_c", "0.1.0" ).unwrap();

    for path in crate_archive.list()
    {
      // take content from a specific file from the archive
      let bytes = crate_archive.content_bytes( path ).unwrap();
      let string = std::str::from_utf8( bytes ).unwrap();

      println!("# {}\n```\n{}```", path.display(), string);
    }
  }
}
```

### To add to your project

```bash
cargo add crates_tools
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools/module/move/crates_tools
cargo r --example show_crate_content
```
