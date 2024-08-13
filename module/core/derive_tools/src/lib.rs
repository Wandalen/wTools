#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/derive_tools/latest/derive_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// // xxx : implement derive new
//
// #[ derive( Debug, PartialEq, Default ) ]
// pub struct Property< Name >
// {
//   name : Name,
//   description : String,
//   code : isize,
// }
//
// /// generated by new
// impl< Name > Property< Name >
// {
//   #[ inline ]
//   pub fn new< Description, Code >( name : Name, description : Description, code : Code ) -> Self
//   where
//     Name : core::convert::Into< Name >,
//     Description : core::convert::Into< String >,
//     Code : core::convert::Into< isize >,
//   {
//     Self { name : name.into(), description : description.into(), code : code.into() }
//   }
// }

// #[ cfg( feature = "enabled" ) ]
// pub mod wtools;

#[ cfg( all( feature = "derive_more" ) ) ]
#[ allow( unused_imports ) ]
mod derive_more
{
  #[ cfg( feature = "derive_add" ) ]
  pub use ::derive_more::{ Add, Sub };
  #[ cfg( feature = "derive_add_assign" ) ]
  pub use ::derive_more::{ AddAssign, SubAssign };
  #[ cfg( feature = "derive_constructor" ) ]
  pub use ::derive_more::Constructor;
  #[ cfg( feature = "derive_error" ) ]
  pub use ::derive_more::Error;
  #[ cfg( feature = "derive_into" ) ]
  pub use ::derive_more::Into;
  // #[ cfg( feature = "derive_iterator" ) ]
  // pub use ::derive_more::Iterator;
  #[ cfg( feature = "derive_into_iterator" ) ]
  pub use ::derive_more::IntoIterator;
  #[ cfg( feature = "derive_mul" ) ]
  pub use ::derive_more::{ Mul, Div };
  #[ cfg( feature = "derive_mul_assign" ) ]
  pub use ::derive_more::{ MulAssign, DivAssign };
  #[ cfg( feature = "derive_sum" ) ]
  pub use ::derive_more::Sum;
  #[ cfg( feature = "derive_try_into" ) ]
  pub use ::derive_more::TryInto;
  #[ cfg( feature = "derive_is_variant" ) ]
  pub use ::derive_more::IsVariant;
  #[ cfg( feature = "derive_unwrap" ) ]
  pub use ::derive_more::Unwrap;

  // qqq : list all
  // qqq : make sure all features of derive_more is reexported
}

#[ doc( inline ) ]
#[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
pub use variadic_from as variadic;

/// Namespace with dependencies.

#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  #[ doc( inline ) ]
  pub use ::derive_tools_meta;

  #[ doc( inline ) ]
  #[ cfg( feature = "derive_clone_dyn" ) ]
  pub use ::clone_dyn::{ self, dependency::* };

  #[ doc( inline ) ]
  #[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
  pub use ::variadic_from::{ self, dependency::* };

  #[ doc( inline ) ]
  #[ cfg( feature = "derive_more" ) ]
  pub use ::derive_more;
  #[ doc( inline ) ]
  #[ cfg( feature = "derive_strum" ) ]
  pub use ::strum;
  #[ doc( inline ) ]
  #[ cfg( feature = "parse_display" ) ]
  pub use ::parse_display;

}

#[ doc( inline ) ]
#[ cfg( feature = "enabled" ) ]
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
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn::orphan::*;
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

  #[ cfg( all( feature = "derive_more" ) ) ]
  #[ doc( inline ) ]
  pub use super::derive_more::*;

  #[ cfg( feature = "derive_strum" ) ]
  #[ doc( inline ) ]
  pub use ::strum::*;

  #[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
  #[ doc( inline ) ]
  pub use ::variadic_from::exposed::*;

  #[ cfg( feature = "derive_strum" ) ]
  #[ doc( inline ) ]
  pub use ::strum::*;

  #[ cfg( feature = "derive_display" ) ]
  #[ doc( inline ) ]
  pub use ::parse_display::Display;

  #[ cfg( feature = "derive_from_str" ) ]
  #[ doc( inline ) ]
  pub use ::parse_display::FromStr;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn::exposed::*;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn;

  #[ doc( inline ) ]
  pub use ::derive_tools_meta::*;

  #[ doc( inline ) ]
  #[ cfg( feature = "derive_from" ) ]
  pub use ::derive_tools_meta::From;

  #[ doc( inline ) ]
  #[ cfg( feature = "derive_inner_from" ) ]
  pub use ::derive_tools_meta::InnerFrom;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn::prelude::*;

  #[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
  #[ doc( inline ) ]
  pub use ::variadic_from::prelude::*;

}
