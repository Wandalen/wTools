/// Internal namespace.
#[ allow( ambiguous_glob_reexports ) ]
mod private
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::mod_interface:: *;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::mod_interface_meta:: *;

  #[ cfg( feature = "meta_for_each" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::for_each:: *;

  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index:: *;
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index_meta:: *;

  #[ cfg( feature = "meta_idents_concat" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::paste:: *;
}

/// Exposed namespace of the module.
#[ allow( clippy::module_inception ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super ::private :: *;

  // Explicitly re-export impls_index macros
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index:: { index, tests_index, impls1, impls_optional, tests_impls, tests_impls_optional, impls2 };
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index_meta::impls3;
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index_meta::impls3 as impls;
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index:: { fn_name, fn_rename, fns, fns2 };

  // Explicitly re-export mod_interface macros
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::mod_interface_meta:: { mod_interface };

  // Explicitly re-export paste for meta_idents_concat
  #[ cfg( feature = "meta_idents_concat" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::paste::paste as meta_idents_concat;

  // Re-export implsindex as a module reference (used in some tests)
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index as implsindex;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed :: *;