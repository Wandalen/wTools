pub( crate ) mod private
{
  use crate::
  {
    Parser, GrammarConverter, ExecutorConverter,
    Executor,

    ProgramParser,

    Command,
    Routine,
    commands_aggregator::help::{ HelpGeneratorFn, HelpVariants, dot_command },
  };

  use wtools::{ HashMap, Result, HashSet };

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
    #[ default( HashSet::from([ HelpVariants::All ]) ) ]
    help_variants : HashSet< HelpVariants >,
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

    pub fn build( self ) -> CommandsAggregator
    {
      let mut ca = self.form();

      if ca.help_variants.contains( &HelpVariants::All )
      {
        HelpVariants::All.generate( &ca.help_generator, &mut ca.grammar_converter, &mut ca.executor_converter );
      }
      else
      {
        for help in &ca.help_variants
        {
          help.generate( &ca.help_generator, &mut ca.grammar_converter, &mut ca.executor_converter );
        }
      }

      dot_command( &mut ca.grammar_converter, &mut ca.executor_converter );

      ca
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
