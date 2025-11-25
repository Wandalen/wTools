/// Internal namespace.
mod private
{
  use former::Former;

  /// Source configuration for a program.
  #[ derive( Debug, Former ) ]
  pub struct Source
  {
    pub file_path : String,
    pub data : String,
  }

  /// Program configuration.
  #[ derive( Debug, Former ) ]
  pub struct Program
  {
    #[ subform_entry ]
    pub source : Vec< Source >,
  }

  /// Plan for compiling and running a Rust program.
  #[ derive( Debug, Former ) ]
  pub struct Plan
  {
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
