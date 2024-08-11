#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn test_empty_string()
{
  use the_module::string;
  let input = "";
  let exp = [ 0, 1 ];
  let got = string::size( input );
  assert_eq!( exp, got );
}

#[ test ]
fn test_single_line_no_newline()
{
  use the_module::string;
  let input = "Hello, World!";
  let exp = [ 13, 1 ];
  let got = string::size( input );
  assert_eq!( exp, got );
}

#[ test ]
fn test_single_line_with_newline()
{
  use the_module::string;
  let input = "Hello, World!\n";
  let exp = [ 13, 2 ];
  let got = string::size( input );
  assert_eq!( exp, got );
}

#[ test ]
fn test_multiple_lines_varying_lengths()
{
  use the_module::string;
  let input = "Hello\nWorld!\nThis is a test.";
  let exp = [ 15, 3 ];
  let got = string::size( input );
  assert_eq!( exp, got );
}

#[ test ]
fn test_only_newlines()
{
  use the_module::string;
  let input = "\n\n\n";
  let exp = [ 0, 4 ];
  let got = string::size( input );
  assert_eq!( exp, got );
}

#[ test ]
fn test_very_long_lines()
{
  use the_module::string;
  let input = "a".repeat( 1000 );
  let exp = [ 1000, 1 ];
  let got = string::size( input );
  assert_eq!( exp, got );
}

#[ test ]
fn test_special_characters_whitespace()
{
  use the_module::string;
  let input = " \t\n \t\n";
  let exp = [ 2, 3 ];
  let got = string::size( input );
  assert_eq!( exp, got );
}