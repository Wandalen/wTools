#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/proc_macro_tools/latest/proc_macro_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// qqq : review every page of generated documentation improve how it look as well as its content
//
// attr
// Protected namespace of the module.
// container_kind
// Protected namespace of the module.
// dependency
// Dependencies of the module.
// derive
// Protected namespace of the module.
// diag
// Protected namespace of the module.
// drop
// Protected namespace of the module.
// exposed
// Exposed namespace of the module.

/// Modular files.
#[ cfg( feature = "enabled" ) ]
#[ path = "." ]
mod file
{
  // use super::*;
  pub mod attr;
  pub mod attr_prop;
  pub mod container_kind;
  pub mod derive;
  pub mod diag;
  pub mod drop;
  pub mod equation;
  pub mod generic_args;
  pub mod generic_params;
  pub mod item;
  pub mod item_struct;
  pub mod iter;
  pub mod name;
  pub mod phantom;
  pub mod punctuated;
  pub mod quantifier;
  pub mod struct_like;
  pub mod tokens;
  pub mod typ;
}

///
/// Dependencies of the module.
///

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::syn;
  pub use ::quote;
  pub use ::proc_macro2;
  pub use ::interval_adapter;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    orphan::*,
  };
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::file::
  {
    attr::orphan::*,
    attr_prop::orphan::*,
    container_kind::orphan::*,
    derive::orphan::*,
    diag::orphan::*,
    drop::orphan::*,
    equation::orphan::*,
    generic_args::orphan::*,
    generic_params::orphan::*,
    item::orphan::*,
    item_struct::orphan::*,
    iter::orphan::*,
    name::orphan::*,
    phantom::orphan::*,
    punctuated::orphan::*,
    quantifier::orphan::*,
    struct_like::orphan::*,
    tokens::orphan::*,
    typ::orphan::*,
  };
}

/// Parented namespace of the module.
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
  pub use quote::
  {
    format_ident,
    quote,
    quote_spanned,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::file::
  {
    attr::exposed::*,
    attr_prop::exposed::*,
    container_kind::exposed::*,
    derive::orphan::*,
    diag::exposed::*,
    drop::exposed::*,
    equation::exposed::*,
    generic_args::exposed::*,
    generic_params::exposed::*,
    item::exposed::*,
    item_struct::exposed::*,
    iter::exposed::*,
    name::exposed::*,
    phantom::exposed::*,
    punctuated::exposed::*,
    quantifier::exposed::*,
    struct_like::exposed::*,
    tokens::exposed::*,
    typ::exposed::*,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::interval_adapter::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
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
    quote as qt,
    format_ident,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::syn::spanned::Spanned;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use syn::
  {
    parse::ParseStream,
    Token,
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

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::file::
  {
    attr::prelude::*,
    attr_prop::prelude::*,
    container_kind::prelude::*,
    derive::orphan::*,
    diag::prelude::*,
    drop::prelude::*,
    equation::prelude::*,
    generic_args::prelude::*,
    generic_params::prelude::*,
    item::prelude::*,
    item_struct::prelude::*,
    iter::prelude::*,
    name::prelude::*,
    phantom::prelude::*,
    punctuated::prelude::*,
    quantifier::prelude::*,
    struct_like::prelude::*,
    tokens::prelude::*,
    typ::prelude::*,
  };

}

// qqq : introduce features. make it smart. discuss list of features before implementing
