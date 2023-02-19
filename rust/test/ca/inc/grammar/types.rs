use super::*;
use wca::TryCast;

//

tests_impls!
{
  fn number()
  {
    // basic
    let number = Type::Number.try_cast( "1".into() );

    a_id!( Ok( Value::Number( 1.0 ) ), number );
    let number = number.unwrap();

    let inner_number : i32 = number.clone().into();
    a_id!( 1, inner_number );

    let inner_number : f64 = number.into();
    a_id!( 1.0, inner_number );

    // negative float number
    let number = Type::Number.try_cast( "-3.14".into() );

    a_id!( Ok( Value::Number( -3.14 ) ), number );
    let number = number.unwrap();

    let inner_number : i32 = number.clone().into();
    a_id!( -3, inner_number );

    let inner_number : u32 = number.clone().into();
    a_id!( 0, inner_number );

    let inner_number : f64 = number.into();
    a_id!( -3.14, inner_number );

    // not a number
    let not_number = Type::Number.try_cast( "text".into() );
    a_true!( not_number.is_err() );
  }

  fn string()
  {
    let string = Type::String.try_cast( "some string".into() );

    a_id!( Ok( Value::String( "some string".into() ) ), string );
    let string = string.unwrap();

    let inner_string : String = string.clone().into();
    a_id!( "some string", inner_string );

    let inner_string : &str = string.into();
    a_id!( "some string", inner_string );
  }

  fn path()
  {
    use std::str::FromStr;
    let path = Type::Path.try_cast( "./some/relative/path".into() );

    a_id!( Ok( Value::Path( "./some/relative/path".into() ) ), path );
    let path = path.unwrap();

    let inner_path : std::path::PathBuf = path.into();
    a_id!( std::path::PathBuf::from_str( "./some/relative/path" ).unwrap(), inner_path );
  }
}

//

tests_index!
{
  number,
  string,
  path,
}
