/// Internal namespace.
pub( crate ) mod private
{

}

//
// xxx : optimize
mod_interface::mod_interface!
{

  #[ cfg( feature = "impls_index" ) ]
  use ::impls_index;
  #[ cfg( feature = "for_each" ) ]
  use ::for_each;
  #[ cfg( feature = "mod_interface" ) ]
  use ::mod_interface;
  #[ cfg( feature = "mod_interface" ) ]
  prelude use ::mod_interface::mod_interface;

  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  use ::woptions;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  prelude use ::woptions as options;

  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  use ::former;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  prelude use ::former as former;

  #[ cfg( feature = "collection_make" ) ]
  prelude use ::literally::*;

}
