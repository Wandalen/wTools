<!-- {{# generate.module_header{} #}} -->

# Module :: werror
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_werror_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_werror_push.yml) [![docs.rs](https://img.shields.io/docsrs/werror?color=e3e8f0&logo=docs.rs)](https://docs.rs/werror) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Falias%2Fwerror%2Fexamples%2Fwerror_tools_trivial.rs,RUN_POSTFIX=--example%20werror_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Basic exceptions handling mechanism.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust ignore
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

fn f1() -> werror::Result< () >
{
  let _read = std::fs::read_to_string( "Cargo.toml" )?;
  Err( werror::BasicError::new( "Some error" ).into() )
}
```

### To add to your project

```sh
cargo add werror
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example werror_tools_trivial
```
