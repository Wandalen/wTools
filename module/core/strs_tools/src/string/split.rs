/// Private namespace.
mod private
{

  use crate::string::parse_request::OpType;

  ///
  /// Either delimeter or delimeted with the slice on its string.
  ///

  #[ allow( dead_code ) ]
  #[ derive( Debug ) ]
  pub struct Split< 'a >
  {
    string : &'a str,
    typ : SplitType,
  }

  impl< 'a > From< Split< 'a > > for String
  {
    fn from( src : Split< '_ > ) -> Self
    {
      src.string.into()
    }
  }

  ///
  /// Either delimeter or delimeted
  ///

  #[ derive( Debug ) ]
  pub enum SplitType
  {
    /// Substring of the original string with text inbetween delimeters.
    Delimeted,
    /// Delimeter.
    Delimeter,
  }

  ///
  /// Find first match in the string.
  ///

  pub trait Searcher
  {
    /// Find positions of delimeter.
    fn pos( &self, src : &str ) -> Option< ( usize, usize ) >;
  }

  impl Searcher for &str
  {
    fn pos( &self, src : &str ) -> Option< ( usize, usize ) >
    {
      src.find( self ).map( | start | ( start, start + self.len() ) )
    }
  }

  impl Searcher for String
  {
    fn pos( &self, src : &str ) -> Option< ( usize, usize ) >
    {
      src.find( self ).map( | start | ( start, start + self.len() ) )
    }
  }

  impl Searcher for Vec<&str>
  {
    fn pos( &self, src : &str ) -> Option< ( usize, usize ) >
    {
      let mut r = vec![];
      for pat in self
      {
        if let Some( x ) =  src.find( pat )
        {
          r.push( ( x, x + pat.len() ) );
        }
      }

      if r.is_empty()
      {
        return None;
      }

      r.into_iter().reduce( | accum, item |
      {
        if accum.0 > item.0 || accum.1 > item.1
        {
          item
        }
        else
        {
          accum
        }
      })
    }
  }

  ///
  /// Split iterator.
  ///

  #[ derive( Debug ) ]
  pub struct SplitFastIterator< 'a, D >
  where
    D : Searcher
  {
    iterable : &'a str,
    counter : i32,
    delimeter : D,
    preserving_empty : bool,
    preserving_delimeters : bool,
    stop_empty : bool,
  }

  //

  impl< 'a, D : Searcher + Clone > SplitFastIterator< 'a, D >
  {
    #[ allow( dead_code, clippy::needless_pass_by_value ) ]
    fn new( o : impl SplitOptionsAdapter< 'a, D > ) -> Self
    {
      Self
      {
        iterable : o.src(),
        delimeter : o.delimeter(),
        counter : 0,
        preserving_empty : o.preserving_empty(),
        preserving_delimeters : o.preserving_delimeters(),
        stop_empty : false,
      }
    }
  }

  //

  impl< 'a, D > Iterator for SplitFastIterator< 'a, D >
  where
    D : Searcher
  {
    type Item = Split< 'a >;

    fn next( &mut self ) -> Option< Self::Item >
    {
      self.counter += 1;

      if self.counter % 2 == 1
      {
        let positions = self.delimeter.pos( self.iterable );
        if let Some( ( mut start, end ) ) = positions
        {
          if self.iterable.is_empty() && start == end
          {
            if self.stop_empty
            {
              return None;
            }

            self.counter -= 1;
            self.stop_empty = true;
            return Some( Split { string : "", typ : SplitType::Delimeted } );
          }

          if start == 0 && end != 0
          {
            return self.next();
          }

          let mut next = &self.iterable[ ..start ];
          if start == end && self.counter >= 3
          {
            next = &self.iterable[ ..=start ];
            start += 1;
          }

          self.iterable = &self.iterable[ start.. ];

          if !self.preserving_empty && next.is_empty()
          {
            return self.next();
          }

          Some( Split { string : next, typ : SplitType::Delimeted } )
        }
        else if self.iterable.is_empty()
        {
          None
        }
        else
        {
          let r = Split { string : self.iterable, typ : SplitType::Delimeted };
          self.iterable = "";
          Some( r )
        }
      }
      else
      {
        if self.delimeter.pos( self.iterable ).is_none()
        {
          self.iterable = "";
          return None;
        }

        let ( start, end ) = self.delimeter.pos( self.iterable ).unwrap();
        let string = &self.iterable[ start..end ];
        self.iterable = &self.iterable[ end.. ];

        if !self.preserving_empty && string.is_empty()
        {
          return self.next();
        }

        if self.preserving_delimeters
        {
          Some( Split { string, typ : SplitType::Delimeter } )
        }
        else
        {
          self.next()
          // return self.next_odd_split();
        }
      }
    }
  }

