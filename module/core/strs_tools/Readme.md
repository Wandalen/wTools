<!-- {{# generate.module_header{} #}} -->

# Module :: strs_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_strs_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_strs_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/strs_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/strs_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fstrs_tools%2Fexamples%2Fstrs_tools_trivial.rs,RUN_POSTFIX=--example%20strs_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools to manipulate strings.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
#[ cfg( all( feature = "split", not( feature = "no_std" ) ) ) ]
{
  /* delimeter exists */
  let src = "abc def";
  let iter = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .perform();
  let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc", " ", "def" ] );

  /* delimeter not exists */
  let src = "abc def";
  let iter = strs_tools::string::split()
  .src( src )
  .delimeter( "g" )
  .perform();
  let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc def" ] );
}
```

### To add to your project

```sh
cargo add strs_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/wstring_tools_trivial
cargo run
```

# Sample

[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=sample%2Frust%2Fstrs_tools_trivial,SAMPLE_FILE=.%2Fsrc%2Fmain.rs/https://github.com/Wandalen/wTools)
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/strs_tools)
