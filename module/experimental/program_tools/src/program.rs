/// Internal namespace.
mod private
{
  use former::Former;

  /// Source configuration for a program.
  #[ derive( Debug, Former ) ]
  pub struct Source
  {
    /// Relative path to source file (e.g., "src/main.rs").
    pub file_path : String,
    /// Source code content as string.
    pub data : String,
  }

  /// Program configuration.
  #[ derive( Debug, Former ) ]
  pub struct Program
  {
    /// Collection of source files comprising the program.
    #[ subform_entry ]
    pub source : Vec< Source >,
  }

  /// Top-level program execution configuration.
  #[ derive( Debug, Former ) ]
  pub struct Plan
  {
    /// Embedded program definition.
    #[ subform_scalar ]
    pub program : Program,
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
