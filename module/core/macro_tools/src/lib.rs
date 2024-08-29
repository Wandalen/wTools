#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/proc_macro_tools/latest/proc_macro_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]


/// Internal namespace.
#[ cfg( feature = "enabled" ) ]
mod private
{
  use crate::*;

  ///
  /// Result with syn::Error.
  ///

  pub type Result< T > = std::result::Result< T, syn::Error >;

}

// qqq : improve description of each file

#[ cfg( all( feature = "enabled", feature = "attr" ) ) ]
pub mod attr;
#[ cfg( all( feature = "enabled", feature = "attr_prop" ) ) ]
pub mod attr_prop;
#[ cfg( all( feature = "enabled", feature = "components" ) ) ]
pub mod components;
#[ cfg( all( feature = "enabled", feature = "ct" ) ) ]
pub mod ct;
#[ cfg( all( feature = "enabled", feature = "container_kind" ) ) ]
pub mod container_kind;
#[ cfg( all( feature = "enabled", feature = "derive" ) ) ]
pub mod derive;
#[ cfg( all( feature = "enabled", feature = "diag" ) ) ]
pub mod diag;
#[ cfg( all( feature = "enabled", feature = "equation" ) ) ]
pub mod equation;
#[ cfg( all( feature = "enabled", feature = "generic_args" ) ) ]
pub mod generic_args;
#[ cfg( all( feature = "enabled", feature = "generic_params" ) ) ]
pub mod generic_params;
#[ cfg( all( feature = "enabled", feature = "item" ) ) ]
pub mod item;
#[ cfg( all( feature = "enabled", feature = "item_struct" ) ) ]
pub mod item_struct;
#[ cfg( all( feature = "enabled", feature = "name" ) ) ]
pub mod name;
#[ cfg( all( feature = "enabled", feature = "kw" ) ) ]
pub mod kw;
#[ cfg( all( feature = "enabled", feature = "phantom" ) ) ]
pub mod phantom;
#[ cfg( all( feature = "enabled", feature = "punctuated" ) ) ]
pub mod punctuated;
#[ cfg( all( feature = "enabled", feature = "quantifier" ) ) ]
pub mod quantifier;
#[ cfg( all( feature = "enabled", feature = "struct_like" ) ) ]
pub mod struct_like;
#[ cfg( all( feature = "enabled", feature = "tokens" ) ) ]
pub mod tokens;
#[ cfg( all( feature = "enabled", feature = "typ" ) ) ]
pub mod typ;
#[ cfg( all( feature = "enabled", feature = "typed" ) ) ]
pub mod typed;

#[ cfg( all( feature = "enabled" ) ) ]
pub mod iter;

///
/// Dependencies of the module.
///

