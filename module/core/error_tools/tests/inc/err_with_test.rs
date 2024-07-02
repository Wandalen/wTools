#![ allow( unused_imports ) ]
use super::*;

#[ test ]
fn err_with()
{

  use the_module::ErrWith;
  let result : Result< (), std::io::Error > = Err( std::io::Error::new( std::io::ErrorKind::Other, "an error occurred" ) );
  let got : Result< (), ( &str, std::io::Error ) > = result.err_with( || "additional context" );
  let exp : Result< (), ( &str, std::io::Error ) > = Err( ( "additional context", std::io::Error::new( std::io::ErrorKind::Other, "an error occurred" ) ) );
  assert_eq!( got.as_ref().unwrap_err().0, exp.as_ref().unwrap_err().0 );
  assert!( got.is_err() );

}

//

#[ test ]
fn err_with_report()
{

  use error_tools::ErrWith;
  let result : Result< (), std::io::Error > = Err( std::io::Error::new( std::io::ErrorKind::Other, "an error occurred" ) );
  let report = "additional context";
  let got : Result< (), ( &str, std::io::Error ) > = result.err_with_report( &report );
  let exp : Result< (), ( &str, std::io::Error ) > = Err( ( "additional context", std::io::Error::new( std::io::ErrorKind::Other, "an error occurred" ) ) );
  assert_eq!( got.as_ref().unwrap_err().0, exp.as_ref().unwrap_err().0 );
  assert!( got.is_err() );

}
