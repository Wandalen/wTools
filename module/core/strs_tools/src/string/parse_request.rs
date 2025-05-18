use core::default::Default;
use std::collections::HashMap;

mod private
{

  use crate::*;

  use string::
  {
    split::*,
    isolate::isolate_right, // Keep the import for the function
  };
  use super::*;

  ///
  /// Wrapper types to make transformation.
  ///
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub enum OpType< T >
  {
    /// Wrapper over single element of type `<T>`.
    Primitive( T ),
    /// Wrapper over vector of elements of type `<T>`.
    Vector( Vec< T > ),
    /// Wrapper over hash map of elements of type `<T>`.
    Map( HashMap<String, T> ),
  }

  impl<T : Default> Default for OpType< T >
  {
    fn default() -> Self
    {
      OpType::Primitive( T::default() )
    }
  }

  impl< T > From< T > for OpType< T >
  {
    fn from( value: T ) -> Self
    {
      OpType::Primitive( value )
    }
  }

  impl< T > From<Vec< T >> for OpType< T >
  {
    fn from( value: Vec< T > ) -> Self
    {
      OpType::Vector( value )
    }
  }

  #[ allow( clippy::from_over_into ) ]
  impl< T > Into<Vec< T >> for OpType< T >
  {
    fn into( self ) -> Vec< T >
    {
      match self
      {
        OpType::Vector( vec ) => vec,
        _ => unimplemented!( "not implemented" ),
      }
    }
  }

  impl<T : Clone> OpType< T >
  {
    /// Append item of `OpType` to current value. If current type is `Primitive`, then it will be converted to
    /// `Vector`.
    /// # Panics
    /// qqq: doc
    #[ must_use ]
    pub fn append( mut self, item : OpType< T > ) -> OpType< T >
    {
      let mut mut_item = item;
      match self
      {
        OpType::Primitive( value ) =>
        {
          match mut_item
          {
            OpType::Primitive( ins ) =>
            {
              let vector = vec![ value, ins ];
              OpType::Vector( vector )
            }
            OpType::Vector( ref mut vector ) =>
            {
              vector.insert( 0, value );
              mut_item
            },
            OpType::Map( _ ) => panic!( "Unexpected operation. Please, use method `insert` to insert item in hash map." ),
          }
        },
        OpType::Vector( ref mut vector ) =>
        {
          match mut_item
          {
            OpType::Primitive( ins ) =>
            {
              vector.push( ins );
              self
            }
            OpType::Vector( ref mut ins_vec ) =>
            {
              vector.append( ins_vec );
              self
            },
            OpType::Map( _ ) => panic!( "Unexpected operation. Please, use method `insert` to insert item in hash map." ),
          }
        },
        OpType::Map( _ ) => panic!( "Unexpected operation. Please, use method `insert` to insert item in hash map." ),
      }
    }

    /// Unwrap primitive value. Consumes self.
    pub fn primitive( self ) -> Option< T >
    {
      match self
      {
        OpType::Primitive( v ) => Some( v ),
        _ => None,
      }
    }

    /// Unwrap vector value. Consumes self.
    pub fn vector( self ) -> Option<Vec< T >>
    {
      match self
      {
        OpType::Vector( vec ) => Some( vec ),
        _ => None,
      }
    }
  }

  ///
  /// Parsed request data.
  ///
  #[ allow( dead_code ) ]
  #[ derive( Debug, Default, PartialEq, Eq ) ]
  pub struct Request< 'a >
  {
    /// Original request string.
    pub original : &'a str,
    /// Delimiter for pairs `key:value`.
    pub key_val_delimeter : &'a str,
    /// Delimiter for commands.
    pub commands_delimeter : &'a str,
    /// Parsed subject of first command.
    pub subject : String,
    /// All subjects of the commands in request.
    pub subjects : Vec< String >,
    /// Options map of first command.
    pub map : HashMap<String, OpType< String >>,
    /// All options maps of the commands in request.
    pub maps : Vec<HashMap<String, OpType< String >>>,
  }

  /// Newtype for the source string slice in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct ParseSrc<'a>( pub &'a str );

  // impl Default for ParseSrc<'_>
  // {
  //   fn default() -> Self
  //   {
  //     Self( "" )
  //   }
  // }

