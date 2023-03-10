pub( crate ) mod private
{
  use crate::
  {
    Parser, GrammarConverter, ExecutorConverter,
    Executor,

    ProgramParser,

    Command,
    Routine,
  };

  use wtools::{ HashMap, Result, Itertools };

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

    fn generate_help_content( &self ) -> String
    {
      if let Some( grammar ) = &self.grammar_converter
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
      else
      {
        "Has no commands".to_owned()
      }
    }

    fn generate_help_routine( &self ) -> Routine
    {
      let text = self.generate_help_content();
      Routine::new( move | _ | { println!( "Help command\n{text}" ); Ok( () ) } )
    }

    fn generate_detailed_help_routine( &self, cmds : &Vec< Command > ) -> Routine
    {
      let text = cmds
      .iter()
      .map
      (
        | cmd |
        {
          let name = cmd.phrase.to_owned();
          let hint = cmd.long_hint.to_owned();
          let subjects = cmd.subjects.iter().enumerate().fold( String::new(), | acc, ( number, subj ) | format!( "{acc} <subject_{number}:{:?}>", subj.kind ) );
          let full_subjects = cmd.subjects.iter().enumerate().map( |( number, subj )| format!( "subject_{number} - {} [{:?}]", subj.hint, subj.kind ) ).join( "\n\t\t" );
          let properties = if cmd.properties.is_empty() { " " } else { " <properties> " };
          let full_properties = cmd.properties.iter().sorted_by_key( |( name, _ )| *name ).map( |( name, value )| format!( "{name} - {} [{:?}]", value.hint, value.kind ) ).join( "\n\t\t" );

          format!( "{name}{subjects}{properties}- {hint}\n\tSubjects:\n\t\t{full_subjects}\n\tProperties:\n\t\t{full_properties}" )
        }
      )
      .join( "\n\n" );
      Routine::new( move | _ |
      {
        println!( "Help for command\n\n{text}" );

        Ok( () )
      })
    }

    /// help for whole program
    // it must be a flag that would generate a command on `form`
    pub fn with_help_command( mut self ) -> Self
    {
      let help = Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "help" )
      .form();

      let grammar_help = help.clone();
      let grammar = if let Some( mut grammar ) = self.grammar_converter
      {
        let command_variants = grammar.commands.entry( grammar_help.phrase.to_owned() ).or_insert_with( Vec::new );
        command_variants.push( grammar_help );
        grammar
      }
      else
      {
        GrammarConverter::former()
        .command( grammar_help )
        .form()
      };
      self.grammar_converter = Some( grammar );

      let phrase = help.phrase;
      let routine = self.generate_help_routine();
      let executor = if let Some( mut executor ) = self.executor_converter
      {
        executor.routines.insert( phrase, routine );
        executor
      }
      else
      {
        ExecutorConverter::former()
        .routine( phrase, routine )
        .form()
      };
      self.executor_converter = Some( executor );

      self
    }

    pub fn with_detailed_help_commands( mut self ) -> Self
    {
      let commands : Vec< _ > = self.grammar_converter.as_ref()
      .map
      (
        | grammar |
        grammar.commands.iter().map( |( name, cmd )| ( format!( "help.{name}" ), cmd.clone() ) ).collect()
      )
      .unwrap_or_default();

      let grammar_helps = commands
      .iter()
      .map( |( help_name, _ )| Command::former().hint( "help" ).long_hint( "" ).phrase( help_name ).form() )
      .collect::< Vec< _ > >();

      let grammar = if let Some( mut grammar ) = self.grammar_converter
      {
        for cmd in grammar_helps
        {
          let command_variants = grammar.commands.entry( cmd.phrase.to_owned() ).or_insert_with( Vec::new );
          command_variants.push( cmd );
        }
        grammar
      }
      else
      {
        GrammarConverter::former().form()
      };
      self.grammar_converter = Some( grammar );

      let executable = commands
      .into_iter()
      .fold( vec![], | mut acc, ( help_name, cmd ) |
      {
        let routine = self.generate_detailed_help_routine( &cmd );
        acc.push(( help_name, routine ));
        acc
      });
      let executor = if let Some( mut executor ) = self.executor_converter
      {
        for ( phrase, routine ) in executable
        {
          executor.routines.insert( phrase, routine );
        }
        executor
      }
      else
      {
        ExecutorConverter::former().form()
      };
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
