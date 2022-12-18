wtools::mod_interface!
{
  /// Init aggregator commands.
  prelude mod init;

  /// Information about package
  prelude mod info;

  /// Publish package
  prelude mod publish;

  /// Iterate over subject
  prelude mod each;
  
  protected use super::init::protected::*;

  prelude use State;
}

mod private
{
  use core::ops::{ Deref, DerefMut };
  use std::collections::HashMap;

  #[ derive( Debug ) ]
  /// Commands context
  pub struct State( HashMap< String, wca::Command > );

  impl State
  {
    /// Construct state object
    pub fn new( val : HashMap< String, wca::Command > ) -> Self
    {
      Self( val )
    }
  }

  impl Deref for State
  {
    type Target = HashMap< String, wca::Command >;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl DerefMut for State
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

  impl Into< wca::Context > for State
  {
    fn into( self ) -> wca::Context
    {
      wca::Context::new( self )
    }
  }
}