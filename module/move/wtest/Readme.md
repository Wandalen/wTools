# Module :: wtest
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewTestPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewTestPush.yml) [![docs.rs](https://img.shields.io/docsrs/wtest?color=e3e8f0&logo=docs.rs)](https://docs.rs/wtest) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Tools for writing and running tests.

## Sample

```rust
use wtest_basic::*;

//

fn _pass1()
{
  assert_eq!( true, true );
}

//

fn _pass2()
{
  assert_eq!( 1, 1 );
}

//

test_suite!
{
  pass1,
  pass2,
}
```

### To add to your project

```sh
cargo add wtest --dev
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/test_trivial
cargo run
```