  ///
  /// Split iterator.
  ///

  #[ derive( Debug ) ]
  #[ allow( clippy::struct_excessive_bools ) ]
  pub struct SplitIterator< 'a >
  {
    iterator : SplitFastIterator< 'a, Vec< &'a str > >,
    src : &'a str,
    stripping : bool,
    preserving_empty : bool,
    preserving_delimeters : bool,
    #[ allow( dead_code ) ]
    preserving_quoting : bool,
    quoting : bool,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }

  //

  impl< 'a > SplitIterator< 'a >
  {
    #[ allow( clippy::needless_pass_by_value ) ]
    fn new( o : impl SplitOptionsAdapter< 'a, Vec< &'a str > > ) -> Self
    {
      let iterator;
      if !o.stripping() && !o.quoting() /* && !onDelimeter */
      {
        iterator = SplitFastIterator
        {
          iterable : o.src(),
          delimeter : o.delimeter(),
          counter : 0,
          preserving_empty : o.preserving_empty(),
          preserving_delimeters : o.preserving_delimeters(),
          stop_empty : false,
        };
      }
      else
      {
        let mut delimeter;
        if o.quoting()
        {
          delimeter = o.quoting_prefixes().clone();
          delimeter.extend( o.quoting_postfixes().clone() );
          delimeter.extend( o.delimeter() );
        }
        else
        {
          delimeter = o.delimeter();
        }

        iterator = SplitFastIterator
        {
          iterable : o.src(),
          delimeter,
          counter : 0,
          preserving_empty : true,
          preserving_delimeters : true,
          stop_empty : false,
        };
      }

      Self
      {
        iterator,
        src : o.src(),
        stripping : o.stripping(),
        preserving_empty : o.preserving_empty(),
        preserving_delimeters : o.preserving_delimeters(),
        preserving_quoting : o.preserving_quoting(),
        quoting : o.quoting(),
        quoting_prefixes : o.quoting_prefixes().clone(),
        quoting_postfixes : o.quoting_postfixes().clone(),
      }
    }
  }

  impl< 'a > Iterator for SplitIterator< 'a >
  {
    type Item = Split< 'a >;

    fn next( &mut self ) -> Option< Self::Item >
    {
      if let Some( mut split ) = self.iterator.next()
      {
        if self.quoting
        {
          split = self.quoted_split( split.string );
        }

        if self.stripping
        {
          split.string = split.string.trim();
          if !self.preserving_empty && split.string.is_empty()
          {
            return self.next();
          }
        }
        else if !self.quoting
        {
          return Some( split );
        }

        if !self.preserving_delimeters
        {
          match self.iterator.delimeter.pos( split.string )
          {
            Some( ( s, e ) ) =>
            {
              if s == 0 && e == split.string.len()
              {
                return self.next();
              }
              return Some( split );
            },
            None =>
            {
              return Some( split );
            },
          }
        }

        if !self.preserving_empty && split.string.is_empty()
        {
          return self.next();
        }

        Some( split )
      }
      else
      {
        None
      }
    }
  }

