
use super::*;

//

#[ test ]
fn concat()
{
  use the_module::ct;

  const KEYWORD : &'static str = "keyword";
  let got = ct::str::concat!
  (
    "Known attirbutes are : ",
    KEYWORD,
    ".",
  );
  let exp = "Known attirbutes are : keyword.";
  a_id!( got, exp );

}

//

#[ test ]
fn format()
{
  use the_module::ct;

  const KEYWORD : &'static str = "keyword";
  let got = ct::str::format!
  (
    "Known attirbutes are : {}{}",
    KEYWORD,
    ".",
  );
  let exp = "Known attirbutes are : keyword.";
  a_id!( got, exp );

}
