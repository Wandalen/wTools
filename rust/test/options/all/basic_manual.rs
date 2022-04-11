mod split
{

  #[cfg( feature = "in_wtools" )]
  use wtools::former::Former;
  #[cfg( not( feature = "in_wtools" ) )]
  use former::Former;

  #[ derive( PartialOrd ) ]
  #[ derive( Former, PartialEq, Debug ) ]
  #[ perform( fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) > ) ]
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
    fn src_mut( &mut self ) -> &mut &'a str;
    fn delimeter_mut( &mut self ) -> &mut &'a str;
    fn left_mut( &mut self ) -> &mut bool;
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

    #[ inline ]
    fn src_mut( &mut self ) -> &mut &'a str
    {
      &mut self.src
    }
    #[ inline ]
    fn delimeter_mut( &mut self ) -> &mut &'a str
    {
      &mut self.delimeter
    }
    #[ inline ]
    fn left_mut( &mut self ) -> &mut bool
    {
      &mut self.left
    }

  }

  #[ inline ]
  pub fn former< 'a >() -> OptionsFormer< 'a >
  {
    Options::< 'a >::former()
  }

  pub mod prelude
  {
    pub use super::OptionsAdapter as SplitOptionsAdapter;
    /* xxx : cover by a test */
  }

}

#[ inline ]
fn split< 'a >() -> split::OptionsFormer< 'a >
{
  split::former::< 'a >()
}

//

include!( "./basic_only_test.rs" );
