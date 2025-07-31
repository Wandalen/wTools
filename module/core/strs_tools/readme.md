
<!-- {{# generate.module_header{} #}} -->
# Module :: `strs_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_strs_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_strs_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/strs_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/strs_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fstrs_tools%2Fexamples%2Fstrs_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fstrs_tools%2Fexamples%2Fstrs_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools to manipulate strings.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
#[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
{
  /* delimeter exists */
  let src = "abc def";
  let iter = strs_tools::string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .perform();
  let iterated = iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc", " ", "def" ] );

  /* delimeter not exists */
  let src = "abc def";
  let iter = strs_tools::string::split()
  .src( src )
  .delimeter( "g" )
  .perform();
  let iterated = iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc def" ] );
}
```

### To add to your project

```sh
cargo add strs_tools
```

### Features

This crate uses a feature-based system to allow you to include only the functionality you need. Key features include:

*   `string_indentation`: Tools for adding indentation to lines of text.
*   `string_isolate`: Functions to isolate parts of a string based on delimiters.
*   `string_parse_request`: Utilities for parsing command-like strings with subjects and key-value parameters.
*   `string_parse_number`: Functions for parsing numerical values from strings.
*   `string_split`: Advanced string splitting capabilities with various options for delimiters, quoting, and segment preservation.

You can enable features in your `Cargo.toml` file, for example:
```toml
[dependencies.strs_tools]
version = "0.18.0" # Or your desired version
features = [ "string_split", "string_indentation" ]
```
The `default` feature enables a common set of functionalities. The `full` feature enables all available string utilities. Refer to the `Cargo.toml` for a complete list of features and their dependencies.

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools/module/core/strs_tools
cargo run --example strs_tools_trivial
```
