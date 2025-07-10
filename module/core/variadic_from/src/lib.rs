#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/variadic_from/latest/variadic_from/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Internal implementation of variadic `From` traits and macro.
#[ cfg( feature = "enabled" ) ]
pub mod variadic
{
  /// Trait for converting from one argument.
  pub trait From1< T1 >
  where
    Self : Sized,
  {
    /// Converts from one argument.
    fn from1( a1 : T1 ) -> Self;
  }

  /// Trait for converting from two arguments.
  pub trait From2< T1, T2 >
  where
    Self : Sized,
  {
    /// Converts from two arguments.
    fn from2( a1 : T1, a2 : T2 ) -> Self;
  }

  /// Trait for converting from three arguments.
  pub trait From3< T1, T2, T3 >
  where
    Self : Sized,
  {
    /// Converts from three arguments.
    fn from3( a1 : T1, a2 : T2, a3 : T3 ) -> Self;
  }

  /// Macro to construct a struct from variadic arguments.
  #[ macro_export ]
  macro_rules! from
  {
    () =>
    {
      core::default::Default::default()
    };
    ( $a1 : expr ) =>
    {
      $crate::variadic::From1::from1( $a1 )
    };
    ( $a1 : expr, $a2 : expr ) =>
    {
      $crate::variadic::From2::from2( $a1, $a2 )
    };
    ( $a1 : expr, $a2 : expr, $a3 : expr ) =>
    {
      $crate::variadic::From3::from3( $a1, $a2, $a3 )
    };
    ( $( $rest : expr ),* ) =>
    {
      compile_error!( "Too many arguments" );
    };
  }
}

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::variadic_from_meta;
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use ::variadic_from_meta::*;

  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  pub use crate::variadic::From1;
  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  pub use crate::variadic::From2;
  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  pub use crate::variadic::From3;

  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  pub use crate::from;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( no_inline ) ]
  pub use ::variadic_from_meta::VariadicFrom;

  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  pub use crate::variadic::From1;
  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  pub use crate::variadic::From2;
  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  pub use crate::variadic::From3;

  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  pub use crate::from;
}
