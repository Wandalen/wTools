# Module :: error_tools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleErrorToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleErrorToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/error_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/error_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ferror_tools_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20error_tools_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Basic exceptions handling mechanism.

### Sample

```rust
#[ cfg( feature = "use_std" ) ]
{
  use error_tools::*;

  let err1 = BasicError::new( "Some error" );
  println!( "err1 : {}", err1 );
  // < err1 : Some error
}
```

### To add to your project

```sh
cargo add error_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/error_tools_trivial
cargo run
```
