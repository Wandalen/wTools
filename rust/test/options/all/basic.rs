
use woptions::*;

Options!{ split< 'a >
{
  #![ derive( PartialOrd ) ]

  pub src : &'a str;
  pub delimeter : &'a str;
  #[ default( true ) ]
  pub left : bool;

  fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) >
  where
    Self : Sized,
  {
    if *self.left()
    {
      Box::new( self.src().split( self.delimeter() ) )
    }
    else
    {
      Box::new( self.src().rsplit( self.delimeter() ) )
    }
  }

}}

//

include!( "./basic_only_test.rs" );
