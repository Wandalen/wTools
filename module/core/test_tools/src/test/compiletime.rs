
//!
//! Try building a program for negative testing.
//!

/// Internal namespace.
mod private
{
  #[ doc( inline ) ]
  pub use ::trybuild::*;
}

// //
//
// #[ doc( inline ) ]
// #[ allow( unused_imports ) ]
// pub use own::*;
//
// #[ doc = r" Own namespace of the module." ]
// #[ allow( unused_imports ) ]
// pub mod own
// {
//   use super::private;
//   mod __all__
//   {
//     pub use super::super::*;
//     pub use super::super::private::*;
//   }
//   #[ doc( inline ) ]
//   pub use super::orphan::*;
//   #[ doc( inline ) ]
//   #[ allow( unused_imports ) ]
//   pub use private::{*};
// }
//
// #[ doc = r" Orphan namespace of the module." ]
// #[ allow( unused_imports ) ]
// pub mod orphan
// {
//   mod __all__
//   {
//     pub use super::super::*;
//     pub use super::super::private::*;
//   }
//   #[ doc( inline ) ]
//   pub use super::exposed::*;
// }
//
// #[ doc = r" Exposed namespace of the module." ]
// #[ allow( unused_imports ) ]
// pub mod exposed
// {
//   mod __all__
//   {
//     pub use super::super::*;
//     pub use super::super::private::*;
//   }
//   #[ doc( inline ) ]
//   pub use super::prelude::*;
//   #[ doc( inline ) ]
//   #[ allow( unused_imports ) ]
//   pub use super::super::compiletime;
// }
//
// #[ doc = r" Prelude to use essentials: `use my_module::prelude::*`." ]
// #[ allow( unused_imports ) ]
// pub mod prelude
// {
//   mod __all__
//   {
//     pub use super::super::*;
//     pub use super::super::private::*;
//   }
// }

crate::mod_interface!
{
  // #![ debug ]
  // xxx : make it working
  // exposed use super;
  exposed use super::super::compiletime;
  own use
  {
    *
  };
}
