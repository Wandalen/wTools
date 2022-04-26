# module::wca

The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.

### Sample

```rust
use wca::*;
use wstring_tools::string::parse::OpType;

fn main()
{
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

