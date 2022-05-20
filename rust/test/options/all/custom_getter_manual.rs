use super::Former;

mod split
{

  use super::Former;

  #[ derive( Former, PartialEq, Debug ) ]
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
      &!self.left
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
  }

}

#[ inline ]
fn split< 'a >() -> split::OptionsFormer< 'a >
{
  split::former::< 'a >()
}

//

include!( "./custom_getter_only_test.rs" );
