#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/assistant/latest/assistant/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]


/// Internal namespace.
pub( crate ) mod private
{

  pub use openai_api_rs::v1::
  {
    api::Client,
    assistant::AssistantObject,
  };

  use std::
  {
    env,
    error::Error,
  };

  use former::Former;

  /// Options for configuring the OpenAI API client.
  #[ derive( Former, Debug ) ]
  pub struct ClientOptions
  {
    /// The API key for authenticating with the OpenAI API.
    pub api_key : Option< String >,
  }

  /// Creates a new OpenAI API client using the API key from the environment variable `OPENAI_API_KEY`.
  pub fn client() -> Result< Client, Box< dyn Error > >
  {
    let api_key = env::var( "OPENAI_API_KEY" )?;
    Ok( Client::new( api_key ) )
  }


}

// /// Reflections.
// pub mod reflect;
// pub use reflect::*;
// /// Universal wrapper.
// pub mod wrapper_ref;
// pub use wrapper_ref::*;
// /// Universal wrapper.
// pub mod wrapper_option_cow_ref;
// pub use wrapper_option_cow_ref::*;
// /// Conversion to string.
// pub mod to_string;
// pub use to_string::*;

/// Nice print.
pub mod print;
// pub use print::*;
/// Nice print's wrapper.
pub mod as_table;
// pub use as_table::*;
/// Table interface.
pub mod table;
// pub use table::*;

#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    print::orphan::*,
    as_table::orphan::*,
    table::orphan::*,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;

}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    print::exposed::*,
    as_table::exposed::*,
    table::exposed::*,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    ClientOptions,
    client,
    AssistantObject,
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
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    print::prelude::*,
    as_table::prelude::*,
    table::prelude::*,
  };

}
