//!
//! List files in OpenAI API (action part).
//!

mod private
{

  use std::fmt;

  use format_tools::AsTable;

  use crate::*;
  use client::Client;

  use debug::FileDataWrap;

  use actions::openai::{ Result, check_table_style };

  use commands::TableConfig;
  use util::display_table::display_tabular_data;

  /// Report for `openai files list`.
  #[ derive( Debug ) ]
  pub struct ListReport
  {
    /// Configure table formatting.
    pub table_config : TableConfig,

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
      display_tabular_data( &AsTable::new( &self.files ), f, &self.table_config )
    }
  }

  /// List OpenAI files action.
  pub async fn action
  (
    client : &Client,
    table_config : TableConfig,
  ) -> Result < ListReport >
  {
    check_table_style( &table_config )?;

    let response = client.file_list().await?;
    let files = response.data.into_iter().map( FileDataWrap ).collect();
    Ok( ListReport { table_config, files } )
  }

}

crate::mod_interface!
{
  own use action;
}