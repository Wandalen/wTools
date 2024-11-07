//!
//! CLI actions of the tool.
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
  use error_tools::typed::Error;
  use derive_tools::{ AsRefStr };

  use ser::DisplayFromStr;

  use crate::*;
  use client::Client;
  use debug::{ AssistantObjectWrap, FileDataWrap, RunObjectWrap };

  /// Collective enum for errors in OpenAI actions.
  #[ ser::serde_as ]
  #[ derive( Debug, Error, AsRefStr, ser::Serialize ) ]
  #[ serde( tag = "type", content = "data" ) ]
  pub enum OpenAiError
  {
    /// API error from the underlying implementation crate.
    #[ error( "OpenAI API returned error:\n{0}" ) ]
    ApiError(#[ from ] #[ serde_as( as = "DisplayFromStr" ) ] openai_api_rs::v1::error::APIError )
  }

  /// Report for `openai_list_assistants`.
  #[ derive( Debug ) ]
  pub struct OpenAiListAssistantsReport
  {
    /// OpenAI assistants.
    pub assistants: Vec<AssistantObjectWrap>
  }

  impl fmt::Display for OpenAiListAssistantsReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ >) -> Result< (), fmt::Error >
    {
      writeln!(f, "{}", AsTable::new( &self.assistants ).table_to_string_with_format( &output_format::Table::default() ) )
    }
  }

  /// List OpenAI assistants action.
  pub async fn openai_list_assistants
  (
    client : &Client,
  ) -> Result < OpenAiListAssistantsReport, OpenAiError >
  {
    let response = client.list_assistant( None, None, None, None ).await?;
    let assistants = response.data.into_iter().map(AssistantObjectWrap).collect();
    Ok( OpenAiListAssistantsReport { assistants } )
  }

  /// Report for `openai_list_files`.
  #[ derive( Debug ) ]
  pub struct OpenAiListFilesReport
  {
    /// Files in OpenAI.
    pub files : Vec<FileDataWrap>
  }

  impl fmt::Display for OpenAiListFilesReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ >) -> Result< (), fmt::Error >
    {
      writeln!(f, "{}", AsTable::new( &self.files ).table_to_string_with_format( &output_format::Table::default() ) )
    }
  }

  /// List OpenAI files action.
  pub async fn openai_list_files
  (
    client : &Client,
  ) -> Result < OpenAiListFilesReport, OpenAiError >
  {
    let response = client.file_list().await?;
    let files = response.data.into_iter().map(FileDataWrap).collect();
    Ok( OpenAiListFilesReport { files } )
  }

  /// Report for `openai_list_runs`.
  #[ derive( Debug ) ]
  pub struct OpenAiListRunsReport
  {
    /// Current OpenAI runs.
    pub runs : Vec<RunObjectWrap>
  }

  impl fmt::Display for OpenAiListRunsReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ >) -> Result< (), fmt::Error >
    {
      writeln!(f, "{}", AsTable::new( &self.runs ).table_to_string_with_format( &output_format::Table::default() ) )
    }
  }

  /// List OpenAI runs action.
  pub async fn openai_list_runs
  (
    client : &Client,
    thread_id : String,
  ) -> Result < OpenAiListRunsReport, OpenAiError >
  {
    let response = client.list_run( thread_id, None, None, None, None ).await?;
    let runs = response.data.into_iter().map(RunObjectWrap).collect();
    Ok( OpenAiListRunsReport { runs } )
  }

}

crate::mod_interface!
{
  exposed use
  {
    OpenAiError,
    OpenAiListAssistantsReport,
    openai_list_assistants,
    OpenAiListFilesReport,
    openai_list_files,
    OpenAiListRunsReport,
    openai_list_runs,
  };
}
