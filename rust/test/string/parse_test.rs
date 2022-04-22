
use wtest_basic::*;

#[cfg( feature = "in_wtools" )]
use wtools::string as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use wstring_tools as TheModule;

use TheModule::string::parse as parse;
use std::collections::HashMap;

//

fn _basic()
{
  let src = "";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = " ";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = " ";
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "  \t ";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = "  \t ";
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );
}

fn _with_subject_and_map()
{
  let src = "subj";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = "subj";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.maps = vec![ HashMap::new() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj with space";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = "subj with space";
  exp.subject = "subj with space".to_string();
  exp.subjects = vec![ "subj with space".to_string() ];
  exp.maps = vec![ HashMap::new() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj v:1";
  let req = TheModule::string::request_parse()
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
  assert_eq!( req, exp );

  let src = "subj v:1 r:some";
  let req = TheModule::string::request_parse()
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
  assert_eq!( req, exp );

  /* */

  let src = "subj1 ; subj2";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = "subj1 ; subj2";
  exp.subject = "subj1".to_string();
  exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
  exp.maps = vec![ HashMap::new(), HashMap::new() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj1 v:1 ; subj2";
  let req = TheModule::string::request_parse()
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
  assert_eq!( req, exp );

  let src = "subj1 v:1 ; subj2 v:2";
  let req = TheModule::string::request_parse()
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
  assert_eq!( req, exp );

  let src = "subj1 v:1 ne:-2 ; subj2 v:2 r:some";
  let req = TheModule::string::request_parse()
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
  assert_eq!( req, exp );
}

//

test_suite!
{
  basic,
  with_subject_and_map,
}
