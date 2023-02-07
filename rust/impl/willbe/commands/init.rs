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
    .routine_with_ctx( crate::commands::info::info )
    .form();

    let each_command = wca::Command::former()
    .hint( "--- each ---" )
    .long_hint( "--- each ---" )
    .phrase( "each" )
    .property_hint( "depth", "____" )
    .routine_with_ctx( crate::commands::each::each )
    .form();

    let dep_command = wca::Command::former()
    .hint( "--- dep ---" )
    .long_hint( "--- dep ---" )
    .phrase( "dep" )
    .subject_hint( "What to do" )
    .property_hint( "dry", "____" )
    .routine_with_ctx( crate::commands::dep::dep )
    .form();

    let publish_command = wca::Command::former()
    .hint( "--- publish ---" )
    .long_hint( "--- publish ---" )
    .phrase( "crate.publish" )
    .routine_with_ctx( crate::commands::publish::publish )
    .form();

    let end_command = wca::Command::former()
    .phrase( "end" )
    .routine_with_ctx( crate::commands::end::end )
    .form();

    [ info_command, each_command, publish_command, dep_command, end_command ]
    .into_iter()
    .map( | command | ( format!( ".{}", command.phrase ), command ) )
    .collect::< std::collections::HashMap< String, wca::Command > >()
  }
}

//

crate::mod_interface!
{
  prelude use commands_form;
}
