//!
//! OpenAI API actions.
//!
//! This module also contains the definition of OpenAI Error.
//!

mod private 
{

  use error_tools::typed::Error;
  use derive_tools::{ AsRefStr };

  use crate::*;
  use ser::DisplayFromStr;

  use commands::TableConfig;

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
    ),

    /// User chosen a mix of table styles instead of a single one.
    /// E.g.: both `--as-table` and `--as-records` were set, however only one style must be chosen
    #[ error( "Select only one table style: either `--as-table`, `--as-records`, or `--columns`" ) ]
    WrongTableStyle,
  }

  /// Shorthand for `Result` in OpenAI actions.
  pub type Result< T > = core::result::Result< T, Error >;

  /// Check the CLI arguments for table style.
  /// There are 3 arguments: `--as-table`, `--as-records`, `--columns`. Only one argument 
  /// should be active a time.
  pub fn check_table_style( table_config: &TableConfig ) -> Result< () >
  {
    if table_config.as_table   && ( table_config.as_records || table_config.columns  )
    || table_config.as_records && ( table_config.as_table   || table_config.columns  )
    || table_config.columns    && ( table_config.as_records || table_config.as_table )
    {
      return Err( Error::WrongTableStyle )
    }

    Ok( () )
  }
}

crate::mod_interface!
{
  own use
  {
    Error,
    Result,
    check_table_style,
  };
}