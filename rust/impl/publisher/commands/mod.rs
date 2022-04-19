#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Library of utility to work with commands.
//!

///
/// Publish module.
///

mod publish;

///
/// Form CA commands.
///

pub fn commands_form() -> std::collections::HashMap<String, wca::command::Command>
{
  let publish_command = wca::CommandOptions::default()
  .hint( "Publish package on `crates.io`." )
  .long_hint( "Publish package on `crates.io`." )
  .phrase( "" )
  .subject_hint( "A path to package. Should be a directory with file `Cargo.toml`." )
  .property_hint( "dry", "Run command dry. Default is false." )
  .property_hint( "verbosity", "Setup level of verbosity." )
  .property_alias( "verbosity", "v" )
  .routine( &publish::publish )
  .form();

  let ca_map = std::collections::HashMap::from
  ([
    ( ".publish".to_string(), publish_command )
  ]);

  ca_map
}

///
/// Print help from map of commands.
///

pub fn print_help( ca_map : &std::collections::HashMap<String, wca::Command> ) -> Result<(), wtools::error::Error>
{
  println!( "Illformed command" );
  for ( command_name, command_descriptor ) in ca_map.iter()
  {
    println!("{} - {}", command_name, command_descriptor.hint );
  }
  Ok( () )
}
