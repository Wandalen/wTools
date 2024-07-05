//!
//! Client of API.
//!

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

#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use orphan::*;

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use private::
  {
    ClientOptions,
    client,
    AssistantObject,
  };

  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use reflect_tools::
  // {
  //   Fields,
  //   _IteratorTrait,
  //   IteratorTrait,
  // };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
