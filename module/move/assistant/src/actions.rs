mod private
{

  /// Collective enum for errors in OpenAI actions.
  #[ derive( Debug, Error )]
  pub enum OpenAiError
  {
    /// API error from the underlying implementation crate.
    #[ error("OpenAI API returned error") ]
    ApiError(#[ from ] openai_api_rs::v1::error::APIError )
  }

  /// Report for `openai_list_assistants`.
  pub struct OpenAiListAssistantsReport
  {
    pub assistants: Vec<AssistantObject>
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
    let assistants = client.list_assistant( None, None, None, None ).await?.data;
    Ok( OpenAiListAssistantsReport { assistants } )
  }

  /// Report for `openai_list_files`.
  pub struct OpenAiListFilesReport
  {
    pub files : Vec<FileData>
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
    let files = client.file_list().await?.data;
    Ok( OpenAiListFilesReport { files } )
  }

  /// Report for `openai_list_runs`.
  pub struct OpenAiListRunsReport
  {
    pub runs : Vec<RunObject>
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
    let runs = client.list_run( thread_id, None, None, None, None ).await?.data;
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
