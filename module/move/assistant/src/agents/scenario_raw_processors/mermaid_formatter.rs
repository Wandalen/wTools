//!
//! Format scenario in YAML format (pretty-printing).
//!

mod private
{
  use std::io;

  /// Represent scenarios as a graph in MermaidJS syntax.
  pub fn mermaid_formatter
  (
    scenario : &ScenarioRaw,
    writer : impl io::Write,
  ) -> Result< (), io::Error >
  {
    todo!()
  }
}

crate::mod_interface!
{
  own use mermaid_formatter;
}