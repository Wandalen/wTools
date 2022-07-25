<!-- {{# generate.module_header{} #}} -->

# Module :: winterval
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewIntervalPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewIntervalPush.yml) [![docs.rs](https://img.shields.io/docsrs/winterval?color=e3e8f0&logo=docs.rs)](https://docs.rs/winterval) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwinterval_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20winterval_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Interval adapter for both open/closed implementations of intervals ( ranges ).

### Sample

<!-- {{# generate.module_sample{} #}} -->

```rust
#[ cfg( feature = "use_std" ) ]
{
  use winterval::*;

  let src = 2..5;
  assert_eq!( src.closed(), ( 2, 4 ) );

  let src = 2..=4;
  assert_eq!( src.closed(), ( 2, 4 ) );
}
```

### To add to your project

```sh
cargo add winterval
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/winterval_trivial
cargo run
```
