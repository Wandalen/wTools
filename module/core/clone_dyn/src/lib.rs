#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn/latest/clone_dyn/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ allow( unused_extern_crates ) ]
#[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
extern crate alloc;

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::clone_dyn_meta;
}

/// Internal namespace.
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "enabled" ) ]
pub( crate ) mod private
{

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  extern crate alloc;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ allow( unused_imports ) ]
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
    // Explanation for the use of `unsafe`:
    // The `unsafe` block is necessary here because we're performing low-level memory manipulations
    // that cannot be checked by the Rust compiler for safety. Specifically, we're manually handling
    // raw pointers and converting them to and from `Box< T >`, which is considered unsafe as it
    // bypasses Rust's ownership and borrowing rules. This is done to dynamically clone a boxed
    // trait object, which doesn't support cloning through the standard `Clone` trait. The operations
    // within this block are carefully crafted to ensure memory safety manually, including proper
    // allocation and deallocation of heap memory for the clone.
    #[ allow( unsafe_code ) ]
    unsafe
    {
      let mut ptr = t as *const T;
      let data_ptr = &mut ptr as *mut *const T as *mut *mut ();
      *data_ptr = Box::into_raw( Box::new( < &T >::clone( &t ) ) ) as *mut ();
      Box::from_raw( ptr as *mut T )
    }
  }

}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  pub use ::clone_dyn_meta::clone_dyn;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  pub use super::private::_clone_boxed;
}