#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod dependency
{
  pub use ::syn;
  pub use ::quote;
  pub use ::proc_macro2;
  pub use ::interval_adapter;
  pub use ::clone_dyn_types;
  pub use ::former_types;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use own::*;

// qqq : put every file of the first level under feature

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  // use super::*;

  mod _all
  {
    use super::super::*;
    pub use orphan::*;

    pub use private::
    {
      Result,
    };

    #[ cfg( feature = "attr" ) ]
    pub use attr::orphan::*;
    #[ cfg( feature = "attr_prop" ) ]
    pub use attr_prop::orphan::*;
    #[ cfg( feature = "components" ) ]
    pub use components::orphan::*;
    #[ cfg( feature = "container_kind" ) ]
    pub use container_kind::orphan::*;
    #[ cfg( feature = "ct" ) ]
    pub use ct::orphan::*;
    #[ cfg( feature = "derive" ) ]
    pub use derive::orphan::*;
    #[ cfg( feature = "diag" ) ]
    pub use diag::orphan::*;
    #[ cfg( feature = "equation" ) ]
    pub use equation::orphan::*;
    #[ cfg( feature = "generic_args" ) ]
    pub use generic_args::orphan::*;
    #[ cfg( feature = "generic_params" ) ]
    pub use generic_params::orphan::*;
    #[ cfg( feature = "item" ) ]
    pub use item::orphan::*;
    #[ cfg( feature = "item_struct" ) ]
    pub use item_struct::orphan::*;
    #[ cfg( feature = "name" ) ]
    pub use name::orphan::*;
    #[ cfg( feature = "kw" ) ]
    pub use kw::orphan::*;
    #[ cfg( feature = "phantom" ) ]
    pub use phantom::orphan::*;
    #[ cfg( feature = "punctuated" ) ]
    pub use punctuated::orphan::*;
    #[ cfg( feature = "quantifier" ) ]
    pub use quantifier::orphan::*;
    #[ cfg( feature = "struct_like" ) ]
    pub use struct_like::orphan::*;
    #[ cfg( feature = "tokens" ) ]
    pub use tokens::orphan::*;
    #[ cfg( feature = "typ" ) ]
    pub use typ::orphan::*;
    #[ cfg( feature = "typed" ) ]
    pub use typed::orphan::*;

    pub use iter::orphan::*;

  }

  #[ doc( inline ) ]
  pub use _all::*;

}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
  #[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  mod _all
  {
    use super::super::*;
    pub use exposed::*;
  }

  #[ doc( inline ) ]
  pub use _all::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  mod _all
  {
    use super::super::*;
    pub use prelude::*;

    #[ cfg( feature = "attr" ) ]
    pub use attr::exposed::*;
    #[ cfg( feature = "attr_prop" ) ]
    pub use attr_prop::exposed::*;
    #[ cfg( feature = "components" ) ]
    pub use components::exposed::*;
    #[ cfg( feature = "container_kind" ) ]
    pub use container_kind::exposed::*;
    #[ cfg( feature = "ct" ) ]
    pub use ct::exposed::*;
    #[ cfg( feature = "derive" ) ]
    pub use derive::exposed::*;
    #[ cfg( feature = "diag" ) ]
    pub use diag::exposed::*;
    #[ cfg( feature = "equation" ) ]
    pub use equation::exposed::*;
    #[ cfg( feature = "generic_args" ) ]
    pub use generic_args::exposed::*;
    #[ cfg( feature = "generic_params" ) ]
    pub use generic_params::exposed::*;
    #[ cfg( feature = "item" ) ]
    pub use item::exposed::*;
    #[ cfg( feature = "item_struct" ) ]
    pub use item_struct::exposed::*;
    #[ cfg( feature = "name" ) ]
    pub use name::exposed::*;
    #[ cfg( feature = "kw" ) ]
    pub use kw::exposed::*;
    #[ cfg( feature = "phantom" ) ]
    pub use phantom::exposed::*;
    #[ cfg( feature = "punctuated" ) ]
    pub use punctuated::exposed::*;
    #[ cfg( feature = "quantifier" ) ]
    pub use quantifier::exposed::*;
    #[ cfg( feature = "struct_like" ) ]
    pub use struct_like::exposed::*;
    #[ cfg( feature = "tokens" ) ]
    pub use tokens::exposed::*;
    #[ cfg( feature = "typ" ) ]
    pub use typ::exposed::*;
    #[ cfg( feature = "typed" ) ]
    pub use typed::exposed::*;

    pub use iter::exposed::*;

  }

  #[ doc( inline ) ]
  pub use _all::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  mod _all
  {
    use super::super::*;
    // pub use prelude::*;

    #[ cfg( feature = "attr" ) ]
    pub use attr::prelude::*;
    #[ cfg( feature = "attr_prop" ) ]
    pub use attr_prop::prelude::*;
    #[ cfg( feature = "components" ) ]
    pub use components::prelude::*;
    #[ cfg( feature = "container_kind" ) ]
    pub use container_kind::prelude::*;
    #[ cfg( feature = "ct" ) ]
    pub use ct::prelude::*;
    #[ cfg( feature = "derive" ) ]
    pub use derive::prelude::*;
    #[ cfg( feature = "diag" ) ]
    pub use diag::prelude::*;
    #[ cfg( feature = "equation" ) ]
    pub use equation::prelude::*;
    #[ cfg( feature = "generic_args" ) ]
    pub use generic_args::prelude::*;
    #[ cfg( feature = "generic_params" ) ]
    pub use generic_params::prelude::*;
    #[ cfg( feature = "item" ) ]
    pub use item::prelude::*;
    #[ cfg( feature = "item_struct" ) ]
    pub use item_struct::prelude::*;
    #[ cfg( feature = "name" ) ]
    pub use name::prelude::*;
    #[ cfg( feature = "kw" ) ]
    pub use kw::exposed::*;
    #[ cfg( feature = "phantom" ) ]
    pub use phantom::prelude::*;
    #[ cfg( feature = "punctuated" ) ]
    pub use punctuated::prelude::*;
    #[ cfg( feature = "quantifier" ) ]
    pub use quantifier::prelude::*;
    #[ cfg( feature = "struct_like" ) ]
    pub use struct_like::prelude::*;
    #[ cfg( feature = "tokens" ) ]
    pub use tokens::prelude::*;
    #[ cfg( feature = "typ" ) ]
    pub use typ::prelude::*;
    #[ cfg( feature = "typed" ) ]
    pub use typed::prelude::*;

    pub use iter::prelude::*;

  }

  #[ doc( inline ) ]
  pub use _all::*;

  #[ doc( inline ) ]
  pub use ::interval_adapter::prelude::*;

  #[ doc( inline ) ]
  pub use ::syn;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::proc_macro2;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::quote;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::quote::
  {
    quote,
    quote as qt,
    quote_spanned,
    format_ident,
  };

  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::syn::spanned::Spanned;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use syn::
  {
    parse::ParseStream,
    Token,
    spanned::Spanned,
    braced,
    bracketed,
    custom_keyword,
    custom_punctuation,
    parenthesized,
    parse_macro_input,
    parse_quote,
    parse_quote as parse_qt,
    parse_quote_spanned,
    parse_quote_spanned as parse_qt_spanned,
  };

}

// qqq : introduce features. make it smart. discuss list of features before implementing
