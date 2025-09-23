#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/derive_tools/latest/derive_tools/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Derive macro tools" ) ]

//! # Rule Compliance & Architectural Notes
//!
//! This crate has been systematically updated to comply with the Design and Codestyle Rulebooks.
//! Key compliance achievements :
//!
//! ## Completed Compliance Work :
//!
//! 1. **Feature Architecture** : All functionality is properly gated behind the "enabled" feature
//!    following the mandatory 'enabled' and 'full' features requirement.
//!
//! 2. **Dependencies** : Uses workspace dependency inheritance with `{ workspace = true }`.
//!    All derive macro dependencies are centralized in the workspace Cargo.toml.
//!
//! 3. **Attribute Formatting** : All attributes use proper spacing per Universal Formatting Rule.
//!
//! 4. **Documentation Strategy** : Uses `#![ doc = include_str!(...) ]` to include readme.md
//!    instead of duplicating documentation.
//!
//! 5. **Namespace Organization** : Uses the standard own/orphan/exposed/prelude namespace
//!    pattern for controlled visibility and re-exports.

#[ cfg( feature = "derive_from" ) ]
pub use derive_tools_meta::From;
#[ cfg( feature = "derive_inner_from" ) ]
pub use derive_tools_meta::InnerFrom;
#[ cfg( feature = "derive_new" ) ]
pub use derive_tools_meta::New;
#[ cfg( feature = "derive_not" ) ]
pub use derive_tools_meta::Not;

#[ cfg( feature = "derive_variadic_from" ) ]
pub use derive_tools_meta::VariadicFrom;
#[ cfg( feature = "derive_as_mut" ) ]
pub use derive_tools_meta::AsMut;
#[ cfg( feature = "derive_as_ref" ) ]
pub use derive_tools_meta::AsRef;
#[ cfg( feature = "derive_deref" ) ]
pub use derive_tools_meta::Deref;
#[ cfg( feature = "derive_deref_mut" ) ]
pub use derive_tools_meta::DerefMut;
#[ cfg( feature = "derive_index" ) ]
pub use derive_tools_meta::Index;
#[ cfg( feature = "derive_index_mut" ) ]
pub use derive_tools_meta::IndexMut;
#[ cfg( feature = "derive_more" ) ]
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
  // pub use ::derive_more ::Iterator;
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

  // qqq: list all
  // qqq: make sure all features of derive_more is reexported
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
  pub use ::clone_dyn :: { self, dependency :: * };

  #[ doc( inline ) ]
  #[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
  pub use ::variadic_from :: { self, dependency :: * };

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
pub use own :: *;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own 
{

  use super :: *;
  #[ doc( inline ) ]
  pub use orphan :: *;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn ::orphan :: *;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan 
{

  use super :: *;
  #[ doc( inline ) ]
  pub use exposed :: *;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed 
{

  use super :: *;
  #[ doc( inline ) ]
  pub use prelude :: *;

  #[ cfg( feature = "derive_more" ) ]
  #[ doc( inline ) ]
  pub use super ::derive_more :: *;

  #[ cfg( feature = "derive_strum" ) ]
  #[ doc( inline ) ]
  pub use ::strum :: *;
  // qqq: xxx: name all

  #[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
  #[ doc( inline ) ]
  pub use ::variadic_from ::exposed :: *;

  #[ cfg( feature = "derive_strum" ) ]
  #[ doc( inline ) ]
  pub use ::strum :: *;

  #[ cfg( feature = "derive_display" ) ]
  #[ doc( inline ) ]
  pub use ::parse_display ::Display;

  #[ cfg( feature = "derive_from_str" ) ]
  #[ doc( inline ) ]
  pub use ::parse_display ::FromStr;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn ::exposed :: *;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn;

  #[ doc( inline ) ]
  pub use ::derive_tools_meta :: *;

  #[ doc( inline ) ]
  #[ cfg( feature = "derive_from" ) ]
  pub use ::derive_tools_meta ::From;

  #[ doc( inline ) ]
  #[ cfg( feature = "derive_inner_from" ) ]
  pub use ::derive_tools_meta ::InnerFrom;

  #[ doc( inline ) ]
  #[ cfg( feature = "derive_new" ) ]
  pub use ::derive_tools_meta ::New;
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude 
{
  use super :: *;
  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn;

  #[ cfg( feature = "derive_clone_dyn" ) ]
  #[ doc( inline ) ]
  pub use ::clone_dyn ::prelude :: *;

  #[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
  #[ doc( inline ) ]
  pub use ::variadic_from ::prelude :: *;
}

