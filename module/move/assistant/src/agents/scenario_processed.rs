//!
//! Scenario representation. Stores parsed representation of templates and paths.
//! This is the type used for running scenarios.
//!
//! For a more simplistic representation use `ScenarioRaw`.
//!

mod private
{
  use std::collections::HashMap;

  use crate::*;
  use agents::
  {
    path::Path,
    scenario_raw::
    {
      ScenarioRaw,
      NodeRaw,
    },
  };

  /// New type for templates in scenarios.
  #[ derive( Debug, PartialEq ) ]
  pub struct TemplateBody( pub String );

  /// Struct that represents user written scenarios.
  ///
  /// This is a processed form of a scenario, paths are distinguished here with types.
  /// For more simplistic representation of scenarios, use `ScenarioRaw` type.
  #[ derive( Debug, PartialEq ) ]
  pub struct ScenarioProcessed
  {
    /// Nodes in the scenario.
    pub nodes: Vec< Node >,
  }

  impl TryFrom< ScenarioRaw > for ScenarioProcessed
  {
    type Error = std::io::Error;

    fn try_from( scenario_raw : ScenarioRaw ) -> Result< Self, Self::Error >
    {
      let nodes : Result< Vec< Node >, Self::Error > = 
      scenario_raw.nodes.into_iter().map( | rn | Node::try_from( rn ) ).collect();
      
      Ok( Self { nodes : nodes? } )
    }
  }

  /// Node representation in a scenario file.
  ///
  /// This is a processed form of a node, paths are distinguished here with types.
  /// For more simplistic representation of scenarios, use `NodeRaw` type.
  #[ derive( Debug, PartialEq ) ]
  pub struct Node
  {
    /// ID of the node. Must be unique, will also identify node output. 
    pub id : String,

    /// Type of the node.
    pub r#type : Path,

    /// Parameters of the node.
    pub params : HashMap< String, String >,

    /// ID of the next node to execute.
    pub next : Path,
  }

  impl TryFrom< NodeRaw > for Node
  {
    type Error = std::io::Error;

    fn try_from( node_raw : NodeRaw ) -> Result< Self, Self::Error >
    {
      Ok
      (
        Self
        {
          id : node_raw.id,
          r#type : Path::try_from( node_raw.r#type )?,
          params : node_raw.params,
          next : Path::try_from( node_raw.next )?,
        }
      )
    }
  }
}

crate::mod_interface!
{
  own use
  {
    TemplateBody,
    ScenarioProcessed,
    Node,
  };
}