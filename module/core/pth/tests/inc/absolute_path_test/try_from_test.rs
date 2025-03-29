use super::*;
use std::convert::TryFrom;

#[ test ]
fn try_from_absolute_path_test()
{
  use std::path::{ Path, PathBuf };
  use the_module::AbsolutePath;

  // Create an AbsolutePath instance
  let absolute_path = AbsolutePath::try_from( "/absolute/path" ).unwrap();

  // Test conversion to &str
  let path_str : &str = TryFrom::try_from( &absolute_path ).unwrap();
  println!( "&str from AbsolutePath: {:?}", path_str );
  assert_eq!( path_str, "/absolute/path" );

  // Test conversion to String
  let path_string : String = TryFrom::try_from( &absolute_path ).unwrap();
  println!( "String from AbsolutePath: {:?}", path_string );
  assert_eq!( path_string, "/absolute/path" );

  // Test conversion to PathBuf
  let path_buf : PathBuf = TryFrom::try_from( absolute_path.clone() ).unwrap();
  println!( "PathBuf from AbsolutePath: {:?}", path_buf );
  assert_eq!( path_buf, PathBuf::from( "/absolute/path" ) );

  // Test conversion to &Path
  let path_ref : &Path = absolute_path.as_ref();
  println!( "&Path from AbsolutePath: {:?}", path_ref );
  assert_eq!( path_ref, Path::new( "/absolute/path" ) );

  // Test conversion from &String
  let string_path : String = String::from( "/absolute/path" );
  let absolute_path_from_string : AbsolutePath = TryFrom::try_from( &string_path ).unwrap();
  println!( "AbsolutePath from &String: {:?}", absolute_path_from_string );
  assert_eq!( absolute_path_from_string, absolute_path );

  // Test conversion from String
  let absolute_path_from_owned_string : AbsolutePath = TryFrom::try_from( string_path.clone() ).unwrap();
  println!( "AbsolutePath from String: {:?}", absolute_path_from_owned_string );
  assert_eq!( absolute_path_from_owned_string, absolute_path );

  // Test conversion from &Path
  let path_ref : &Path = Path::new( "/absolute/path" );
  let absolute_path_from_path_ref : AbsolutePath = TryFrom::try_from( path_ref ).unwrap();
  println!( "AbsolutePath from &Path: {:?}", absolute_path_from_path_ref );
  assert_eq!( absolute_path_from_path_ref, absolute_path );

  // Test conversion from PathBuf
  let path_buf_instance : PathBuf = PathBuf::from( "/absolute/path" );
  let absolute_path_from_path_buf : AbsolutePath = TryFrom::try_from( path_buf_instance.clone() ).unwrap();
  println!( "AbsolutePath from PathBuf: {:?}", absolute_path_from_path_buf );
  assert_eq!( absolute_path_from_path_buf, absolute_path );
}