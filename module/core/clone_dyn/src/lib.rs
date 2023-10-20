#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn/latest/clone_dyn/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

// #![ feature( trait_alias ) ]
// #![ feature( type_name_of_val ) ]

//!
//! Derive to clone dyn structures.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
extern crate alloc;

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::clone_dyn_meta;
}

/// Internal namespace.
// #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "enabled" ) ]
pub( crate ) mod private
{
  #[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
  use alloc::boxed::Box;
  #[ cfg( all( feature = "use_std", not( feature = "use_alloc" ) ) ) ]
  use std::boxed::Box;

  /// Clone boxed dyn.
  ///
  /// Not intended to be used directly.
  #[ inline ]
  pub fn _clone_boxed< T >( t : &T ) -> Box< T >
  where
    T : ?Sized,
  {
    unsafe
    {
      let mut ptr = t as *const T;
      let data_ptr = &mut ptr as *mut *const T as *mut *mut ();
      *data_ptr = Box::into_raw( Box::new( < &T >::clone( &t ) ) ) as *mut ();
      Box::from_raw( ptr as *mut T )
    }
  }

}

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  pub use ::clone_dyn_meta::clone_dyn;
  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ doc( inline ) ]
  pub use super::private::_clone_boxed;
}
