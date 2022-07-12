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

  exposed use private::Drawing;

}

// /// Protected namespace of the module.
// pub mod protected
// {
//   pub use super::
//   {
//     orphan::*,
//     command::orphan::*,
//     queue::orphan::*,
//     rect_change::orphan::*,
//     rect_changer::orphan::*,
//   };
// }
//
// pub use protected::*;
//
// /// Parented namespace of the module.
// pub mod orphan
// {
//   pub use super::exposed::*;
// }
//
// /// Exposed namespace of the module.
// pub mod exposed
// {
//   pub use super::
//   {
//     prelude::*,
//     command::exposed::*,
//     queue::exposed::*,
//     rect_change::exposed::*,
//     rect_changer::exposed::*,
//     private::Drawing,
//   };
// }
//
// pub use exposed::*;
//
// /// Prelude to use essentials: `use my_module::prelude::*`.
// pub mod prelude
// {
//   pub use super::
//   {
//     command::prelude::*,
//     queue::prelude::*,
//     rect_change::prelude::*,
//     rect_changer::prelude::*,
//   };
// }
