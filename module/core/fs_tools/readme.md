<!-- {{# generate.module_header{} #}} -->

# Module :: fs_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_fs_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_fs_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/fs_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/fs_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools to manipulate files.

## Features

- **TempDir**: Temporary directory management with RAII cleanup
  - `full_path()` - Compose path from base/prefix/postfix components
  - `create()` - Create directory (parent must exist)
  - `create_all()` - Create directory with all parents
  - Automatic cleanup on drop for created directories
- **glob** (optional): Unix shell-style pattern matching via re-export

## Basic Usage

### TempDir with RAII Cleanup

```rust,ignore
use fs_tools::TempDir;
use std::path::PathBuf;

let mut temp = TempDir::new();
temp.base_path = std::env::temp_dir();
temp.prefix_path = PathBuf::from( "my_app" );
temp.postfix_path = PathBuf::from( "session_1" );

// Create directory (enables automatic cleanup)
let path = temp.create_all().expect( "failed to create" );
assert!( path.is_dir() );

// Directory is automatically removed when `temp` goes out of scope
```

### Glob Pattern Matching

```rust
use fs_tools::glob::glob;

// Find all Rust files in current directory
for entry in glob( "*.rs" ).expect( "valid pattern" )
{
  if let Ok( path ) = entry
  {
    println!( "{:?}", path );
  }
}
```

### Recursive Glob

```rust
use fs_tools::glob::glob;

// Find all Rust files recursively
for entry in glob( "src/**/*.rs" ).expect( "valid pattern" )
{
  if let Ok( path ) = entry
  {
    println!( "{:?}", path );
  }
}
```

### Pattern Matching

```rust
use fs_tools::glob::Pattern;

let pattern = Pattern::new( "*.rs" ).expect( "valid pattern" );

assert!( pattern.matches( "lib.rs" ) );
assert!( !pattern.matches( "Cargo.toml" ) );
```

## To add to your project

```sh
# Basic (TempDir only)
cargo add fs_tools

# With glob support
cargo add fs_tools --features glob

# All features
cargo add fs_tools --features full
```

## Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/test_trivial
cargo run
```
