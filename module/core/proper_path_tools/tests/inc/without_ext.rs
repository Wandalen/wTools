#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn empty_path() 
{
  let path = "";
  let expected = None;
  assert_eq!( the_module::path::without_ext( path ), expected );
}

#[ test ]
fn txt_extension() 
{
  let path = "some.txt";
  let expected = "some";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn path_with_non_empty_dir_name() 
{
  let path = "/foo/bar/baz.asdf";
  let expected = "/foo/bar/baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn hidden_file() 
{
  let path = "/foo/bar/.baz";
  let expected = "/foo/bar/.baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn file_with_composite_file_name() 
{
  let path = "/foo.coffee.md";
  let expected = "/foo.coffee";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn path_without_extension() 
{
  let path = "/foo/bar/baz";
  let expected = "/foo/bar/baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn relative_path_1() 
{
  let path = "./foo/.baz";
  let expected = "./foo/.baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn relative_path_2() 
{
  let path = "./.baz";
  let expected = "./.baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn relative_path_3() 
{
  let path = ".baz.txt";
  let expected = ".baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn relative_path_4() 
{
  let path = "./baz.txt";
  let expected = "./baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn relative_path_5() 
{
  let path = "./foo/baz.txt";
  let expected = "./foo/baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn relative_path_6() 
{
  let path = "./foo/";
  let expected = "./foo/";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn relative_path_7()
{
  let path = "baz";
  let expected = "baz";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}

#[ test ]
fn relative_path_8() 
{
  let path = "baz.a.b";
  let expected = "baz.a";
  assert_eq!( the_module::path::without_ext( path ).unwrap().to_string_lossy(), expected );
}