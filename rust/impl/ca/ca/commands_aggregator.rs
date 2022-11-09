
pub( crate ) mod private
{
  use crate::command::*;
  use crate::instruction::*;
  use wtools::error::{ Result, BasicError };
  use wtools::former::Former;

  ///
  /// Commands aggregator.
  ///

  /*
     Dmytro : owned types are used because Former does not work with combination of links and
     containers
   */

  #[ derive( PartialEq, Debug ) ]
  #[ derive( Former ) ]
  pub struct CommandsAggregator
  {
    /// Command delimiter.
    #[ default( ".".to_string() ) ]
    pub delimiter : String,
    /// Commands have help.
    #[ default( true ) ]
    pub with_help : bool,
    /// If set terminates the current process with the specified exit code on error.
    /// Otherwise returns result.
    pub exit_code_on_error : Option< i32 >,
    /// Commands.
    pub commands : std::collections::HashMap< String, Command >,
  }

  impl CommandsAggregator
  {
    /// Perform instructions queue as single program.
    pub fn program_perform( &self, program : impl AsRef< str > ) -> Result< () >
    {
      let program = program.as_ref().trim();

      println!( "Command \"{}\"", program );

      let instructions = self.instructions_parse( program )?;

      for instruction in &instructions
      {
        if let Err( err ) = self._instruction_perform( instruction )
        {
          if let Some( exit_code ) = self.exit_code_on_error
          {
            eprintln!( "{}", err.to_string() );
            std::process::exit( exit_code );
          }
          else
          {
            return Err( err );
          }
        }
      }

      Ok( () )
    }

    /// Perform instruction.
    pub fn instruction_perform( &self, instruction : impl AsRef< str > ) -> Result< () >
    {
      let parsed : Instruction = DefaultInstructionParser::former()
      .quoting( true )
      .unquoting( true )
      .form()
      .parse( instruction.as_ref() )?;

      let result = self._instruction_perform( &parsed );
      if let ( true, Some( code ) ) = ( result.is_err(), self.exit_code_on_error )
      {
        eprintln!( "{}", result.err().unwrap().to_string() );
        std::process::exit( code );
      }
      result
    }

    //

    fn _instruction_perform( &self, instruction : &Instruction ) -> Result< () >
    {
      let result = match self.command_resolve( instruction )
      {
        Some( command ) =>
        {
          command.perform( instruction )
        },
        None =>
        {
          if self.with_help
          {
            match self.on_ambiguity( instruction.command_name.as_str() )
            {
              _ => (),
            }
          }
          if let Some( code ) = self.exit_code_on_error
          {
            std::process::exit( code );
          }
          Ok( () )
        },
      };

      result
    }

    /// Print help for command.
    fn command_help( &self, command : impl AsRef< str > )
    {
      if command.as_ref() == ""
      {
        for ( _name, command_descriptor ) in self.commands.iter()
        {
          println!( "{}", command_descriptor.help_short() );
        }
      }
      else if let Some( command_descriptor ) = self.commands.get( command.as_ref() )
      {
        println!( "{}", command_descriptor.help_long() );
      }
      else
      {
        match self.on_unknown_command_error( command.as_ref() )
        {
          _ => ()
        };
      }
    }

    /// Find command in dictionary.
    fn command_resolve( &self, instruction : &Instruction ) -> Option< &Command >
    {
      self.commands.get( &instruction.command_name )
    }

    /// Parse multiple instructions.
    pub fn instructions_parse( &self, program : impl AsRef< str > ) -> Result< Vec< Instruction > >
    {
      let program = program.as_ref().trim();
      if program.is_empty()
      {
        return Ok( vec![] );
      }

      let parser = DefaultInstructionParser::former()
      .several_values( true )
      .form();
      let mut instructions = vec![];
      let mut instruction = String::new();
      for part in program.split_inclusive( ' ' )
      {
        if part.starts_with( &self.delimiter ) && !self.dotted_path_is( part ) && !instruction.is_empty()
        {
          instructions.push( parser.parse( &instruction )? );
          instruction.clear();
        }

        instruction.push_str( part );
      }

      instructions.push( parser.parse( &instruction )? );

      Ok( instructions )
    }

    //

    fn dotted_path_is( &self, src : impl AsRef< str > ) -> bool
    {
      let part = src.as_ref();

      if part == "." || part == ".."
      {
        return true;
      }

      if part.starts_with( "./" ) || part.starts_with( "../" )
      || part.starts_with( ".\\" ) || part.starts_with( "..\\" )
      {
        return true;
      }

      false
    }
  }

  //

  ///
  /// On error helper.
  ///

  pub trait OnError
  {
    /// Handle error.
    fn on_error( &self, err : BasicError ) -> Result< () >;
  }

  ///
  /// On syntax error helper.
  ///

