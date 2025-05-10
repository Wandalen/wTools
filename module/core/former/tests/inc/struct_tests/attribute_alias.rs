#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

//

tests_impls!
{
  fn test_alias()
  {
    #[ derive( Debug, PartialEq, the_module::Former ) ]
    // #[ derive( Debug, PartialEq, the_module::Former ) ] #[ debug ]
    // #[ derive( Debug, PartialEq ) ]
    pub struct AliasTestStruct
    {
      #[ scalar( name = first_field ) ]
      string_field : String,
      #[ scalar( name = second_field ) ]
      i32_field : i32,
      i8_field : i8,
    }

    // == begin of generated

    // == end of generated

    let test_struct = AliasTestStruct::former()
    .first_field( "first_field" )
    .second_field( 2 )
    // .i32_field( 2 )
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
