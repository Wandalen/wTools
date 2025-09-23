use super :: *;

//

#[ test ]
fn concat() 
{
  use the_module ::ct;

  const KEYWORD: &str = "keyword";
  let got = ct ::str ::concat!("Known attirbutes are: ", KEYWORD, ".",);
  let exp = "Known attirbutes are: keyword.";
  assert_eq!(got, exp);
}

//

#[ test ]
fn format() 
{
  use the_module ::ct;

  const KEYWORD: &str = "keyword";
  let got = ct ::str ::format!("Known attirbutes are: {}{}", KEYWORD, ".",);
  let exp = "Known attirbutes are: keyword.";
  assert_eq!(got, exp);
}
