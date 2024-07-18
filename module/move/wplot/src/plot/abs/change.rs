/// Internal namespace.
pub( crate ) mod private
{
  // use crate::own::*;
  use core::fmt;

  use crate::abs::changer::private::ChangerInterface;

  /// Context.
  // #[ clone_dyn ]
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

::meta_tools::mod_interface!
{

  prelude use ChangeInterface;

}
