///!
///! CLI commands of the tool.
///!

/// Internal namespace.
mod private
{

    use clap::{ Parser, Subcommand };

    /// CLI commands of the tool.
    #[ derive ( Parser ) ]
    pub struct Cli
    {
        /// Root of the CLI commands.
        #[ command ( subcommand ) ]
        pub command: CliCommand,
    }

    /// Root of the CLI commands.
    #[ derive ( Subcommand ) ]
    pub enum CliCommand
    {
        /// OpenAI API commands.
        #[ command ( subcommand ) ]
        OpenAi(OpenAiCommand),
    }

    /// OpenAI API commands.
    #[ derive ( Subcommand ) ]
    pub enum OpenAiCommand
    {
        /// OpenAI assistants.
        #[ command ( subcommand ) ]
        Assistants
        (
            OpenAiAssistantsCommand
        ),

        /// OpenAI files.
        #[ command ( subcommand ) ]
        Files
        (
            OpenAiFilesCommand
        ),

        /// OpenAI runs.
        #[ command ( subcommand ) ]
        Runs
        (
            OpenAiRunsCommand
        ),
    }

    /// Execute OpenAI command.
    pub async fn openai
    (
        client: &Client,
        command: OpenAiCommand,
    )
    {
        match command
        {
            OpenAiCommand::Assistants( assistants_command ) =>
            {
                openai_assistants( client, assistants_command ).await;
            }

            OpenAiCommand::Files( files_command ) =>
            {
                openai_files( client, files_command ).await;
            }

            OpenAiCommand::Runs( runs_command ) =>
            {
                openai_runs( client, runs_command ).await;
            }
        }
    }

    /// OpenAI assistants.
    #[ derive ( Subcommand ) ]
    pub enum OpenAiAssistantsCommand
    {
        /// List OpenAI assistants.
        List,
    }

    /// Execute OpenAI command related to assistants.
    pub async fn openai_assistants
    (
        client: &Client,
        command: OpenAiAssistantsCommand,
    )
    {
        match command
        {
            OpenAiAssistantsCommand::List => 
            {
                openai_list_assistants( client ).await;
            }
        }
    }

    /// List OpenAI assistants.
    pub async fn openai_list_assistants( client: &Client )
    {
        let result = actions::openai_list_assistants( client ).await;

        match result
        {
            Ok ( report ) => println!( "{}", report ),
            Err ( error ) => println!( "{}", error )
        }
    }

    /// OpenAI files.
    #[ derive ( Subcommand ) ]
    pub enum OpenAiFilesCommand
    {
        /// List OpenAI files.
        List,
    }

    /// Execute OpenAI commands related to files.
    pub async fn openai_files
    (
        client: &Client,
        command: OpenAiFilesCommand,
    )
    {
        match command
        {
            OpenAiFilesCommand::List => 
            {
                openai_list_files( client ).await;
            }
        }
    }

    /// List files in your OpenAI API.
    pub async fn openai_list_files( client: &Client )
    {
        let result = actions::openai_list_files( client ).await;

        match result
        {
            Ok ( report ) => println!( "{}", report ),
            Err ( error ) => println!( "{}", error )
        }
    }

    /// OpenAI runs.
    #[ derive ( Subcommand ) ]
    pub enum OpenAiRunsCommand
    {
        /// List OpenAI runs in a thread.
        List
        {
            /// Thread ID.
            thread_id: String,
        },
    }

    /// Execute OpenAI commands related to runs.
    pub async fn openai_runs
    (
        client: &Client,
        command: OpenAiRunsCommand,
    )
    {
        match command
        {
            OpenAiRunsCommand::List => 
            {
                openai_list_runs( client ).await;
            }
        }
    }

    /// List runs in the thread in OpenAI API.
    pub async fn openai_list_runs( client: &Client )
    {
        let result = actions::openai_list_runs( client ).await;

        match result
        {
            Ok ( report ) => println!( "{}", report ),
            Err ( error ) => println!( "{}", error )
        }
    }

}

crate::mod_interface!
{
  exposed use
  {
    Cli,
    CliCommand,
    OpenAiCommand,
    OpenAiAssistantsCommand,
    OpenAiThreadsCommand,
    OpenAiRunsCommand,
    openai,
    openai_assistants,
    openai_list_assistants,
    openai_threads,
    openai_list_threads,
    openai_runs,
    openai_list_runs,
  };
}
