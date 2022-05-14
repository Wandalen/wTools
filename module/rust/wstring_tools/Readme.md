# Module :: wstring_tools [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

String tools.

<!-- xxx : qqq for Rust : write me --> <!-- aaa : done -->

### Sample

```rust
use wstring_tools::*;

fn main()
{
  /* delimeter exists */
  let src = "abc def";
  let iter = string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .perform();
  let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc", " ", "def" ] );

  /* delimeter no exists */
  let src = "abc def";
  let iter = string::split()
  .src( src )
  .delimeter( "g" )
  .perform();
  let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc def" ] );
}
```

### To add to your project

```sh
cargo add wstring_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/wstring_tools_trivial
cargo run
```
