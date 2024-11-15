mod private
{
  use crate::*;

  use ca::help::{ HelpGeneratorOptions, generate_help_content, LevelOfDetail };
  use verifier::VerifiedCommand;
  use parser::Program;
  use grammar::Dictionary;
  use executor::{ Routine, Context };

  // aaa : for Bohdan : how is it useful? where is it used?
  // aaa : `ExecutorType` has been removed

  #[ derive( Debug, error::typed::Error ) ]
  pub enum CommandError
  {
    #[ error( "Internal command: `.{}` failed with: {}", command.phrase, error ) ]
    Internal { command: VerifiedCommand, error: InternalCommandError },
    #[ error( "Command: `.{}` failed with: {}", command.phrase, error ) ]
    User { command: VerifiedCommand, error: error::untyped::Error },
  }

  /// Executor that is responsible for executing the program's commands.
  /// It uses the given `Context` to store and retrieve values during runtime.
  #[ derive( Debug, former::Former ) ]
  pub struct Executor
  {
    /// The default context for the executor
    #[ former( default = Context::default() ) ]
    pub context : Context,
  }

  impl Executor
  {
    /// Executes a program
    ///
    /// Iterates over the commands in the program and executes each command using the provided dictionary.
    /// This method returns a `Result` indicating whether the execution was successful or not.
    ///
    /// # Arguments
    ///
    /// * `dictionary` - A reference to the dictionary used to look up the command routine.
    /// * `program` - The program to be executed, which is a `Program` object consisting of a list of commands.
    ///
    /// # Returns
    ///
    /// A `Result` with `Ok( () )` if the execution was successful, or an `Err` containing an error message if an error occurred.
    ///
    // aaa : use typed error
    // aaa : done
    pub fn program( &self, dictionary : &Dictionary, program : Program< VerifiedCommand > )
    -> Result< (), CommandError >
    {
      for command in program.commands
      {
        self.command( dictionary, command )?;
      }

      Ok( () )
    }

    /// Executes a given command using a provided dictionary and command.
    ///
    /// Calls the command callback with the given context if it is necessary.
    ///
    /// # Arguments
    ///
    /// * `dictionary` - A reference to the dictionary used to look up the command routine.
    /// * `command` - The verified command that needs to be executed.
    ///
    /// # Returns
    ///
    /// Returns a Result indicating success or failure. If successful, returns `Ok(())`, otherwise returns an error.
    // aaa : use typed error
    // aaa : done
    pub fn command( &self, dictionary : &Dictionary, command : VerifiedCommand )
    -> Result< (), CommandError >
    {
      if command.internal_command
      {
        _exec_internal_command( dictionary, command.clone() )
        .map_err( | error | CommandError::Internal { command, error } )
      }
      else
      {
        let routine = dictionary.command( &command.phrase ).unwrap().routine.clone();
        _exec_command( command.clone(), routine, self.context.clone() )
        .map_err( | error | CommandError::User { command, error } )
      }
    }

    // aaa : for Bohdan : probably redundant
    // aaa : removed `parallel_execution_loop`
  }

  // qqq : use typed error
  // aaa : should it be typed? it is user command with unknown error type
  fn _exec_command( command : VerifiedCommand, routine : Routine, ctx : Context )
  -> error::untyped::Result< () >
  {
    match routine
    {
      Routine::WithoutContext( routine ) => routine( command ),
      Routine::WithContext( routine ) => routine( ctx, command ),
    }
  }

  #[ derive( Debug, error::typed::Error ) ]
  pub enum InternalCommandError
  {
    #[ error( "Encountered an unrecognized internal command: `.{user_input}`." ) ]
    UnknownInternalCommand { user_input: String },
    #[ error( "Not found command that starts with `.{user_input}`." ) ]
    CommandNotFound { user_input: String },
  }

  // aaa : use typed error
  // aaa : done
  fn _exec_internal_command( dictionary : &Dictionary, command : VerifiedCommand )
  -> Result< (), InternalCommandError >
  {
    match command.phrase.as_str()
    {
      "." =>
      {
        let generator_args = HelpGeneratorOptions::former()
        .command_prefix( "." )
        .form();

        let content = generate_help_content( dictionary, generator_args );
        println!( "{content}" );
      }
      ".?" =>
      {
        let generator_args = HelpGeneratorOptions::former()
        .description_detailing( LevelOfDetail::Simple )
        .subject_detailing( LevelOfDetail::Simple )
        .property_detailing( LevelOfDetail::Simple )
        .form();

        let content = generate_help_content( dictionary, generator_args );
        println!( "{content}" );
      }
      name if name.ends_with( '.' ) =>
      {
        let name = name.strip_suffix( '.' ).unwrap();
        let commands = dictionary.search( name.strip_prefix( '.' ).unwrap_or( name ) );
        if commands.is_empty()
        {
          return Err( InternalCommandError::CommandNotFound { user_input : name.into() } );
        }
        let generator_args = HelpGeneratorOptions::former()
        .command_prefix( "." )
        .for_commands( commands )
        .form();

        let content = generate_help_content( dictionary, generator_args );
        println!( "{content}" );
      }
      name if name.ends_with( ".?" ) =>
      {
        let name = name.strip_suffix( ".?" ).unwrap();
        let command = dictionary.command( &name.strip_prefix( '.' ).unwrap_or( name ).to_string() );
        if let Some( command ) = command
        {
          let generator_args = HelpGeneratorOptions::former()
          .for_commands([ command ])
          .description_detailing( LevelOfDetail::Detailed )
          .subject_detailing( LevelOfDetail::Simple )
          .property_detailing( LevelOfDetail::Simple )
          .with_footer( true )
          .form();

          let content = generate_help_content( dictionary, generator_args );
          println!( "{content}" );
        }
        else
        {
          return Err( InternalCommandError::CommandNotFound { user_input : name.into() } );
        }
      }
      unexpected => return Err( InternalCommandError::UnknownInternalCommand { user_input: unexpected.into() }),
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use Executor;
}