  pub trait OnSyntaxError
  {
    /// Handle syntax error.
    fn on_syntax_error( &self, command : impl AsRef< str > ) -> Result< () >;
  }

  ///
  /// On ambiguity helper.
  ///

  pub trait OnAmbiguity
  {
    /// Handle ambiguity.
    fn on_ambiguity( &self, command : impl AsRef< str > ) -> Result< () >;
  }

  ///
  /// On unknown command error helper.
  ///

  pub trait OnUnknownCommandError
  {
    /// Handle unknown command error.
    fn on_unknown_command_error( &self, command : impl AsRef< str > ) -> Result< () >;
  }

  ///
  /// Help helper.
  ///

  pub trait OnGetHelp
  {
    /// Get help.
    fn on_get_help( &self ) -> Result< () >;
  }

  ///
  /// Printing commands helper.
  ///

  pub trait OnPrintCommands
  {
    /// Print all commands.
    fn on_print_commands( &self ) -> Result< () >;
  }

  ///
  /// Super trait that checks that all helpers are implemented.
  ///

  pub trait CommandsAggregatorHandlers : OnError + OnSyntaxError + OnAmbiguity + OnUnknownCommandError + OnGetHelp + OnPrintCommands
  {
  }

  impl CommandsAggregatorHandlers for CommandsAggregator {}

  #[ cfg( feature = "on_error_default" ) ]
  impl OnError for CommandsAggregator
  {
    /// Handle error.
    fn on_error( &self, err : BasicError ) -> Result< () >
    {
      if self.exit_code_on_error.is_some()
      {
        /* rrr : for Dmytro : implement */
        // unimplemented!();
      }
      Err( err )
    }
  }

  #[ cfg( feature = "on_syntax_error_default" ) ]
  impl OnSyntaxError for CommandsAggregator
  {
    /// Handle syntax error.
    fn on_syntax_error( &self, command : impl AsRef< str > ) -> Result< () >
    {
      let err_formatted = format!( "Illformed command \"{}\"", command.as_ref() );
      eprintln!( "{}", err_formatted );
      self.on_get_help().unwrap();

      let err = BasicError::new( err_formatted );
      self.on_error( err )
    }
  }

  #[ cfg( feature = "on_ambiguity_default" ) ]
  impl OnAmbiguity for CommandsAggregator
  {
    /// Handle ambiguity.
    fn on_ambiguity( &self, command : impl AsRef< str > ) -> Result< () >
    {
      eprintln!( "Ambiguity. Did you mean?" );
      self.command_help( command.as_ref() );
      println!();

      let err_formatted = format!( "Ambiguity \"{}\"", command.as_ref() );
      let err = BasicError::new( err_formatted );
      self.on_error( err )
    }
  }

  #[ cfg( feature = "on_unknown_command_error_default" ) ]
  impl OnUnknownCommandError for CommandsAggregator
  {
    /// Handle unknown command error.
    fn on_unknown_command_error( &self, command : impl AsRef< str > ) -> Result< () >
    {
      let mut err_formatted = format!( "Unknown command \"{}\"", command.as_ref() );

      let instruction = DefaultInstructionParser::former().form().parse( ".help" )?;
      if self.command_resolve( &instruction ).is_some()
      {
        err_formatted.push_str( "\nTry \".help\"" );
      }
      let err = BasicError::new( err_formatted );
      self.on_error( err )
    }
  }

  #[ cfg( feature = "on_get_help_default" ) ]
  impl OnGetHelp for CommandsAggregator
  {
    /// Get help.
    fn on_get_help( &self ) -> Result< () >
    {
      let instruction = DefaultInstructionParser::former().form().parse( ".help" )?;
      return if let Some( command ) = self.command_resolve( &instruction )
      {
        let instruction = DefaultInstructionParser::former().form().parse( "" )?;
        command.perform( &instruction )
      }
      else
      {
        self.command_help( "" );
        Ok( () )
      }
    }
  }

  #[ cfg( feature = "on_print_commands_default" ) ]
  impl OnPrintCommands for CommandsAggregator
  {
    /// Print all commands.
    fn on_print_commands( &self ) -> Result< () >
    {
      println!();
      self.command_help( "" );
      println!();
      Ok( () )
    }
  }

  //

  ///
  /// Get instruction parser builder.
  ///

  pub fn commands_aggregator() -> CommandsAggregatorFormer
  {
    CommandsAggregator::former()
  }

}

crate::mod_interface!
{
  prelude use CommandsAggregator;
  prelude use OnError;
  prelude use OnSyntaxError;
  prelude use OnAmbiguity;
  prelude use OnUnknownCommandError;
  prelude use OnGetHelp;
  prelude use OnPrintCommands;
  prelude use commands_aggregator;
}
