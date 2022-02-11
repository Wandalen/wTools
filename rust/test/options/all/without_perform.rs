
#[cfg( feature = "in_wtools" )]
use wtools::options::*;
#[cfg( not( feature = "in_wtools" ) )]
use woptions::*;

Options!{ split< 'a >
{
  #![ derive( PartialOrd ) ]

  pub src : &'a str;
  pub delimeter : &'a str;
  #[ default( true ) ]
  pub left : bool;

}}

//

include!( "./without_perform_only_test.rs" );
