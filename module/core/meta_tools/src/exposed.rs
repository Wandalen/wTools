/// Internal namespace.
mod private
{
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super ::private ::
  {
 };

  // Re-export impls_index macros to make them available at meta_tools::exposed
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
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index as implsindex;

  // Re-export mod_interface macros
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::mod_interface_meta:: { mod_interface };

  // Re-export for_each macros
  #[ cfg( feature = "meta_for_each" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::for_each:: { for_each };

  // Re-export paste for meta_idents_concat
  #[ cfg( feature = "meta_idents_concat" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::paste::paste as meta_idents_concat;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed :: *;