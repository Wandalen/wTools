//!
//! List assistants in OpenAI API (action part).
//!

mod private
{

  use std::fmt;

  use format_tools::AsTable;

  use crate::*;
  use client::Client;

  use debug::AssistantObjectWrap;

  use actions::openai::{ Result, check_table_style };

  use commands::TableConfig;
  use util::display_table::display_tabular_data;

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
      display_tabular_data( &AsTable::new( &self.assistants ), f, &self.table_config )
    }
  }

  /// List OpenAI assistants action.
  pub async fn action
  (
    client : &Client,
    table_config : TableConfig,
  ) -> Result < ListReport >
  {
    check_table_style( &table_config )?;

    let response = client.list_assistant( None, None, None, None ).await?;
    let assistants = response.data.into_iter().map( AssistantObjectWrap ).collect();
    Ok( ListReport { table_config, assistants } )
  }
}

crate::mod_interface!
{
  own use action;
}