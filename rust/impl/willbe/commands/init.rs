/// Internal namespace.
pub( crate ) mod private
{
  ///
  /// Form CA commands.
  ///

  pub fn commands_form() -> std::collections::HashMap< String, wca::command::Command >
  {
    let info_command = wca::Command::former()
    .hint( "Prints information about package" )
    .long_hint( "Prints information about package at current directory" )
    .phrase( "crate.info" )
    .routine( &crate::commands::info::info )
    .form();

    let each_command = wca::Command::former()
    .hint( "--- each ---" )
    .long_hint( "--- each ---" )
    .phrase( "each" )
    .subject_hint( "What to iterate(?)" )
    .routine( &crate::commands::each::each )
    .form();

    let publish_command = wca::Command::former()
    .hint( "--- publish ---" )
    .long_hint( "--- publish ---" )
    .phrase( "publish" )
    .property_hint( "push_remote", "Remote url to push" )
    .routine( &crate::commands::publish::publish )
    .form();

    std::collections::HashMap::from
    ([
      ( ".crate.info".to_string(), info_command ),
      ( ".each".to_string(), each_command ),
      ( ".crate.publish".to_string(), publish_command ),
    ])
  }
}

//

crate::mod_interface!
{
  prelude use commands_form;
}
