
pub( crate ) mod private
{
  use crate::command::*;
  use crate::instruction::*;
  use wtools::meta::*;
  use wtools::error::{ Result, BasicError };
  use wtools::string::split;
  use wtools::former::Former;

  ///
  /// Commands aggregator.
  ///

  /*
     Dmytro : owned types are used because Former does not work with combination of links and
     containers
   */

  #[ derive( Debug, PartialEq ) ]
  #[ derive( Former ) ]
  #[ allow( missing_docs ) ]
  pub struct CommandsAggregator
  {
    pub base_path : Option<std::path::PathBuf>,
    #[ default( "".to_string() ) ]
    pub command_prefix : String,
    #[ default( vec![ ".".to_string(), " ".to_string() ] ) ]
    pub delimeter : Vec< String >,
    #[ default( ";".to_string() ) ]
    pub command_explicit_delimeter : String,
    #[ default( " ".to_string() ) ]
    pub command_implicit_delimeter : String,
    #[ default( true ) ]
    pub commands_explicit_delimiting : bool,
    #[ default( false ) ]
    pub commands_implicit_delimiting : bool,
    #[ default( false ) ]
    pub properties_map_parsing : bool,
    #[ default( true ) ]
    pub several_values : bool,
    #[ default( true ) ]
    pub with_help : bool,
    #[ default( true ) ]
    pub changing_exit_code : bool,
    // logger : Option<Logger>, /* qqq : implement */
    pub commands : std::collections::HashMap< String, Command >,
    // pub vocabulary : Option<vocabulary>, /* qqq : implement */
  }

  impl CommandsAggregator
  {
    /// Perform instructions queue as single program.
    pub fn program_perform( &self, program : impl AsRef< str > ) -> Result< () >
    {
      let program = program.as_ref().trim();

      if !program.starts_with( '.' /* aggregator.vocabulary.default_delimeter */ )
        || program.starts_with( "./" /* `${aggregator.vocabulary.default_delimeter}/` */ )
        || program.starts_with( ".\\" /* `${aggregator.vocabulary.default_delimeter}\\` */ )
      {
        return self.on_syntax_error( program );
      }

      /* should use logger and condition */
      println!( "Command \"{}\"", program );

      let instructions = self.instructions_parse( program );

      for instruction in &instructions
      {
        match self._instruction_perform( instruction )
        {
          Ok( _ ) => {},
          Err( err ) =>
          {
            if self.changing_exit_code
            {
              eprintln!( "{}", err.to_string() );
              std::process::exit( 1 );
            }
            else
            {
              return Err( err )
            }
          }
        }
      }

      Ok( () )
    }

