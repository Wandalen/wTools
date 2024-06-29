#![ no_std ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn_types/latest/clone_dyn_types/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
}

/// Internal namespace.
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "enabled" ) ]
pub( crate ) mod private
{

  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  extern crate alloc;
  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  // #[ allow( unused_imports ) ]
  use alloc::boxed::Box;
  // #[ cfg( all( feature = "use_std", not( feature = "use_alloc" ) ) ) ]
  // use std::boxed::Box;

  /// A trait to upcast a clonable entity and clone it.
  /// It's implemented for all entities which can be cloned.
  pub trait CloneDyn : Sealed
  {
    #[ doc( hidden ) ]
    fn __clone_dyn( &self, _ : DontCallMe ) -> *mut ();
  }

  // clonable
  impl< T > CloneDyn for T
  where
    T : Clone,
  {
    #[ inline ]
    fn __clone_dyn( &self, _ : DontCallMe ) -> *mut ()
    {
      Box::< T >::into_raw( Box::new( self.clone() ) ) as *mut ()
    }
  }

  // slice
  impl< T > CloneDyn for [ T ]
  where
    T : Clone,
  {
    #[ inline ]
    fn __clone_dyn( &self, _ : DontCallMe ) -> *mut ()
    {
      Box::< [ T ] >::into_raw( self.iter().cloned().collect() ) as *mut ()
    }
  }

  // str slice
  impl CloneDyn for str
  {
    #[ inline ]
    fn __clone_dyn( &self, _ : DontCallMe ) -> *mut ()
    {
      Box::< str >::into_raw( Box::from( self ) ) as *mut ()
    }
  }

  ///
  /// True clone which is applicable not only to clonable entities, but to trait objects implementing CloneDyn.
  ///
  /// # Example
  ///
  /// ```
  /// use clone_dyn_types::clone;
  ///
  /// #[ derive( Clone ) ]
  /// struct MyStruct
  /// {
  ///   value : i32,
  /// }
  ///
  /// let original = MyStruct { value : 42 };
  /// let cloned = clone( &original );
  ///
  /// assert_eq!( original.value, cloned.value );
  /// ```

  #[ inline ]
  pub fn clone< T >( src : &T ) -> T
  where
    T : CloneDyn,
  {
    // # Safety
    //
    // This function uses an `unsafe` block because it performs low-level memory manipulations. Specifically, it handles
    // raw pointers and converts them to and from `Box< T >`. This is necessary to dynamically clone a trait object, which
    // does not support cloning through the standard `Clone` trait. The safety of this function depends on the guarantee
    // that the `CloneDyn` trait is correctly implemented for the given type `T`, ensuring that `__clone_dyn` returns a
    // valid pointer to a cloned instance of `T`.
    //
    #[ allow( unsafe_code ) ]
    unsafe
    {
      *Box::from_raw( < T as CloneDyn >::__clone_dyn( src, DontCallMe ) as *mut T )
    }
  }

  ///
  /// Clone boxed dyn.
  ///
  /// Clones a dynamically sized trait object into a `Box< T >`.
  ///
  /// # Example
  ///
  /// ```
  /// use clone_dyn_types::{ CloneDyn, clone_into_box };
  ///
  /// #[ derive( Clone ) ]
  /// struct MyStruct
  /// {
  ///   value : i32,
  /// }
  ///
  /// trait MyTrait : CloneDyn
  /// {
  ///   fn val( &self ) -> i32;
  /// }
  ///
  /// impl MyTrait for MyStruct
  /// {
  ///   fn val( &self ) -> i32
  ///   {
  ///     self.value
  ///   }
  /// }
  ///
  /// #[ allow( non_local_definitions ) ]
  /// impl < 'c > Clone
  /// for Box< dyn MyTrait + 'c >
  /// {
  ///   #[ inline ]
  ///   fn clone( &self ) -> Self { clone_into_box( &**self ) }
  /// }
  ///
  /// #[ allow( non_local_definitions ) ]
  /// impl < 'c > Clone
  /// for Box< dyn MyTrait + Send + 'c >
  /// {
  ///   #[ inline ]
  ///   fn clone( &self ) -> Self { clone_into_box( &**self ) }
  /// }
  ///
  /// #[ allow( non_local_definitions ) ]
  /// impl < 'c > Clone
  /// for Box< dyn MyTrait + Sync + 'c >
  /// {
  ///   #[ inline ]
  ///   fn clone( &self ) -> Self { clone_into_box( &**self ) }
  /// }
  ///
  /// #[ allow( non_local_definitions ) ]
  /// impl < 'c > Clone
  /// for Box< dyn MyTrait + Send + Sync + 'c >
  /// {
  ///   #[ inline ]
  ///   fn clone( &self ) -> Self { clone_into_box( &**self ) }
  /// }
  ///
  /// let cloned : Box< dyn MyTrait > = clone_into_box( &MyStruct { value : 42 } );
  ///
  /// ```

  #[ inline ]
  pub fn clone_into_box< T >( ref_dyn : &T ) -> Box< T >
  where
    T : ?Sized + CloneDyn,
  {
    // # Safety
    //
    // This function uses an `unsafe` block because it performs low-level memory manipulations involving raw pointers.
    // The `unsafe` block is necessary here because we're manually handling raw pointers and converting them to and from
    // `Box<T>`. This bypasses Rust's ownership and borrowing rules to achieve dynamic cloning of a boxed trait object.
    // The safety of this function relies on the correct implementation of the `CloneDyn` trait for the given type `T`.
    // Specifically, `__clone_dyn` must return a valid pointer to a cloned instance of `T`.
    //
    #[ allow( unsafe_code ) ]
    unsafe
    {
      let mut ptr = ref_dyn as *const T;
      let data_ptr = &mut ptr as *mut *const T as *mut *mut ();
      *data_ptr = < T as CloneDyn >::__clone_dyn( ref_dyn, DontCallMe );
      Box::from_raw( ptr as *mut T )
    }
  }

  #[ doc( hidden ) ]
  mod sealed
  {
    #[ doc( hidden ) ]
    #[ allow( missing_debug_implementations ) ]
    pub struct DontCallMe;
    #[ doc( hidden ) ]
    pub trait Sealed {}
    impl< T : Clone > Sealed for T {}
    impl< T : Clone > Sealed for [ T ] {}
    impl Sealed for str {}
  }
  use sealed::*;

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
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
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
  pub use super::private::
  {
    CloneDyn,
    clone_into_box,
    clone,
  };
}
