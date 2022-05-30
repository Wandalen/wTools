#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Macro to answer the question: does it implement a trait?
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// #[ macro_use ]
mod implements_impl;

/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Macro `implements` to answer the question: does it implement a trait?
  ///
  /// ### Sample
  /// ```
  /// use implements::*;
  ///
  /// dbg!( implements!( 13_i32 => Copy ) );
  /// // < implements!( 13_i32 => Copy ) : true
  /// dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
  /// // < implements!( 13_i32 => Copy ) : false
  /// ```

  #[ macro_export ]
  macro_rules! implements
  {
    ( $( $arg : tt )+ ) =>
    {
      $crate::_implements!( $( $arg )+ );
    }
  }

  ///
  /// Macro `instance_of` to answer the question: does it implement a trait? Alias of the macro `implements`.
  ///
  /// ### Sample
  /// ```
  /// use implements::instance_of;
  ///
  /// dbg!( instance_of!( 13_i32 => Copy ) );
  /// // < instance_of!( 13_i32 => Copy ) : true
  /// dbg!( instance_of!( Box::new( 13_i32 ) => Copy ) );
  /// // < instance_of!( 13_i32 => Copy ) : false
  /// ```

  #[ macro_export ]
  macro_rules! instance_of
  {
    ( $( $arg : tt )+ ) =>
    {
      $crate::_implements!( $( $arg )+ );
    }
  }

  pub use implements;
  pub use instance_of;
}

/// Protected namespace of the module.
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
  #[ doc( inline ) ]
  pub use super::private::
  {
    implements,
    instance_of,
  };
}
