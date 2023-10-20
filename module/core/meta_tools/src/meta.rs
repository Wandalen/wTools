//!
//! Collection of general purpose meta tools.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

//

#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{

  #[ cfg( feature = "impls_index" ) ]
  use ::impls_index;
  #[ cfg( feature = "for_each" ) ]
  use ::for_each;
  // #[ cfg( feature = "mod_interface" ) ]
  use ::mod_interface;
  // #[ cfg( feature = "mod_interface" ) ]
  prelude use ::mod_interface::mod_interface;

  #[ cfg( feature = "collection_make" ) ]
  prelude use ::literally::*;
  #[ cfg( feature = "idents_concat" ) ]
  prelude use ::paste::paste as idents_concat;

  // #[ cfg( feature = "options" ) ]
  // use ::woptions;
  // #[ cfg( feature = "options" ) ]
  // prelude use ::woptions as options;

  // #[ cfg( feature = "former" ) ]
  // use ::former;
  // #[ cfg( feature = "former" ) ]
  // prelude use ::former as former;

}
