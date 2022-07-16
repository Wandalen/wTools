
mod_interface::mod_interface!
{
  #[ cfg( feature = "impls_index" ) ]
  #[ doc( inline ) ]
  prelude use ::impls_index::prelude::*;
  #[ cfg( feature = "mod_interface" ) ]
  #[ doc( inline ) ]
  prelude use ::mod_interface::prelude::*;

  prelude use ::mod_interface::prelude::mod_interface;

  #[ cfg( feature = "for_each" ) ]
  #[ doc( inline ) ]
  prelude use ::for_each::prelude::*;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  #[ doc( inline ) ]
  prelude use ::woptions::prelude::*;
  #[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
  #[ doc( inline ) ]
  prelude use ::woptions as options;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  #[ doc( inline ) ]
  prelude use ::former::prelude::*;
  #[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
  #[ doc( inline ) ]
  prelude use ::former as former;
  #[ cfg( feature = "collection_make" ) ]
  #[ doc( inline ) ]
  prelude use ::literally::*;
}
