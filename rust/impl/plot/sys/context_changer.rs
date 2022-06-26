/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Context.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct ContextChanger
  {
    /// Id.
    pub( crate ) id : Id,
    /// Stroke brush.
    pub( crate ) stroke : Option< Id >,
    /// Drawing.
    pub( crate ) drawing : Option< Id >,
    /// Queue of changes.
    pub changes : Vec< Box< dyn ChangeInterface > >,
  }

  impl ContextChanger
  {
    /// Parameters of stroke.
    #[ inline ]
    pub fn stroke( self ) -> StrokeBrushChanger
    {
      let changer = StrokeBrushChanger::_new( self );
      changer
    }
    /// Draw.
    #[ inline ]
    pub fn draw( self ) -> DrawChanger
    {
      let changer = DrawChanger::_new( self );
      changer
    }
  }

  impl ChangerInterface for ContextChanger
  {
    type Parent = ContextChanger;
    type Root = ContextChanger;

    #[ inline ]
    fn root( &mut self ) -> &mut Self::Root
    {
      self
    }

    #[ inline ]
    fn context( self ) -> Self::Root
    {
      self
    }

    #[ inline ]
    fn parent( &mut self ) -> &mut Self::Parent
    {
      self
    }

    #[ inline ]
    fn end( self ) -> Self::Parent
    {
      self
    }

    #[ inline ]
    fn change_add< Change >( &mut self, change : Change ) -> &mut Self
    where
      Change : ChangeInterface + 'static,
    {
      self.changes.push( Box::new( change ) );
      self
    }

  }

  impl HasIdInterface for ContextChanger
  {
    #[ inline ]
    fn id( &self ) -> Id
    {
      self.id
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
  };
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
    private::ContextChanger,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}
