# DEPRECATED: Module :: wstring_tools
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=wstring_tools)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/wstring_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/wstring_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Falias%2Fwstring_tools%2Fexamples%2Fwstring_toolst_trivial_sample.rs,RUN_POSTFIX=--example%20wstring_toolst_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Tools to manipulate strings.

### Scope

Re-exports the complete public API of `strs_tools` under the `wstring_tools` name.
Enables feature-gated access to string indentation and number parsing by default;
string splitting, isolation, and request parsing are available as optional features.

### Basic use-case

```rust
#[ cfg( all( feature = "split", not( feature = "no_std" ) ) ) ]
{
  /* delimeter exists */
  let src = "abc def";
  let iter = wstring_tools::string::split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .perform();
  let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc", " ", "def" ] );

  /* delimeter not exists */
  let src = "abc def";
  let iter = wstring_tools::string::split()
  .src( src )
  .delimeter( "g" )
  .perform();
  let iterated = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
  assert_eq!( iterated, vec![ "abc def" ] );
}
```

### To add to your project

```sh
cargo add wstring_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd module/alias/wstring_tools
cargo run --example wstring_toolst_trivial_sample --all-features
```
