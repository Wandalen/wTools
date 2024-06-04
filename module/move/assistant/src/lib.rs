#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/assistant/latest/assistant/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

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

/// Reflections.
pub mod reflect;
pub use reflect::*;
/// Nice print.
pub mod print;
pub use print::*;

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
