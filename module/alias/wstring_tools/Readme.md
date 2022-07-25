<!-- {{# generate.module_header{} #}} -->

# Module :: wstring_tools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewStringToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewStringToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/wstring_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/wstring_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Tools to manipulate strings.

### Sample

<!-- {{# generate.module_sample{} #}} -->

```rust
#[ cfg( all( feature = "split", feature = "use_std" ) ) ]
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
cd sample/rust/wstring_tools_trivial
cargo run
```
