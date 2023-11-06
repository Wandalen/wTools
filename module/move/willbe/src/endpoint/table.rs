mod private 
{
  use std::fs;

    // use wtools::error::Result;
  use anyhow::*;

    /// Create table
  pub fn create_table( path: String ) -> Result< () >
  {
    let entries = fs::read_dir(path)?;
    for entry in entries 
    {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() 
      {
        if let Some( dir_name ) = path.file_name() 
          {
            if let Some( dir_name_str ) = dir_name.to_str() 
            {
              println!( "{}", dir_name_str );
            }
          }
        }
      }
      dbg!( "Create table endpoint" );
      Ok( () )
    }
    
}

crate::mod_interface!
{
  /// Create Table.
  prelude use create_table;
}