  impl< 'a > SplitIterator< 'a >
  {
    pub fn quoted_split( &mut self, split_str : &'a str ) -> Split< 'a >
    {
      match self.quoting_prefixes.iter().position( | &quote | quote == split_str )
      {
        Some( index ) =>
        {
          let postfix = self.quoting_postfixes[ index ];
          let pos = self.src.find( self.iterator.iterable ).unwrap();
          let start = pos - split_str.len();
          let end = self.iterator.iterable.find( postfix );

          if let Some( end ) = end
          {
            while self.iterator.next().unwrap().string != postfix {}
            if self.preserving_quoting
            {
              Split { string : &self.src[ start..pos + end + postfix.len() ], typ : SplitType::Delimeted }
            }
            else
            {
              Split { string : &self.src[ start + split_str.len() ..pos + end ], typ : SplitType::Delimeted }
            }
          }
          else
          {
            self.iterator.iterable = "";
            Split { string : &self.src[ start.. ], typ : SplitType::Delimeted }
          }
        },
        None => Split { string : split_str, typ : SplitType::Delimeted },
      }
    }
  }

  ///
  /// Options of function split.
  ///

  #[ derive( Debug ) ]
  #[ allow( clippy::struct_excessive_bools ) ]
  pub struct SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone,
  {
    src : &'a str,
    delimeter : D,
    preserving_empty : bool,
    preserving_delimeters : bool,
    preserving_quoting : bool,
    stripping : bool,
    quoting : bool,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }

  impl< 'a > SplitOptions< 'a, Vec< &'a str > >
  {
    /// Produces `SplitIterator`.
    #[ must_use ]
    pub fn split( self ) -> SplitIterator< 'a >
    where
      Self : Sized,
    {
      SplitIterator::new( self )
    }
  }

  impl< 'a, D > SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone
  {
    /// Produces `SplitFastIterator`.
    pub fn split_fast( self ) -> SplitFastIterator< 'a, D >
    where
      Self : Sized,
    {
      SplitFastIterator::new( self )
    }
  }

  ///
  /// Adapter for Split Options.
  ///

  pub trait SplitOptionsAdapter< 'a, D >
  where
    D : Clone
  {
    /// A string to split.
    fn src( &self ) -> &'a str;
    /// A delimeter to split string.
    fn delimeter( &self ) -> D;
    /// Preserving or dropping empty splits.
    fn preserving_empty( &self ) -> bool;
    /// Preserving or dropping delimeters.
    fn preserving_delimeters( &self ) -> bool;
    /// Preserving or dropping quotes.
    fn preserving_quoting( &self ) -> bool;
    /// Stripping.
    fn stripping( &self ) -> bool;
    /// Quoting.
    fn quoting( &self ) -> bool;
    /// Quoting prefixes.
    fn quoting_prefixes( &self ) -> &Vec< &'a str >;
    /// Quoting postfixes.
    fn quoting_postfixes( &self ) -> &Vec< &'a str >;
  }

  //

  impl< 'a, D : Searcher + Clone + Default > SplitOptionsAdapter< 'a, D > for SplitOptions< 'a, D >
  {
    fn src( &self ) -> &'a str
    {
      self.src
    }
    fn delimeter( &self ) -> D
    {
      self.delimeter.clone()
    }
    fn preserving_empty( &self ) -> bool
    {
      self.preserving_empty
    }
    fn preserving_delimeters( &self ) -> bool
    {
      self.preserving_delimeters
    }
    fn preserving_quoting( &self ) -> bool
    {
      self.preserving_quoting
    }
    fn stripping( &self ) -> bool
    {
      self.stripping
    }
    fn quoting( &self ) -> bool
    {
      self.quoting
    }
    fn quoting_prefixes( &self ) -> &Vec< &'a str >
    {
      &self.quoting_prefixes
    }
    fn quoting_postfixes( &self ) -> &Vec< &'a str >
    {
      &self.quoting_postfixes
    }
  }

  //

