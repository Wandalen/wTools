#[ allow( unused_imports ) ]
use super::*;
// xxx2 : implement

#[ derive( Debug, PartialEq, the_module::Former ) ]
#[ storage_fields( a : i32, b : Option< String > ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  c : String,
}

// == begin of generated

// == end of generated

tests_impls!
{
  fn test_complex()
  {
    let got = Struct1::former().a( 13 ).b( "abc" ).c( "def" ).form();
    let exp = Struct1
    {
      c : "def".to_string(),
    };
    a_id!( got, exp );
  }
}

tests_index!
{
  test_complex,
}
