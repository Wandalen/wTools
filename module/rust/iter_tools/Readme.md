# Module :: iter_tools [![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleIterToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleIterToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/iter_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/iter_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

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
