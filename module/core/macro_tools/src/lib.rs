#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/proc_macro_tools/latest/proc_macro_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Modular files.
#[ cfg( feature = "enabled" ) ]
#[ path = "." ]
mod file
{
  use super::*;
  pub mod attr;
  pub mod container_kind;
  pub mod diag;
  pub mod generic_analyze;
  pub mod generic_args;
  pub mod generic_params;
  pub mod name;
  pub mod punctuated;
  pub mod quantifier;
  pub mod tokens;
  pub mod typ;
  pub mod type_struct;
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

  pub use super::file::
  {
    attr::orphan::*,
    container_kind::orphan::*,
    diag::orphan::*,
    generic_analyze::orphan::*,
    generic_args::orphan::*,
    generic_params::orphan::*,
    name::orphan::*,
    punctuated::orphan::*,
    quantifier::orphan::*,
    tokens::orphan::*,
    typ::orphan::*,
    type_struct::orphan::*,
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
  pub use super::file::
  {
    attr::exposed::*,
    container_kind::exposed::*,
    diag::exposed::*,
    generic_analyze::exposed::*,
    generic_args::exposed::*,
    generic_params::exposed::*,
    name::exposed::*,
    punctuated::orphan::*,
    quantifier::exposed::*,
    tokens::exposed::*,
    typ::exposed::*,
    type_struct::exposed::*,
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
  pub use ::quote::quote as qt;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::syn::parse_quote as parse_qt;
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
    parse_quote_spanned,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::file::
  {
    attr::prelude::*,
    container_kind::prelude::*,
    diag::prelude::*,
    generic_analyze::prelude::*,
    generic_args::prelude::*,
    generic_params::prelude::*,
    name::prelude::*,
    punctuated::orphan::*,
    quantifier::prelude::*,
    tokens::prelude::*,
    typ::prelude::*,
    type_struct::prelude::*,
  };

}

// qqq : introduce features. make it smart. discuss list of features before implementing
