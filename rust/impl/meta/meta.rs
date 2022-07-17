/// Internal namespace.
pub( crate ) mod private
{

}

//

// qqq : xxx : optimize
mod_interface::mod_interface!
{

  #[ cfg( feature = "impls_index" ) ]
  prelude use ::impls_index::prelude::*;
  #[ cfg( feature = "mod_interface" ) ]
  prelude use ::mod_interface::prelude::*;
  prelude use ::mod_interface::prelude::mod_interface;

  #[ cfg( feature = "for_each" ) ]
  prelude use ::for_each::prelude::*;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  prelude use ::woptions::prelude::*;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  prelude use ::woptions as options;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  prelude use ::former::prelude::*;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  prelude use ::former as former;
  #[ cfg( feature = "collection_make" ) ]
  prelude use ::literally::*;

}
