
pub( crate ) mod internal
{
  use former::Former;
  use crate::string::split::*;
  use std::collections::HashMap;

  ///
  /// Wrapper types to make transformation.
  ///

  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum OpType<T>
  {
    /// Wrapper over single element of type <T>.
    Primitive( T ),
    /// Wrapper over vector of elements of type <T>.
    Vector( Vec<T> ),
    /// Wrapper over hash map of elements of type <T>.
    Map( HashMap<String, T> ),
  }

  impl<T : Default> Default for OpType<T>
  {
    fn default() -> Self
    {
      OpType::Primitive( T::default() )
    }
  }

  ///
  /// Parsed request data.
  ///

  #[ allow( dead_code ) ]
  #[ derive( Debug, Default, PartialEq ) ]
  pub struct Request< 'a >
  {
    /// Original request string.
    pub original : &'a str,
    /// Delimeter for pairs `key:value`.
    pub key_val_delimeter : &'a str,
    /// Delimeter for commands.
    pub commands_delimeter : &'a str,
    /// Parsed subject of first command.
    pub subject : String,
    /// All subjects of the commands in request.
    pub subjects : Vec<String>,
    /// Options map of first command.
    pub map : HashMap<String, OpType<String>>,
    /// All options maps of the commands in request.
    pub maps : Vec<HashMap<String, OpType<String>>>,
  }

  ///
  /// Options for parser.
  ///

  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  #[ perform( fn parse( mut self ) -> Request< 'a > ) ]
  pub struct ParseOptions< 'a >
  {
    #[ default( "" ) ]
    src : &'a str,
    #[ default( ":" ) ]
    key_val_delimeter : &'a str,
    #[ default( ";" ) ]
    commands_delimeter : &'a str,
    #[ default( true ) ]
    quoting : bool,
    #[ default( true ) ]
    unquoting : bool,
    #[ default( true ) ]
    parsing_arrays : bool,
    #[ default( false ) ]
    several_values : bool,
    #[ default( false ) ]
    subject_win_paths_maybe : bool,
  }

  ///
  /// Adapter for ParseOptions.
  ///

  pub trait ParseOptionsAdapter< 'a >
  {
    /// A string to parse.
    fn src( &self ) -> &'a str;
    /// A delimeter for pairs `key:value`.
    fn key_val_delimeter( &self ) -> &'a str;
    /// Delimeter for commands.
    fn commands_delimeter( &self ) -> &'a str;
    /// Quoting of strings.
    fn quoting( &self ) -> bool;
    /// Unquoting of string.
    fn unquoting( &self ) -> bool;
    /// Parse arrays of values.
    fn parsing_arrays( &self ) -> bool;
    /// Append to a vector a values.
    fn several_values( &self ) -> bool;
    /// Parse subject on Windows taking into account colon in path.
    fn subject_win_paths_maybe( &self ) -> bool;

    /// Do parsing.
    fn parse( self ) -> Request< 'a >
    where
      Self : Sized,
    {
      Request::default()
    }
  }

  impl< 'a > ParseOptionsAdapter< 'a > for ParseOptions< 'a >
  {
    fn src( &self ) -> &'a str
    {
      self.src
    }
    fn key_val_delimeter( &self ) -> &'a str
    {
      self.key_val_delimeter
    }
    fn commands_delimeter( &self ) -> &'a str
    {
      self.commands_delimeter
    }
    fn quoting( &self ) -> bool
    {
      self.quoting
    }
    fn unquoting( &self ) -> bool
    {
      self.unquoting
    }
    fn parsing_arrays( &self ) -> bool
    {
      self.parsing_arrays
    }
    fn several_values( &self ) -> bool
    {
      self.several_values
    }
    fn subject_win_paths_maybe( &self ) -> bool
    {
      self.subject_win_paths_maybe
    }

