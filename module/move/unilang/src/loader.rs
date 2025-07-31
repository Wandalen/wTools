//!
//! Handles loading command definitions from external files (YAML/JSON).
//!

/// Internal namespace.
mod private
{
  use crate::
  {
    data::{ CommandDefinition, OutputData },
    error::Error,
    registry::CommandRoutine,
  };

///
/// Loads command definitions from a YAML string.
///
/// # Errors
///
/// Returns an `Error::Yaml` if the YAML string is invalid.
///
pub fn load_command_definitions_from_yaml_str( yaml_str : &str ) -> Result< Vec< CommandDefinition >, Error >
{
  let definitions : Vec< CommandDefinition > = serde_yaml::from_str( yaml_str ).map_err( Error::Yaml )?;
  Ok( definitions )
}

///
/// Loads command definitions from a JSON string.
///
/// # Errors
///
/// Returns an `Error::Json` if the JSON string is invalid.
///
pub fn load_command_definitions_from_json_str( json_str : &str ) -> Result< Vec< CommandDefinition >, Error >
{
  let definitions : Vec< CommandDefinition > = serde_json::from_str( json_str ).map_err( Error::Json )?;
  Ok( definitions )
}

///
/// Resolves a routine link string to a `CommandRoutine`.
///
/// This is a placeholder for now. In a later increment, this will handle
/// dynamic loading of routines from shared libraries or Rust modules.
///
/// # Errors
///
/// Returns an `Error::Execution` if the link is not recognized or if
/// dynamic loading fails (in future increments).
///
pub fn resolve_routine_link( _link : &str ) -> Result< CommandRoutine, Error >
{
  // qqq: This is a placeholder. Actual dynamic loading will be implemented in a later increment.
  // For now, return a dummy routine or an error if the link is not recognized.
  // For testing purposes, we can return a routine that just prints the link.
  Ok( Box::new( move | _args, _context |
  {
    // println!( "Dummy routine executed for link: {}", link );
    Ok( OutputData
    {
      content : String::new(),
      format : String::new(),
    })
  }) )
}

}

mod_interface::mod_interface!
{
  exposed use private::load_command_definitions_from_yaml_str;
  exposed use private::load_command_definitions_from_json_str;
  exposed use private::resolve_routine_link;
  
  prelude use private::load_command_definitions_from_yaml_str;
  prelude use private::load_command_definitions_from_json_str;
}
