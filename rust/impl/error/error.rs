#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

/// Alias for std::error::Error.

pub use std::error::Error as ErrorAdapter;

///
/// Macro to generate error.
///
/// # Sample
/// ```
/// # use werror::*;
/// err!( "No attr" );
/// ```
///

#[ macro_export ]
macro_rules! err
{

  ( $msg : expr ) =>
  {
    $crate::Error::new( $msg )
  };
  ( $msg : expr, $( $arg : expr ),+ ) =>
  {
    $crate::Error::new( format!( $msg, $( $arg ),+ ) )
  };

}

/// baic implementation of generic Error

#[ derive( core::fmt::Debug, core::clone::Clone, core::cmp::PartialEq ) ]
pub struct Error
{
  msg : String,
}

impl Error
{
  /// Constructor expecting message with description.
  pub fn new< Msg : Into< String > >( msg : Msg ) -> Error
  {
    Error { msg : msg.into() }
  }
  /// Message with description getter.
  pub fn msg( &self ) -> &String
  {
    &self.msg
  }
}

impl core::fmt::Display for Error
{
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result
  {
    write!( f, "{}", self.msg )
  }
}

impl std::error::Error for Error
{
  fn description( &self ) -> &str
  {
    &self.msg
  }
}
