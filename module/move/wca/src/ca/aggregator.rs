pub( crate ) mod private
{
  use crate::*;
  use ca::
  {
    Parser, Verifier, ExecutorConverter,
    Executor,
    ProgramParser,
    Command,
    Routine,
    help::{ HelpGeneratorFn, HelpVariants, dot_command },
  };

  use std::collections::{ HashMap, HashSet };
  use std::fmt;
  use std::fmt::Formatter;
  use std::rc::Rc;
  use wtools::thiserror;
  use wtools::error::
  {
    Result,
    for_app::Error as wError,
    for_lib::*,
  };

  /// Validation errors that can occur in application.
  #[ derive( Error, Debug ) ]
  pub enum ValidationError
  {
    /// This variant is used to represent parser errors.
    /// It carries a `String` payload that provides additional information about the error.
    #[ error( "The following input is not recognized: `{input}`.\nDetails: {error}" ) ]
    Parser
    {
      /// source of the program
      input : String,
      /// original error
      error : wError,
    },
    /// This variant represents errors that occur during grammar conversion.
    #[ error( "Can not identify a command.\nDetails: {0}" ) ]
    Verifier( wError ),
    /// This variant is used to represent errors that occur during executor conversion.
    #[ error( "Can not find a routine for a command.\nDetails: {0}" ) ]
    ExecutorConverter( wError ),
  }

  /// Errors that can occur in application.
  #[ derive( Error, Debug ) ]
  pub enum Error
  {
    /// This variant is used to represent validation errors.
    /// It carries a `ValidationError` payload that provides additional information about the error.
    #[ error( "Validation error. {0}" ) ]
    Validation( ValidationError ),
    /// This variant represents execution errors.
    #[ error( "Execution failed. {0:?}" ) ]
    Execution( wError ),
  }

  // xxx : qqq : qqq2 : for Bohdan : one level is obviously redundant
  // Program< Namespace< ExecutableCommand_ > > -> Program< ExecutableCommand_ >
  struct CommandsAggregatorCallback( Box< dyn Fn( &str, &Program< Namespace< ExecutableCommand_ > > ) > );

  impl fmt::Debug for CommandsAggregatorCallback
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f.debug_struct( "CommandsAggregatorCallback" ).finish_non_exhaustive()
    }
  }

  /// The `CommandsAggregator` struct is responsible for aggregating all commands that the user defines,
  /// and for parsing and executing them. It is the main entry point of the library.
  ///
  /// CommandsAggregator component brings everything together. This component is responsible for configuring the `Parser`, `Grammar`, and `Executor` components based on the user’s needs. It also manages the entire pipeline of processing, from parsing the raw text input to executing the final command(parse -> validate -> execute).
  ///
  /// # Example:
  ///
  /// ```
  /// use wca::{ CommandsAggregator, Command, Routine, Type };
  ///
  /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
  /// let ca = CommandsAggregator::former()
  /// .grammar(
  /// [
  ///   Command::former()
  ///   .phrase( "echo" )
  ///   .hint( "prints all subjects and properties" )
  ///   .subject( "argument", Type::String, false )
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
    // qqq : for Bohdan : should not have fields help_generator and help_variants
    // help_generator generateds VerifiedCommand(s) and stop to exist

    #[ default( Verifier::former().form() ) ]
    verifier : Verifier,

    #[ default( ExecutorConverter::former().form() ) ]
    executor_converter : ExecutorConverter,

    callback_fn : Option< CommandsAggregatorCallback >,
  }

  impl CommandsAggregator
  {
    /// Create a new instance of `CommandsAggregatorFluentBuilder`.
    ///
    /// This method ensures a fluent interface to build a `CommandsAggregator`.
    ///
    /// # Returns
    ///
    /// Returns a `CommandsAggregatorFluentBuilder` instance.
    /// ```
    /// use wca::{ Args, Context };
    ///
    /// let ca = wca::CommandsAggregator::fluent()
    /// .command( "echo" )
    ///   .hint( "prints all subjects and properties" )
    ///   .subject( "Subject", wca::Type::String, true )
    ///   .property( "property", "simple property", wca::Type::String, true )
    ///   .routine( | args : Args, props | { println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" ) } )
    ///   .perform()
    /// .command( "inc" )
    ///   .hint( "This command increments a state number each time it is called consecutively. (E.g. `.inc .inc`)" )
    ///   .routine( | ctx : Context | { let i : &mut i32 = ctx.get_or_default(); println!( "i = {i}" ); *i += 1; } )
    ///   .perform()
    /// .command( "error" )
    ///   .hint( "prints all subjects and properties" )
    ///   .subject( "Error message", wca::Type::String, true )
    ///   .routine( | args : Args | { println!( "Returns an error" ); Err( format!( "{}", args.get_owned::< String >( 0 ).unwrap_or_default() ) ) } )
    ///   .perform()
    /// .command( "exit" )
    ///   .hint( "just exit" )
    ///   .routine( || { println!( "exit" ); std::process::exit( 0 ) } )
    ///   .perform()
    /// .perform();
    /// ```
    pub fn fluent() -> CommandsAggregatorFluentBuilder
    {
      CommandsAggregatorFluentBuilder( Self::former().form() )
    }
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
      let verifier = Verifier::former()
      .commands( commands )
      .form();
      self.container.verifier = Some( verifier );
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

      self.container.executor_converter = Some( executor );
      self
    }

    /// Setter for help content generator
    ///
    /// ```
    /// use wca::CommandsAggregator;
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
      HelpFunction : Fn( &Verifier, Option< &Command > ) -> String + 'static
    {
      self.container.help_generator = Some( HelpGeneratorFn::new( func ) );
      self
    }
    // qqq : it is good access method, but formed structure should not have help_generator anymore

    /// Set callback function that will be executed after validation state
    ///
    /// ```
    /// use wca::CommandsAggregator;
    ///
    /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
    /// let ca = CommandsAggregator::former()
    /// // ...
    /// .callback( | _input, _program | println!( "Program is valid" ) )
    /// .build();
    ///
    /// // prints the "Program is valid" and after executes the program
    /// ca.perform( ".help" )?;
    /// # Ok( () ) }
    /// ```
    pub fn callback< Callback >( mut self, callback : Callback ) -> Self
    where
      Callback : Fn( &str, &Program< Namespace< ExecutableCommand_ > > ) + 'static,
    {
      self.container.callback_fn = Some( CommandsAggregatorCallback( Box::new( callback ) ) );
      self
    }

    /// Construct CommandsAggregator
    pub fn build( self ) -> CommandsAggregator
    {
      let mut ca = self.form();

      if ca.help_variants.contains( &HelpVariants::All )
      {
        HelpVariants::All.generate( &ca.help_generator, &mut ca.verifier, &mut ca.executor_converter );
      }
      else
      {
        for help in &ca.help_variants
        {
          help.generate( &ca.help_generator, &mut ca.verifier, &mut ca.executor_converter );
        }
      }

      dot_command( &mut ca.verifier, &mut ca.executor_converter );

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
      S : IntoInput
    {
      let Input( ref program ) = program.into_input();

      let raw_program = self.parser.program( program ).map_err( | e | Error::Validation( ValidationError::Parser { input : program.to_string(), error : e } ) )?;
      let grammar_program = self.verifier.to_program( raw_program ).map_err( | e | Error::Validation( ValidationError::Verifier( e ) ) )?;
      let exec_program = self.executor_converter.to_program( grammar_program ).map_err( | e | Error::Validation( ValidationError::ExecutorConverter( e ) ) )?;

      if let Some( callback ) = &self.callback_fn
      {
        callback.0( program, &exec_program )
      }

      self.executor.program( exec_program ).map_err( | e | Error::Execution( e ) )
    }
  }

  /// The `CommandsAggregatorFluentBuilder` struct is a builder for creating instances of the `CommandsAggregator` struct using a fluent interface.
  ///
  /// It allows for chaining multiple configuration methods together to customize the `CommandsAggregator` instance before building it.
  #[ derive( Debug ) ]
  pub struct CommandsAggregatorFluentBuilder( CommandsAggregator );

  impl CommandsAggregatorFluentBuilder
  {
    pub fn command< P : Into< String > >( self, phrase : P ) -> CommandHandler
    {
      CommandHandler
      {
        ca : self.0,
        grammar : Command::former().phrase( phrase ),
        routine : Routine::WithoutContext( Rc::new( | _ | { panic!( "No routine available: A handler function for the command is missing" ) } ) )
      }
    }

    pub fn perform( self ) -> CommandsAggregator
    {
      self.0
    }
  }

  // qqq: rename
  pub struct CommandHandler
  {
    ca : CommandsAggregator,
    grammar : ca::grammar::CommandFormer,
    routine : Routine,
  }

  impl std::fmt::Debug for CommandHandler
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> fmt::Result
    {
      f
      .debug_struct( "CommandHandler" )
      .field( "ca", &( self.ca ) )
      .field( "grammar", &"" )
      .finish()
    }
  }


  impl CommandHandler
  {
    pub fn hint< H : Into< String > >( mut self, hint : H ) -> Self
    {
      self.grammar = self.grammar.hint( hint );
      self
    }

    pub fn subject< H : Into< String > >( mut self, hint : H, kind : Type, optional : bool ) -> Self
    {
      self.grammar = self.grammar.subject( hint, kind, optional );
      self
    }

    pub fn property< K : AsRef< str >, H : Into< String > >( mut self, key : K, hint : H, kind : Type, optional : bool ) -> Self
    {
      self.grammar = self.grammar.property( key, hint, kind, optional );
      self
    }

    pub fn routine< I, R, F : Into< Handler< I, R > > >( mut self, f : F ) -> Self
    where
      Routine: From< Handler< I, R > >,
    {
      let h = f.into();
      self.routine = h.into();
      self
    }

    pub fn perform( mut self ) -> CommandsAggregatorFluentBuilder
    {
      let cmd= self.grammar.form();
      let phrase = cmd.phrase.clone();

      self.ca.verifier.commands.entry( phrase.clone() ).or_default().push( cmd );
      assert!( !self.ca.executor_converter.routines.contains_key( &phrase ), "routine was duplicated" );
      self.ca.executor_converter.routines.insert( phrase, self.routine );

      CommandsAggregatorFluentBuilder( self.ca )
    }
  }
}

//

crate::mod_interface!
{
  prelude use CommandsAggregator;
  exposed use Error;
  exposed use ValidationError;
}
