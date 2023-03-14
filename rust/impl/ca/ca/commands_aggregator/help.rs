pub( crate ) mod private
{
  use crate::{ GrammarConverter, Command };
  
  use wtools::Itertools;

  use std::rc::Rc;

  fn generate_help_content( grammar : &GrammarConverter, command : Option< &Command > ) -> String
  {
    if let Some( command ) = command
    {
      let name = &command.phrase;
      let hint = &command.long_hint;
      let subjects = command.subjects.iter().enumerate().fold( String::new(), | acc, ( number, subj ) | format!( "{acc} <subject_{number}:{:?}>", subj.kind ) );
      let full_subjects = command.subjects.iter().enumerate().map( |( number, subj )| format!( "subject_{number} - {} [{:?}]", subj.hint, subj.kind ) ).join( "\n\t\t" );
      let properties = if command.properties.is_empty() { " " } else { " <properties> " };
      let full_properties = command.properties.iter().sorted_by_key( |( name, _ )| *name ).map( |( name, value )| format!( "{name} - {} [{:?}]", value.hint, value.kind ) ).join( "\n\t\t" );

      format!( "{name}{subjects}{properties}- {hint}\n\tSubjects:\n\t\t{full_subjects}\n\tProperties:\n\t\t{full_properties}" )
    }
    else
    {
      grammar.commands
      .iter()
      .sorted_by_key( |( name, _ )| *name )
      .map( |( name, cmd )|
      {
        cmd.iter().fold( String::new(), | acc, cmd |
        {
          let subjects = cmd.subjects.iter().fold( String::new(), | acc, subj | format!( "{acc} <{:?}>", subj.kind ) );
          let properties = if cmd.properties.is_empty() { " " } else { " <properties> " };
          format!( "{acc}\n{name}{subjects}{properties}- {}", cmd.hint )
        })
      })
      .fold( String::new(), | acc, cmd |
      {
        format!( "{acc}\n{cmd}" )
      })
    }
  }

  /// Available help commands variants
  #[ derive( Debug, Hash, PartialEq, Eq ) ]
  pub enum HelpVariants
  {
    /// Make all available variants
    All,
    /// Help for whole program. E.g. `.help`
    General,
    /// Detailed help for one command as separete help command. E.g. `.help.command_name`
    DotCommand,
    /// Detailed help for one command as subject in help command. E.g. `.help command_name`
    SubjectCommand,
  }

  type HelpFucntionFn = Rc< dyn Fn( &GrammarConverter, Option< &Command > ) -> String >;

  /// Container for function that generates help string for any command
  #[ derive( Clone ) ]
  pub struct HelpGeneratorFn( HelpFucntionFn );

  impl Default for HelpGeneratorFn
  {
    fn default() -> Self
    {
      Self( Rc::new( generate_help_content ) )
    }
  }

  impl HelpGeneratorFn
  {
    /// Wrap a help fucntion
    pub fn new< HelpFunction >( func : HelpFunction ) -> Self
    where
      HelpFunction : Fn( &GrammarConverter, Option< &Command > ) -> String + 'static
    {
        Self( Rc::new( func ) )
    }
  }

  impl HelpGeneratorFn
  {
    /// Executes the function
    pub fn exec( &self, grammar : &GrammarConverter, command : Option< &Command > ) -> String
    {
      self.0( grammar, command )
    }
  }

  impl wtools::fmt::Debug for HelpGeneratorFn
  {
    fn fmt( &self, f : &mut wtools::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f.write_str( "HelpGenerator" )
    }
  }
}

//

crate::mod_interface!
{
  protected use HelpGeneratorFn;
  prelude use HelpVariants;
}
