/// Internal namespace.
mod private
{
  use crate::own::*;

  /// ChangerInterface of brush stroke.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct DrawChanger
  {
    pub( crate ) id : Id,
    pub( crate ) context_changer : ContextChanger,
  }

  impl DrawChanger
  {
    /// Constructor.
    #[ inline ]
    pub( crate ) fn _new( mut context_changer : ContextChanger ) -> Self
    {
      let id = &mut context_changer.drawing;
      if id.is_none()
      {
        *id = Some( Id::new::< Self >() );
        DrawingChangeNew::new( id.unwrap() ).add_to( &mut context_changer );
      }
      let id = context_changer.drawing.unwrap();
      Self
      {
        id,
        context_changer,
      }
    }
    /// ChangeInterface color.
    #[ inline ]
    pub fn rect( self ) -> RectChanger
    {
      RectChanger::_new( self )
    }
  }

  impl ChangerInterface for DrawChanger
  {
    type Parent = ContextChanger;
    type Root = ContextChanger;

    #[ inline ]
    fn context( self ) -> Self::Root
    {
      self.context_changer
    }

    #[ inline ]
    fn parent( &mut self ) -> &mut Self::Parent
    {
      &mut self.context_changer
    }

    #[ inline ]
    fn end( self ) -> Self::Parent
    {
      self.context_changer
    }

  }

  impl HasIdInterface for DrawChanger
  {
    #[ inline ]
    fn id( &self ) -> Id
    {
      self.context_changer.id()
    }
  }

}

crate::mod_interface!
{
  exposed use DrawChanger;
}
