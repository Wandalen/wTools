
mod split
{

  use former::Former;

  #[ derive( PartialOrd ) ]
  #[ derive( Former, PartialEq, Debug ) ]
  #[ form_after( fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) > ) ]
  pub struct Options< 'a >
  {
    pub src : &'a str,
    pub delimeter : &'a str,
    #[ default( true ) ]
    pub left : bool,
  }

  pub trait OptionsAdapter< 'a >
  {
    fn src( &self ) -> &'a str;
    fn delimeter( &self ) -> &'a str;
    fn left( &self ) -> &bool;
    #[ inline ]
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
  }

  impl< 'a > OptionsAdapter< 'a > for Options< 'a >
  {
    #[ inline ]
    fn src( &self ) -> &'a str
    {
      &self.src
    }
    #[ inline ]
    fn delimeter( &self ) -> &'a str
    {
      &self.delimeter
    }
    #[ inline ]
    fn left( &self ) -> &bool
    {
      &self.left
    }
  }

  #[ inline ]
  pub fn former< 'a >() -> OptionsFormer< 'a >
  {
    Options::< 'a >::former()
  }

}

#[ inline ]
fn split< 'a >() -> split::OptionsFormer< 'a >
{
  split::former::< 'a >()
}

//

include!( "./basic_only_test.rs" );
