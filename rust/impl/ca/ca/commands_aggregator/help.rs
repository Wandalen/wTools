pub( crate ) mod private
{
  use crate::
  {
    GrammarConverter, ExecutorConverter,

    Command,
    Routine, Type
  };
  
  use wtools::Itertools;

  use std::rc::Rc;

  /// Generate `dot` command
  pub fn dot_command( grammar : &mut GrammarConverter, executor : &mut ExecutorConverter )
  {
      let empty = Command::former()
      .hint( "prints all available commands" )
      .phrase( "" )
      .property( "command_prefix", "", Type::String )
      .form();

      let to_command = Command::former()
      .hint( "prints all available commands that starts with" )
      .phrase( "" )
      .subject( "command name", Type::String )
      .property( "command_prefix", "", Type::String )
      .form();

      let command_variants = grammar.commands.entry( "".to_string() ).or_insert_with( Vec::new );
      *command_variants = vec![ empty, to_command ];

      let mut available_commands = grammar.commands.keys().cloned().collect::< Vec< _ > >();
      available_commands.sort();

      let routine = Routine::new
      (
        move |( args, props )|
        {
          let prefix : String = props.get_owned( "command_prefix" ).unwrap();
          if let Some( command ) = args.get_owned::< String >( 0 )
          {
            let ac = available_commands
            .iter()
            .filter( | cmd | cmd.starts_with( &command ) )
            .map( | cmd | format!( "{prefix}{cmd}" ) )
            .collect::< Vec< _ > >();

            if ac.is_empty()
            {
              eprintln!( "Have no commands that starts with `{prefix}{command}`" );
            }
            else
            {
              println!( "{}", ac.join( "\n" ) );
            }
          }
          else
          {
            println!( "{}", available_commands.iter().map( | cmd | format!( "{prefix}{cmd}" ) ).join( "\n" ) );
          };

          Ok( () )
        }
      );

      executor.routines.insert( "".to_string(), routine );
  }

  fn generate_help_content( grammar : &GrammarConverter, command : Option< &Command > ) -> String
  {
    if let Some( command ) = command
    {
      let name = &command.phrase;
      let hint = if command.long_hint.is_empty() { &command.hint } else { &command.long_hint };
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
          let hint = if cmd.hint.is_empty() { &cmd.long_hint } else { &cmd.hint };

          format!( "{acc}\n{name}{subjects}{properties}- {hint}" )
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
    /// Detailed help for one command as subject in help command. E.g. `.help command_name`
    SubjectCommand,
    /// Detailed help for one command as separete help command. E.g. `.help.command_name`
    DotCommand,
  }

  impl HelpVariants
  {
    /// Generates help commands
    pub fn generate( &self, helper : &HelpGeneratorFn, grammar : &mut GrammarConverter, executor : &mut ExecutorConverter )
    {
      match self
      {
        HelpVariants::All =>
        {
          self.general_help( helper, grammar, executor );
          self.subject_command_help( helper, grammar, executor );
          self.dot_command_help( helper, grammar, executor );
        },
        HelpVariants::General => self.general_help( helper, grammar, executor ),
        HelpVariants::SubjectCommand => self.subject_command_help( helper, grammar, executor ),
        HelpVariants::DotCommand => self.dot_command_help( helper, grammar, executor ),
      }
    }

    // .help
    fn general_help( &self, helper : &HelpGeneratorFn, grammar : &mut GrammarConverter, executor : &mut ExecutorConverter )
    {
      let phrase = "help".to_string();

      let help = Command::former()
      .hint( "prints information about existing commands" )
      .phrase( &phrase )
      .form();

      let command_variants = grammar.commands.entry( phrase.to_owned() ).or_insert_with( Vec::new );
      command_variants.push( help );

      // generate and add routine of help command
      // replace old help command with new one
      let subject_help = executor.routines.remove( &phrase );
      let text = helper.exec( grammar, None );
      let routine = Routine::new
      (
        move |( args, props )|
        {
          match &subject_help
          {
            Some( Routine::WithoutContext( help ) ) if !args.is_empty() => help(( args, props ))?,
            _ => println!( "Help command\n{text}" ),
          }

          Ok( () )
        }
      );

      executor.routines.insert( phrase, routine );
    }

    // .help command_name
    fn subject_command_help( &self, helper : &HelpGeneratorFn, grammar : &mut GrammarConverter, executor : &mut ExecutorConverter )
    {
      let phrase = "help".to_string();

      // generate and add grammar of help command
      let help = Command::former()
      .hint( "prints full information about a specified command" )
      .phrase( &phrase )
      .subject( "command name", Type::String )
      .form();

      let command_variants = grammar.commands.entry( phrase.to_owned() ).or_insert_with( Vec::new );
      command_variants.push( help );

      // generate and add routine of help command
      // replace old help command with new one
      let full_help = executor.routines.remove( &phrase );
      // TODO: Fix it somehow( Cloning grammar and helper )
      let grammar = grammar.clone();
      let generator = helper.clone();
      let routine = Routine::new
      (
        move |( args, props )|
        {
          match &full_help
          {
            Some( Routine::WithoutContext( help ) ) if args.is_empty() => help(( args, props ))?,
            _ =>
            {
              let command = args.get_owned::< String >( 0 ).unwrap();
              let cmds = grammar.commands.get( &command ).unwrap_or_else( || panic!( "Command `{command}` not found" ) );

              let text = cmds.iter().map
              (
                | cmd |
                generator.exec( &grammar, Some( cmd ) )
              )
              .join( "\n\n" );

              println!( "{text}" );
            }
          };

          Ok( () )
        }
      );

      executor.routines.insert( phrase, routine );
    }

    // .help.command_name
    fn dot_command_help( &self, helper : &HelpGeneratorFn, grammar : &mut GrammarConverter, executor : &mut ExecutorConverter )
    {
      // generate commands names
      let commands : Vec< _ > = grammar.commands.iter().map( |( name, cmd )| ( format!( "help.{name}" ), cmd.clone() ) ).collect();

      // generate Commands grammar
      let grammar_helps = commands
      .iter()
      .map( |( help_name, _ )| Command::former().hint( "prints full information about a specified command" ).phrase( help_name ).form() )
      .collect::< Vec< _ > >();

      // add commands to GrammarConverter
      for cmd in grammar_helps
      {
        let command_variants = grammar.commands.entry( cmd.phrase.to_owned() ).or_insert_with( Vec::new );
        command_variants.push( cmd );
      }

      // generate Commands routines
      let executable = commands
      .into_iter()
      .fold( vec![], | mut acc, ( help_name, cmds ) |
      {
        let text = cmds.iter()
        .map
        (
          | cmd | helper.exec( grammar, Some( cmd ) )
        )
        .join( "\n\n" );

        // TODO: compile time or binary size?
        let routine = Routine::new( move | _ |
        {
          println!( "Help for command\n\n{text}" );

          Ok( () )
        });
        acc.push(( help_name, routine ));

        acc
      });

      // add commands to ExecutorConverter
      for ( phrase, routine ) in executable
      {
        executor.routines.insert( phrase, routine );
      }
    }
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
  protected use dot_command;
  prelude use HelpVariants;
}
