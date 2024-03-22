use super::*;
use super::the_module::string::parse_request as parse;
use std::collections::HashMap;

//

tests_impls!
{
  fn op_type_from_into()
  {
    let got = parse::OpType::from( 1 );
    let exp = parse::OpType::Primitive( 1 );
    a_id!( got, exp );

    let got = parse::OpType::from( vec![ 1, 2 ] );
    let exp = parse::OpType::Vector( vec![ 1, 2 ] );
    a_id!( got, exp );

    /* */

    let op = parse::OpType::from( vec![ 1, 2 ] );
    let got : Vec< isize > = op.into();
    a_id!( got, vec![ 1, 2 ] );

    /* */

    let op = parse::OpType::from( 1 );
    let got = op.primitive(); /* rrr : for Dmytro : does not work properly, find better way to convert types */
    a_id!( got.unwrap(), 1 );

    let op = parse::OpType::from( vec![ 1, 2 ] );
    let got : Vec< isize > = op.vector().unwrap();
    a_id!( got, vec![ 1, 2 ] );

    let op = parse::OpType::from( 1 );
    let got = op.vector();
    a_id!( got, None );

    let op : parse::OpType< usize > = parse::OpType::from( vec![ 1, 2 ] );
    let got = op.primitive();
    a_id!( got, None );
  }

  //

  fn basic()
  {
    let src = "";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut exp = parse::Request::default();
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = " ";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut exp = parse::Request::default();
    exp.original = " ";
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "  \t ";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut exp = parse::Request::default();
    exp.original = "  \t ";
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );
  }

  //

  fn with_subject_and_map()
  {
    let src = "subj";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut exp = parse::Request::default();
    exp.original = "subj";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.maps = vec![ HashMap::new() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj with space";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut exp = parse::Request::default();
    exp.original = "subj with space";
    exp.subject = "subj with space".to_string();
    exp.subjects = vec![ "subj with space".to_string() ];
    exp.maps = vec![ HashMap::new() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj v:1";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:1";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj v:1 r:some";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
    options.insert( String::from( "r" ), parse::OpType::Primitive( String::from( "some" ) ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:1 r:some";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    /* */

    let src = "subj1 ; subj2";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut exp = parse::Request::default();
    exp.original = "subj1 ; subj2";
    exp.subject = "subj1".to_string();
    exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
    exp.maps = vec![ HashMap::new(), HashMap::new() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj1 v:1 ; subj2";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
    let mut exp = parse::Request::default();
    exp.original = "subj1 v:1 ; subj2";
    exp.subject = "subj1".to_string();
    exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone(), HashMap::new() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj1 v:1 ; subj2 v:2";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut options1 = HashMap::new();
    options1.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
    let mut options2 = HashMap::new();
    options2.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "2" ) ) );
    let mut exp = parse::Request::default();
    exp.original = "subj1 v:1 ; subj2 v:2";
    exp.subject = "subj1".to_string();
    exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
    exp.map = options1.clone();
    exp.maps = vec![ options1.clone(), options2.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj1 v:1 ne:-2 ; subj2 v:2 r:some";
    let req = the_module::string::request_parse()
    .src( src )
    .perform();
    let mut options1 = HashMap::new();
    options1.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
    options1.insert( String::from( "ne" ), parse::OpType::Primitive( String::from( "-2" ) ) );
    let mut options2 = HashMap::new();
    options2.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "2" ) ) );
    options2.insert( String::from( "r" ), parse::OpType::Primitive( String::from( "some" ) ) );
    let mut exp = parse::Request::default();
    exp.original = "subj1 v:1 ne:-2 ; subj2 v:2 r:some";
    exp.subject = "subj1".to_string();
    exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
    exp.map = options1.clone();
    exp.maps = vec![ options1.clone(), options2.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );
  }

  //

  fn with_several_values()
  {
    let src = "subj v:1 v:2";
    let req = the_module::string::request_parse()
    .src( src )
    .several_values( false )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Primitive( "2".to_string() ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:1 v:2";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj v:1 v:2";
    let req = the_module::string::request_parse()
    .src( src )
    .several_values( true )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "1".to_string(), "2".to_string() ] ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:1 v:2";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );
  }

  //

  fn with_parsing_arrays()
  {
    let src = "subj v:[1,2]";
    let req = the_module::string::request_parse()
    .src( src )
    .parsing_arrays( false )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Primitive( "[1,2]".to_string() ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:[1,2]";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj v:[1,2]";
    let req = the_module::string::request_parse()
    .src( src )
    .parsing_arrays( true )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "1".to_string(), "2".to_string() ] ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:[1,2]";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    /* */

    let src = "subj v:[1,2] v:3";
    let req = the_module::string::request_parse()
    .src( src )
    .parsing_arrays( true )
    .several_values( true )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string() ] ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:[1,2] v:3";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj v:3 v:[1,2]";
    let req = the_module::string::request_parse()
    .src( src )
    .parsing_arrays( true )
    .several_values( true )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "3".to_string(), "1".to_string(), "2".to_string() ] ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:3 v:[1,2]";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );

    let src = "subj v:[1,2] v:[3,4]";
    let req = the_module::string::request_parse()
    .src( src )
    .parsing_arrays( true )
    .several_values( true )
    .perform();
    let mut options = HashMap::new();
    options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string() ] ) );
    let mut exp = parse::Request::default();
    exp.original = "subj v:[1,2] v:[3,4]";
    exp.subject = "subj".to_string();
    exp.subjects = vec![ "subj".to_string() ];
    exp.map = options.clone();
    exp.maps = vec![ options.clone() ];
    exp.key_val_delimeter = ":";
    exp.commands_delimeter = ";";
    a_id!( req, exp );
  }
}

//

tests_index!
{
  op_type_from_into,
  basic,
  with_subject_and_map,
  with_several_values,
  with_parsing_arrays,
}
