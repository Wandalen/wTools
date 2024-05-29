<!-- {{# generate.module_header{} #}} -->

# Module :: test_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_test_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_test_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/test_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/test_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ftest_tools%2Fexamples%2Ftest_tools_trivial.rs,RUN_POSTFIX=--example%20test_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools for writing and running tests.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
use test_tools::*;

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
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
#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
tests_index!
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
cd examples/test_trivial
cargo run
```

# Sample

[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=sample%2Frust%2Ftest_tools_trivial,SAMPLE_FILE=.%2Fsrc%2Fmain.rs/https://github.com/Wandalen/wTools)
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/test_tools)
