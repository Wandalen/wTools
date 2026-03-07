//! Conversation entry types and parsing
//!
//! An entry represents a single message in a conversation (user or assistant).

use core::fmt::Write as FmtWrite;
use std::path::PathBuf;
use crate::{ json::{ JsonValue, parse_json }, Error, Result };

/// A single conversation entry (one line in a JSONL file)
#[derive( Debug, Clone )]
pub struct Entry
{
  /// Unique identifier for this entry
  pub uuid : String,

  /// Parent entry UUID (for threaded conversations)
  pub parent_uuid : Option< String >,

  /// Timestamp (ISO 8601 format)
  pub timestamp : String,

  /// Type of entry (user or assistant)
  pub entry_type : EntryType,

  /// Working directory when message was sent
  pub cwd : PathBuf,

  /// Session UUID this entry belongs to
  pub session_id : String,

  /// Claude Code version
  pub version : String,

  /// Git branch (if in git repo)
  pub git_branch : Option< String >,

  /// User type
  pub user_type : String,

  /// Whether this is a sidechain conversation
  pub is_sidechain : bool,

  /// Message content (user or assistant)
  pub message : MessageContent,
}

/// Type of conversation entry
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum EntryType
{
  /// User message
  User,
  /// Assistant message
  Assistant,
}

/// Message content (user or assistant)
#[derive( Debug, Clone )]
pub enum MessageContent
{
  /// User message
  User( UserMessage ),
  /// Assistant message
  Assistant( AssistantMessage ),
}

/// User message content
#[derive( Debug, Clone )]
pub struct UserMessage
{
  /// User's message text
  pub content : String,

  /// Thinking metadata (if extended thinking was enabled)
  pub thinking_metadata : Option< ThinkingMetadata >,
}

/// Assistant message content
#[derive( Debug, Clone )]
pub struct AssistantMessage
{
  /// Claude model used
  pub model : String,

  /// API message ID
  pub message_id : String,

  /// Content blocks (text, thinking, `tool_use`, `tool_result`)
  pub content : Vec< ContentBlock >,

  /// Why generation stopped
  pub stop_reason : Option< String >,

  /// Stop sequence that triggered (if any)
  pub stop_sequence : Option< String >,

  /// API request ID
  pub request_id : String,
}

/// Content block in assistant message
#[derive( Debug, Clone )]
pub enum ContentBlock
{
  /// Text block
  Text
  {
    /// Text content
    text : String,
  },
  /// Thinking block (extended reasoning)
  Thinking
  {
    /// Thinking content
    thinking : String,
    /// Cryptographic signature
    signature : String,
  },
  /// Tool use block
  ToolUse
  {
    /// Tool use ID
    id : String,
    /// Tool name
    name : String,
    /// Tool input (raw JSON)
    input : JsonValue,
  },
  /// Tool result block
  ToolResult
  {
    /// Tool use ID this result belongs to
    tool_use_id : String,
    /// Result content
    content : String,
    /// Whether this is an error result
    is_error : bool,
  },
}

/// Metadata about thinking/reasoning process
#[derive( Debug, Clone )]
pub struct ThinkingMetadata
{
  /// Thinking level: "low", "medium", "high"
  pub level : String,

  /// Whether thinking is disabled
  pub disabled : bool,
}

