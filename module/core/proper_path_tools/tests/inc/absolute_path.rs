#[ allow( unused_imports ) ]
use super::*;
use the_module::AbsolutePath;
use std::path::Path;
use std::path::PathBuf;

#[ test ]
fn basic()
{
  let path1 = "/some/absolute/path";
  let got : AbsolutePath = path1.try_into().unwrap();
  println!( "got : {}", &got );
  println!( "path1 : {}", &path1 );
  a_id!( &got.to_string(), path1 );
}

#[ test ]
fn test_to_string_lossy() 
{
  let path : AbsolutePath = "/path/to/file.txt".try_into().unwrap();
  let result = path.to_string_lossy();
  assert_eq!( result, "/path/to/file.txt" );
}
#[test]
fn test_to_string_lossy_hard() 
{
  let abs_path : AbsolutePath = "/path/with/😀/unicode.txt".try_into().unwrap();
  let string_lossy = abs_path.to_string_lossy();
  assert_eq!( string_lossy, "/path/with/\u{1F600}/unicode.txt" );
}


#[test]
fn test_try_from_pathbuf() 
{
  
  let path_buf = PathBuf::from( "/path/to/some/file.txt" );
  let abs_path : AbsolutePath = path_buf.try_into().unwrap();
  assert_eq!( abs_path.to_string_lossy(), "/path/to/some/file.txt" );
}

#[test]
fn test_try_from_path() 
{
  let path = Path::new( "/path/to/some/file.txt" );
  let abs_path : AbsolutePath = path.try_into().unwrap();
  assert_eq!( abs_path.to_string_lossy(), "/path/to/some/file.txt" );
}

#[test]
fn test_parent() 
{
  let abs_path : AbsolutePath = "/path/to/some/file.txt".try_into().unwrap();
  let parent_path = abs_path.parent().unwrap();
  assert_eq!( parent_path.to_string_lossy(), "/path/to/some" );
}

#[test]
fn test_join() 
{
  let abs_path : AbsolutePath = "/path/to/some".try_into().unwrap();
  let joined_path = abs_path.join( "file.txt" );
  assert_eq!( joined_path.to_string_lossy(), "/path/to/some/file.txt" );
}