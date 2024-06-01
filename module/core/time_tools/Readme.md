<!-- {{# generate.module_header{} #}} -->

# Module :: time_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_time_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_time_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/time_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/time_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ftime_tools%2Fexamples%2Ftime_tools_trivial.rs,RUN_POSTFIX=--example%20time_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose time tools.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
#[ cfg( feature = "chrono" ) ]
{
  use time_tools::*;

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
cd examples/time_tools_trivial
cargo run
```
