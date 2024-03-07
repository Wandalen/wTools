#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/derive_tools/latest/derive_tools/" ) ]

//!
//! Collection of derives which extend STD.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
pub mod wtools;

#[ cfg( feature = "derive_reflect" ) ]
pub mod reflect;

// use derive_tools_meta::Deref;
// use derive_tools_meta::VariadicFrom;

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  #[ cfg( feature = "derive_more" ) ]
  pub use ::derive_more;
  #[ cfg( feature = "strum" ) ]
  pub use ::strum;
  #[ cfg( feature = "parse_display" ) ]
  pub use ::parse_display;
  #[ cfg( feature = "clone_dyn" ) ]
  pub use ::clone_dyn;
  #[ cfg( feature = "clone_dyn" ) ]
  pub use ::clone_dyn::dependency::*;
  #[ cfg( any_derive ) ]
  pub use ::derive_tools_meta;
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
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::wtools::orphan::*;
  #[ cfg( feature = "derive_reflect" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::reflect::orphan::*;
}

#[ cfg( all( feature = "derive_more" ) ) ]
#[ allow( unused_imports ) ]
mod derive_more
{
  #[ cfg( feature = "derive_add" ) ]
  pub use ::derive_more::Add;
  #[ cfg( feature = "derive_is_variant" ) ]
  pub use ::derive_more::IsVariant;

  // qqq2 : list all
  // qqq2 : make sure all features of derive_more is reexported
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

  #[ cfg( all( feature = "derive_more" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::derive_more::*;

  // #[ cfg( all( feature = "derive_more", feature = "derive_add" ) ) ]
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::derive_more::Add;

  // #[ allow( ambiguous_glob_reexports ) ]
  // #[ cfg( feature = "derive_more" ) ]
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::derive_more::
  // {
  //   Add,
  //   AddAssign,
  //   AsMut,
  //   AsRef,
  //   Binary,
  //   BitAnd,
  //   BitAndAssign,
  //   BitOr,
  //   BitOrAssign,
  //   BitXor,
  //   BitXorAssign,
  //   Constructor,
  //   Debug,
  //   Deref,
  //   DerefMut,
  //   Display,
  //   Div,
  //   DivAssign,
  //   Error,
  //   From,
  //   FromStr,
  //   Index,
  //   IndexMut,
  //   Into,
  //   IntoIterator,
  //   IsVariant,
  //   LowerExp,
  //   LowerHex,
  //   Mul,
  //   MulAssign,
  //   Neg,
  //   Not,
  //   Octal,
  //   Pointer,
  //   Product,
  //   Rem,
  //   RemAssign,
  //   Shl,
  //   ShlAssign,
  //   Shr,
  //   ShrAssign,
  //   Sub,
  //   SubAssign,
  //   Sum,
  //   TryFrom,
  //   TryInto,
  //   TryUnwrap,
  //   Unwrap,
  //   UpperExp,
  //   UpperHex,
  // };

  #[ cfg( feature = "strum" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::strum::*;

  #[ cfg( feature = "derive_display" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::parse_display::Display;

  #[ cfg( feature = "derive_from_str" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::parse_display::FromStr;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn::exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::wtools::exposed::*;

  #[ cfg( feature = "derive_reflect" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::reflect::exposed::*;

  // #[ cfg( any_derive ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::derive_tools_meta::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "derive_from" ) ]
  pub use ::derive_tools_meta::From;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn::prelude::*;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::clone_dyn::clone_dyn;
  #[ cfg( feature = "derive_reflect" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::reflect::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::wtools::prelude::*;
  #[ doc( no_inline ) ]
  pub use super::wtools;

}
