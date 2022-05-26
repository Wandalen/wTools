
mod internal
{

  ///
  /// Compile-time assertion that memory behind two references have the same size.
  ///

  #[ cfg( feature = "ct" ) ]
  #[macro_export]
  macro_rules! ct_ptr_same_size
  {
    ( $Ins1:expr, $Ins2:expr $(,)? ) =>
    {{
      #[ allow( unsafe_code, unknown_lints, forget_copy, useless_transmute ) ]
      let _ = || unsafe
      {
        let mut ins1 = core::ptr::read( $Ins1 );
        core::ptr::write( &mut ins1, core::mem::transmute( core::ptr::read( $Ins2 ) ) );
        core::mem::forget( ins1 );
      };
      true
    }}
  }

  ///
  /// Compile-time assertion that two values have the same size.
  ///
  /// Does not consume values.
  ///

  #[ cfg( feature = "ct" ) ]
  #[macro_export]
  macro_rules! ct_mem_same_size
  {
    ( $Ins1:expr, $Ins2:expr $(,)? ) =>
    {{
      $crate::ct_ptr_same_size!( &$Ins1, &$Ins2 );
    }}
  }

  pub use ct_ptr_same_size;
  pub use ct_mem_same_size;
}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {
    ct_ptr_same_size,
    ct_mem_same_size,
  };
}
