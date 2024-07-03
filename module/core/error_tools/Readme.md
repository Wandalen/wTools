<!-- {{# generate.module_header{} #}} -->

# Module :: error_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_error_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_error_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/error_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/error_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ferror_tools%2Fexamples%2Ferror_tools_trivial.rs,RUN_POSTFIX=--example%20error_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Basic exceptions handling mechanism.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust ignore
#[ cfg( feature = "enabled" ) ]
fn main()
{
  let err = f1();
  println!( "{err:#?}" );
  // < Err(
  // <    BasicError {
  // <        msg: "Some error",
  // <    },
  // < )
}

#[ cfg( feature = "enabled" ) ]
fn f1() -> error_tools::untyped::Result< () >
{
  let _read = std::fs::read_to_string( "Cargo.toml" )?;
  Err( error_tools::BasicError::new( "Some error" ).into() )
}
```

<!-- qqq : investigate use-cases and write good documentation -->
<!-- qqq : make sure it work in no_std -->

### To add to your project

```sh
cargo add error_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example error_tools_trivial
```
