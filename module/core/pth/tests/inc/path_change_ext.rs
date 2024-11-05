#[ allow( unused_imports ) ]
use super::*;


#[ test ]
fn test_empty_ext() 
{
  let got = the_module::path::change_ext( "some.txt", "" );
  let expected = "some";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_simple_change_extension() 
{
  let got = the_module::path::change_ext( "some.txt", "json" );
  let expected = "some.json";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_path_with_non_empty_dir_name() 
{
  let got = the_module::path::change_ext( "/foo/bar/baz.asdf", "txt" );
  let expected = "/foo/bar/baz.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_change_extension_of_hidden_file() 
{
  let got = the_module::path::change_ext( "/foo/bar/.baz", "sh" );
  let expected = "/foo/bar/.baz.sh";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_change_extension_in_composite_file_name() 
{
  let got = the_module::path::change_ext( "/foo.coffee.md", "min" );
  let expected = "/foo.coffee.min";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_add_extension_to_file_without_extension() 
{
  let got = the_module::path::change_ext( "/foo/bar/baz", "txt" );
  let expected = "/foo/bar/baz.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_path_folder_contains_dot_file_without_extension() 
{
  let got = the_module::path::change_ext( "/foo/baz.bar/some.md", "txt" );
  let expected = "/foo/baz.bar/some.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_relative_path_1() 
{
  let got = the_module::path::change_ext( "./foo/.baz", "txt" );
  let expected = "./foo/.baz.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_relative_path_2() 
{
  let got = the_module::path::change_ext( "./.baz", "txt" );
  let expected = "./.baz.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_relative_path_3() 
{
  let got = the_module::path::change_ext( ".baz", "txt" );
  let expected = ".baz.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_relative_path_4() 
{
  let got = the_module::path::change_ext( "./baz", "txt" );
  let expected = "./baz.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_relative_path_5() 
{
  let got = the_module::path::change_ext( "./foo/baz", "txt" );
  let expected = "./foo/baz.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}

#[ test ]
fn test_relative_path_6() 
{
  let got = the_module::path::change_ext( "./foo/", "txt" );
  let expected = "./foo/.txt";
  assert_eq!( got.unwrap().to_string_lossy(), expected );
}