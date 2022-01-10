
use woptions::*;

Options!{ split< 'a >
{
  #![ derive( PartialOrd ) ]

  pub src : &'a str;
  pub delimeter : &'a str;
  #[ default( true ) ]
  pub left : bool;

  fn perform( self ) -> std::str::Split< 'a, &'a str >
  where
    Self : Sized,
  {
    self.src().split( self.delimeter() )
  };

}}

//

include!( "./basic_only_test.rs" );
