/// Define a private namespace for all its items.
mod private
{
  use crate::own::*;

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

  /// Draw changer.
  layer changer;
  /// ChangeInterface for drawing constructor.
  layer change_new;
  /// Draw command.
  layer command;
  /// Draw queue.
  layer queue;
  /// New shape : rectangle.
  layer rect_change_new;
  /// Change region of the rectangle.
  layer rect_change_region;
  /// Rectangle change.
  layer rect_changer;

  exposed use Drawing;

}
