/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Drawing.
  #[ derive( Debug, Clone ) ]
  pub struct Drawing
  {
    pub( crate ) id : Id,
  }

  impl Drawing
  {
    /// Constructor.
    pub fn new() -> Self
    {
      let id = Id::new::< Self >();
      Self
      {
        id,
      }
    }
  }

  impl HasIdInterface for Drawing
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

  /// Draw command.
  layer command;
  /// Draw queue.
  layer queue;
  /// Rectangle change.
  layer rect_change;
  /// Rectangle change.
  layer rect_changer;

  exposed use Drawing;

}
