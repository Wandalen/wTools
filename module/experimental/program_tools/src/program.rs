/// Internal namespace.
mod private
{
  use former::Former;
  use crate::run_options::RunOptions;

  /// Source configuration for a program.
  #[ derive( Debug, Former ) ]
  pub struct Source
  {
    /// Relative path to source file within the workspace (e.g., `"src/main.rs"`).
    pub file_path : String,
    /// Source code content as plain text.
    pub data : String,
  }

  /// Program configuration: an ordered collection of source files.
  #[ derive( Debug, Former ) ]
  pub struct Program
  {
    /// Ordered collection of source files comprising the compilable crate.
    #[ subform_entry ]
    pub source : Vec< Source >,
    /// Optional inline Cargo manifest content. When `None`, the runner generates
    /// a minimal default manifest. Must be valid TOML when provided.
    pub manifest : Option< String >,
  }

  /// Top-level execution plan: program definition and runtime configuration.
  #[ derive( Debug, Former ) ]
  pub struct Plan
  {
    /// The program to compile and execute.
    #[ subform_scalar ]
    pub program : Program,
    /// Execution configuration. `None` means use `RunOptions::default()`.
    pub run_options : Option< RunOptions >,
  }
}

mod_interface::mod_interface!
{
  exposed use private::
  {
    Source,
    Program,
    Plan,
  };

  prelude use private::
  {
    Source,
    Program,
    Plan,
  };
}