    /// Perform instruction.
    pub fn instruction_perform( &self, instruction : impl AsRef< str > ) -> Result< () >
    {
      let parsed : Instruction = instruction_parse()
      .instruction( instruction.as_ref() )
      .several_values( self.several_values )
      .properties_map_parsing( self.properties_map_parsing )
      .quoting( true )
      .unquoting( true )
      .perform();

      self._instruction_perform( &parsed )
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
          if self.changing_exit_code
          {
            std::process::exit( 1 );
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
      else
      {
        if let Some( command_descriptor ) = self.commands.get( command.as_ref() )
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
    }

    /// Find command in dictionary.
    fn command_resolve( &self, instruction : &Instruction ) -> Option<&Command>
    {
      self.commands.get( &instruction.command_name )
    }

    /// Parse multiple instructions.
    pub fn instructions_parse( &self, program : impl AsRef< str > ) -> Vec< Instruction >
    {
      let commands = split()
      .src( program.as_ref().trim() )
      .delimeter( self.command_explicit_delimeter.as_str() )
      .preserving_empty( false )
      .preserving_delimeters( false )
      .preserving_quoting( false )
      .perform();
      let commands = commands.map( | e | String::from( e ) ).collect::< Vec< _ > >();

      let mut string_commands = vec![];
      for command in commands
      {
        let splitted = split()
        .src( command.trim() )
        .delimeter( self.command_implicit_delimeter.as_str() )
        .preserving_empty( false )
        .preserving_delimeters( false )
        .preserving_quoting( false )
        .perform();
        let splitted = splitted.map( | e | String::from( e ) ).collect::< Vec< _ > >();

        if self.command_implicit_delimeter == " "
        {
          let start_index = if splitted[ 0 ].is_empty() { 1 } else { 0 };
          let mut string_command = String::from( &splitted[ start_index ] );

          for i in start_index + 1 .. splitted.len()
          {
            let part = splitted[ i ].trim();
            if part.starts_with( '.' ) && !self.dotted_path_is( part )
            {
              string_commands.push( string_command );
              string_command = String::from( part );
            }
            else
            {
              string_command.push( ' ' );
              string_command.push_str( part );
            }
          }

          string_commands.push( string_command );
        }
        else
        {
          for command in splitted
          {
            string_commands.push( String::from( command.trim() ) );
          }
        }
      }

      let instructions = string_commands.iter().map( | instruction |
      {
        instruction_parse()
        .instruction( instruction.as_str() )
        .properties_map_parsing( true )
        .several_values( true )
        .perform()
      }).collect::< Vec< Instruction > >();

      instructions
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
      if self.changing_exit_code
      {
        /* qqq : implement */
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
      return self.on_error( err );
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
      println!( "" );

      let err_formatted = format!( "Ambiguity \"{}\"", command.as_ref() );
      let err = BasicError::new( err_formatted );
      return self.on_error( err );
    }
  }

  #[ cfg( feature = "on_unknown_command_error_default" ) ]
  impl OnUnknownCommandError for CommandsAggregator
  {
    /// Handle unknown command error.
    fn on_unknown_command_error( &self, command : impl AsRef< str > ) -> Result< () >
    {
      let mut err_formatted = format!( "Unknown command \"{}\"", command.as_ref() );

      let instruction = instruction_parse()
      .instruction( ".help" )
      .perform();
      if self.command_resolve( &instruction ).is_some()
      {
        err_formatted.push_str( "\nTry \".help\"" );
      }
      let err = BasicError::new( err_formatted );
      return self.on_error( err );
    }
  }

  #[ cfg( feature = "on_get_help_default" ) ]
  impl OnGetHelp for CommandsAggregator
  {
    /// Get help.
    fn on_get_help( &self ) -> Result< () >
    {
      let instruction = instruction_parse()
      .instruction( ".help" )
      .perform();
      if let Some( command ) = self.command_resolve( &instruction )
      {
        let instruction = instruction_parse()
        .instruction( "" )
        .perform();
        return command.perform( &instruction );
      }
      else
      {
        self.command_help( "" );
        return Ok( () );
      }
    }
  }

  #[ cfg( feature = "on_print_commands_default" ) ]
  impl OnPrintCommands for CommandsAggregator
  {
    /// Print all commands.
    fn on_print_commands( &self ) -> Result< () >
    {
      println!( "" );
      self.command_help( "" );
      println!( "" );
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

/// Protected namespace of the module.
pub mod protected
{
  pub use super::private::CommandsAggregator;
  pub use super::private::OnError;
  pub use super::private::OnSyntaxError;
  pub use super::private::OnAmbiguity;
  pub use super::private::OnUnknownCommandError;
  pub use super::private::OnGetHelp;
  pub use super::private::OnPrintCommands;
  pub use super::private::commands_aggregator;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::private::CommandsAggregator;
  pub use super::private::OnError;
  pub use super::private::OnSyntaxError;
  pub use super::private::OnAmbiguity;
  pub use super::private::OnUnknownCommandError;
  pub use super::private::OnGetHelp;
  pub use super::private::OnPrintCommands;
  pub use super::private::commands_aggregator;
}

