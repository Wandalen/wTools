#![ cfg_attr( feature = "no_std", no_std ) ]

/// Internal namespace.
mod private
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::mod_interface;
  #[ cfg( feature = "meta_for_each" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::for_each;
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index;
  #[ cfg( feature = "meta_idents_concat" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::paste;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    mod_interface,
  };
  #[ cfg( feature = "meta_for_each" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    for_each,
  };
  #[ cfg( feature = "meta_impls_index" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    impls_index,
  };
  #[ cfg( feature = "meta_idents_concat" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    paste,
  };
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed::*;