#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/assistant/latest/assistant/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]


/// Internal namespace.
pub( crate ) mod private
{
}

pub mod client;

#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    client::orphan::*,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    client::exposed::*,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use reflect_tools::
  {
    Fields,
    _IteratorTrait,
    IteratorTrait,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    client::prelude::*,
  };

}
