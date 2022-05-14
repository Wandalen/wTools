# Module :: wtest_basic [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

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
