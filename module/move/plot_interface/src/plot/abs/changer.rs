/// Internal namespace.
mod private
{
  use crate::own::*;

  /// Context.
  pub trait ChangerInterface
  where
    Self :
      fmt::Debug +
      Clone +
    ,
  {
    /// Type of root changer.
    type Root : ChangerInterface;
    /// Type of parent changer.
    type Parent : ChangerInterface;

    /// Get root.
    #[ inline ]
    fn root( &mut self ) -> &mut Self::Root
    {
      // Safaty : that's safe becuase root type is the same for all nodes.
      unsafe
      {
        core::mem::transmute::< _, _ >( self.parent().root() )
      }
    }

    /// Get back to root changer.
    fn context( self ) -> Self::Root;

    /// Get parent.
    fn parent( &mut self ) -> &mut Self::Parent;

    /// Get back to parent changer.
    fn end( self ) -> Self::Parent;

    /// Add change.
    #[ inline ]
    fn change_add< Change >( &mut self, change : Change ) -> &mut Self
    where
      Change : ChangeInterface + 'static,
    {
      self.root().change_add( change );
      self
    }

  }

}

crate::mod_interface!
{

  prelude use ChangerInterface;

}
