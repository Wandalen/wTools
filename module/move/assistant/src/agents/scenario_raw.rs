//!
//! Raw scenario representation. Captures only the basic syntax of scenario file.
//!
//! For more detailed representation, use `ScenarioProcessed`.
//!

mod private
{
  use std::io;

  use former::Former;
  use serde::
  {
    serialize,
    deserialize,
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

    /// Type of the node.
    pub r#type : String,

    /// Specific type of `NodeRaw`.
    pub kind : NodeRawKind,

    /// ID of the next node to execute.
    pub next : String,
  }

  impl format_tools::Fields< &'_ str, &'_ str > for NodeRaw
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = &'v str;

    fn fields( &self ) -> impl format_tools::IteratorTrait< Item = ( &'_ str, &'_ str ) >
    {
      let mut dst = Vec::new();

      dst.push( ( "id", self.id.as_str() ) );
      dst.push( ( "type", self.r#type.as_str() ) );

      dst.extend( self.kind.fields() );

      dst.push( ( "next", self.next.as_str() ) );

      dst.into_iter()
    }
  }

  /// Representation of different types of nodes. Contains parameters unique to types.
  /// Used in `Node` struct, where you can find common parameters like `id`, `type`, and others.
  #[ derive( Debug, Serialize, Deserialize, PartialEq ) ]
  pub enum NodeRawKind
  {
    /// Read input from `stdin`.
    TriggerStdin( TriggerStdinRaw ),

    /// Get output from LLM.
    AgentsCompletion( AgentsCompletionRaw ),

    /// Print output to `stdout`.
    EventStdout( EventStdoutRaw ),
  }

  impl format_tools::Fields< &'_ str, &'_ str > for NodeRawKind
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = &'v str;

    fn fields( &self ) -> impl format_tools::IteratorTrait< Item = ( &'_ str, &'_ str ) >
    {
      match self
      {
        Self::TriggerStdin( v ) => v.fields(),
        Self::AgentsCompletion( v ) => v.fields(),
        Self::EventStdout( v ) => v.fields(),
      }
    }
  }

  /// Read input from `stdin`.
  #[ derive( Debug, Serialize, Deserialize, Former, PartialEq ) ]
  pub struct TriggerStdinRaw
  {
    pub prompt : String;
  }

  impl From< TriggerStdinRaw > for NodeRawKind
  {
    fn from( value : TriggerStdinRaw ) -> Self
    {
      Self::TriggerStdin( value )
    }
  }

  impl format_tools::Fields< &'_ str, &'_ str > for TriggerStdinRaw
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = &'v str;

    fn fields( &self ) -> impl format_tools::IteratorTrait< Item = ( &'_ str, &'_ str ) >
    {
      std::iter::once( ( "prompt", prompt.as_str() ) )
    }
  }

  /// Get output from LLM.
  #[ derive( Debug, Serialize, Deserialize, Former, PartialEq ) ]
  pub struct AgentsCompletionRaw
  {
    /// Agent's system message template.
    pub system_message : String,

    /// Agent's user message template.
    pub user_message : String,

    /// Reuse chat history of other agent.
    pub agent_reuse : Option< String >,
  }

  impl From< AgentsCompletionRaw > for NodeRawKind
  {
    fn from( value : AgentsCompletionRaw ) -> Self
    {
      Self::AgentsCompletion( value )
    }
  }

  impl format_tools::Fields< &'_ str, &'_ str > for AgentsCompletionRaw
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = &'v str;

    fn fields( &self ) -> impl format_tools::IteratorTrait< Item = ( &'_ str, &'_ str ) >
    {
      let mut dst = Vec::new();

      dst.push( ( "system_message", system_message.as_str() ) );
      dst.push( ( "user_message", user_message.as_str() ) );

      if let Some( agent_reuse ) = agent_reuse
      {
        dst.push( ( "agent_reuse", agent_reuse.as_str() ) );
      }

      dst.into_iter()
    }
  }

  /// Print output to `stdout`.
  #[ derive( Debug, Serialize, Deserialize, Former, PartialEq ) ]
  pub struct EventStdoutRaw
  {
    /// Prompt to display to `stdout`.
    pub output : String,
  }

  impl From< EventsStdoutRaw > for NodeRawKind
  {
    fn from( value : EventsStdoutRaw ) -> Self
    {
      Self::EventsStdout( value )
    }
  }

  impl format_tools::Fields< &'_ str, &'_ str > for EventStdoutRaw
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = &'v str;

    fn fields( &self ) -> impl format_tools::IteratorTrait< Item = ( &'_ str, &'_ str ) >
    {
      let mut dst = Vec::new();

      dst.push( ( "system_message", system_message.as_str() ) );
      dst.push( ( "user_message", user_message.as_str() ) );

      if let Some( agent_reuse ) = agent_reuse
      {
        dst.push( ( "agent_reuse", agent_reuse.as_str() ) );
      }

      dst.into_iter()
    }
  }

}

crate::mod_interface!
{
  own use
  {
    ScenarioRaw,
    NodeRaw,
    NodeRawKind,
  };
}