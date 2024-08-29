//!
//! Collection of general purpose meta tools.
//!

/// Internal namespace.
mod private
{
}

//

#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{

  #[ cfg( feature = "meta_impls_index" ) ]
  use ::impls_index;
  #[ cfg( feature = "meta_for_each" ) ]
  use ::for_each;
  // #[ cfg( feature = "meta_mod_interface" ) ]
  use ::mod_interface;
  // #[ cfg( feature = "meta_mod_interface" ) ]
  prelude use ::mod_interface::mod_interface;

  #[ cfg( feature = "meta_constructors" ) ]
  prelude use ::literally::*;
  #[ cfg( feature = "meta_idents_concat" ) ]
  prelude use ::paste::paste as meta_idents_concat;

  // #[ cfg( feature = "options" ) ]
  // use ::woptions;
  // #[ cfg( feature = "options" ) ]
  // prelude use ::woptions as options;

  // #[ cfg( feature = "former" ) ]
  // use ::former;
  // #[ cfg( feature = "former" ) ]
  // prelude use ::former as former;

}
