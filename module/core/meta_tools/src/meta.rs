//!
//! Collection of general purpose meta tools.
//!

/// Define a private namespace for all its items.
mod private
{
}

//

// #[ cfg( feature = "enabled" ) ]
// mod_interface::mod_interface!
// {
//   #![ debug ]
//
//   #[ cfg( feature = "meta_impls_index" ) ]
//   use ::impls_index;
//   #[ cfg( feature = "meta_for_each" ) ]
//   use ::for_each;
//   // #[ cfg( feature = "meta_mod_interface" ) ]
//   use ::mod_interface;
//   // #[ cfg( feature = "meta_mod_interface" ) ]
//   prelude use ::mod_interface::mod_interface;
//
//   #[ cfg( feature = "meta_idents_concat" ) ]
//   prelude use ::paste::paste as meta_idents_concat;
//
// }

#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod own
{
  use super::*;
  pub use ::impls_index::orphan::*;
  pub use ::for_each::orphan::*;
  pub use ::mod_interface::orphan::*;
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  // pub use ::impls_index;
  // pub use ::for_each;
  // pub use ::mod_interface;

  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use prelude::*;
  pub use super::super::meta;
  pub use ::impls_index::exposed::*;
  pub use ::for_each::exposed::*;
  pub use ::mod_interface::exposed::*;
  pub use ::paste::paste as meta_idents_concat;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  pub use ::impls_index::prelude::*;
  pub use ::for_each::prelude::*;
  pub use ::mod_interface::prelude::*;
}
