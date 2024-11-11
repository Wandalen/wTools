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
  use commands::TableConfig;

  /// Report for `openai assistants list`.
  #[ derive( Debug ) ]
  pub struct ListReport
  {
    /// Configure table formatting.
    pub table_config : TableConfig,

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
      if self.table_config.as_records
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
    table_config : TableConfig,
  ) -> Result < ListReport >
  {
    let response = client.list_assistant( None, None, None, None ).await?;
    let assistants = response.data.into_iter().map( AssistantObjectWrap ).collect();
    Ok( ListReport { table_config, assistants } )
  }
}

crate::mod_interface!
{
  own use action;
}