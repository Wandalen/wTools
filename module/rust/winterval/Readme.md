# Module :: winterval
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewIntervalPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewIntervalPush.yml) [![docs.rs](https://img.shields.io/docsrs/winterval?color=e3e8f0&logo=docs.rs)](https://docs.rs/winterval) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Interval adapter for both open/closed implementations of intervals ( ranges ).

### Sample

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
