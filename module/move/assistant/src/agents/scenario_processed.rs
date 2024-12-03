//!
//! Scenario representation. Stores parsed representation of templates and paths.
//! This is the type used for running scenarios.
//!
//! For a more simplistic representation use `ScenarioRaw`.
//!

mod private
{
  use serde::
  {
    Serialize,
    Deserialize,
  };

  use crate::*;
  use agents::
  {
    path::Path,
    scenario_raw::ScenarioRaw,
  };

  /// New type for templates in scenarios.
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
  pub struct TemplateBody( pub String );

  /// Struct that represents user written scenarios.
  ///
  /// This is a processed form of a scenario, templates and paths are distinguished here with types.
  /// For more simplistic representation of scenarios, use `ScenarioRaw` type.
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
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
      let nodes : Result< Vec< Node >, Self::Error > = scenario_raw.nodes.into_iter().map( | n | n.try_from() ).collect();
      
      Ok( Self { nodes } )
    }
  }

  /// Node representation in a scenario file.
  ///
  /// This is a processed form of a node, templates and paths are distinguished here with types.
  /// For more simplistic representation of scenarios, use `NodeRaw` type.
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
  pub struct Node
  {
    /// ID of the node. Must be unique, will also identify node output. 
    pub id : String,

    /// Type of the node.
    pub r#type : Path,

    /// Specific type of `Node`.
    pub kind : NodeKind,

    /// ID of the next node to execute.
    pub next : Path,
  }

  impl TryFrom< NodeRaw > for Node
  {
    type Error = std::io::Error;

    fn try_form( node_raw : NodeRaw ) -> Result< Self, Self::Error >
    {
      Ok
      (
        Self
        {
          id : node_raw.id,
          r#type : Path::try_from( node_raw.r#type )?,
          kind : NodeKind::try_from( node_raw.params )?,
          next : Path::try_from( node_raw.path )?,
        }
      )
    }
  }

  /// Representation of different types of nodes. Contains parameters unique to types.
  /// Used in `Node` struct, where you can find common parameters like `id`, `type`, and others.
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
  pub enum NodeKind
  {
    /// Read input from `stdin`.
    TriggerStdin
    {
      /// Prompt to display to `stdout` before reading the `stdin`.
      prompt : TemplateBody,
    },

    /// Get output from LLM.
    AgentsCompletion
    {
      /// Agent's system message template.
      system_message : TemplateBody,
      /// Agent's user message template.
      user_message : TemplateBody,
      /// Reuse chat history of other agent.
      agent_reuse : Path,
    },

    /// Print output to `stdout`.
    EventStdout
    {
      /// Prompt to display to `stdout`.
      output : TemplateBody,
    },
  }

  impl NodeKind
  {
    /// Convert `NodeRaw` into `NodeKind`.
    pub fn try_from_params
    (
      params : HashMap< String, String >,
      r#type : &Path,
    ) -> Result< Self, std::io::Error >
    {
      match path.inner().as_str()
      {
        "::trigger::stdin" =>
        {
          let Some()
        }
      }
    }
  }

  impl TryFrom< HashMap< String, String > > for NodeKind
  {
    type Error = std::io::Error;

    fn try_from( map : HashMap< String, String> ) -> Result< Self, Self::Error >
    {
      
    }
  }
}

crate::mod_interface!
{
  own use
  {
    TemplateBody,
    ScenarioProcessed,
  };
}