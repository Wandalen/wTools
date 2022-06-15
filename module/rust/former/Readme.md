# Module :: former
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleFormerPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleFormerPush.yml) [![docs.rs](https://img.shields.io/docsrs/former?color=e3e8f0&logo=docs.rs)](https://docs.rs/former) [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fformer_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20former_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Former - variation of builder pattern.

### Sample

```rust
use former::Former;

#[derive( Debug, PartialEq, Former )]
pub struct Structure1
{
  int_1 : i32,
  string_1 : String,
  vec_1 : Vec< i32 >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

let struct1 = Structure1::former()
.int_1( 13 )
.string_1( "Abcd".to_string() )
.vec_1().replace( vec![ 1, 3 ] ).end()
.hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
.string_optional_1( "dir1" )
.form();
dbg!( &struct1 );

// <  &struct1 = Structure1 {
// <   int_1: 13,
// <   string_1: "Abcd",
// <   vec_1: [
// <       1,
// <       3,
// <   ],
// <   hashmap_strings_1: {
// <       "k1": "v1",
// <       "k2": "v2",
// <   },
// <   int_optional_1: None,
// <   string_optional_1: Some(
// <       "dir1",
// <   ),
// < }
```

### To add to your project

```sh
cargo add former
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/former_trivial
cargo run
```
