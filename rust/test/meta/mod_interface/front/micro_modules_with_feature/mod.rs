
use super::*;

/// Private namespace of the module.
mod private
{
}

TheModule::mod_interface!
{
  #![ debug ]
  /// mod_protected
  #[ cfg( feature = "use_std" ) ]
  protected mod mod_protected;
}

// #[doc = " mod_protected"]
// #[cfg(feature = "use_std")]
// pub mod mod_protected ;
//
// #[doc(inline)]
// pub use protected :: * ;
//
// #[doc = r" Protected namespace of the module."]
// pub mod protected
// {
//     #[doc(inline)]
//     pub use super :: orphan :: * ;
//     #[doc(inline)]
//     pub use super::mod_protected ;
// }
// #[doc = r" Orphan namespace of the module."] pub mod orphan
// { #[doc(inline)] pub use super :: exposed :: * ; }
// #[doc = r" Exposed namespace of the module."] pub mod exposed
// { #[doc(inline)] pub use super :: prelude :: * ; }
// #[doc = r" Prelude to use essentials: `use my_module::prelude::*`."] pub mod
// prelude {}

//

include!( "../../only_test/micro_modules_with_feature_only_test.rs" );
