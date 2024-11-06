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

        /// OpenAI threads.
        #[ command ( subcommand ) ]
        Threads
        (
            OpenAiThreadsCommand
        ),

        /// OpenAI runs.
        #[ command ( subcommand ) ]
        Runs
        (
            OpenAiRunsCommand
        ),
    }

    /// OpenAI assistants.
    #[ derive ( Subcommand ) ]
    pub enum OpenAiAssistantsCommand
    {
        /// List OpenAI assistants.
        List,
    }

    /// OpenAI threads.
    #[ derive ( Subcommand ) ]
    pub enum OpenAiThreadsCommand
    {
        /// List OpenAI threads.
        List,
    }

    /// OpenAI runs.
    #[ derive ( Subcommand ) ]
    pub enum OpenAiRunsCommand
    {
        /// List OpenAI runs.
        List,
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
  };
}
