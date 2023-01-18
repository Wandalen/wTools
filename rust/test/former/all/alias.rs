#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

only_for_wtools!
{
  #[ allow( unused_imports ) ]
  use wtools::meta::*;
  #[ allow( unused_imports ) ]
  use wtools::former::Former;
}

only_for_local_module!
{
  #[ allow( unused_imports ) ]
  use meta_tools::*;
  #[ allow( unused_imports ) ]
  use former::Former;
}

//

tests_impls!
{
  fn test_alias()
  {
    #[ derive( Debug, PartialEq, Former) ]
    pub struct AliasTestStruct
    {
      #[ alias( first_field ) ]
      string_field: String,
      #[ alias( second_field ) ]
      i32_field: i32,
      i8_field: i8,
    }

    let test_struct = AliasTestStruct::former()
    .first_field( "first_field" )
    .i32_field( 2 )
    .i8_field( 1 )
    .form();

    let expected_struct = AliasTestStruct
    {
      string_field: "first_field".to_string(),
      i32_field: 2,
      i8_field: 1,
    };

    a_id!( test_struct, expected_struct );
  }
}

//

tests_index!
{
  test_alias,
}
