
pub( crate ) mod private
{
  use crate::command::*;
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
  // #[ perform( fn run( &self ) -> Result<(), &'static str> ) ]
  pub struct CommandsAggregator
  {
    pub base_path : std::path::PathBuf,
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
    pub commands : Vec< Command >,
    // pub vocabulary : Option<Logger>, /* qqq : implement */
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

