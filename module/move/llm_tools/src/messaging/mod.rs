//! Messaging.

mod private
{

  /// Represents potential errors that may occur during LLM invocations.
  #[ derive( Debug ) ]
  pub enum Error
  {
    /// Catch-all variant for unknown or miscellaneous errors.
    Unknown( String ),
  }

  /// Alias for JSON-based values commonly used for LLM parameters.
  pub type Value = serde_json::Value;

}

crate::mod_interface!
{

  layer conversation;
  layer message;
  layer messages;
  layer send;
  layer stream;

  own use private::
  {
    Error,
    Value,
  };
}
