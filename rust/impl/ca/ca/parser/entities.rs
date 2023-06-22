pub( crate ) mod private
{
  use wtools::HashMap;

  /// Program representation
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct Program< Namespace >
  {
    /// list of namespaces with commands
    pub namespaces : Vec< Namespace >,
  }

  /// Namespace representation
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct Namespace< Command >
  {
    /// list of commands
    pub commands : Vec< Command >,
  }

  /// Command representation
  #[ derive( Debug, Default, Clone, PartialEq, Eq ) ]
  pub struct RawCommand
  {
    /// name of command without delimiter
    pub name : String,
    /// list of subjects
    pub subjects : Vec< String >,
    /// map of properties
    pub properties : HashMap< String, String >
  }
}

//

crate::mod_interface!
{
  prelude use Program;
  prelude use Namespace;
  prelude use RawCommand;
}
