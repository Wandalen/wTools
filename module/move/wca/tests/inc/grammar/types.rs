use super::*;
use the_module::{ TryCast, Type, Value };

//

tests_impls!
{
  fn number()
  {
    // basic
    let number = Type::Number.try_cast( "1".into() );

    let number = number.unwrap();
    a_id!( Value::Number( 1.0 ) , number );

    let inner_number : i32 = number.clone().into();
    a_id!( 1, inner_number );

    let inner_number : f64 = number.into();
    a_id!( 1.0, inner_number );

    // negative float number
    let number = Type::Number.try_cast( "-3.14".into() );

    let number = number.unwrap();
    a_id!( Value::Number( -3.14 ) , number );

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

    let string = string.unwrap();
    a_id!( Value::String( "some string".into() ) , string );

    let inner_string : String = string.clone().into();
    a_id!( "some string", inner_string );

    let inner_string : &str = string.into();
    a_id!( "some string", inner_string );
  }

  fn boolean()
  {
    // 1 -> Value(true) -> true
    let boolean = Type::Bool.try_cast( "1".into() );

    let boolean = boolean.unwrap();
    a_id!( Value::Bool( true ) , boolean );

    let inner_boolean : bool = boolean.into();
    a_id!( true, inner_boolean );

    // 0 -> Value(false) -> false
    let boolean = Type::Bool.try_cast( "0".into() );

    let boolean = boolean.unwrap();
    a_id!( Value::Bool( false ) , boolean );

    let inner_boolean : bool = boolean.into();
    a_id!( false, inner_boolean );

    // true -> Value(true)
    let boolean = Type::Bool.try_cast( "true".into() );

    let boolean = boolean.unwrap();
    a_id!( Value::Bool( true ) , boolean );

    // false -> Value(false)
    let boolean = Type::Bool.try_cast( "false".into() );

    let boolean = boolean.unwrap();
    a_id!( Value::Bool( false ) , boolean );
  }

  fn path()
  {
    use std::str::FromStr;
    let path = Type::Path.try_cast( "./some/relative/path".into() );

    let path = path.unwrap();
    a_id!( Value::Path( "./some/relative/path".into() ) , path );

    let inner_path : std::path::PathBuf = path.into();
    a_id!( std::path::PathBuf::from_str( "./some/relative/path" ).unwrap(), inner_path );
  }

  fn values_list()
  {
    // strings
    let string = Type::List( Type::String.into(), ',' ).try_cast( "some,string".into() ).unwrap();

    a_id!(
      Value::List( vec![ Value::String( "some".into() ), Value::String( "string".into() ) ] )
    , string );

    let inner_string : Vec< String > = string.clone().into();
    a_id!( vec![ "some".to_string(), "string".into() ], inner_string );

    let inner_string : Vec< &str > = string.into();
    a_id!( vec![ "some", "string" ], inner_string );

    // numbers
    let numbers = Type::List( Type::Number.into(), ';' ).try_cast( "100;3.14".into() );
    let numbers = numbers.unwrap();
    a_id!(
      Value::List( vec![ Value::Number( 100.0 ), Value::Number( 3.14 ) ] )
    , numbers );

    let inner_numbers : Vec< i32 > = numbers.clone().into();
    a_id!( vec![ 100, 3 ], inner_numbers );

    let inner_numbers : Vec< f64 > = numbers.into();
    a_id!( vec![ 100.0, 3.14 ], inner_numbers );
  }

  // xxx : The try_cast method on value is designed to convert user input strings into parsed values, such as lists of strings or numbers. However, when converting these parsed values back into their original string representations using the display method, the resulting string may not match the original user input.
  fn values_list_display()
  {
    let origin_string = "some,string";
    let string = Type::List( Type::String.into(), ',' ).try_cast( origin_string.into() ).unwrap();
    a_id!( origin_string, string.to_string() );

    // xxx clarification is needed : qqq : that fails now. suggest solution
    // let origin_string = "100;3.14";
    // let string = Type::List( Type::Number.into(), ';' ).try_cast( origin_string.into() ).unwrap();
    // a_id!( origin_string, string.to_string() );
  }

}

//

tests_index!
{
  number,
  string,
  path,
  boolean,
  values_list,
  values_list_display,
}
