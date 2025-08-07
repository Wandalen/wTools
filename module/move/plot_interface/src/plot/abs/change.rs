/// Define a private namespace for all its items.
mod private
{
  use crate::own::*;

  /// Context.
  #[ clone_dyn ]
  pub trait ChangeInterface
  where
    Self :
      fmt::Debug +
    ,
  {

    /// Add change to queue of events.
    fn add_to< C : ChangerInterface >( self, changer : &mut C ) -> &mut C
    where
      Self : Sized + 'static,
    {
      changer.change_add( self )
    }

  }

  //

}

crate::mod_interface!
{

  prelude use ChangeInterface;

}
