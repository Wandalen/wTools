/// Internal namespace.
mod private
{
  use crate::own::*;

  /// Command to draw rectangle.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct RectChanger
  {
    /// Id.
    pub( crate ) id : Id,
    /// Draw changer.
    pub( crate ) draw : DrawChanger,
  }

  impl RectChanger
  {

    /// Constructor.
    #[ inline ]
    pub fn _new( draw : DrawChanger ) -> Self
    {
      let id = Id::new::< Self >();
      let change = RectChangeNew::new( id );
      let mut result = Self{ id, draw };
      change.add_to( &mut result );
      result
    }

    /// ChangeInterface region.
    #[ inline ]
    pub fn region( mut self, left_top : X2< f32 >, right_bottom : X2< f32 > ) -> Self
    {
      let change = RectChangeRegion::new( self.id() ).region( left_top, right_bottom );
      self.change_add( change );
      self
    }

    /// Get back to draw.
    #[ inline ]
    pub fn draw( self ) -> DrawChanger
    {
      self.draw
    }

    /// Get back to context.
    #[ inline ]
    pub fn context( self ) -> ContextChanger
    {
      self.draw.context_changer
    }

  }

  impl ChangerInterface for RectChanger
  {

    type Parent = DrawChanger;
    type Root = ContextChanger;

    fn context( self ) -> Self::Root
    {
      self.draw.context_changer
    }

    fn parent( &mut self ) -> &mut Self::Parent
    {
      &mut self.draw
    }

    fn end( self ) -> Self::Parent
    {
      self.draw
    }

  }

  impl HasIdInterface for RectChanger
  {
    #[ inline ]
    fn id( &self ) -> Id
    {
      self.draw.id()
    }
  }

}

crate::mod_interface!
{
  exposed use RectChanger;
}