impl Entry
{
  /// Parse an entry from a JSONL line (JSON string)
  ///
  /// This uses the hand-written JSON parser to maintain zero dependencies.
  ///
  /// # Errors
  ///
  /// Returns error if the line is not valid JSON, if required fields are missing
  /// or have the wrong type, or if the entry type is unrecognized.
  pub fn from_json_line( line : &str ) -> Result< Self >
  {
    let json = parse_json( line )
      .map_err( | e | Error::parse( 0, line, &format!( "JSON parse error: {e}" ) ) )?;

    let obj = json.as_object()
      .ok_or_else( || Error::parse( 0, line, "expected JSON object" ) )?;

    // Parse common fields
    let uuid = obj.get( "uuid" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'uuid' field" ) )?
      .to_string();

    let parent_uuid = obj.get( "parentUuid" )
      .and_then( | v | v.as_str() )
      .map( std::string::ToString::to_string );

    let timestamp = obj.get( "timestamp" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'timestamp' field" ) )?
      .to_string();

    let entry_type_str = obj.get( "type" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'type' field" ) )?;

    let entry_type = match entry_type_str
    {
      "user" => EntryType::User,
      "assistant" => EntryType::Assistant,
      _ => return Err( Error::parse( 0, line, &format!( "invalid entry type: {entry_type_str}" ) ) ),
    };

    let cwd = obj.get( "cwd" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'cwd' field" ) )?;

    let session_id = obj.get( "sessionId" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'sessionId' field" ) )?
      .to_string();

    let version = obj.get( "version" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'version' field" ) )?
      .to_string();

    let git_branch = obj.get( "gitBranch" )
      .and_then( | v | v.as_str() )
      .map( std::string::ToString::to_string );

    let user_type = obj.get( "userType" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'userType' field" ) )?
      .to_string();

    let is_sidechain = obj.get( "isSidechain" )
      .and_then( super::json::JsonValue::as_bool )
      .unwrap_or( false );

    // Parse message content based on type
    let message = match entry_type
    {
      EntryType::User => MessageContent::User( Self::parse_user_message( obj, line )? ),
      EntryType::Assistant => MessageContent::Assistant( Self::parse_assistant_message( obj, line )? ),
    };

    Ok( Entry
    {
      uuid,
      parent_uuid,
      timestamp,
      entry_type,
      cwd : PathBuf::from( cwd ),
      session_id,
      version,
      git_branch,
      user_type,
      is_sidechain,
      message,
    })
  }

  /// Parse user message from JSON object
  fn parse_user_message( obj : &std::collections::HashMap< String, JsonValue >, line : &str ) -> Result< UserMessage >
  {
    let message_obj = obj.get( "message" )
      .and_then( | v | v.as_object() )
      .ok_or_else( || Error::parse( 0, line, "missing 'message' object in user entry" ) )?;

    let content = message_obj.get( "content" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'message.content' in user entry" ) )?
      .to_string();

    let thinking_metadata = obj.get( "thinkingMetadata" )
      .and_then( | v | v.as_object() )
      .map( | tm | ThinkingMetadata
      {
        level : tm.get( "level" ).and_then( | v | v.as_str() ).unwrap_or( "low" ).to_string(),
        disabled : tm.get( "disabled" ).and_then( super::json::JsonValue::as_bool ).unwrap_or( true ),
      });

    Ok( UserMessage
    {
      content,
      thinking_metadata,
    })
  }

  /// Parse assistant message from JSON object
  fn parse_assistant_message( obj : &std::collections::HashMap< String, JsonValue >, line : &str ) -> Result< AssistantMessage >
  {
    let message_obj = obj.get( "message" )
      .and_then( | v | v.as_object() )
      .ok_or_else( || Error::parse( 0, line, "missing 'message' object in assistant entry" ) )?;

    let model = message_obj.get( "model" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'message.model'" ) )?
      .to_string();

    let message_id = message_obj.get( "id" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'message.id'" ) )?
      .to_string();

    let content_array = message_obj.get( "content" )
      .and_then( | v | v.as_array() )
      .ok_or_else( || Error::parse( 0, line, "missing 'message.content' array" ) )?;

    let mut content = Vec::new();
    for block_value in content_array
    {
      content.push( Self::parse_content_block( block_value, line )? );
    }

    let stop_reason = message_obj.get( "stop_reason" )
      .and_then( | v | v.as_str() )
      .map( std::string::ToString::to_string );

    let stop_sequence = message_obj.get( "stop_sequence" )
      .and_then( | v | v.as_str() )
      .map( std::string::ToString::to_string );

    let request_id = obj.get( "requestId" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'requestId'" ) )?
      .to_string();

    Ok( AssistantMessage
    {
      model,
      message_id,
      content,
      stop_reason,
      stop_sequence,
      request_id,
    })
  }

  /// Parse content block from JSON value
  fn parse_content_block( value : &JsonValue, line : &str ) -> Result< ContentBlock >
  {
    let obj = value.as_object()
      .ok_or_else( || Error::parse( 0, line, "content block must be an object" ) )?;

    let block_type = obj.get( "type" )
      .and_then( | v | v.as_str() )
      .ok_or_else( || Error::parse( 0, line, "missing 'type' in content block" ) )?;

    match block_type
    {
      "text" =>
      {
        let text = obj.get( "text" )
          .and_then( | v | v.as_str() )
          .ok_or_else( || Error::parse( 0, line, "missing 'text' in text block" ) )?
          .to_string();

        Ok( ContentBlock::Text { text } )
      }
      "thinking" =>
      {
        let thinking = obj.get( "thinking" )
          .and_then( | v | v.as_str() )
          .ok_or_else( || Error::parse( 0, line, "missing 'thinking' in thinking block" ) )?
          .to_string();

        let signature = obj.get( "signature" )
          .and_then( | v | v.as_str() )
          .unwrap_or( "" )
          .to_string();

        Ok( ContentBlock::Thinking { thinking, signature } )
      }
      "tool_use" =>
      {
        let id = obj.get( "id" )
          .and_then( | v | v.as_str() )
          .ok_or_else( || Error::parse( 0, line, "missing 'id' in tool_use block" ) )?
          .to_string();

        let name = obj.get( "name" )
          .and_then( | v | v.as_str() )
          .ok_or_else( || Error::parse( 0, line, "missing 'name' in tool_use block" ) )?
          .to_string();

        let input = obj.get( "input" )
          .ok_or_else( || Error::parse( 0, line, "missing 'input' in tool_use block" ) )?
          .clone();

        Ok( ContentBlock::ToolUse { id, name, input } )
      }
      "tool_result" =>
      {
        let tool_use_id = obj.get( "tool_use_id" )
          .and_then( | v | v.as_str() )
          .ok_or_else( || Error::parse( 0, line, "missing 'tool_use_id' in tool_result block" ) )?
          .to_string();

        let content = obj.get( "content" )
          .and_then( | v | v.as_str() )
          .ok_or_else( || Error::parse( 0, line, "missing 'content' in tool_result block" ) )?
          .to_string();

        let is_error = obj.get( "is_error" )
          .and_then( super::json::JsonValue::as_bool )
          .unwrap_or( false );

        Ok( ContentBlock::ToolResult { tool_use_id, content, is_error } )
      }
      _ => Err( Error::parse( 0, line, &format!( "unknown content block type: {block_type}" ) ) ),
    }
  }

  /// Extract all text content from entry for searching
  ///
  /// Returns combined text from all content blocks.
  #[must_use] 
  pub fn content_text( &self ) -> String
  {
    match &self.message
    {
      MessageContent::User( user_msg ) =>
      {
        user_msg.content.clone()
      }
      MessageContent::Assistant( assistant_msg ) =>
      {
        let mut text = String::new();
        for block in &assistant_msg.content
        {
          match block
          {
            ContentBlock::Text { text : block_text } =>
            {
              if !text.is_empty()
              {
                text.push( '\n' );
              }
              text.push_str( block_text );
            }
            ContentBlock::Thinking { thinking, .. } =>
            {
              if !text.is_empty()
              {
                text.push( '\n' );
              }
              text.push_str( thinking );
            }
            ContentBlock::ToolUse { name, input, .. } =>
            {
              if !text.is_empty()
              {
                text.push( '\n' );
              }
              write!( text, "Tool: {name} Input: {input:?}" ).unwrap();
            }
            ContentBlock::ToolResult { content, .. } =>
            {
              if !text.is_empty()
              {
                text.push( '\n' );
              }
              text.push_str( content );
            }
          }
        }
        text
      }
    }
  }

  /// Get entry type
  #[must_use] 
  pub fn entry_type( &self ) -> EntryType
  {
    self.entry_type
  }

  /// Serialize entry to JSON line (not implemented yet)
  ///
  /// # Errors
  ///
  /// Always returns error as JSON serialization is not yet implemented.
  pub fn to_json_line( &self ) -> Result< String >
  {
    // TODO: Implement JSON serialization
    Err( Error::write_failed
    (
      "entry",
      "JSON serialization not yet implemented"
    ))
  }
}

impl ThinkingMetadata
{
  /// Create empty thinking metadata
  #[must_use] 
  pub fn empty() -> Self
  {
    Self
    {
      level : "low".to_string(),
      disabled : true,
    }
  }

  /// Create thinking metadata with settings
  #[must_use] 
  pub fn new( level : String, disabled : bool ) -> Self
  {
    Self
    {
      level,
      disabled,
    }
  }
}

#[cfg( test )]
mod tests
{
  use super::*;

  #[test]
  fn test_thinking_metadata_empty()
  {
    let metadata = ThinkingMetadata::empty();
    assert!( metadata.disabled );
    assert_eq!( metadata.level, "low" );
  }

  #[test]
  fn test_thinking_metadata_new()
  {
    let metadata = ThinkingMetadata::new( "high".to_string(), false );
    assert!( !metadata.disabled );
    assert_eq!( metadata.level, "high" );
  }

  #[test]
  fn test_parse_minimal_user_entry()
  {
    let json = r#"{"uuid":"a6f3bd8c-5575-4eab-82b0-b856f7a02833","parentUuid":null,"timestamp":"2025-11-08T23:30:10.039Z","type":"user","cwd":"/home/user","sessionId":"8d795a1c-c81d-4010-8d29-b4e678272419","version":"2.0.31","gitBranch":null,"userType":"external","isSidechain":false,"message":{"role":"user","content":"Hello"},"thinkingMetadata":{"level":"low","disabled":true,"triggers":[]}}"#;

    let entry = Entry::from_json_line( json ).unwrap();

    assert_eq!( entry.uuid, "a6f3bd8c-5575-4eab-82b0-b856f7a02833" );
    assert!( entry.parent_uuid.is_none() );
    assert_eq!( entry.entry_type, EntryType::User );
    assert_eq!( entry.session_id, "8d795a1c-c81d-4010-8d29-b4e678272419" );

    #[ allow( clippy::match_wildcard_for_single_variants ) ]
    match entry.message
    {
      MessageContent::User( user_msg ) =>
      {
        assert_eq!( user_msg.content, "Hello" );
        assert!( user_msg.thinking_metadata.is_some() );
      }
      _ => panic!( "expected user message" ),
    }
  }

  #[test]
  fn test_parse_minimal_assistant_entry()
  {
    let json = r#"{"uuid":"56a226b5-0ec6-4214-af16-b13cc326f8dc","parentUuid":"a6f3bd8c-5575-4eab-82b0-b856f7a02833","timestamp":"2025-11-08T23:30:21.913Z","type":"assistant","cwd":"/home/user","sessionId":"8d795a1c-c81d-4010-8d29-b4e678272419","version":"2.0.31","gitBranch":null,"userType":"external","isSidechain":false,"message":{"model":"claude-sonnet-4-5-20250929","id":"msg_01ABC","type":"message","role":"assistant","content":[{"type":"text","text":"Hi there!"}],"stop_reason":"end_turn","stop_sequence":null,"usage":{"input_tokens":5,"output_tokens":3}},"requestId":"req_01ABC"}"#;

    let entry = Entry::from_json_line( json ).unwrap();

    assert_eq!( entry.uuid, "56a226b5-0ec6-4214-af16-b13cc326f8dc" );
    assert_eq!( entry.parent_uuid, Some( "a6f3bd8c-5575-4eab-82b0-b856f7a02833".to_string() ) );
    assert_eq!( entry.entry_type, EntryType::Assistant );

    #[ allow( clippy::match_wildcard_for_single_variants ) ]
    match entry.message
    {
      MessageContent::Assistant( asst_msg ) =>
      {
        assert_eq!( asst_msg.model, "claude-sonnet-4-5-20250929" );
        assert_eq!( asst_msg.content.len(), 1 );

        #[ allow( clippy::match_wildcard_for_single_variants ) ]
        match &asst_msg.content[ 0 ]
        {
          ContentBlock::Text { text } => assert_eq!( text, "Hi there!" ),
          _ => panic!( "expected text block" ),
        }
      }
      _ => panic!( "expected assistant message" ),
    }
  }

  #[test]
  fn test_parse_assistant_with_thinking()
  {
    let json = r#"{"uuid":"test","parentUuid":null,"timestamp":"2025-11-08T23:30:21.913Z","type":"assistant","cwd":"/tmp","sessionId":"test-session","version":"2.0.31","gitBranch":null,"userType":"external","isSidechain":false,"message":{"model":"claude-sonnet-4-5","id":"msg_01","type":"message","role":"assistant","content":[{"type":"thinking","thinking":"Let me think...","signature":"sig123"},{"type":"text","text":"Answer"}],"stop_reason":"end_turn","stop_sequence":null},"requestId":"req_01"}"#;

    let entry = Entry::from_json_line( json ).unwrap();

    #[ allow( clippy::match_wildcard_for_single_variants ) ]
    match entry.message
    {
      MessageContent::Assistant( asst_msg ) =>
      {
        assert_eq!( asst_msg.content.len(), 2 );

        #[ allow( clippy::match_wildcard_for_single_variants ) ]
        match &asst_msg.content[ 0 ]
        {
          ContentBlock::Thinking { thinking, signature } =>
          {
            assert_eq!( thinking, "Let me think..." );
            assert_eq!( signature, "sig123" );
          }
          _ => panic!( "expected thinking block" ),
        }

        #[ allow( clippy::match_wildcard_for_single_variants ) ]
        match &asst_msg.content[ 1 ]
        {
          ContentBlock::Text { text } => assert_eq!( text, "Answer" ),
          _ => panic!( "expected text block" ),
        }
      }
      _ => panic!( "expected assistant message" ),
    }
  }
}