    fn parse( mut self ) -> Request< 'a >
    where
      Self : Sized,
    {
      let mut result = Request::default();

      result.original = self.src();
      result.key_val_delimeter = self.key_val_delimeter();
      result.commands_delimeter = self.commands_delimeter();

      self.src = self.src.trim();

      if self.src.is_empty()
      {
        return result;
      }

      let commands;
      if self.commands_delimeter.trim().is_empty()
      {
        commands = vec![ self.src().to_string() ];
      }
      else
      {
        /* qqq : should use quoting */
        let iter = split()
        .src( self.src() )
        .delimeter( self.commands_delimeter() )
        // .quoting( self.quoting() )
        .stripping( true )
        .preserving_empty( false )
        .preserving_delimeters( false )
        .perform();
        commands = iter.map( | e | String::from( e ) ).collect::< Vec< _ > >();
      }

      for command in commands
      {
        let mut map_entries;
        if self.key_val_delimeter.trim().is_empty()
        {
          map_entries =  ( command.as_str(), None, "" );
        }
        else
        {
          map_entries = match command.split_once( self.key_val_delimeter )
          {
            Some( entries ) => ( entries.0, Some( self.key_val_delimeter ), entries.1 ),
            None => ( command.as_str(), None, "" ),
          };
        }

        let subject;
        let mut map : HashMap<String, OpType<String>> = HashMap::new();

        if map_entries.1.is_some()
        {
          /* qqq : should be isolate_right* with option quoting */
          let subject_and_key = isolate_right_or_none( map_entries.0.trim(), " " );
          subject = subject_and_key.0;
          map_entries.0 = subject_and_key.2;

          let mut join = String::from( map_entries.0 );
          join.push_str( map_entries.1.unwrap() );
          join.push_str( map_entries.2 );

          /* qqq : implement preserving_quoting */
          let mut splits = split()
          .src( join.as_str() )
          .delimeter( self.key_val_delimeter )
          .stripping( false )
          // .quoting( self.quoting )
          .preserving_empty( true )
          .preserving_delimeters( true )
          // .preserving_quoting( true )
          .perform()
          .map( | e | String::from( e ) ).collect::< Vec< _ > >();


          let mut pairs = vec![];
          for a in ( 0..splits.len() - 2 ).step_by( 2 )
          {
            let mut right = splits[ a + 2 ].clone();

            while a < ( splits.len() - 3 )
            {
              let cuts = isolate_right_or_none( &right.trim(), " " );

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
            if self.unquoting
            {
              if left.contains( "\"" ) || left.contains( "'" ) || right.contains( "\"" ) || right.contains( "'" )
              {
                unimplemented!( "not implemented" );
              }
              // left = str_unquote( left );
              // right = str_unquote( right );
            }

            pairs.push( left );
            pairs.push( right );
          }

          for a in ( 0..pairs.len() - 1 ).step_by( 2 )
          {
            let left = &pairs[ a ];
            let right = &pairs[ a + 1 ];
            // right = _.numberFromStrMaybe( right );
            // right = strToArrayMaybe( right );

            if self.several_values
            {
              unimplemented!( "not implemented" );
              // map[ left ] = _.scalarAppendOnce( map[ left ], right );
            }
            else
            {
              map.insert( left.to_string(), OpType::Primitive( right.to_string() ) );
            }
          }
        }
        else
        {
          subject = map_entries.0;
        }

        if self.unquoting
        {
          if subject.contains( "\"" ) || subject.contains( "'" )
          {
            unimplemented!( "not implemented" );
          }
          // subject = _.strUnquote( subject );
        }

        if self.subject_win_paths_maybe
        {
          unimplemented!( "not implemented" );
          // subject = win_path_subject_check( subject, map );
        }

        result.subjects.push( subject.to_string() );
        result.maps.push( map );
      }

      if result.subjects.len() > 0
      {
        result.subject = result.subjects[ 0 ].clone();
      }
      if result.maps.len() > 0
      {
        result.map = result.maps[ 0 ].clone();
      }

      result
    }
  }

  fn isolate_right_or_none<'a>( src : &'a str, delimeter : &'a str ) -> ( &'a str, Option<&'a str>, &'a str )
  {
    let result = match src.trim().rsplit_once( delimeter )
    {
      Some( entries ) => ( entries.0, Some( delimeter ), entries.1 ),
      None => ( "", None, src ),
    };
    result
  }

  ///
  /// Function to parse a string with command request.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///

  pub fn request_parse<'a>() -> ParseOptionsFormer<'a>
  {
    ParseOptions::former()
  }
}

/// Owned namespace of the module.
pub mod own
{
  use super::internal as i;

  pub use i::OpType;
  pub use i::Request;
  pub use i::ParseOptions;
  pub use i::ParseOptionsAdapter;
  pub use i::request_parse;
}

pub use own::*;

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;

  pub use i::ParseOptionsAdapter;
  pub use i::request_parse;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  use super::internal as i;

  pub use i::ParseOptionsAdapter;
}
