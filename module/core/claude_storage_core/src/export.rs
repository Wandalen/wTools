//! Export functionality for Claude Code storage sessions
//!
//! Supports exporting sessions to multiple formats:
//! - Markdown: Human-readable conversation format
//! - JSON: Machine-readable structured format
//! - Text: Simple conversation transcript

use crate::{ Session, Entry, ContentBlock, MessageContent, EntryType, Result };
use std::io::Write;
use std::path::Path;
use std::fs::File;

/// Export format specification
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum ExportFormat
{
  /// Markdown format (.md)
  Markdown,

  /// JSON format (.json)
  Json,

  /// Plain text format (.txt)
  Text,
}

impl ExportFormat
{
  /// Get file extension for format
  #[must_use] 
  pub fn extension( &self ) -> &'static str
  {
    match self
    {
      ExportFormat::Markdown => "md",
      ExportFormat::Json => "json",
      ExportFormat::Text => "txt",
    }
  }

  /// Parse format from string
  ///
  /// Note: This is not the standard `FromStr` trait to avoid confusion with
  /// Result type mismatch. CLI code uses this method directly.
  /// # Errors
  ///
  /// Returns error if the string does not match a known export format name.
  #[ allow( clippy::should_implement_trait ) ]
  pub fn from_str( s : &str ) -> Result< Self >
  {
    match s.to_lowercase().as_str()
    {
      "markdown" | "md" => Ok( ExportFormat::Markdown ),
      "json" => Ok( ExportFormat::Json ),
      "text" | "txt" => Ok( ExportFormat::Text ),
      _ =>
      {
        Err
        (
          std::io::Error::new
          (
            std::io::ErrorKind::InvalidInput,
            format!( "Unknown export format: {s}" )
          ).into()
        )
      }
    }
  }
}

/// Export a session to a writer
///
/// Streams session content to the provided writer in the specified format.
/// Memory-efficient for large sessions (doesn't load entire session into memory).
///
/// # Errors
///
/// Returns error if loading session entries fails or if writing to the writer fails.
pub fn export_session< W : Write >
(
  session : &mut Session,
  format : ExportFormat,
  writer : &mut W,
) -> Result< () >
{
  match format
  {
    ExportFormat::Markdown => export_markdown( session, writer ),
    ExportFormat::Json => export_json( session, writer ),
    ExportFormat::Text => export_text( session, writer ),
  }
}

/// Export a session to a file
///
/// Convenience function that creates a file and exports to it.
///
/// # Errors
///
/// Returns error if the output file cannot be created, if exporting fails,
/// or if flushing the file to disk fails.
pub fn export_session_to_file
(
  session : &mut Session,
  format : ExportFormat,
  output_path : &Path,
) -> Result< () >
{
  let mut file = File::create( output_path )?;
  export_session( session, format, &mut file )?;
  file.sync_all()?;
  Ok( () )
}

/// Export session as markdown
fn export_markdown< W : Write >
(
  session : &mut Session,
  writer : &mut W,
) -> Result< () >
{
  // Get session metadata before loading entries (to avoid borrow issues)
  let session_id = session.id().to_string();
  let storage_path = session.storage_path().to_path_buf();

  // Get stats first (before entries to avoid double borrow)
  let stats = session.stats()?;
  let first_timestamp = stats.first_timestamp.clone();
  let last_timestamp = stats.last_timestamp.clone();
  let total_entries = stats.total_entries;

  // Load entries
  let entries = session.entries()?;

  // Write header
  writeln!( writer, "# Session: {session_id}\n" )?;
  writeln!( writer, "**Path**: `{}`", storage_path.display() )?;
  writeln!( writer, "**Entries**: {total_entries}" )?;

  if let Some( first ) = first_timestamp
  {
    writeln!( writer, "**Created**: {first}" )?;
  }

  if let Some( last ) = last_timestamp
  {
    writeln!( writer, "**Last Updated**: {last}" )?;
  }

  writeln!( writer, "\n---\n" )?;

  // Write entries
  for ( idx, entry ) in entries.iter().enumerate()
  {
    write_markdown_entry( writer, entry, idx + 1 )?;
  }

  Ok( () )
}

/// Write a single entry in markdown format
fn write_markdown_entry< W : Write >
(
  writer : &mut W,
  entry : &Entry,
  entry_num : usize,
) -> Result< () >
{
  let role_name = match entry.entry_type
  {
    EntryType::User => "User",
    EntryType::Assistant => "Assistant",
  };

  writeln!( writer, "## Entry {entry_num} - {role_name}" )?;
  writeln!( writer, "*{}*\n", entry.timestamp )?;

  match &entry.message
  {
    MessageContent::User( user_msg ) =>
    {
      writeln!( writer, "{}\n", user_msg.content )?;
    }
    MessageContent::Assistant( assistant_msg ) =>
    {
      // Process content blocks
      for block in &assistant_msg.content
      {
        match block
        {
          ContentBlock::Thinking { thinking, .. } =>
          {
            // Collapsible thinking block
            let token_count = thinking.split_whitespace().count();
            writeln!( writer, "<details>" )?;
            writeln!( writer, "<summary>Thinking ({token_count} tokens)</summary>\n" )?;
            writeln!( writer, "{thinking}" )?;
            writeln!( writer, "</details>\n" )?;
          }
          ContentBlock::Text { text } =>
          {
            writeln!( writer, "{text}\n" )?;
          }
          ContentBlock::ToolUse { name, input, .. } =>
          {
            writeln!( writer, "**Tool Use**: `{name}`" )?;
            writeln!( writer, "```json" )?;
            writeln!( writer, "{input:#?}" )?;
            writeln!( writer, "```\n" )?;
          }
          ContentBlock::ToolResult { content, .. } =>
          {
            writeln!( writer, "**Tool Result**:" )?;
            writeln!( writer, "```" )?;
            writeln!( writer, "{content}" )?;
            writeln!( writer, "```\n" )?;
          }
        }
      }
    }
  }

  writeln!( writer, "---\n" )?;

  Ok( () )
}

