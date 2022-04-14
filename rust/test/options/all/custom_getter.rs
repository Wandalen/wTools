
#[cfg( feature = "in_wtools" )]
use wtools::options::*;
#[cfg( not( feature = "in_wtools" ) )]
use woptions::*;

Options!{ split< 'a >
{

  pub src : &'a str;
  pub delimeter : &'a str;
  #[ default( true ) ]
  pub left : bool;

  /* xxx */
  fn left( &self ) -> &bool
  {
    &!self.left
  };

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

include!( "./custom_getter_only_test.rs" );