  /// Newtype for the key-value delimiter string slice in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  #[derive(Default)] // Moved derive here
  pub struct ParseKeyValDelimeter<'a>( pub &'a str );

  // impl Default for ParseKeyValDelimeter<'_> // Removed manual impl
  // {
  //   fn default() -> Self
  //   {
  //     Self( ":" )
  //   }
  // }

  /// Newtype for the commands delimiter string slice in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  #[derive(Default)] // Moved derive here
  pub struct ParseCommandsDelimeter<'a>( pub &'a str );

  // impl Default for ParseCommandsDelimeter<'_> // Removed manual impl
  // {
  //   fn default() -> Self
  //   {
  //     Self( ";" )
  //   }
  // }

  /// Newtype for the quoting boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  #[derive(Default)] // Moved derive here
  pub struct ParseQuoting( pub bool );

  // impl Default for ParseQuoting // Removed manual impl
  // {
  //   fn default() -> Self
  //   {
  //     Self( true )
  //   }
  // }

  /// Newtype for the unquoting boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  #[derive(Default)] // Moved derive here
  pub struct ParseUnquoting( pub bool );

  // impl Default for ParseUnquoting // Removed manual impl
  // {
  //   fn default() -> Self
  //   {
  //     Self( true )
  //   }
  // }

  /// Newtype for the `parsing_arrays` boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
  #[derive(Default)] // Moved derive here
  pub struct ParseParsingArrays( pub bool );

  // impl Default for ParseParsingArrays // Removed manual impl
  // {
  //   fn default() -> Self
  //   {
  //     Self( true )
  //   }
  // }

  /// Newtype for the `several_values` boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct ParseSeveralValues( pub bool );

  // impl Default for ParseSeveralValues
  // {
  //   fn default() -> Self
  //   {
  //     Self( false )
  //   }
  // }

  /// Newtype for the `subject_win_paths_maybe` boolean flag in `ParseOptions`.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default ) ]
  pub struct ParseSubjectWinPathsMaybe( pub bool );

  // impl Default for ParseSubjectWinPathsMaybe
  // {
  //   fn default() -> Self
  //   {
  //     Self( false )
  //   }
  // }

  ///
  /// Options for parser.
  ///
  #[ allow( clippy::struct_excessive_bools ) ]
  #[ derive( Debug, Default ) ] // Added Default here, Removed former::Former derive
  pub struct ParseOptions< 'a >
  {
    /// Source string slice.
    pub src : ParseSrc<'a>,
    /// Delimiter for pairs `key:value`.
    pub key_val_delimeter : ParseKeyValDelimeter<'a>,
    /// Delimeter for commands.
    pub commands_delimeter : ParseCommandsDelimeter<'a>,
    /// Quoting of strings.
    pub quoting : ParseQuoting,
    /// Unquoting of string.
    pub unquoting : ParseUnquoting,
    /// Parse arrays of values.
    pub parsing_arrays : ParseParsingArrays,
    /// Append to a vector a values.
    pub several_values : ParseSeveralValues,
    /// Parse subject on Windows taking into account colon in path.
    pub subject_win_paths_maybe : ParseSubjectWinPathsMaybe,
  }

  // impl Default for ParseOptions<'_> // Removed manual impl
  // {
  //   fn default() -> Self
  //   {
  //     Self
  //     {
  //       src : ParseSrc::default(),
  //       key_val_delimeter : ParseKeyValDelimeter::default(),
  //       commands_delimeter : ParseCommandsDelimeter::default(),
  //       quoting : ParseQuoting::default(),
  //       unquoting : ParseUnquoting::default(),
  //       parsing_arrays : ParseParsingArrays::default(),
  //       several_values : ParseSeveralValues::default(),
  //       subject_win_paths_maybe : ParseSubjectWinPathsMaybe::default(),
  //     }
  //   }
  // }

  impl< 'a > ParseOptions< 'a >
  {
    /// Do parsing.
    #[ allow( clippy::assigning_clones, clippy::too_many_lines, clippy::collapsible_if ) ]
    /// # Panics
    /// Panics if `map_entries.1` is `None` when `join.push_str` is called.
    pub fn parse( &mut self ) -> Request< 'a > // Changed to inherent method, takes &mut self
    {
      let mut result = Request
      {
        original : self.src.0, // Accessing newtype field
        key_val_delimeter : self.key_val_delimeter.0, // Accessing newtype field
        commands_delimeter : self.commands_delimeter.0, // Accessing newtype field
        ..Default::default()
      };

      self.src.0 = self.src.0.trim(); // Accessing newtype field

      if self.src.0.is_empty() // Accessing newtype field
      {
        return result;
      }

      let commands =
      if self.commands_delimeter.0.trim().is_empty() // Accessing newtype field
      {
        vec![ self.src.0.to_string() ] // Accessing newtype field
      }
      else
      {
        let iter = split()
        .src( self.src.0 ) // Accessing newtype field
        .delimeter( self.commands_delimeter.0 ) // Accessing newtype field
        .quoting( self.quoting.0 ) // Accessing newtype field
        .stripping( true )
        .preserving_empty( false )
        .preserving_delimeters( false )
        .perform();
        iter.map( String::from ).collect::< Vec< _ > >()
      };

      for command in commands
      {
        let mut map_entries;
        if self.key_val_delimeter.0.trim().is_empty() // Accessing newtype field
        {
          map_entries =  ( command.as_str(), None, "" );
        }
        else
        {
          map_entries = match command.split_once( self.key_val_delimeter.0 ) // Accessing newtype field
          {
            Some( entries ) => ( entries.0, Some( self.key_val_delimeter.0 ), entries.1 ), // Accessing newtype field
            None => ( command.as_str(), None, "" ),
          };
        }

        let subject;
        let mut map : HashMap<String, OpType< String >> = HashMap::new();

        if map_entries.1.is_some()
        {
          let options = isolate_right(); // Removed mut
          let subject_and_key = options.isolate(); // Removed field assignments
          subject = subject_and_key.0;
          map_entries.0 = subject_and_key.2;

          let mut join = String::from( map_entries.0 );
          join.push_str( map_entries.1.unwrap() );
          join.push_str( map_entries.2 );

          let mut splits = split()
          .src( join.as_str() )
          .delimeter( self.key_val_delimeter.0 ) // Accessing newtype field
          .stripping( false )
          .quoting( self.quoting.0 ) // Accessing newtype field
          .preserving_empty( true )
          .preserving_delimeters( true )
          .preserving_quoting( true )
          .perform()
          .map( String::from ).collect::< Vec< _ > >();


          let mut pairs = vec![];
          for a in ( 0..splits.len() - 2 ).step_by( 2 )
          {
            let mut right = splits[ a + 2 ].clone();

            while a < ( splits.len() - 3 )
            {
              let options = isolate_right(); // Removed mut
              let cuts = options.isolate(); // Removed field assignments

              if cuts.1.is_none()
              {
                let mut joined = splits[ a + 2 ].clone();
                joined.push_str( splits[ a + 3 ].as_str() );
                joined.push_str( splits[ a + 4 ].as_str() );

                splits[ a + 2 ] = joined;
                right = splits[ a + 2 ].clone();
                splits.remove( a + 3 );
                splits.remove( a + 4 );
                continue;
              }

              splits[ a + 2 ] = cuts.2.to_string();
              right = cuts.0.to_string();
              break;
            }

            let left = splits[ a ].clone();
            let right = right.trim().to_string();
            if self.unquoting.0 // Accessing newtype field
            {
              if left.contains( '\"' ) || left.contains( '\'' ) || right.contains( '\"' ) || right.contains( '\'' )
              {
                unimplemented!( "not implemented" );
              }
              // left = str_unquote( left );
              // right = str_unquote( right );
            }

            pairs.push( left );
            pairs.push( right );
          }

          /* */

          let str_to_vec_maybe = | src : &str | -> Option<Vec< String >>
          {
            if !src.starts_with( '[' ) || !src.ends_with( ']' )
            {
              return None;
            }

            let splits = split()
            .src( &src[ 1..src.len() - 1 ] )
            .delimeter( "," )
            .stripping( true )
            .quoting( self.quoting.0 ) // Accessing newtype field
            .preserving_empty( false )
            .preserving_delimeters( false )
            .preserving_quoting( false )
            .perform()
            .map( | e | String::from( e ).trim().to_owned() ).collect::< Vec< String > >();
            Some( splits )
          };

          /* */

          for a in ( 0..pairs.len() - 1 ).step_by( 2 )
          {
            let left = &pairs[ a ];
            let right_str = &pairs[ a + 1 ];
            let mut right = OpType::Primitive( pairs[ a + 1 ].to_string() );

            if self.parsing_arrays.0 // Accessing newtype field
            {
              if let Some( vector ) = str_to_vec_maybe( right_str )
              {
                right = OpType::Vector( vector );
              }
            }

            if self.several_values.0 // Accessing newtype field
            {
              if let Some( op ) = map.get( left )
              {
                let value = op.clone().append( right );
                map.insert( left.to_string(), value );
              }
              else
              {
                map.insert( left.to_string(), right );
              }
            }
            else
            {
              map.insert( left.to_string(), right );
            }
          }
        }
        else
        {
          subject = map_entries.0;
        }

        if self.unquoting.0 // Accessing newtype field
        {
          if subject.contains( '\"' ) || subject.contains( '\'' )
          {
            unimplemented!( "not implemented" );
          }
          // subject = _.strUnquote( subject );
        }

        if self.subject_win_paths_maybe.0 // Accessing newtype field
        {
          unimplemented!( "not implemented" );
          // subject = win_path_subject_check( subject, map );
        }

        result.subjects.push( subject.to_string() );
        result.maps.push( map );
      }

      if !result.subjects.is_empty()
      {
        result.subject = result.subjects[ 0 ].clone();
      }
      if !result.maps.is_empty()
      {
        result.map = result.maps[ 0 ].clone();
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
pub mod own
{
  #[allow(unused_imports)] use super::*;
  pub use orphan::*;
  pub use private::
  {
    OpType,
    Request,
    ParseOptions,
    // ParseOptionsAdapter, // Removed
    request_parse,
  };
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[allow(unused_imports)] use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[allow(unused_imports)] use super::*;
  pub use prelude::*; // Added
  pub use super::own as parse_request;

  pub use private::
  {
    // ParseOptionsAdapter, // Removed
    request_parse,
  };
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[allow(unused_imports)] use super::*;
  // pub use private::ParseOptionsAdapter; // Removed
}
