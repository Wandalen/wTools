# module::time_tools [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/RustPublish.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/RustPublish.yml)

Collection of general purpose time tools.

### Sample

``` rust test
use time_tools::*;

fn main()
{
  /* get milliseconds from UNIX epoch */
  let now = time::now();
  let now_chrono = chrono::prelude::Utc::now().timestamp_millis();
  assert_eq!( now, now_chrono );

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

``` shell
cargo add time_tools
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/time_tools_trivial
cargo run
```
