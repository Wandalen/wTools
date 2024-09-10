
use super::*;

/// Private namespace of the module.
mod private
{
}

// #[ doc = " mod_own" ]
// pub mod mod_own;
// #[ doc = " mod_orphan" ]
// pub mod mod_orphan;
// #[ doc = " mod_exposed" ]
// pub mod mod_exposed;
// #[ doc = " mod_prelude" ]
// pub mod mod_prelude;
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
//   #[ doc = " mod_own" ]
//   pub use mod_own;
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
//   #[ doc( inline ) ]
//   #[ allow( unused_imports ) ]
//   #[ doc = " mod_orphan" ]
//   pub use mod_orphan;
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
//   #[ doc = " mod_exposed" ]
//   pub use mod_exposed;
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
//   #[ doc( inline ) ]
//   #[ allow( unused_imports ) ]
//   #[ doc = " mod_prelude" ]
//   pub use mod_prelude;
// }

mod_interface!
{
  #![ debug ]

  /// mod_own
  own mod mod_own;
  /// mod_orphan
  orphan mod mod_orphan;
  /// mod_exposed
  exposed mod mod_exposed;
  /// mod_prelude
  prelude mod mod_prelude;

}

//

include!( "../../only_test/micro_modules_only_test.rs" );
