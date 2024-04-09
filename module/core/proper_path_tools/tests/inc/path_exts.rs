#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn empty_path() 
{
  let path = "";
  let expected : Vec< String > = vec![];
  assert_eq!( the_module::path::exts( path ), expected );
}

#[ test ]
fn txt_extension() 
{
  let path = "some.txt";
  let expected : Vec< String > = vec![ "txt".to_string() ];
  assert_eq!( the_module::path::exts( path ), expected );
}

#[ test ]
fn path_with_non_empty_dir_name() 
{
  let path = "/foo/bar/baz.asdf";
  let expected : Vec< String > = vec![ "asdf".to_string() ];
  assert_eq!( the_module::path::exts( path ), expected );
}

#[ test ]
fn hidden_file() 
{
  let path = "/foo/bar/.baz";
  let expected : Vec< String > = vec![];
  assert_eq!( the_module::path::exts( path ), expected );
}

#[ test ]
fn several_extension() 
{
  let path = "/foo.coffee.md";
  let expected : Vec< String > = vec![ "coffee".to_string(), "md".to_string() ];
  assert_eq!( the_module::path::exts( path ), expected );
}

#[ test ]
fn hidden_file_extension() 
{
  let path = "/foo/bar/.baz.txt";
  let expected : Vec< String > = vec![ "txt".to_string() ];
  assert_eq!( the_module::path::exts( path ), expected );
}