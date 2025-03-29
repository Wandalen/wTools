#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn empty_string()
{
  use the_module::string;
  let input = "";
  let exp = [ 0, 1 ];
  let got = string::size( input );
  assert_eq!( got, exp );
}

#[ test ]
fn single_line_no_newline()
{
  use the_module::string;

  let input = "Hello, World!";
  let exp = [ 13, 1 ];
  let got = string::size( input );
  assert_eq!( got, exp );

  let input = "[\"file1\", \"file2\"]";
  let exp = [ 18, 1 ];
  let got = string::size( input );
  assert_eq!( got, exp );

}

#[ test ]
fn single_line_with_newline()
{
  use the_module::string;
  let input = "Hello, World!\n";
  let exp = [ 13, 2 ];
  let got = string::size( input );
  assert_eq!( got, exp );
}

#[ test ]
fn multiple_lines_varying_lengths()
{
  use the_module::string;
  let input = "Hello\nWorld!\nThis is a test.";
  let exp = [ 15, 3 ];
  let got = string::size( input );
  assert_eq!( got, exp );
}

#[ test ]
fn only_newlines()
{
  use the_module::string;
  let input = "\n\n\n";
  let exp = [ 0, 4 ];
  let got = string::size( input );
  assert_eq!( got, exp );
}

#[ test ]
fn very_long_lines()
{
  use the_module::string;
  let input = "a".repeat( 1000 );
  let exp = [ 1000, 1 ];
  let got = string::size( input );
  assert_eq!( got, exp );
}

#[ test ]
fn special_characters_whitespace()
{
  use the_module::string;
  let input = " \t\n \t\n";
  let exp = [ 2, 3 ];
  let got = string::size( input );
  assert_eq!( got, exp );
}

#[ test ]
fn assumption_str_lines_skip_the_last_line()
{

  let src = "abc";
  let got : Vec< &str > = src.lines().collect();
  let exp = vec![ "abc" ];
  assert_eq!( got, exp );

  let src = "";
  let got : Vec< &str > = src.lines().collect();
  let exp : Vec< &str > = vec![];
  // let exp = vec![ "" ]; // should be
  assert_eq!( got, exp );

  let src = "\n";
  let got : Vec< &str > = src.lines().collect();
  let exp = vec![ "" ];
  // let exp = vec![ "", "" ]; // should be
  assert_eq!( got, exp );

  let src = "a\nb";
  let got : Vec< &str > = src.lines().collect();
  let exp = vec![ "a", "b" ];
  assert_eq!( got, exp );

  let src = "\na\nb\n";
  let got : Vec< &str > = src.lines().collect();
  let exp = vec![ "", "a", "b" ];
  // let exp = vec![ "", "a", "b", "" ]; should be
  assert_eq!( got, exp );

}

#[ test ]
fn lines_basic()
{
  use the_module::string;

  let src = "abc";
  let got : Vec< &str > = string::lines( src ).collect();
  let exp = vec![ "abc" ];
  assert_eq!( got, exp );

  let src = "";
  let got : Vec< &str > = string::lines( src ).collect();
  let exp = vec![ "" ];
  assert_eq!( got, exp );

  let src = "\n";
  let got : Vec< &str > = string::lines( src ).collect();
  let exp = vec![ "", "" ];
  assert_eq!( got, exp );

  let src = "a\nb";
  let got : Vec< &str > = string::lines( src ).collect();
  let exp = vec![ "a", "b" ];
  assert_eq!( got, exp );

  let src = "\na\nb\n";
  let got : Vec< &str > = string::lines( src ).collect();
  let exp = vec![ "", "a", "b", "" ];
  assert_eq!( got, exp );
}
