<!-- {{# generate.module_header{} #}} -->

# Module :: wtest_basic
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_wtest_basic_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_wtest_basic_push.yml) [![docs.rs](https://img.shields.io/docsrs/wtest_basic?color=e3e8f0&logo=docs.rs)](https://docs.rs/wtest_basic) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools for writing and running tests. The most basic things.

### Basic use-case.

<!-- {{# generate.module{} #}} -->

```rust
use wtest_basic::*;

//

tests_impls!
{

  fn pass1()
  {
    assert_eq!( true, true );
  }

  //

  fn pass2()
  {
    assert_eq!( 1, 1 );
  }

}

//

tests_index!
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
cd examples/test_basic_trivial
cargo run
```

