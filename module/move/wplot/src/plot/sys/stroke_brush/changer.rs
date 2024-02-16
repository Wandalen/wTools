/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;

  /// ChangerInterface of brush stroke.
  #[ allow( dead_code ) ]
  #[ derive( Debug ) ]
  pub struct StrokeBrushChanger
  {
    pub( crate ) id : Id,
    pub( crate ) context_changer : ContextChanger,
  }

  impl StrokeBrushChanger
  {

    /// Constructor.
    #[ inline ]
    pub( crate ) fn _new( mut context_changer : ContextChanger ) -> Self
    {
      let id = &mut context_changer.stroke;
      if id.is_none()
      {
        *id = Some( Id::new::< StrokeBrush >() );
        StrokeBrushChangeNew::new( context_changer.stroke.unwrap() ).add_to( &mut context_changer );
      }
      let id = context_changer.stroke.unwrap();
      Self
      {
        id,
        context_changer,
      }
    }

    // /// Get back to context.
    // #[ inline ]
    // pub fn context( self ) -> ContextChanger
    // {
    //   self.context_changer
    // }

    /// ChangeInterface color.
    #[ inline ]
    pub fn color< Color >( mut self, color : Color ) -> Self
    where
      Color : RgbaInterface< f32 >,
    {
      let id = self.id;
      let change = StrokeBrushChangeColor::new( id, color.into_rgba() );
      self.change_add( change );
      self
    }

    /// Width.
    #[ inline ]
    pub fn width( mut self, val : f32 ) -> Self
    {
      let id = self.id;
      let change = StrokeBrushChangeWidth::new( id, val );
      self.change_add( change );
      self
    }

  }

  impl ChangerInterface for StrokeBrushChanger
  {

    type Parent = ContextChanger;
    type Root = ContextChanger;

    fn context( self ) -> Self::Root
    {
      self.context_changer
    }

    fn parent( &mut self ) -> &mut Self::Parent
    {
      &mut self.context_changer
    }

    fn end( self ) -> Self::Parent
    {
      self.context_changer
    }

  }

  impl HasIdInterface for StrokeBrushChanger
  {
    #[ inline ]
    fn id( &self ) -> Id
    {
      self.id
    }
  }

}

::meta_tools::mod_interface!
{
  exposed use StrokeBrushChanger;
}
