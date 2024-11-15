mod private
{

  use crate::*;
  use iter_tools::Itertools;
  use ca::aggregator::Order;
  use grammar::Dictionary;

  /// Enum representing the format options for generating help content.
  ///
  /// `HelpFormat` defines the output format of help content, enabling the choice
  /// between different styles, such as `Markdown` for structured text, or other
  /// custom formats.
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum HelpFormat
  {
    /// Generates help content in Markdown format, suitable for environments
    ///   that support Markdown rendering (e.g., documentation platforms, text editors).
    Markdown,
    /// Represents an alternative format, customizable for different needs.
    Another,
  }

  /// Generates Markdown-formatted help content based on a dictionary of terms and a specified order.
  ///
  /// The `md_generator` function takes a reference to a `Dictionary` and an `Order` to produce
  /// a help document in Markdown format. This function is useful for generating structured,
  /// readable help documentation suitable for Markdown-compatible platforms.
  pub fn md_generator( grammar : &Dictionary, order: Order ) -> String
  {
    let text = grammar.commands()
    .into_iter()
    .map( |( name, cmd )|
    {
      let subjects = cmd.subjects.iter().fold( String::new(), | _, _ | format!( " `[argument]`" ) );
      let properties = if cmd.properties.is_empty() { " " } else { " `[properties]` " };
      format!
      (
        "[.{}{subjects}{properties}](#{}{}{})",
        name,
        name.replace( '.', "" ),
        if cmd.subjects.is_empty() { "" } else { "-argument" },
        if cmd.properties.is_empty() { "" } else { "-properties" },
      )
    })
    .fold( String::new(), | acc, cmd |
    {
      format!( "{acc}\n- {cmd}" )
    });

    let list_of_commands = format!( "## Commands\n\n{}", text );

    let about_each_command = grammar.commands()
    .into_iter()
    .map( |( name, cmd )|
    {
      let subjects = cmd.subjects.iter().fold( String::new(), | _, _ | format!( " `[Subject]`" ) );
      let properties = if cmd.properties.is_empty() { " " } else { " `[properties]` " };
      let hint = if cmd.hint.is_empty() { &cmd.long_hint } else { &cmd.hint };

      let heading = format!( "## .{}{subjects}{properties}\n__{}__\n", name, hint );

      let hint = if cmd.long_hint.is_empty() { &cmd.hint } else { &cmd.long_hint };
      let full_subjects = cmd
      .subjects
      .iter()
      .enumerate()
      .map
      (
        |( number, subj )|
        format!( "\n- {}subject_{number} - {} `[{:?}]`", if subj.optional { "`< optional >` " } else { "" }, subj.hint, subj.kind )
      )
      .join( "\n" );
      let full_properties = cmd
      .properties( order )
      .into_iter()
      .map
      (
        |( name, value )|
        format!( "\n- {}{} - {} `[{:?}]`", if value.optional { "`< optional >` " } else { "" }, value.hint, name, value.kind )
      )
      .join( "\n" );
      // aaa : for Bohdan : toooooo log lines. 130 is max
      // aaa : done.

      format!
      (
        "{heading}\n{}{}\n\n{hint}\n",
        if cmd.subjects.is_empty() { "".to_string() } else { format!( "\n\nSubjects:{}", &full_subjects ) },
        if cmd.properties.is_empty() { "".to_string() } else { format!( "\n\nProperties:{}",&full_properties ) },
      )

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
  own use HelpFormat;
  own use md_generator;
}