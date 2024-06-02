#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn/latest/clone_dyn/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ allow( unused_extern_crates ) ]
#[ cfg( feature = "enabled" ) ]
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

  /// A trait to upcast a clonable entity and clone it.
  /// It's implemented for all entities which can be cloned.
  pub trait CloneDyn
  {
    fn __clone_dyn( &self ) -> *mut ();
  }

  // clonable
  impl< T > CloneDyn for T
  where
    T : Clone,
  {
    fn __clone_dyn( &self ) -> *mut ()
    {
      Box::< T >::into_raw( Box::new( self.clone() ) ) as *mut ()
    }
  }

  // slice
  impl< T > CloneDyn for [ T ]
  where
    T : Clone,
  {
    fn __clone_dyn( &self ) -> *mut ()
    {
      Box::< [ T ] >::into_raw( self.iter().cloned().collect() ) as *mut ()
    }
  }

  // str slice
  impl CloneDyn for str
  {
    fn __clone_dyn( &self ) -> *mut ()
    {
      Box::< str >::into_raw( Box::from( self ) ) as *mut ()
    }
  }

  ///
  /// True clone which is applicable not only to clonable entities, but to trait object implementing CloneDyn.
  ///
  pub fn clone< T >( src : &T ) -> T
  where
    T : CloneDyn,
  {
    unsafe
    {
      *Box::from_raw( < T as CloneDyn >::__clone_dyn( src ) as *mut T )
    }
  }

  /// Clone boxed dyn.
  ///
  /// Not intended to be used directly.
  #[ inline ]
  pub fn clone_into_box< T >( ref_dyn : &T ) -> Box< T >
  where
    T : ?Sized + CloneDyn,
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
      let mut ptr = ref_dyn as *const T;
      // println!( "ptr : {:p} | size : {}", ptr, core::mem::size_of_val( &ptr ) );
      // inspect_type::inspect_type_of!( ptr );
      let data_ptr = &mut ptr as *mut *const T as *mut *mut ();
      // println!( "data_ptr : {:p} | size : {}", data_ptr, core::mem::size_of_val( &data_ptr ) );
      // inspect_type::inspect_type_of!( data_ptr );
      // println!( "*data_ptr : {:p} | size : {}", *data_ptr, core::mem::size_of_val( &*data_ptr ) );
      // inspect_type::inspect_type_of!( data_ptr );
      *data_ptr = < T as CloneDyn >::__clone_dyn( ref_dyn );
      // println!( "" );
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
  pub use super::private::
  {
    CloneDyn,
    clone_into_box,
    clone,
  };
}
