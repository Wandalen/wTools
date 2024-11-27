//!
//! Format scenario in PlantUML diagram.
//!

mod private
{
  use std::io;

  use crate::*;
  use agents::scenario_raw::ScenarioRaw;

  /// Format scenario in PlantUML diagram.
  pub fn plantuml_formatter
  (
    scenario : &ScenarioRaw,
    writer : &mut impl io::Write,
  ) -> Result< (), io::Error >
  {
    writer.write( b"@startuml\n" )?;

    for node in &scenario.nodes
    {
      writer.write( b"json " )?;
      writer.write( node.id.as_bytes() )?;
      writer.write( b" {\n" )?;

      writer.write( b"  \"type\": \"" )?;
      writer.write( node.r#type.as_bytes() )?;
      writer.write( b"\"" )?;

      if node.params.len() > 0
      {
        writer.write( b"," )?; 
      }

      writer.write( b"\n" )?;

      for ( i, ( key, value ) ) in node.params.iter().enumerate()
      {
        writer.write( b"  \"" )?;
        writer.write( key.as_bytes() )?;
        writer.write( b"\": \"" )?;
        writer.write( value.as_bytes() )?;
        writer.write( b"\"" )?;

        if i != node.params.len() - 1
        {
          writer.write( b"," )?;
        }

        writer.write( b"\n" )?;
      }

      writer.write( b"}\n" )?;
    }

    writer.write( b"json ::scenario::termination {\n" )?;
    writer.write( b"}\n" )?;

    for node in &scenario.nodes
    {
      writer.write( node.id.as_bytes() )?;
      writer.write( b" --> " )?;
      writer.write( node.next.as_bytes() )?;
      writer.write( b" : next\n" )?;
    }

    writer.write( b"@enduml" )?;
    Ok( () )
  }
}

crate::mod_interface!
{
  own use plantuml_formatter;
}