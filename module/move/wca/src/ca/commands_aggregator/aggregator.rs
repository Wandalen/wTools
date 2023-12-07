pub( crate ) mod private
{
  use crate::
  { 
    ca::
    {
      Parser, GrammarConverter, ExecutorConverter,
      Executor,

      ProgramParser,

      Command,
      Routine,
      commands_aggregator::help::{ HelpGeneratorFn, HelpVariants, dot_command },
    }, 
    wtools 
  };
  use thiserror::Error;

  use std::collections::{ HashMap, HashSet };
  use wtools::error::
  { 
    Result, 
    for_app::Error as wError 
  };

  /// Validation errors that can occur in application.
  #[ derive( Error, Debug ) ]
  pub enum ValidationError 
  {
    /// This variant is used to represent parser errors. 
    /// It carries a `String` payload that provides additional information about the error.
    #[ error( "Parser error.\nCause:\n{0}" ) ]
    Parser( wError ),
    /// This variant represents errors that occur during grammar conversion.
    #[ error( "Grammar converter error.\nCause:\n{0}" ) ]
    GrammarConverter( wError ),
    /// This variant is used to represent errors that occur during executor conversion.
    #[ error( "Executor converter error.\nCause:\n{0}" ) ]
    ExecutorConverter( wError ),
  }

  /// Errors that can occur in application.
  #[ derive( Error, Debug ) ]
  pub enum Error 
  {
    /// This variant is used to represent validation errors. 
    /// It carries a `ValidationError` payload that provides additional information about the error.
    #[ error( "Validation error:\n{0}" ) ]
    Validation( ValidationError ),
    /// This variant represents execution errors.
    #[ error( "Execution error.\nCause:\n{0}" ) ]
    Execution( wError ),
  }

  /// The `CommandsAggregator` struct is responsible for aggregating all commands that the user defines,
  /// and for parsing and executing them. It is the main entry point of the library.
  ///
  /// CommandsAggregator component brings everything together. This component is responsible for configuring the `Parser`, `Grammar`, and `Executor` components based on the user’s needs. It also manages the entire pipeline of processing, from parsing the raw text input to executing the final command(parse -> validate -> execute).
  ///
  /// # Example:
  ///
  /// ```
  /// use wca::prelude::*;
  ///
  /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
  /// let ca = CommandsAggregator::former()
  /// .grammar(
  /// [
  ///   Command::former()
  ///   .phrase( "echo" )
  ///   .hint( "prints all subjects and properties" )
  ///   .subject( "Subject", Type::String, false )
  ///   .property( "property", "simple property", Type::String, false )
  ///   .form(),
  /// ])
  /// .executor(
  /// [
  ///   ( "echo".to_owned(), Routine::new( |( args, props )|
  ///   {
  ///     println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" );
  ///     Ok( () )
  ///   })),
  /// ])
  /// .build();
  ///
  /// ca.perform( ".echo something" )?;
  /// # Ok( () ) }
  /// ```
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
    #[ default( GrammarConverter::former().form() ) ]
    grammar_converter : GrammarConverter,
    #[ default( ExecutorConverter::former().form() ) ]
    executor_converter : ExecutorConverter,
  }

  impl CommandsAggregatorFormer
  {
    /// Setter for grammar
    ///
    /// Gets list of available commands
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

    /// Setter for executor
    ///
    /// Gets dictionary of routines( command name -> callback )
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

    /// Setter for help content generator
    ///
    /// ```
    /// use wca::prelude::*;
    ///
    /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
    /// let ca = CommandsAggregator::former()
    /// // ...
    /// .help( | grammar, command | format!( "Replaced help content" ) )
    /// .build();
    ///
    /// ca.perform( ".help" )?;
    /// # Ok( () ) }
    /// ```
    pub fn help< HelpFunction >( mut self, func : HelpFunction ) -> Self
    where
      HelpFunction : Fn( &GrammarConverter, Option< &Command > ) -> String + 'static
    {
      self.help_generator = Some( HelpGeneratorFn::new( func ) );
      self
    }

    /// Construct CommandsAggregator
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
    ///
    /// Takes a string with program and executes it
    pub fn perform< S >( &self, program : S ) -> Result< (), Error >
    where
      S : AsRef< str >
    {
      let raw_program = self.parser.program( program.as_ref() ).map_err( | e | Error::Validation( ValidationError::Parser( e ) ) )?;
      let grammar_program = self.grammar_converter.to_program( raw_program ).map_err( | e | Error::Validation( ValidationError::GrammarConverter( e ) ) )?;
      let exec_program = self.executor_converter.to_program( grammar_program ).map_err( | e | Error::Validation( ValidationError::ExecutorConverter( e ) ) )?;

      self.executor.program( exec_program ).map_err( | e | Error::Execution( e ) )
    }
  }
}

//

crate::mod_interface!
{
  prelude use CommandsAggregator;
  prelude use Error;
  prelude use ValidationError;
}
