/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use string::
  {
    split::*,
    // isolate::isolate_right,
  };
  use std::collections::HashMap;

  ///
  /// Wrapper types to make transformation.
  ///

  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub enum OpType< T >
  {
    /// Wrapper over single element of type < T >.
    Primitive( T ),
    /// Wrapper over vector of elements of type < T >.
    Vector( Vec< T > ),
    /// Wrapper over hash map of elements of type < T >.
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

  impl< T > Into<Vec< T > > for OpType< T >
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
    /// Append item of OpType to current value. If current type is `Primitive`, then it will be converted to
    /// `Vector`.
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
    /// Delimeter for pairs `key:value`.
    pub key_val_delimeter : &'a str,
    /// Delimeter for commands.
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

  ///
  /// Options for parser.
  ///

  #[ derive( Debug, former::Former ) ]
  #[ perform( fn parse( mut self ) -> Request< 'a > ) ]
  pub struct ParseOptions< 'a >
  {
    #[ former( default = "" ) ]
    src : &'a str,
    #[ former( default = ":" ) ]
    key_val_delimeter : &'a str,
    #[ former( default = ";" ) ]
    commands_delimeter : &'a str,
    #[ former( default = true ) ]
    quoting : bool,
    #[ former( default = true ) ]
    unquoting : bool,
    #[ former( default = true ) ]
    parsing_arrays : bool,
    #[ former( default = false ) ]
    several_values : bool,
    #[ former( default = false ) ]
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
      let mut result = Request
      {
        original : self.src(),
        key_val_delimeter : self.key_val_delimeter(),
        commands_delimeter : self.commands_delimeter(),
        ..Default::default()
      };

      self.src = self.src.trim();

      if self.src.is_empty()
      {
        return result;
      }

      let commands =
      if self.commands_delimeter.trim().is_empty()
      {
        vec![ self.src().to_string() ]
      }
      else
      {
        let iter = split()
        .src( self.src() )
        .delimeter( self.commands_delimeter() )
        .quoting( self.quoting() )
        .stripping( true )
        .preserving_empty( false )
        .preserving_delimeters( false )
        .perform();
        iter.map( String::from ).collect::< Vec< _ > >()
      };

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
        let mut map : HashMap<String, OpType< String >> = HashMap::new();

        if map_entries.1.is_some()
        {
          let subject_and_key = isolate_right()
          .src( map_entries.0.trim() )
          .delimeter( " " )
          .none( false )
          .perform();
          subject = subject_and_key.0;
          map_entries.0 = subject_and_key.2;

          let mut join = String::from( map_entries.0 );
          join.push_str( map_entries.1.unwrap() );
          join.push_str( map_entries.2 );

          let mut splits = split()
          .src( join.as_str() )
          .delimeter( self.key_val_delimeter )
          .stripping( false )
          .quoting( self.quoting )
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
              let cuts = isolate_right()
              .src( right.trim() )
              .delimeter( " " )
              .none( false )
              .perform();

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
            .quoting( self.quoting )
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

            if self.parsing_arrays
            {
              if let Some( vector ) = str_to_vec_maybe( right_str )
              {
                right = OpType::Vector( vector );
              }
            }

            if self.several_values
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

        if self.unquoting
        {
          if subject.contains( '\"' ) || subject.contains( '\'' )
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
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///

  pub fn request_parse<'a>() -> ParseOptionsFormer<'a>
  {
    ParseOptions::former()
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  pub use super::private::
  {
    OpType,
    Request,
    ParseOptions,
    ParseOptionsAdapter,
    request_parse,
  };
}

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::protected as parse_request;

  pub use super::private::
  {
    ParseOptionsAdapter,
    request_parse,
  };
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::private::ParseOptionsAdapter;
}
