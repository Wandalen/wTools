#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn empty_path() 
{
  let path = "";
  assert_eq!( the_module::path::ext( path ), "" );
}

#[ test ]
fn txt_extension() 
{
  let path = "some.txt";
  assert_eq!( the_module::path::ext( path ), "txt" );
}

#[ test ]
fn path_with_non_empty_dir_name() 
{
  let path = "/foo/bar/baz.asdf";
  assert_eq!( the_module::path::ext( path ), "asdf" );
}

#[ test ]
fn hidden_file() 
{
  let path = "/foo/bar/.baz";
  assert_eq!( the_module::path::ext( path ), "" );
}

#[ test ]
fn several_extension() 
{
  let path = "/foo.coffee.md";
  assert_eq!( the_module::path::ext( path ), "md" );
}

#[ test ]
fn file_without_extension() 
{
  let path = "/foo/bar/baz";
  assert_eq!( the_module::path::ext( path ), "" );
}