use super::*;

use the_module::
{
  AbsolutePath,
  Path,
  PathBuf,
};

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
#[ cfg( not( feature="no_std" ) ) ]
fn test_try_from_pathbuf()
{

  let path_buf = PathBuf::from( "/path/to/some/file.txt" );
  let abs_path : AbsolutePath = path_buf.try_into().unwrap();
  assert_eq!( abs_path.to_string_lossy(), "/path/to/some/file.txt" );
}

#[test]
#[ cfg( not( feature="no_std" ) ) ]
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

#[test]
fn test_relative_path_try_from_str()
{
  let rel_path_str = "src/main.rs";
  let rel_path = AbsolutePath::try_from( rel_path_str ).unwrap();
  assert_eq!( rel_path.to_string_lossy(), "src/main.rs" );
}

#[test]
#[ cfg( not( feature="no_std" ) ) ]
fn test_relative_path_try_from_pathbuf()
{
  let rel_path_buf = PathBuf::from( "src/main.rs" );
  let rel_path = AbsolutePath::try_from( rel_path_buf.clone() ).unwrap();
  assert_eq!( rel_path.to_string_lossy(), "src/main.rs" );
}

#[test]
#[ cfg( not( feature="no_std" ) ) ]
fn test_relative_path_try_from_path()
{
  let rel_path = Path::new( "src/main.rs" );
  let rel_path_result = AbsolutePath::try_from( rel_path );
  assert!( rel_path_result.is_ok() );
  assert_eq!( rel_path_result.unwrap().to_string_lossy(), "src/main.rs" );
}

#[test]
fn test_relative_path_parent()
{
  let rel_path = AbsolutePath::try_from( "src/main.rs" ).unwrap();
  let parent_path = rel_path.parent().unwrap();
  assert_eq!( parent_path.to_string_lossy(), "src" );
}

#[test]
fn test_relative_path_join()
{
  let rel_path = AbsolutePath::try_from( "src" ).unwrap();
  let joined = rel_path.join( "main.rs" );
  assert_eq!( joined.to_string_lossy(), "src/main.rs" );
}
