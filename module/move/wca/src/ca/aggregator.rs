pub( crate ) mod private
{
  use crate::*;
  use ca::
  {
    Verifier,
    Executor,
    grammar::command::private::
    {
      CommandFormer,
      CommandAsSubformer,
      CommandAsSubformerEnd,
      CommandFormerStorage
    },
    help::{ HelpGeneratorFn, HelpGeneratorOptions, HelpVariants },
  };

  // qqq : group uses
  use std::collections::HashSet;
  use std::fmt;
  use former::StoragePreform;
  // use wtools::
  // {
  // };
  // use wtools::thiserror;
  use error::
  {
    // Result,
    untyped::Error as wError, // xxx
    for_lib::*,
  };
  use iter_tools::Itertools;

  /// Order of commands and properties.
  #[ derive( Debug, Default, Clone, Copy, Eq, PartialOrd, PartialEq ) ]
  pub enum Order
  {
    /// Natures order.
    #[ default ]
    Nature,
    /// Lexicography order.
    Lexicography,
  }

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
  // aaa : done. The concept of `Namespace` has been removed
  struct CommandsAggregatorCallback( Box< dyn Fn( &str, &Program< VerifiedCommand > ) > );

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
  /// use wca::{ CommandsAggregator, VerifiedCommand, Type };
  ///
  /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
  /// let ca = CommandsAggregator::former()
  /// .command( "echo" )
  ///   .hint( "prints all subjects and properties" )
  ///   .subject().hint( "argument" ).kind( Type::String ).optional( false ).end()
  ///   .property( "property" ).hint( "simple property" ).kind( Type::String ).optional( false ).end()
  ///   .routine( | o : VerifiedCommand | println!( "= Args\n{:?}\n\n= Properties\n{:?}\n", o.args, o.props ) )
  ///   .end()
  /// .perform();
  ///
  /// ca.perform( ".echo something" )?;
  /// # Ok( () ) }
  /// ```
  #[ derive( Debug ) ]
  #[ derive( former::Former ) ]
  #[ storage_fields( help_generator : HelpGeneratorFn, help_variants : HashSet< HelpVariants >, order : Order ) ]
  #[ mutator( custom ) ]
  // #[ debug ]
  pub struct CommandsAggregator
  {
    #[ former( default = Dictionary::default() ) ]
    dictionary : Dictionary,

    #[ former( default = Parser ) ]
    parser : Parser,

    #[ scalar( setter = false ) ]
    #[ former( default = Executor::former().form() ) ]
    executor : Executor,

    #[ former( default = Verifier ) ]
    verifier : Verifier,

    callback_fn : Option< CommandsAggregatorCallback >,
  }

  impl< Context, Formed > former::FormerMutator for CommandsAggregatorFormerDefinitionTypes< Context, Formed >
  {
    fn form_mutation( storage : &mut Self::Storage, _context : &mut Option< Self::Context > )
    {
      let ca = storage;
      let dictionary = ca.dictionary.get_or_insert_with( Dictionary::default );
      dictionary.order = ca.order.unwrap_or_default();

      let help_generator = std::mem::take( &mut ca.help_generator ).unwrap_or_default();
      let help_variants = std::mem::take( &mut ca.help_variants ).unwrap_or_else( || HashSet::from([ HelpVariants::All ]) );

      if help_variants.contains( &HelpVariants::All )
      {
        HelpVariants::All.generate( &help_generator, dictionary, ca.order.unwrap_or_default() );
      }
      else
      {
        for help in help_variants.iter().sorted()
        {
          help.generate( &help_generator, dictionary, ca.order.unwrap_or_default() );
        }
      }
    }
  }

  impl< Definition > CommandsAggregatorFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < CommandsAggregator as former::EntityToStorage >::Storage >,
  {
    /// Creates a command in the command chain.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the command.
    pub fn command< IntoName >( self, name : IntoName ) -> CommandAsSubformer< Self, impl CommandAsSubformerEnd< Self > >
    where
      IntoName : Into< String >,
    {
      let name = name.into();
      let on_end = | command : CommandFormerStorage, super_former : Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        let mut dictionary = super_former.storage.dictionary.unwrap_or_default();

        dictionary.register( command.preform() );

        super_former.storage.dictionary = Some( dictionary );

        super_former
      };
      let former = CommandFormer::begin( None, Some( self ), on_end );
      former.phrase( name )
    }
  }

  impl CommandsAggregatorFormer
  {
    /// Adds a context to the executor.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be used as the context.
    ///
    /// # Returns
    ///
    /// The modified instance of `Self`.
    // `'static` means that the value must be owned or live at least as a `Context'
    pub fn with_context< T >( mut self, value : T ) -> Self
    where
      T : Sync + Send + 'static,
    {
      let mut executor = self.storage.executor.unwrap_or_else( || Executor::former().form() );

      executor.context = Context::new( value );

      self.storage.executor = Some( executor );

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
    /// .perform();
    ///
    /// ca.perform( ".help" )?;
    /// # Ok( () ) }
    /// ```
    pub fn help< HelpFunction >( mut self, func : HelpFunction ) -> Self
    where
      HelpFunction : Fn( &Dictionary, HelpGeneratorOptions< '_ > ) -> String + 'static
    {
      self.storage.help_generator = Some( HelpGeneratorFn::new( func ) );
      self
    }
    // aaa : it is good access method, but formed structure should not have help_generator anymore
    // aaa : mutator used

    /// Set callback function that will be executed after validation state
    ///
    /// ```
    /// use wca::CommandsAggregator;
    ///
    /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
    /// let ca = CommandsAggregator::former()
    /// // ...
    /// .callback( | _input, _program | println!( "Program is valid" ) )
    /// .perform();
    ///
    /// // prints the "Program is valid" and after executes the program
    /// ca.perform( ".help" )?;
    /// # Ok( () ) }
    /// ```
    pub fn callback< Callback >( mut self, callback : Callback ) -> Self
    where
      Callback : Fn( &str, &Program< VerifiedCommand > ) + 'static,
    {
      self.storage.callback_fn = Some( CommandsAggregatorCallback( Box::new( callback ) ) );
      self
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

      let raw_program = self.parser.parse( program ).map_err( | e | Error::Validation( ValidationError::Parser { input : format!( "{:?}", program ), error : e } ) )?;
      let grammar_program = self.verifier.to_program( &self.dictionary, raw_program ).map_err( | e | Error::Validation( ValidationError::Verifier( e ) ) )?;

      if let Some( callback ) = &self.callback_fn
      {
        callback.0( &program.join( " " ), &grammar_program )
      }

      self.executor.program( &self.dictionary, grammar_program ).map_err( | e | Error::Execution( e ) )
    }
  }
}

//

crate::mod_interface!
{
  exposed use CommandsAggregator;
  exposed use CommandsAggregatorFormer;
  exposed use Error;
  exposed use ValidationError;
  exposed use Order;
}
