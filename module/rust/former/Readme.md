# module::Former

Former - variation of builder pattern.

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/implements_trivial
cargo run
```

### To add to your project

```
cargo add implements
```

### Sample

``` rust sample test
use former::Former;

#[derive( Debug, PartialEq, Former )]
pub struct Structure1
{
  int_1 : i32,
  string_1 : String,
  vec_1 : Vec< String >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

fn main()
{

  let command1 = Structure1::former()
  .int_1( 13 )
  .string_1( "Abcd".to_string() )
  .vec_1().push( "ghi" ).push( "klm" ).end()
  .hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
  .string_optional_1( "dir1" )
  .form();
  dbg!( &command1 );

// <  &command1 = Structure1 {
// <   int_1: 13,
// <   string_1: "Abcd",
// <   vec_1: [
// <       "ghi",
// <       "klm",
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

}
```
