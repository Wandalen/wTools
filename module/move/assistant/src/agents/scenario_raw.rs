//!
//! Raw scenario representation. Captures only the basic syntax of scenario file.
//!
//! For more detailed representation, use `ScenarioProcessed`.
//!

mod private
{
  use std::
  {
    io,
    collections::HashMap,
  };

  use former::Former;
  use serde::
  {
    Serialize,
    Deserialize,
  };

  /// Struct that represents user written scenarios.
  ///
  /// This is a raw form of a scenario, only the general structure is captured there.
  /// For more detailed representation of scenarios, use `ScenarioProcessed` type.
  #[ derive( Debug, Serialize, Deserialize, Former, PartialEq ) ]
  pub struct ScenarioRaw
  {
    /// Nodes in the scenario.
    pub nodes: Vec< NodeRaw >,
  }

  impl ScenarioRaw
  {
    /// Read scenario file in YAML format.
    pub fn read( reader : impl io::Read ) -> Result< Self, serde_yaml::Error >
    {
      serde_yaml::from_reader( reader )
    }
  }

  /// Node representation in a scenario file.
  ///
  /// This is a raw form of a node, only the general structure is captured there.
  /// For more detailed representation of scenarios, use `Node` type.
  #[ derive( Debug, Serialize, Deserialize, Former, PartialEq ) ]
  pub struct NodeRaw
  {
    /// ID of the node. Must be unique, will also identify node output. 
    pub id : String,

    /// Type of the node. Represented as a path.
    pub r#type : String,

    /// Rest of the key-value pairs in the node that are specific to node types.
    #[ serde( flatten ) ]
    pub params : HashMap< String, String >,

    /// ID of the next node to execute. Represented as a path.
    pub next : String,
  }
}

crate::mod_interface!
{
  own use
  {
    ScenarioRaw,
    NodeRaw,
  };
}