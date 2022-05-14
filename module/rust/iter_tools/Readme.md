# Module :: iter_tools [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Collection of general purpose tools to iterate. Currently it simply reexport itertools.

### Sample

```rust
use iter_tools::*;

fn main()
{
  /* standard functions */
  let vec = vec![ 5, 1, -2 ];
  let min = min( &vec );
  assert_eq!( *min.unwrap(), -2 );

  /* non standard functions */
  let vec = vec![ 5, 1, -2 ];
  let added = vec![ "a", "b", "c" ];
  let mut result = vec![];
  let zipped = zip( &vec, &added );
  for ( left, right ) in zipped
  {
    result.push( ( *left, *right ) );
  }
  assert_eq!( result, vec![ ( 5, "a" ), ( 1, "b" ), ( -2, "c" ) ] );
}
```

<!-- # qqq : for Rust dev : please add --> <!-- aaa : done -->

### To add to your project

```sh
cargo add iter_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/iter_tools_trivial
cargo run
```
