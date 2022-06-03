# Module :: test_tools
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/test_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/test_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee)](https://discord.gg/JwTG6d2b)

Tools for writing and running tests.

### Sample

```rust
use test_tools::*;

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
cargo add test_tools --dev
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/test_trivial
cargo run
```