  macro_rules! builder_impls_from
  {
    ( $name : ident, $( ( $field : ident, $type : ty ) ),* $( , )? ) =>
    {
      impl< 'a > $name< 'a >
      {
        $(
          pub fn $field( &mut self, value : $type ) -> &mut $name< 'a >
          {
            self.$field = value;
            self
          }
        )*

        pub fn form( &mut self ) -> SplitOptions< 'a, Vec< &'a str > >
        {
          if self.quoting
          {
            if self.quoting_prefixes.is_empty()
            {
              self.quoting_prefixes = vec![ "\"", "`", "'" ];
            }
            if self.quoting_postfixes.is_empty()
            {
              self.quoting_postfixes = vec![ "\"", "`", "'" ];
            }
          }
          SplitOptions
          {
            src : self.src,
            delimeter : self.delimeter.clone().vector().unwrap(),
            preserving_empty : self.preserving_empty,
            preserving_delimeters : self.preserving_delimeters,
            preserving_quoting : self.preserving_quoting,
            stripping : self.stripping,
            quoting : self.quoting,
            quoting_prefixes : self.quoting_prefixes.clone(),
            quoting_postfixes : self.quoting_postfixes.clone(),
          }
        }
      }
    }
  }

  ///
  /// Former for `SplitOptions`.
  ///

  #[ allow( clippy::struct_excessive_bools ) ]
  #[ derive( Debug ) ]
  pub struct SplitOptionsFormer< 'a >
  {
    src : &'a str,
    delimeter : OpType< &'a str >,
    preserving_empty : bool,
    preserving_delimeters : bool,
    preserving_quoting : bool,
    stripping : bool,
    quoting : bool,
    quoting_prefixes : Vec< &'a str >,
    quoting_postfixes : Vec< &'a str >,
  }
  builder_impls_from!
  (
    SplitOptionsFormer,
    ( src, &'a str ),
    ( preserving_empty, bool ),
    ( preserving_delimeters, bool ),
    ( preserving_quoting, bool ),
    ( stripping, bool ),
    ( quoting, bool ),
    ( quoting_prefixes, Vec< &'a str > ),
    ( quoting_postfixes, Vec< &'a str > ),
  );

  impl< 'a > SplitOptionsFormer< 'a >
  {
    pub fn new< D : Into< OpType< &'a str > > >( delimeter : D ) -> SplitOptionsFormer< 'a >
    {
      let op_vec : OpType<&'a str> = OpType::Vector( vec![] );
      Self
      {
        src : "",
        delimeter : op_vec.append( delimeter.into() ),
        preserving_empty : true,
        preserving_delimeters : true,
        preserving_quoting : true,
        stripping : true,
        quoting : true,
        quoting_prefixes : vec![],
        quoting_postfixes : vec![],
      }
    }

    pub fn delimeter< D : Into< OpType< &'a str > > >( &mut self, value : D ) -> &mut SplitOptionsFormer< 'a >
    {
      let op_vec : OpType<&'a str> = OpType::Vector( vec![] );
      let op : OpType<&'a str> = value.into();
      self.delimeter = op_vec.append( op );
      self
    }

    pub fn perform( &mut self ) -> SplitIterator< 'a >
    {
      let opts = self.form();
      opts.split()
    }
  }

  ///
  /// Function to split a string.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `form()`.
  ///
  /// # Sample
  /// ```
  ///   let iter = strs_tools::string::split()
  ///   .src( "abc def" )
  ///   .delimeter( " " )
  ///   .perform();
  /// ```

  #[ must_use ]
  pub fn split< 'a >() -> SplitOptionsFormer< 'a >
  {
    SplitOptionsFormer::new( < &str >::default() )
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use orphan::*;
  pub use private::
  {
    Split,
    SplitType,
    SplitFastIterator,
    SplitOptions,
    SplitOptionsAdapter,
    split,
  };
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use super::own as split;

  pub use private::
  {
    SplitOptionsAdapter,
    split,
  };
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use private::SplitOptionsAdapter;
}
