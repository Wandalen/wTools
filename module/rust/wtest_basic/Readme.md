# Module :: wtest_basic
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/wtest_basic.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/wtest_basic.yml) [![docs.rs](https://img.shields.io/docsrs/wtest_basic?color=e3e8f0&logo=docs.rs)](https://docs.rs/wtest_basic) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Tools for writing and running tests. The most basic things.

### Sample

```rust
use wtest_basic::*;

//

fn pass1_test()
{
  assert_eq!( true, true );
}

//

fn pass2_test()
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
cargo add wtest_basic --dev
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/test_basic_trivial
cargo run
```
