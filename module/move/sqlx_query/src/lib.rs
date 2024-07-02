#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/sqlx_query/latest/sqlx_query/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]
// #![ allow( unused_macros ) ]
// #![ allow( unused_imports ) ]

// #![ feature( type_name_of_val ) ]

//!
//! Expands to either sqlx function `query` or macro `query!` call
//! depending on `sqlx_compiletime_checks` has been enabled during the build.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/../../../", "Readme.md" ) ) ]

/// Internal namespace.
#[ cfg( feature = "enabled" ) ]
pub( crate ) mod private
{

  #[ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/../../../", "Readme.md" ) ) ]
  #[ macro_export ]
  macro_rules! query
  {
    (
      $sql : literal
    ) =>
    {
      {
        #[ cfg( feature = "sqlx_compiletime_checks" ) ]
        let q = ::sqlx::query( $sql );
        #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
        let q = ::sqlx::query!( $sql );
        q
      }
    };
    (
      $sql : literal, $( $binds : expr ),+
    ) =>
    {
      {
        #[ cfg( feature = "sqlx_compiletime_checks" ) ]
        let q = ::sqlx::query($sql)$(.bind($binds))+;
        #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
        let q = ::sqlx::query!( $sql, $( $binds )+ );
        q
      }
    };
  }

  ///
  ///
  ///
  #[ macro_export ]
  macro_rules! query_as
  {
    (
      $as : ident, $sql : literal
    ) =>
    {
      {
        #[ cfg( feature = "sqlx_compiletime_checks" ) ]
        let q = ::sqlx::query_as::< _, $as >( $sql );
        #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
        let q = ::sqlx::query_as!( $as, $sql );
        q
      }
    };
    (
      $as : ident, $sql : literal, $( $binds : expr ),+
    ) =>
    {
      {
        #[ cfg( feature = "sqlx_compiletime_checks" ) ]
        let q = ::sqlx::query_as::< _, $as >( $sql )$( .bind( $binds ) )+;
        #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
        let q = ::sqlx::query_as!( $as, $sql, $( $binds )+ );
        q
      }
    };
  }

  #[ allow( unused_imports ) ]
  pub use query;

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
  #[ allow( unused_imports ) ]
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::query;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::query_as;
}