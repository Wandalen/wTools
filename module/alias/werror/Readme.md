<!-- {{# generate.module_header{} #}} -->

# Module :: werror
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleErrorToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleErrorToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/werror?color=e3e8f0&logo=docs.rs)](https://docs.rs/werror) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Basic exceptions handling mechanism.

### Sample

<!-- {{# generate.module_sample{} #}} -->

```rust
#[ cfg( feature = "use_std" ) ]
{
  use werror::*;

  let err1 = BasicError::new( "Some error" );
  println!( "err1 : {}", err1 );
  // < err1 : Some error
}
```

### To add to your project

```sh
cargo add werror
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/error_tools_trivial
cargo run
```
