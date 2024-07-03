
#[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
pub( crate ) mod private
{

  ///
  /// Compile-time assertion that two types have the same size.
  ///


  #[ macro_export ]
  macro_rules! cta_type_same_size
  {
    ( $Type1:ty, $Type2:ty $(,)? ) =>
    {{
      const _ : fn() = ||
      {
        let _ : [ () ; core::mem::size_of::< $Type1 >() ] = [ () ; core::mem::size_of::< $Type2 >() ];
      };
      // let _ = core::mem::transmute::< $Type1, $Type2 >;
      true
    }}
  }

  ///
  /// Compile-time assertion of having the same align.
  ///


  #[ macro_export ]
  macro_rules! cta_type_same_align
  {
    ( $Type1:ty, $Type2:ty $(,)? ) =>
    {{
      const _ : fn() = ||
      {
        let _ : [ () ; core::mem::align_of::< $Type1 >() ] = [ () ; core::mem::align_of::< $Type2 >() ];
      };
      true
    }};
  }

  ///
  /// Compile-time assertion that memory behind two references have the same size.
  ///


  #[ macro_export ]
  macro_rules! cta_ptr_same_size
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


  #[ macro_export ]
  macro_rules! cta_mem_same_size
  {
    ( $Ins1:expr, $Ins2:expr $(,)? ) =>
    {{
      $crate::cta_ptr_same_size!( &$Ins1, &$Ins2 )
    }}
  }

  pub use cta_type_same_size;
  pub use cta_type_same_align;

  pub use cta_ptr_same_size;
  pub use cta_mem_same_size;
}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::
  {
    cta_type_same_size,
    cta_type_same_align,
    cta_ptr_same_size,
    cta_mem_same_size,
  };
}
