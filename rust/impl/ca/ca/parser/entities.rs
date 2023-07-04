pub( crate ) mod private
{
  use wtools::HashMap;

  /// Represents a program that contains one or more namespaces, where each namespace contains a list of commands.
  ///
  /// A `Program` consists of one or more Namespaces, where each namespace contains a list of commands. The `Namespace` can be any type that represents a namespace of commands, such as `RawCommand`, `GrammarCommand`, or `ExecutableCommand`.
  ///
  /// The program can be executed by iterating over each namespace and executing its commands sequentially or in parallel.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ RawCommand, Namespace, Program }; 
  /// # use wtools::HashMap; 
  /// let namespace1 = Namespace
  /// {
  ///   commands : vec!
  ///   [
  ///     RawCommand
  ///     {
  ///       name : "cmd1".to_string(), 
  ///       subjects : vec![ "sub1".to_string() ],
  ///       properties: HashMap::new(), 
  ///     },
  ///     RawCommand
  ///     { 
  ///       name: "cmd2".to_string(), 
  ///       subjects: vec![ "sub2".to_string(), "sub3".to_string() ], 
  ///       properties: HashMap::new(), 
  ///     }, 
  ///   ], 
  /// }; 
  /// 
  /// let namespace2 = Namespace
  /// {
  ///   commands : vec!
  ///   [
  ///     RawCommand
  ///     {
  ///       name : "cmd1".to_string(), 
  ///       subjects : vec![ "sub1".to_string() ],
  ///       properties: HashMap::new(), 
  ///     },
  ///   ], 
  /// }; 
  /// let program = Program { namespaces : vec![ namespace1, namespace2, /* ... */ ] };
  /// ```
  ///
  /// In the above example, a Program is created with two Namespace objects. Each namespace contains a different set of RawCommand objects with different sets of subjects. The Program can be executed by iterating over each namespace and executing its commands in sequence.
  /// 
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct Program< Namespace >
  {
    /// list of namespaces with commands
    pub namespaces : Vec< Namespace >,
  }

  /// Represents a namespace of commands with the specified Command type. This is done to be flexible and not to duplicate code.
  ///
  /// A `Namespace` contains a list of commands, where each command can be a `RawCommand`, `GrammarCommand`, `ExecutableCommand`, or any other command type that you define.
  /// 
  /// In the future, each namespace can be executed in parallel. This means that commands in namespace will be executed synchronous but each namespace can be executed in parallel to each other. 
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::RawCommand;
  /// # use wca::Namespace;
  /// # use wtools::HashMap;
  ///
  /// let commands = vec!
  /// [
  ///     RawCommand
  ///     {
  ///         name: "cmd1".to_string(),
  ///         subjects: vec![ "sub1".to_string() ],
  ///         properties: HashMap::new(),
  ///     },
  ///     RawCommand
  ///     {
  ///         name: "cmd2".to_string(),
  ///         subjects: vec![ "sub2".to_string(), "sub3".to_string() ],
  ///         properties: HashMap::new(),
  ///     },
  ///     RawCommand
  ///     {
  ///         name: "cmd3".to_string(),
  ///         subjects: vec![],
  ///         properties: HashMap::new(),
  ///     },
  ///     /* ... */
  /// ];
  ///
  /// let namespace = Namespace { commands };
  /// ```
  ///
  /// In the above example, a `Namespace` is created with three `RawCommand` objects. Each command has a different set of subjects.
  ///
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct Namespace< Command >
  {
      /// list of commands
      pub commands : Vec< Command >,
  }

  /// Represents a parsed command that has been extracted from an input string by a `Parser`.
  /// 
  /// The `RawCommand` struct is designed to be flexible and allow for a wide variety of commands to be parsed and represented. However, this flexibility also means that a `RawCommand` may contain invalid or unexpected data.
  /// 
  /// # Example:
  /// 
  /// ```
  /// # use wca::RawCommand;
  /// # use wtools::HashMap;
  /// RawCommand
  /// {
  ///   name : "command".to_string(),
  ///   subjects : vec![ "subject_value".to_string(), /* ... */ ],
  ///   properties : HashMap::from_iter(
  ///   [
  ///     ( "prop_name".to_string(), "raw_prop_value".to_string() ),
  ///     /* ... */
  ///   ])
  /// };
  /// ```
  /// 
  /// In the above example, a `RawCommand` instance is created with the name "command", a single subject "subject_value", and one property "prop_name" with a raw value of "raw_prop_value".
  /// 
  #[ derive( Default, Debug, Clone, PartialEq, Eq ) ]
  pub struct RawCommand
  {
    /// name of command without delimiter
    pub name : String,
    /// list of all subjects for the command
    pub subjects : Vec< String >,
    /// dictionary of properties. Each property has a name and a raw value
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
