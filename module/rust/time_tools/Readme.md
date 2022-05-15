# Module :: time_tools [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Collection of general purpose time tools.

### Sample

```rust
use time_tools::*;

fn main()
{
  /* get milliseconds from UNIX epoch */
  let now = time::now();
  println!( "now {}", now );

  /* get nanoseconds from UNIX epoch */
  let now = time::now();
  let now_ns = time::ns::now();
  assert_eq!( now, now_ns / 1000000 );

  /* get seconds from UNIX epoch */
  let now = time::now();
  let now_s = time::s::now();
  assert_eq!( now / 1000, now_s );
}
```

<!-- # qqq : for Rust dev : please add --> <!-- aaa : done -->

### To add to your project

```sh
cargo add time_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/time_tools_trivial
cargo run
```
