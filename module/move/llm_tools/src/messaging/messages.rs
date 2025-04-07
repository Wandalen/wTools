//! Messaging.

mod private
{
  use crate::*;
  use messaging::{ Message };

  use std::
  {
    ops::{ Deref, DerefMut },
  };

  /// A thin wrapper around a vector of messages to store a conversation log.
  #[ repr( transparent ) ]
  #[ derive( Debug ) ]
  pub struct Messages( pub Vec< Message > );

  impl Messages
  {
    /// Creates a new, empty list of messages.
    pub fn new() -> Self
    {
      Self( Vec::new() )
    }
  }

  impl Deref for Messages
  {
    type Target = Vec< Message >;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl DerefMut for Messages
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }


}

crate::mod_interface!
{
  orphan use private::
  {
    Messages,
  };
}
