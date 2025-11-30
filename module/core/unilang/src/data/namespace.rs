//! Namespace and output data structures
//!
//! Provides hierarchical organization for commands through namespaces,
//! and standardized output data structure for command execution results.

use super::command_definition::CommandDefinition;

  ///
  /// Represents a namespace within the command system.
  ///
  /// Namespaces provide hierarchical organization for commands, allowing
  /// related commands to be grouped together (e.g., `math.add`, `math.subtract`).
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize ) ]
  pub struct Namespace
  {
    /// The name of the namespace.
    pub name : String,
    /// Commands that belong to this namespace.
    pub commands : Vec< CommandDefinition >,
  }

  ///
  /// Represents the output of a successfully executed command.
  ///
  /// This struct provides a standardized way to return data from command execution,
  /// including both the actual content and metadata about its format.
  #[ derive( Debug, Clone /*, Former*/ ) ]
  pub struct OutputData
  {
    /// The actual content produced by the command.
    pub content : String,
    /// The format of the content (e.g., "`text`", "`json`", "`xml`").
    pub format : String,
    /// Execution time in milliseconds (if available).
    ///
    /// This field captures how long the command took to execute, useful for
    /// performance monitoring and optimization. If not set, the command execution
    /// time was not measured.
    pub execution_time_ms : Option< u64 >,
  }

  impl OutputData
  {
    /// Creates a new `OutputData` with the specified content and format.
    ///
    /// The execution time is initially set to `None` and will be populated
    /// by the interpreter during command execution.
    ///
    /// # Examples
    ///
    /// ```
    /// use unilang::data::OutputData;
    ///
    /// let output = OutputData::new( "Hello, World!", "text" );
    /// assert_eq!( output.content, "Hello, World!" );
    /// assert_eq!( output.format, "text" );
    /// assert_eq!( output.execution_time_ms, None );
    /// ```
    #[ must_use ]
    pub fn new( content : impl Into< String >, format : impl Into< String > ) -> Self
    {
      Self
      {
        content : content.into(),
        format : format.into(),
        execution_time_ms : None,
      }
    }
  }
