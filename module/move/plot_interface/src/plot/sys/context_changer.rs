/// Internal namespace.
pub( crate ) mod private
{
  use crate::own::*;

  /// Context.
  #[ allow( dead_code ) ]
  #[ derive( Clone ) ]
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
      StrokeBrushChanger::_new( self )
    }
    /// Draw.
    #[ inline ]
    pub fn draw( self ) -> DrawChanger
    {
      DrawChanger::_new( self )
    }
  }

  impl fmt::Debug for ContextChanger
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_str( "ContextChanger" )?;
      for ( _i, e ) in self.changes.iter().enumerate()
      {
        f.write_str( &wtools::string::indentation( "  ", format!( "\n{:?}", e ), "" ) )?;
      }
      Ok( () )
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

crate::mod_interface!
{
  exposed use ContextChanger;
}
