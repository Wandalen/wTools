<!-- {{# generate.module_header{} #}} -->

# Module :: crates_tools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTemplateBlankPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTemplateBlankPush.yml) [![docs.rs](https://img.shields.io/docsrs/template_blank?color=e3e8f0&logo=docs.rs)](https://docs.rs/template_blank) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

A crate file is a package of Rust source code that can be downloaded from crates.io, the official Rust package registry. A crate file has the extension `.crate` and contains a compressed archive of the source code and other files needed to compile and run the crate.

`crate_tools` allows you to download and read and decode the `.crate` files. You can then use the `CrateArchive` struct to list and access the contents of the file as bytes.

This crate is useful for developers who want to inspect and analyze Rust crates.
Some possible use cases are:

- Compare the source code of different versions of a crate to see what has changed;
- Search for leftover confidential data before publishing;
- Analyze the size of packed files.

## Sample  :: show crate content

<!-- {{# generate.module_sample{} #}} -->

```rust
use crates_tools::*;

fn main()
{
  let krate = CrateArchive::download_crates_io( "test_experimental_c", "0.1.0" ).unwrap();
  
  for path in krate.list()
  {
    let bytes = krate.content_bytes( path ).unwrap();
    let string = std::str::from_utf8( bytes ).unwrap();
    
    println!( "# {}\n```\n{}```", path.display(), string );
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
