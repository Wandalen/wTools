mod private 
{

  use error_tools::typed::Error;
  use derive_tools::{ AsRefStr };
  use ser::DisplayFromStr;

  /// Collective enum for errors in OpenAI actions.
  #[ ser::serde_as ]
  #[ derive( Debug, Error, AsRefStr, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum Error
  {
    /// API error from the underlying implementation crate.
    #[ error( "OpenAI API returned error:\n{0}" ) ]
    ApiError
    (
      #[ from ] 
      #[ serde_as( as = "DisplayFromStr" ) ] 
      openai_api_rs::v1::error::APIError 
    )
  }

  /// Shorthand for `Result` in OpenAI actions.
  pub type Result< T > = core::result::Result< T, OpenAiError >;

}

crate::mod_interface!
{
  own use
  {
    Error,
    Result
  }
}