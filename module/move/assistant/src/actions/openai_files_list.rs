//!
//! List files in OpenAI API (action part).
//!

mod private
{

  use std::fmt;

  use format_tools::
  {
    AsTable,
    TableFormatter,
    output_format,
  };

  use crate::*;
  use client::Client;
  use debug::FileDataWrap;
  use actions::openai::Result;

  /// Report for `openai files list`.
  #[ derive( Debug ) ]
  pub struct ListReport
  {
    /// Show records as separate tables.
    pub show_records_as_tables : bool,

    /// Files in OpenAI.
    pub files : Vec< FileDataWrap >
  }

  impl fmt::Display for ListReport
  {
    fn fmt
    ( 
      &self, 
      f : &mut fmt::Formatter< '_ >
    ) -> fmt::Result
    {
      if self.show_records_as_tables
      {
        writeln!(f, "{}", AsTable::new( &self.files ).table_to_string_with_format( &output_format::Records::default() ) )
      }
      else
      {
        writeln!(f, "{}", AsTable::new( &self.files ).table_to_string_with_format( &output_format::Table::default() ) )
      }
    }
  }

  /// List OpenAI files action.
  pub async fn action
  (
    client : &Client,
    show_records_as_tables : bool,
  ) -> Result < ListReport >
  {
    let response = client.file_list().await?;
    let files = response.data.into_iter().map( FileDataWrap ).collect();
    Ok( ListReport { show_records_as_tables, files } )
  }

}

crate::mod_interface!
{
  own use action;
}