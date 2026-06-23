use core::default::Default;

// Import standard collections with proper precedence
#[ cfg( feature = "std" ) ]
use std::collections::HashMap;
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
use alloc::collections::BTreeMap as HashMap;

// Import vec macro and common types
#[ cfg( feature = "std" ) ]
use std::{ vec, vec::Vec, string::String };
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
use alloc::{ vec, vec::Vec, string::String };

mod op_type;

/// Internal implementation details exposed for testing
pub mod private {
  #[ cfg( all( feature = "string_split", feature = "string_isolate", feature = "std" ) ) ]
  use crate::string::split::split;

  // Fix(compilation-error-strs-tools): Import Src and Delimiter from isolate::private
  // Root cause: Src and Delimiter types are in isolate::private module, not re-exported to isolate::
  // Pitfall: Module re-export structure must match imports - types in private submodules require explicit private:: path
  #[ cfg( all( feature = "string_split", feature = "string_isolate", feature = "std" ) ) ]
  use crate::string::{
    isolate::isolate_right,
    isolate::private::Src,
    isolate::private::Delimiter,
  };
  use super::*;
  pub use super::op_type::OpType;

  ///
  /// Parsed request data.
  ///
  #[ allow( dead_code ) ]
  #[ derive( Debug, Default, PartialEq, Eq ) ]
  pub struct Request<'a> {
    /// Original request string.
    pub original: &'a str,
    /// Delimiter for pairs `key:value`.
    pub key_val_delimiter: &'a str,
    /// Delimiter for commands.
    pub commands_delimiter: &'a str,
    /// Parsed subject of first command.
    pub subject: String,
    /// All subjects of the commands in request.
    pub subjects: Vec< String >,
    /// Options map of first command.
    pub map: HashMap<String, OpType<String>>,
    /// All options maps of the commands in request.
    pub maps: Vec<HashMap<String, OpType<String>>>,
  }

  /// Newtype for the source string slice in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct ParseSrc<'a>(pub &'a str);

  /// Newtype for the key-value delimiter string slice in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  pub struct ParseKeyValDelimiter<'a>(pub &'a str);

  impl Default for ParseKeyValDelimiter<'_>
  {
    fn default() -> Self
    {
      Self( ": " )
    }
  }

  /// Newtype for the commands delimiter string slice in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  pub struct ParseCommandsDelimiter<'a>(pub &'a str);

  impl Default for ParseCommandsDelimiter<'_>
  {
    fn default() -> Self
    {
      Self( ";" )
    }
  }

  /// Newtype for the quoting boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  pub struct ParseQuoting(pub bool);

  impl Default for ParseQuoting
  {
    fn default() -> Self
    {
      Self( true )
    }
  }

  /// Newtype for the unquoting boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  pub struct ParseUnquoting(pub bool);

  impl Default for ParseUnquoting
  {
    fn default() -> Self
    {
      Self( true )
    }
  }

  /// Newtype for the `parsing_arrays` boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  pub struct ParseParsingArrays(pub bool);

  impl Default for ParseParsingArrays
  {
    fn default() -> Self
    {
      Self( true )
    }
  }

  /// Newtype for the `several_values` boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct ParseSeveralValues(pub bool);

  /// Newtype for the `subject_win_paths_maybe` boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct ParseSubjectWinPathsMaybe(pub bool);

  ///
  /// Options for parser.
  ///
  #[ allow( clippy::struct_excessive_bools ) ]
  #[ derive( Debug, Default ) ] // Added Default here, Removed former::Former derive
  pub struct ParseOptions<'a> {
    /// Source string slice.
    pub src: ParseSrc<'a>,
    /// Delimiter for pairs `key:value`.
    pub key_val_delimiter: ParseKeyValDelimiter<'a>,
    /// Delimiter for commands.
    pub commands_delimiter: ParseCommandsDelimiter<'a>,
    /// Quoting of strings.
    pub quoting: ParseQuoting,
    /// Unquoting of string.
    pub unquoting: ParseUnquoting,
    /// Parse arrays of values.
    pub parsing_arrays: ParseParsingArrays,
    /// Append to a vector a values.
    pub several_values: ParseSeveralValues,
    /// Parse subject on Windows taking into account colon in path.
    pub subject_win_paths_maybe: ParseSubjectWinPathsMaybe,
  }

  impl<'a> ParseOptions<'a> {
    /// Do parsing.
    #[ allow( clippy::assigning_clones, clippy::too_many_lines, clippy::collapsible_if ) ]
    /// # Panics
    /// Panics if `map_entries.1` is `None` when `join.push_str` is called.
    #[ cfg( all( feature = "string_split", feature = "string_isolate", feature = "std" ) ) ]
    pub fn parse(&mut self) -> Request<'a> // Changed to inherent method, takes &mut self
    {
      let mut result = Request {
        original: self.src.0,                          // Accessing newtype field
        key_val_delimiter: self.key_val_delimiter.0,   // Accessing newtype field
        commands_delimiter: self.commands_delimiter.0, // Accessing newtype field
        ..Default::default()
      };

      self.src.0 = self.src.0.trim(); // Accessing newtype field

      if self.src.0.is_empty()
      // Accessing newtype field
      {
        return result;
      }

      let commands = if self.commands_delimiter.0.trim().is_empty()
      // Accessing newtype field
      {
        vec![self.src.0.to_string()] // Accessing newtype field
      } else {
        let iter = split()
        .src( self.src.0 ) // Accessing newtype field
        .delimiter( self.commands_delimiter.0 ) // Accessing newtype field
        .quoting( self.quoting.0 ) // Accessing newtype field
        .stripping( true )
        .preserving_empty( false )
        .preserving_delimiters( false )
        .perform();
        iter.map(String::from).collect::<Vec< _ >>()
      };

      for command in commands {
        let mut map_entries;
        if self.key_val_delimiter.0.trim().is_empty()
        // Accessing newtype field
        {
          map_entries = (command.as_str(), None, "");
        } else {
          map_entries = match command.split_once( self.key_val_delimiter.0 ) // Accessing newtype field
          {
            Some( entries ) => ( entries.0, Some( self.key_val_delimiter.0 ), entries.1 ), // Accessing newtype field
            None => ( command.as_str(), None, "" ),
          };
        }

        let subject;
        let mut map: HashMap<String, OpType<String>> = HashMap::new();

        if let Some(map_entry_1) = map_entries.1 {
          let mut options = isolate_right();
          options.src = Src( map_entries.0 );
          options.delimiter = Delimiter( " " );
          let subject_and_key = options.isolate();
          subject = subject_and_key.0;
          map_entries.0 = subject_and_key.2;

          let mut join = String::from(map_entries.0);
          join.push_str(map_entry_1);
          join.push_str(map_entries.2);

          let mut splits = split()
          .src( join.as_str() )
          .delimiter( self.key_val_delimiter.0 ) // Accessing newtype field
          .stripping( false )
          .quoting( self.quoting.0 ) // Accessing newtype field
          .preserving_empty( true )
          .preserving_delimiters( true )
          .preserving_quoting( true )
          .perform()
          .map( String::from ).collect::< Vec<  _  > >();

          let mut pairs = vec![];
          for a in (0..splits.len() - 2).step_by(2) {
            let mut right = splits[a + 2].clone();

            while a < (splits.len() - 3) {
              let mut options = isolate_right();
              options.src = Src( &splits[a + 2] );
              options.delimiter = Delimiter( " " );
              let cuts = options.isolate();

              if cuts.1.is_none() {
                let mut joined = splits[a + 2].clone();
                joined.push_str(splits[a + 3].as_str());
                joined.push_str(splits[a + 4].as_str());

                splits[a + 2] = joined;
                right = splits[a + 2].clone();
                splits.remove(a + 3);
                splits.remove(a + 4);
                continue;
              }

              let cuts_2_owned = cuts.2.to_string();
              let cuts_0_owned = cuts.0.to_string();
              splits[a + 2] = cuts_2_owned;
              right = cuts_0_owned;
              break;
            }

            let left = splits[a].clone();
            let right = right.trim().to_string();
            if self.unquoting.0
            // Accessing newtype field
            {
              // left = str_unquote( left );
              // right = str_unquote( right );
            }

            pairs.push(left);
            pairs.push(right);
          }

          /* */

          let str_to_vec_maybe = |src: &str| -> Option<Vec< String >> {
            if !src.starts_with('[') || !src.ends_with(']') {
              return None;
            }

            let splits = split()
            .src( &src[ 1..src.len() - 1 ] )
            .delimiter( "," )
            .stripping( true )
            .quoting( self.quoting.0 ) // Accessing newtype field
            .preserving_empty( false )
            .preserving_delimiters( false )
            .preserving_quoting( false )
            .perform()
            .map( | e | String::from( e ).trim().to_owned() ).collect::< Vec<  String  > >();
            Some(splits)
          };

          /* */

          for a in (0..pairs.len() - 1).step_by(2) {
            let left = &pairs[a];
            let right_str = &pairs[a + 1];
            let mut right = OpType::Primitive(pairs[a + 1].clone());

            if self.parsing_arrays.0
            // Accessing newtype field
            {
              if let Some(vector) = str_to_vec_maybe(right_str) {
                right = OpType::Vector(vector);
              }
            }

            if self.several_values.0
            // Accessing newtype field
            {
              if let Some(op) = map.get(left) {
                let value = op.clone().append(right);
                map.insert(left.clone(), value);
              } else {
                map.insert(left.clone(), right);
              }
            } else {
              map.insert(left.clone(), right);
            }
          }
        } else {
          subject = map_entries.0;
        }

        if self.unquoting.0
        // Accessing newtype field
        {
          // subject = _.strUnquote( subject );
        }

        if self.subject_win_paths_maybe.0
        // Accessing newtype field
        {
          // subject = win_path_subject_check( subject, map );
        }

        result.subjects.push(subject.to_string());
        result.maps.push(map);
      }

      if !result.subjects.is_empty() {
        result.subject = result.subjects[0].clone();
      }
      if !result.maps.is_empty() {
        result.map = result.maps[0].clone();
      }

      result
    }
  }

  ///
  /// Function to parse a string with command request.
  ///
  /// It produces `former`. To convert `former` into options and run algorithm of splitting call `perform()`.
  ///
  ///
  ///
  #[ must_use ]
  #[ cfg( all( feature = "string_split", feature = "string_isolate", feature = "std" ) ) ]
  pub fn request_parse<'a>() -> ParseOptions<'a> // Return ParseOptions directly
  {
    ParseOptions::default()
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;


/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use orphan::*;
  pub use private::{
    OpType,
    Request,
    ParseOptions,
    ParseSrc,
    ParseKeyValDelimiter,
    ParseCommandsDelimiter,
    ParseQuoting,
    ParseUnquoting,
    ParseParsingArrays,
    ParseSeveralValues,
    ParseSubjectWinPathsMaybe,
  };
  #[ cfg( all( feature = "string_split", feature = "string_isolate", feature = "std" ) ) ]
  pub use private::request_parse;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*; // Added
  pub use super::own as parse_request;

  #[ cfg( all( feature = "string_split", feature = "string_isolate", feature = "std" ) ) ]
  pub use private::request_parse;
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude {
  #[ allow( unused_imports ) ]
  use super::*;
}
