
fn main()
{
  use former::Former;

  #[derive( Debug, PartialEq, Former )]
  pub struct Structure1
  {
    int_1 : i32,
    string_1 : String,
    vec_1 : Vec< u32 >,
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

}
