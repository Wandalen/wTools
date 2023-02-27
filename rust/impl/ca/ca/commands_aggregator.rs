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

  use wtools::{ HashMap, Result };

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

    fn generate_help_routine( &self ) -> Routine
    {
      let text = format!( "{:#?}", self.grammar_converter );
      Routine::new( move | _ | { println!( "Help command\n{text}" ); Ok( () ) } )
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