/// Export session as JSON
fn export_json< W : Write >
(
  session : &mut Session,
  writer : &mut W,
) -> Result< () >
{
  use std::io::{ BufRead, BufReader };
  use std::fs::File as StdFile;

  // Get session metadata before opening file (to avoid borrow issues)
  let session_id = session.id().to_string();
  let storage_path = session.storage_path().to_path_buf();

  // Open session file
  let file = StdFile::open( &storage_path )?;
  let reader = BufReader::new( file );

  // Write JSON array opening
  writeln!( writer, "{{" )?;
  writeln!( writer, "  \"session_id\": \"{session_id}\"," )?;
  writeln!( writer, "  \"storage_path\": \"{}\",", storage_path.display() )?;
  writeln!( writer, "  \"entries\": [" )?;

  let mut first = true;

  // Stream entries
  for line in reader.lines()
  {
    let line = line?;

    if !first
    {
      writeln!( writer, "," )?;
    }
    first = false;

    // Pretty-print the JSON line with indentation
    let parsed = crate::json::parse_json( &line )?;
    write_json_value( writer, &parsed, 4 )?;
  }

  writeln!( writer, "\n  ]" )?;
  writeln!( writer, "}}" )?;

  Ok( () )
}

/// Write JSON value with indentation
fn write_json_value< W : Write >
(
  writer : &mut W,
  value : &crate::JsonValue,
  indent : usize,
) -> Result< () >
{
  use crate::JsonValue;

  let indent_str = " ".repeat( indent );

  match value
  {
    JsonValue::Null => write!( writer, "null" )?,
    JsonValue::Bool( b ) => write!( writer, "{b}" )?,
    JsonValue::Number( n ) => write!( writer, "{n}" )?,
    JsonValue::String( s ) =>
    {
      // Escape string for JSON
      write!( writer, "\"" )?;
      for ch in s.chars()
      {
        match ch
        {
          '"' => write!( writer, "\\\"" )?,
          '\\' => write!( writer, "\\\\" )?,
          '\n' => write!( writer, "\\n" )?,
          '\r' => write!( writer, "\\r" )?,
          '\t' => write!( writer, "\\t" )?,
          _ => write!( writer, "{ch}" )?,
        }
      }
      write!( writer, "\"" )?;
    }
    JsonValue::Array( arr ) =>
    {
      writeln!( writer, "[" )?;
      for ( i, item ) in arr.iter().enumerate()
      {
        write!( writer, "{}", " ".repeat( indent + 2 ) )?;
        write_json_value( writer, item, indent + 2 )?;
        if i < arr.len() - 1
        {
          writeln!( writer, "," )?;
        }
        else
        {
          writeln!( writer )?;
        }
      }
      write!( writer, "{indent_str}]" )?;
    }
    JsonValue::Object( obj ) =>
    {
      writeln!( writer, "{{" )?;
      let keys : Vec< &String > = obj.keys().collect();
      for ( i, key ) in keys.iter().enumerate()
      {
        write!( writer, "{}\"{}\": ", " ".repeat( indent + 2 ), key )?;
        write_json_value( writer, &obj[ *key ], indent + 2 )?;
        if i < keys.len() - 1
        {
          writeln!( writer, "," )?;
        }
        else
        {
          writeln!( writer )?;
        }
      }
      write!( writer, "{indent_str}}}" )?;
    }
  }

  Ok( () )
}

/// Export session as plain text
fn export_text< W : Write >
(
  session : &mut Session,
  writer : &mut W,
) -> Result< () >
{
  // Get session metadata before loading entries (to avoid borrow issues)
  let session_id = session.id().to_string();
  let storage_path = session.storage_path().to_path_buf();

  // Get stats first (before entries to avoid double borrow)
  let stats = session.stats()?;
  let total_entries = stats.total_entries;

  // Load entries
  let entries = session.entries()?;

  // Write header
  writeln!( writer, "Session: {session_id}" )?;
  writeln!( writer, "Path: {}", storage_path.display() )?;
  writeln!( writer, "Entries: {total_entries}" )?;
  writeln!( writer, "\n---\n" )?;

  // Write entries
  for entry in entries
  {
    write_text_entry( writer, entry )?;
  }

  Ok( () )
}

/// Write a single entry in text format
fn write_text_entry< W : Write >
(
  writer : &mut W,
  entry : &Entry,
) -> Result< () >
{
  let role_name = match entry.entry_type
  {
    EntryType::User => "User",
    EntryType::Assistant => "Assistant",
  };

  writeln!( writer, "[{}] {}", role_name, entry.timestamp )?;

  match &entry.message
  {
    MessageContent::User( user_msg ) =>
    {
      writeln!( writer, "{}\n", user_msg.content )?;
    }
    MessageContent::Assistant( assistant_msg ) =>
    {
      // Extract text content only (skip thinking blocks and tool use)
      for block in &assistant_msg.content
      {
        if let ContentBlock::Text { text } = block
        {
          writeln!( writer, "{text}" )?;
        }
        // Skip thinking, tool use, tool results in text format
      }
      writeln!( writer )?;
    }
  }

  writeln!( writer, "---\n" )?;

  Ok( () )
}
