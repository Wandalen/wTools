//!
//! Tool's secret
//!

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

  #[ ser::serde_as ]
  #[ derive( Debug, Error, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum Error
  {
    #[ error( "Secret file is illformed\n{0}" ) ]
    SecretFileIllformed
    (
      #[ from ]
      #[ serde_as( as = "DisplayFromStr" ) ]
      dotenv::Error
    ),

    #[ error( "Secret missing the variable {0}" ) ]
    VariableMissing( &'static str ),

    #[ error( "Secret error processing in the variable {0}\n{1}" ) ]
    VariableIllformed( &'static str, String ),

  }

  pub type Result< R > = std::result::Result< R, Error >;

  #[ derive( Debug ) ]
  #[ allow( non_snake_case ) ]
  pub struct Secret
  {
    pub CLIENT_SECRET : String,
    pub CLIENT_ID: String,
    pub AUTH_URI : String,
    pub TOKEN_URI : String,
  }

  impl Secret
  {
    #[ allow( non_snake_case ) ]
    pub fn load() -> Result< Self >
    {
      let path = "./.key/-env.sh";

      let r = dotenv::from_path( path );
      if let Err( ref err ) = r
      {
        if !matches!( err, dotenv::Error::Io(_) )
        {
          return Err( r.expect_err( &format!( "Failed to load {path}" ) ).into() );
        }
      }

      let config = Self
      {
        CLIENT_SECRET : var( "CLIENT_SECRET", None )?,
        CLIENT_ID : var( "CLIENT_ID", None )?,
        AUTH_URI : var ( "AUTH_URI", None )?,
        TOKEN_URI : var ( "TOKEN_URI", None )?
      };
      Ok( config )
    }

    pub fn read() -> Secret
    {
      Self::load().unwrap_or_else( | err |
      {
        let example = include_str!("../.key/readme.md");
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

    pub fn get() -> &'static Secret
    {
      static INSTANCE : OnceLock< Secret > = OnceLock::new();
      INSTANCE.get_or_init( || Self::read() )
    }

  }

  fn var
  (
    name : &'static str,
    default : Option<&'static str>,
  ) -> Result < String >
  {
    match env::var(name)
    {
      Ok( val ) => Ok ( val ),
      Err( _ ) =>
      {
        if let Some( default_value ) = default
        {
          Ok( default_value.to_string() )
        }
        else
        {
          Err ( Error::VariableMissing( name ) )
        }
      }
    }
  }

  fn _var_path
  (
    name : &'static str,
    default : Option<&'static str>,
  ) -> Result < pth::AbsolutePath >
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