pub( crate ) mod private
{
  use crate::
  {
    Parser, GrammarConverter, ExecutorConverter,
    Executor,

    ProgramParser,

    Command,
    Routine, Type,
    commands_aggregator::help::{HelpGeneratorFn, private::HelpVariants},
  };

  use wtools::{ HashMap, Result, Itertools, HashSet };

  /// CommandsAggragator
  #[ derive( Debug ) ] 
  #[ derive( former::Former ) ]
  pub struct CommandsAggregator
  {
    #[ default( Parser::former().form() ) ]
    parser : Parser,
    #[ setter( false ) ]
    #[ default( Executor::former().form() ) ]
    executor : Executor,
    help_generator : HelpGeneratorFn,
    // #[ default( HashSet::from([ HelpVariants::All ]) ) ]
    // pub help_variants : HashSet< HelpVariants >,
    grammar_converter : GrammarConverter,
    executor_converter : ExecutorConverter,
  }

  impl CommandsAggregatorFormer
  {
    pub fn grammar< V >( mut self, commands : V ) -> Self
    where
      V : Into< Vec< Command > >
    {
      let grammar = GrammarConverter::former()
      .commands( commands )
      .form();

      self.grammar_converter = Some( grammar );
      self
    }

    pub fn executor< H >( mut self, routines : H ) -> Self
    where
      H : Into< HashMap< String, Routine > >
    {
      let executor = ExecutorConverter::former()
      .routines( routines )
      .form();

      self.executor_converter = Some( executor );
      self
    }

    pub fn help< HelpFunction >( mut self, func : HelpFunction ) -> Self
    where
      HelpFunction : Fn( &GrammarConverter, Option< &Command > ) -> String + 'static
    {
      self.help_generator = Some( HelpGeneratorFn::new( func ) );
      self
    }

    pub fn help_variants< V >( mut self, variants : V ) -> Self
    where
      V : Into< HashSet< HelpVariants > >,
    {
      let variants = variants.into();
      if variants.contains( &HelpVariants::All ) || variants.contains( &HelpVariants::SubjectCommand )
      {
        self = self.with_detailed_help_from_subject();
      }
      if variants.contains( &HelpVariants::All ) || variants.contains( &HelpVariants::General )
      {
        self = self.with_help_command();
      }
      if variants.contains( &HelpVariants::All ) || variants.contains( &HelpVariants::DotCommand )
      {
        self = self.with_detailed_help_commands();
      }

      self
    }
  }

  // Help commands
  impl CommandsAggregatorFormer
  {
    fn generate_help_routine( &self ) -> Routine
    {
      let text = if let Some( grammar ) = &self.grammar_converter
      {
        self.help_generator.clone().unwrap_or_default().exec( grammar, None )
      }
      else
      {
        "Has no commands".into()
      };

      Routine::new( move | _ | { println!( "Help command\n{text}" ); Ok( () ) } )
    }

    fn generate_detailed_help_routine( &self, cmds : &[ Command ] ) -> Routine
    {
      let text = if let Some( grammar ) = &self.grammar_converter
      {
        cmds.iter().map
        (
          | cmd |
          self.help_generator.clone().unwrap_or_default().exec( grammar, Some( cmd ) )
        )
        .join( "\n\n" )
      }
      else
      {
        "Has no commands".into()
      };
      // TODO: compile time or binary size?
      Routine::new( move | _ |
      {
        println!( "Help for command\n\n{text}" );

        Ok( () )
      })
    }

    /// help for whole program
    // it must be a flag that would generate a command on `form`
    fn with_help_command( mut self ) -> Self
    {
      let help = Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "help" )
      .form();

      let phrase = help.phrase.clone();

      let mut grammar = self.grammar_converter.unwrap_or_else( || GrammarConverter::former().form() );
      let command_variants = grammar.commands.entry( help.phrase.to_owned() ).or_insert_with( Vec::new );
      command_variants.push( help );
      self.grammar_converter = Some( grammar );

      let routine = self.generate_help_routine();
      let mut executor = self.executor_converter.unwrap_or_else( || ExecutorConverter::former().form() );
      executor.routines.insert( phrase, routine );
      self.executor_converter = Some( executor );

      self
    }

    fn with_detailed_help_from_subject( mut self ) -> Self
    {
      let phrase = "help".to_string();

      // generate and add grammar of help command
      let mut grammar = self.grammar_converter.unwrap_or_else( || GrammarConverter::former().form() );
      let command_variant = Command::former().hint( "help" ).long_hint( "" ).phrase( &phrase ).subject( "command name", Type::String ).form();
      let command_variants = grammar.commands.entry( command_variant.phrase.to_owned() ).or_insert_with( Vec::new );
      command_variants.push( command_variant );
      self.grammar_converter = Some( grammar );

      // generate and add routine of help command
      let mut executor = self.executor_converter.unwrap_or_else( || ExecutorConverter::former().form() );
      // replace old help command with new one
      let full_help = executor.routines.remove( &phrase );
      // TODO: Fix it
      let grammar = self.grammar_converter.clone().unwrap();
      let generator = self.help_generator.clone().unwrap_or_default();
      executor.routines.insert( phrase.clone(), Routine::new
      (
        move |( args, props )|
        {
          match &full_help
          {
            Some( Routine::WithoutContext( help ) ) if args.is_empty() => help(( args, props ))?,
            _ =>
            {
              let command : String = args.get( 0 ).unwrap().to_owned().into();
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
      ));
      self.executor_converter = Some( executor );

      self
    }

    fn with_detailed_help_commands( mut self ) -> Self
    {
      // generate commands names
      let commands : Vec< _ > = self.grammar_converter.as_ref()
      .map
      (
        | grammar |
        grammar.commands.iter().map( |( name, cmd )| ( format!( "help.{name}" ), cmd.clone() ) ).collect()
      )
      .unwrap_or_default();

      // generate Commands grammar
      let grammar_helps = commands
      .iter()
      .map( |( help_name, _ )| Command::former().hint( "help" ).long_hint( "" ).phrase( help_name ).form() )
      .collect::< Vec< _ > >();

      // add commands to GrammarConverter
      let mut grammar = self.grammar_converter.unwrap_or_else( || GrammarConverter::former().form() );
      for cmd in grammar_helps
      {
        let command_variants = grammar.commands.entry( cmd.phrase.to_owned() ).or_insert_with( Vec::new );
        command_variants.push( cmd );
      }
      self.grammar_converter = Some( grammar );

      // generate Commands routines
      let executable = commands
      .into_iter()
      .fold( vec![], | mut acc, ( help_name, cmd ) |
      {
        let routine = self.generate_detailed_help_routine( &cmd );
        acc.push(( help_name, routine ));
        acc
      });

      // add commands to ExecutorConverter
      let mut executor = self.executor_converter.unwrap_or_else( || ExecutorConverter::former().form() );

      for ( phrase, routine ) in executable
      {
        executor.routines.insert( phrase, routine );
      }

      self.executor_converter = Some( executor );

      self
    }
  }

  impl CommandsAggregator
  {
    /// Parse, converts and executes a program
    pub fn perform< S >( &self, program : S ) -> Result< () >
    where
      S : AsRef< str >
    {
      let raw_program = self.parser.program( program.as_ref() )?;
      let grammar_program = self.grammar_converter.to_program( raw_program )?;
      let exec_program = self.executor_converter.to_program( grammar_program )?;

      self.executor.program( exec_program )
    }
  }
}

//

crate::mod_interface!
{
  prelude use CommandsAggregator;
}
