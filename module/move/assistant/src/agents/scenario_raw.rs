//!
//! Raw scenario representation. Captures only the basic syntax of scenario file.
//!
//! For more detailed representation, use `ScenarioProcessed`.
//!

mod private
{
  use former::Former;

  /// Struct that represents user written scenarios.
  ///
  /// This is a raw form of a scenario, only the general structure is captured there.
  /// For more detailed representation of scenarios, use `ScenarioProcessed` type.
  #[ derive( Debug, Deserialize, Former ) ]
  pub struct ScenarioRaw
  {
    /// Nodes in the scenario.
    pub nodes: Vec< NodeRaw >,
  }

  /// Node representation in a scenario file.
  ///
  /// This is a raw form of a node, only the general structure is captured there.
  /// For more detailed representation of scenarios, use `Node` type.
  #[ derive( Debug, Deserialize, Former ) ]
  pub struct NodeRaw
  {
    /// ID of the node. Must be unique, will also identify node output. 
    pub id : String,

    /// Type of the node. Represented as a path.
    pub r#type : String,

    /// ID of the next node to execute. Represented as a path.
    pub next : String,

    /// Rest of the key-value pairs in the node that are specific to node types.
    #[ serde( flatten ) ]
    pub params : HashMap< String, String >,
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