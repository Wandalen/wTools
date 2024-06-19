#[ allow( unused_imports ) ]
use super::*;
use std::path::PathBuf;
use the_module::path;

#[ test ]
fn assumptions()
{

  assert_eq!( PathBuf::from( "c:/src/" ).is_absolute(), true );
  assert_eq!( PathBuf::from( "/c/src/" ).is_absolute(), false );
  assert_eq!( PathBuf::from( "/c:/src/" ).is_absolute(), false );
  assert_eq!( PathBuf::from( "/c/src/" ).is_absolute(), false );

}

#[ test ]
fn basic()
{

  let got = path::canonicalize( PathBuf::from( "src" ) );
  let exp = PathBuf::from( "src" );
  assert_eq!( got.unwrap(), exp );

  let got = path::canonicalize( PathBuf::from( "\\src" ) );
  let exp = PathBuf::from( "/src" );
  assert_eq!( got.unwrap(), exp );

  let got = path::canonicalize( PathBuf::from( "\\src\\" ) );
  let exp = PathBuf::from( "/src/" );
  assert_eq!( got.unwrap(), exp );

  let got = path::canonicalize( PathBuf::from( "/src" ) );
  let exp = PathBuf::from( "/src" );
  assert_eq!( got.unwrap(), exp );

  let got = path::canonicalize( PathBuf::from( "/src/" ) );
  let exp = PathBuf::from( "/src/" );
  assert_eq!( got.unwrap(), exp );

  let got = path::canonicalize( PathBuf::from( "./src/" ) );
  let exp = PathBuf::from( "./src/" );
  assert_eq!( got.unwrap(), exp );

  // xxx : qqq : does not work
  // let got = path::canonicalize( PathBuf::from( "c:/src/" ) );
  // let exp = PathBuf::from( "/c/src/" );
  // assert_eq!( got.unwrap(), exp );

}
