<!-- {{# generate.module_header{} #}} -->

# Module :: time_tools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTimeToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTimeToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/time_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/time_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftime_tools_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20time_tools_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Collection of general purpose time tools.

### Sample

<!-- {{# generate.module_sample{} #}} -->

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
cd sample/rust/time_tools_trivial
cargo run
```
