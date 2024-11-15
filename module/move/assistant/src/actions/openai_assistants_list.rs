//!
//! List assistants in OpenAI API (action part).
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
  use debug::AssistantObjectWrap;
  use actions::openai::Result;

  /// Report for `openai assistants list`.
  #[ derive( Debug ) ]
  pub struct ListReport
  {
    /// Show records as separate tables.
    pub show_records_as_tables : bool,

    /// OpenAI assistants.
    pub assistants: Vec< AssistantObjectWrap >
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
        writeln!(f, "{}", AsTable::new( &self.assistants ).table_to_string_with_format( &output_format::Records::default() ) )
      }
      else
      {
        writeln!(f, "{}", AsTable::new( &self.assistants ).table_to_string_with_format( &output_format::Table::default() ) )
      }
    }
  }

  /// List OpenAI assistants action.
  pub async fn action
  (
    client : &Client,
    show_records_as_tables : bool,
  ) -> Result < ListReport >
  {
    let response = client.list_assistant( None, None, None, None ).await?;
    let assistants = response.data.into_iter().map( AssistantObjectWrap ).collect();
    Ok( ListReport { show_records_as_tables, assistants } )
  }
}

crate::mod_interface!
{
  own use action;
}