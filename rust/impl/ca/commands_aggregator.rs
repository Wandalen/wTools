
pub( crate ) mod private
{
  use crate::command::*;
  use crate::instruction::*;
  use wtools::meta::*;
  use wtools::error::*;
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
    // pub vocabulary : Option<Logger>, /* qqq : implement */
  }

  impl CommandsAggregator
  {
    /// Perform instructions queue as single program.
    pub fn program_perform( &self, _program : &str ) -> Result< (), BasicError >
    {
      unimplemented!( "not implemented" );
      // let parsed = program_parse( program );
    }

    /// Perform instruction.
    pub fn instruction_perform( &self, instruction : &str ) -> Result< (), BasicError >
    {
      let parsed : Instruction = instruction_parse()
      .instruction( instruction )
      .perform();

      let result = match self.command_resolve( &parsed )
      {
        Some( command ) =>
        {
          command.perform( &parsed )
        },
        None =>
        {
          if self.with_help
          {
            self.print_help();
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

    /// Print help information.
    fn print_help( &self )
    {
      println!( "Illformed command" );
      for ( command_name, command_descriptor ) in self.commands.iter()
      {
        println!( "{} - {}", command_name, command_descriptor.hint );
      }
    }

    /// Find command in dictionary.
    fn command_resolve( &self, instruction : &Instruction ) -> Option<&Command>
    {
      self.commands.get( &instruction.command_name )
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
  pub use super::private::commands_aggregator;
}

