<!-- {{# generate.module_header{} #}} -->

# Module :: wca
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewCaPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewCaPush.yml) [![docs.rs](https://img.shields.io/docsrs/wca?color=e3e8f0&logo=docs.rs)](https://docs.rs/wca) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwca_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20wca_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.

## Sample

<!-- {{# generate.module_sample{} #}} -->

```rust
#[ cfg( feature = "use_std" ) ]
{
  use wca::*;
  use wca::string::parse_request::OpType;

  let instruction = instruction::instruction_parse()
  .instruction( ".get some v:1" )
  .perform();
  let properties_map = std::collections::HashMap::from([ ( "v".to_string(), OpType::Primitive( "1".to_string() ) ) ]);
  let exp = instruction::Instruction
  {
    err : None,
    command_name : ".get".to_string(),
    subject : "some".to_string(),
    properties_map,
  };
  assert_eq!( instruction, exp );
}
```

### To add to your project

```sh
cargo add wca
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/wca_trivial
cargo run
```

