
use wtest_basic::*;

#[cfg( feature = "in_wtools" )]
use wtools::string as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use wstring_tools as TheModule;

use TheModule::string::parse as parse;

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

//

test_suite!
{
  basic,
}
