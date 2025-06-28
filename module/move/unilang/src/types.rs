//! # Types
//!
//! This module defines the parsing and validation logic for the various argument types (`kind`) supported by `unilang`.
//! It is responsible for converting raw string inputs from the command line into strongly-typed Rust values.

use crate::data::Kind;
use std::path::{ Path, PathBuf };
use url::Url;
use chrono::{ DateTime, FixedOffset };
use regex::Regex;
use core::fmt;

/// Represents a parsed and validated value of a specific kind.
#[derive( Debug, Clone )]
pub enum Value
{
  /// A sequence of characters.
  String( String ),
  /// A whole number.
  Integer( i64 ),
  /// A floating-point number.
  Float( f64 ),
  /// A true or false value.
  Boolean( bool ),
  /// A URI representing a file system path.
  Path( PathBuf ),
  /// A `Path` that must point to a file.
  File( PathBuf ),
  /// A `Path` that must point to a directory.
  Directory( PathBuf ),
  /// A string that must be one of the predefined, case-sensitive choices.
  Enum( String ),
  /// A Uniform Resource Locator.
  Url( Url ),
  /// A date and time.
  DateTime( DateTime< FixedOffset > ),
  /// A regular expression pattern string.
  Pattern( Regex ),
}

impl PartialEq for Value
{
  fn eq( &self, other: &Self ) -> bool
  {
    match ( self, other )
    {
      ( Self::String( l ), Self::String( r ) ) | ( Self::Enum( l ), Self::Enum( r ) ) => l == r,
      ( Self::Integer( l ), Self::Integer( r ) ) => l == r,
      ( Self::Float( l ), Self::Float( r ) ) => l == r,
      ( Self::Boolean( l ), Self::Boolean( r ) ) => l == r,
      ( Self::Path( l ), Self::Path( r ) ) | ( Self::File( l ), Self::File( r ) ) | ( Self::Directory( l ), Self::Directory( r ) ) => l == r,
      ( Self::Url( l ), Self::Url( r ) ) => l == r,
      ( Self::DateTime( l ), Self::DateTime( r ) ) => l == r,
      ( Self::Pattern( l ), Self::Pattern( r ) ) => l.as_str() == r.as_str(),
      _ => false,
    }
  }
}

impl fmt::Display for Value
{
  fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Value::String( s ) | Value::Enum( s ) => write!( f, "{s}" ),
      Value::Integer( i ) => write!( f, "{i}" ),
      Value::Float( fl ) => write!( f, "{fl}" ),
      Value::Boolean( b ) => write!( f, "{b}" ),
      Value::Path( p ) | Value::File( p ) | Value::Directory( p ) => write!( f, "{}", p.to_string_lossy() ),
      Value::Url( u ) => write!( f, "{u}" ),
      Value::DateTime( dt ) => write!( f, "{}", dt.to_rfc3339() ),
      Value::Pattern( r ) => write!( f, "{}", r.as_str() ),
    }
  }
}

/// An error that can occur during type parsing or validation.
#[derive( Debug, Clone, PartialEq, Eq )]
pub struct TypeError
{
  /// The expected kind of the value.
  pub expected_kind: Kind,
  /// A message describing the reason for the failure.
  pub reason: String,
}

/// Parses a raw string input into a `Value` based on the specified `Kind`.
///
/// # Errors
///
/// Returns a `TypeError` if the input string cannot be parsed into the
/// specified `Kind` or if it fails validation for that `Kind`.
pub fn parse_value( input: &str, kind: &Kind ) -> Result< Value, TypeError >
{
  match kind
  {
    Kind::String => Ok( Value::String( input.to_string() ) ),
    Kind::Integer => input.parse::< i64 >().map( Value::Integer ).map_err( |e| TypeError { expected_kind: kind.clone(), reason: e.to_string() } ),
    Kind::Float => input.parse::< f64 >().map( Value::Float ).map_err( |e| TypeError { expected_kind: kind.clone(), reason: e.to_string() } ),
    Kind::Boolean =>
    {
      match input.to_lowercase().as_str()
      {
        "true" | "1" | "yes" => Ok( Value::Boolean( true ) ),
        "false" | "0" | "no" => Ok( Value::Boolean( false ) ),
        _ => Err( TypeError { expected_kind: kind.clone(), reason: "Invalid boolean value".to_string() } ),
      }
    }
    Kind::Path =>
    {
      if input.is_empty()
      {
        return Err( TypeError { expected_kind: kind.clone(), reason: "Path cannot be empty".to_string() } );
      }
      Ok( Value::Path( PathBuf::from( input ) ) )
    },
    Kind::File =>
    {
      let path = Path::new( input );
      if path.is_dir()
      {
        return Err( TypeError { expected_kind: kind.clone(), reason: "Expected a file, but found a directory".to_string() } );
      }
      // Further validation (like existence) would be a validation rule, not a type error.
      Ok( Value::File( path.to_path_buf() ) )
    },
    Kind::Directory =>
    {
      let path = Path::new( input );
      if path.is_file()
      {
        return Err( TypeError { expected_kind: kind.clone(), reason: "Expected a directory, but found a file".to_string() } );
      }
      // Further validation (like existence) would be a validation rule, not a type error.
      Ok( Value::Directory( path.to_path_buf() ) )
    },
    Kind::Enum( choices ) =>
    {
      if choices.contains( &input.to_string() )
      {
        Ok( Value::Enum( input.to_string() ) )
      }
      else
      {
        Err( TypeError { expected_kind: kind.clone(), reason: format!( "Value '{input}' is not one of the allowed choices: {choices:?}" ) } )
      }
    },
    Kind::Url => Url::parse( input ).map( Value::Url ).map_err( |e| TypeError { expected_kind: kind.clone(), reason: e.to_string() } ),
    Kind::DateTime => DateTime::parse_from_rfc3339( input ).map( Value::DateTime ).map_err( |e| TypeError { expected_kind: kind.clone(), reason: e.to_string() } ),
    Kind::Pattern => Regex::new( input ).map( Value::Pattern ).map_err( |e| TypeError { expected_kind: kind.clone(), reason: e.to_string() } ),
  }
}
