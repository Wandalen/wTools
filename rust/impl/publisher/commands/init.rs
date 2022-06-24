
///
/// Form CA commands.
///

#[ cfg( feature = "use_std" ) ]
pub fn commands_form() -> std::collections::HashMap< String, wca::command::Command >
{
  let publish_command = wca::CommandOptions::default()
  .hint( "Publish package on `crates.io`." )
  .long_hint( "Publish package on `crates.io`." )
  .phrase( "publish" )
  .subject_hint( "A path to package. Should be a directory with file `Cargo.toml`." )
  .property_hint( "dry", "Run command dry. Default is false." )
  .property_hint( "verbosity", "Setup level of verbosity." )
  .property_alias( "verbosity", "v" )
  .routine( &super::publish::publish )
  .form();

  let list_command = wca::CommandOptions::default()
  .hint( "List packages." )
  .long_hint( "List packages" )
  .phrase( "list" )
  .subject_hint( "A path to directory with packages. Should be a glob." )
  .routine( &super::list::list )
  .form();

  let ca_map = std::collections::HashMap::from
  ([
    ( ".publish".to_string(), publish_command ),
    ( ".list".to_string(), list_command ),
  ]);

  ca_map
}
