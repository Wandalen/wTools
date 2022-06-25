
///
/// Form CA commands.
///

pub fn commands_form() -> std::collections::HashMap< String, wca::command::Command >
{
  let smoke_command = wca::CommandOptions::default()
  .hint( "Perform smoke testing on module." )
  .long_hint( "Perform smoke testing on module." )
  .phrase( "smoke" )
  .subject_hint( "A path to module. Should be a directory with file `Cargo.toml`. Default is current directory." )
  .property_hint( "smoke", "A variant of smoke testing of module. It can be:\n  local - local module in directory.\n  published - module published on `crates.io`. true - local and published version.\n  Default is \"local\"" )
  .property_hint( "code_path", "A path to code snippet to test. By default utility imports module into binary." )
  .property_hint( "version", "A string version of module. By default \"*\"" )
  .routine( &super::smoke::smoke )
  .form();

  let ca_map = std::collections::HashMap::from
  ([
    ( ".smoke".to_string(), smoke_command ),
  ]);

  ca_map
}
