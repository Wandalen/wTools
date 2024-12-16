//!
//! Tool's secrets.
//!

/// Internal namespace.
mod private
{
  use crate::*;
  use std::
  {
    env,
    sync::OnceLock,
  };

  use error_tools::typed::Error;
  use ser::DisplayFromStr;

  /// Typed secret error.
  #[ ser::serde_as ]
  #[ derive( Debug, Error, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum Error
  {

    /// Secret file is illformed.
    #[ error( "Secret file is illformed\n{0}" ) ]
    SecretFileIllformed
    ( 
      #[ from ] 
      #[ serde_as( as = "DisplayFromStr" ) ] 
      dotenv::Error
    ),

    /// Some variable in the secrets is missing.
    #[ error( "Secret misssing the variable {0}" ) ]
    VariableMissing( &'static str ),

    /// Some variable in the secrets is illformed.
    #[ error( "Secret error processing the variable {0}\n{1}" ) ]
    VariableIllformed( &'static str, String ),

  }

  /// Result type for `Secret` methods.
  pub type Result< R > = core::result::Result< R, Error >;

  /// Represents the application secrets loaded from environment variables.
  #[ derive( Debug ) ]
  #[ allow( non_snake_case ) ]
  pub struct Secret
  {
    /// OpenAI API key.
    pub OPENAI_API_KEY : String,
  }

  impl Secret
  {

    /// Loads secrets from environment variables.
    ///
    /// # Returns
    ///
    /// * `Result< Self >` - On success, returns a `Secret` instance with values from environment variables.
    /// * On failure, returns an error indicating which environment variable is missing or invalid.
    #[ allow( non_snake_case ) ]
    pub fn load() -> Result< Self >
    {
      let path = "./.key/-env.sh";

      // Attempt to load environment variables from the specified file
      let r = dotenv::from_filename( path );
      if let Err( ref err ) = r
      {
        // Only return an error if it's not an Io error, and include the file path in the error message
        if !matches!( err, dotenv::Error::Io( _ ) )
        {
          return Err( r.expect_err( &format!( "Failed to load {path}" ) ).into() );
        }
      }

      let config = Self
      {
        OPENAI_API_KEY : var( "OPENAI_API_KEY", None )?,
      };
      Ok( config )
    }

    /// Reads the secrets, panicking with an explanation if loading fails.
    ///
    /// # Returns
    ///
    /// * `Secret` - The loaded secrets.
    ///
    /// # Panics
    ///
    /// * Panics with a detailed explanation if the secrets cannot be loaded.

    pub fn read() -> Secret
    {
      Self::load().unwrap_or_else( | err |
      {
        let example = include_str!( "../.key/readme.md" );
        let explanation = format!
        (
r#" = Lack of secrets

Failed to load secret or some its parameters.
{err}

 = Fix

Either define missing environment variable or make sure `./.key/-env.toml` file has it defined.

 = More information

{example}
"#
        );
        panic!( "{}", explanation );
      })
    }

    /// Retrieves a static reference to the secrets, initializing it if necessary.
    ///
    /// # Returns
    ///
    /// * `&'static Secret` - A static reference to the secrets.
    ///
    /// # Warning
    ///
    /// * Do not use this function unless absolutely necessary.
    /// * Avoid using it in `lib.rs`.
    pub fn get() -> &'static Secret
    {
      static INSTANCE : OnceLock< Secret > = OnceLock::new();
      INSTANCE.get_or_init( || Self::read() )
    }

  }

  /// Retrieves the value of an environment variable as a `String`.
  ///
  /// This function attempts to fetch the value of the specified environment variable.
  /// If the variable is not set, it returns a provided default value if available, or an error if not.
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the environment variable to retrieve.
  /// * `default` - An optional default value to return if the environment variable is not set.
  ///
  /// # Returns
  ///
  /// * `Result<String>` - On success, returns the value of the environment variable or the default value.
  /// * On failure, returns an error indicating the missing environment variable.
  fn var
  (
    name : &'static str, 
    default : Option< &'static str >,
  ) -> Result< String >
  {
    match env::var( name )
    {
      Ok( value ) => Ok( value ),
      Err( _ ) =>
      {
        if let Some( default_value ) = default
        {
          Ok( default_value.to_string() )
        }
        else
        {
          Err( Error::VariableMissing( name ) )
        }
      }
    }
  }

  /// Retrieves the value of an environment variable as an `AbsolutePath`.
  ///
  /// This function attempts to fetch the value of the specified environment variable and convert it into an `AbsolutePath`.
  /// If the variable is not set, it returns a provided default value if available, or an error if not.
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the environment variable to retrieve.
  /// * `default` - An optional default value to return if the environment variable is not set.
  ///
  /// # Returns
  ///
  /// * `Result<pth::AbsolutePath>` - On success, returns the parsed `AbsolutePath`.
  /// * On failure, returns an error indicating the missing or ill-formed environment variable.
  fn _var_path
  (
    name : &'static str, 
    default : Option< &'static str >,
  ) -> Result< pth::AbsolutePath >
  {
    let p = var( name, default )?;
    pth::AbsolutePath::from_paths( ( pth::CurrentPath, p ) )
    .map_err( |e| Error::VariableIllformed( name, e.to_string() ) )
  }

}

crate::mod_interface!
{

  own use
  {
    Error,
    Result,
  };

  orphan use
  {
    Secret,
  };

}