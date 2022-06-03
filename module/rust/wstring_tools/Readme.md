# Module :: wstring_tools
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewStringToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewStringToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/wstring_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/wstring_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

String tools.

<!-- xxx : qqq for Rust : write me --> <!-- aaa : done -->

### Sample

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

  /* delimeter no exists */
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
