pub( crate ) mod private
{

  use crate::GrammarConverter;
  use iter_tools::Itertools;

  /// -
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum HelpFormat
  {
    Markdown,
    Another,
  }

  pub fn md_generator( grammar : &GrammarConverter ) -> String
  {
    let text = grammar.commands
    .iter()
    .sorted_by_key( |( name, _ )| *name )
    .map( |( name, cmd )|
    {
      cmd.iter().fold( String::new(), | _, cmd |
      {

        let subjects = cmd.subjects.iter().fold( String::new(), | _, _ | format!( " `[Subject]`" ) );
        let properties = if cmd.properties.is_empty() { " " } else { " `[properties]` " };


        format!( "[.{name}{subjects}{properties}](#{}{}{})", name.replace( '.', "" ), if cmd.subjects.is_empty() { "" } else { "-subject" }, if cmd.properties.is_empty() { "" } else { "-properties" } )
      })
    })
    .fold( String::new(), | acc, cmd |
    {
      format!( "{acc}\n- {cmd}" )
    });

    let list_of_commands = format!( "## Commands\n\n{}", text );

    let about_each_command = grammar.commands
    .iter()
    .sorted_by_key( |( name, _ )| *name )
    .map( |( name, cmd )|
    {
      cmd.iter().fold( String::new(), | _, cmd |
      {
        let subjects = cmd.subjects.iter().fold( String::new(), | _, _ | format!( " `[Subject]`" ) );
        let properties = if cmd.properties.is_empty() { " " } else { " `[properties]` " };
        let hint = if cmd.hint.is_empty() { &cmd.long_hint } else { &cmd.hint };

        let heading = format!( "## .{name}{subjects}{properties}\n__{}__\n", hint );

        let hint = if cmd.long_hint.is_empty() { &cmd.hint } else { &cmd.long_hint };
        let full_subjects = cmd.subjects.iter().enumerate().map( |( number, subj )| format!( "\n- {}subject_{number} - {} `[{:?}]`", if subj.optional { "`<optional>` " } else { "" }, subj.hint, subj.kind ) ).join( "\n" );
        let full_properties = cmd.properties.iter().sorted_by_key( |( name, _ )| *name ).map( |( name, value )| format!( "\n- {}{name} - {} `[{:?}]`", if value.optional { "`<optional>` " } else { "" }, value.hint, value.kind ) ).join( "\n" );

        format!
        ( 
          "{heading}\n{}{}\n\n{hint}\n", 
          if cmd.subjects.is_empty() { "".to_string() } else { format!( "\n\nSubjects:{}", &full_subjects ) }, 
          if cmd.properties.is_empty() { "".to_string() } else { format!( "\n\nProperties:{}",&full_properties ) }
        )

      })
    })
    .fold( String::new(), | acc, cmd |
    {
      format!( "{acc}\n\n{cmd}" )
    });
    format!( "{list_of_commands}\n{about_each_command}" )
  }



}

crate::mod_interface!
{
  
}