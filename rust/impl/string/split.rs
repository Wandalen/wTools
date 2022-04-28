// ! Spit string with a delimeter.

// xxx

pub( crate ) mod internal
{

  /* xxx : qqq : tab after git sync */

  // use woptions::*; /* xxx : use prelude */
  use crate::string::parse::OpType;

  ///
  /// Either delimeter or delimeted with the slice on its string.
  ///

  #[allow(dead_code)]
  #[ derive( Debug ) ]
  pub struct Split< 'a >
  {
    string : &'a str,
    typ : SplitType,
  }

  impl< 'a > From< Split< 'a > > for String
  {
    fn from( src : Split ) -> Self
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
        match src.find( pat )
        {
          Some( x ) => r.push( ( x, x + pat.len() ) ),
          None => (),
        }
      }

      if r.is_empty()
      {
        return None;
      }

      r.into_iter().reduce( | accum, item |
      {
        if accum.0 > item.0
        {
          item
        }
        else
        {
          if accum.1 > item.1
          {
            item
          }
          else
          {
            accum
          }
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

  impl< 'a, D : Searcher > SplitFastIterator< 'a, D >
  {
    #[ allow( dead_code ) ]
    fn new
    (
      src : &'a str,
      delimeter : D,
      preserving_empty : bool,
      preserving_delimeters : bool,
    ) -> Self
    {
      Self
      {
        iterable : src,
        delimeter,
        counter : 0,
        preserving_empty,
        preserving_delimeters,
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
          if self.iterable == "" && start == end
          {
            if self.stop_empty
            {
              return None;
            }
            else
            {
              self.counter -= 1;
              self.stop_empty = true;
              return Some( Split { string : "", typ : SplitType::Delimeted } );
            }
          }
          let mut next = &self.iterable[ ..start ];
          if start == end
          {
            if self.counter >= 3
            {
              next = &self.iterable[ ..start + 1 ];
              start += 1;
            }
          }

          self.iterable = &self.iterable[ start.. ];

          Some( Split { string : next, typ : SplitType::Delimeted } )
        }
        else
        {
          return self.next_end_split();
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

        if self.preserving_delimeters
        {
          return Some( Split { string, typ : SplitType::Delimeter } );
        }
        else
        {
          return self.next_odd_split();
        }
      }
    }
  }

  impl< 'a, D > SplitFastIterator< 'a, D >
  where
    D : Searcher
  {
    fn next_end_split( &mut self ) -> Option< Split< 'a > >
    {
      if self.iterable == ""
      {
        return None;
      }
      else
      {
        let r = Split { string : self.iterable, typ : SplitType::Delimeted };
        self.iterable = "";
        return Some( r );
      }
    }

    fn next_odd_split( &mut self ) -> Option< Split< 'a > >
    {
      match self.delimeter.pos( self.iterable )
      {
        Some( ( start, mut end ) ) =>
        {
          let mut string = &self.iterable[ ..start ];

          if start == end
          {
            string = &self.iterable[ ..start + 1 ];
            end += 1;
          }
          self.iterable = &self.iterable[ end.. ];
          return Some( Split { string, typ : SplitType::Delimeted } );
        },
        None =>
        {
          self.next_end_split()
        },
      }
    }
  }

  ///
  /// Split iterator.
  ///

  #[ derive( Debug ) ]
  pub struct SplitIterator< 'a, D >
  where
    D : Searcher
  {
    iterator : SplitFastIterator< 'a, D >,
    counter : usize,
    stripping : bool,
  }

  //

  impl< 'a, D : Searcher > SplitIterator< 'a, D >
  {
    fn new
    (
      src : &'a str,
      delimeter : D,
      preserving_empty : bool,
      preserving_delimeters : bool,
      stripping : bool,
    ) -> Self
    {
      let iterator = SplitFastIterator
      {
        iterable : src,
        delimeter,
        counter : 0,
        preserving_empty,
        preserving_delimeters,
        stop_empty : false,
      };

      Self
      {
        iterator,
        counter : 0,
        stripping,
      }
    }
  }

  impl< 'a, D > Iterator for SplitIterator< 'a, D >
  where
    D : Searcher
  {
    type Item = Split< 'a >;

    fn next( &mut self ) -> Option< Self::Item >
    {
      self.counter += 1;

      if let Some( mut split ) = self.iterator.next()
      {
        if self.stripping
        {
          split.string = split.string.trim();
          if !self.iterator.preserving_empty && split.string.is_empty() && self.counter % 2 == 0
          {
            self.counter += 1;
            return self.iterator.next();
          }
        }
        Some( split )
      }
      else
      {
        None
      }
    }
  }

  ///
  /// Options of function split.
  ///

  #[ derive( Debug ) ]
  pub struct SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone,
  {
    src : &'a str,
    delimeter : D,
    preserving_empty : bool,
    preserving_delimeters : bool,
    stripping : bool,
  }

  ///
  /// Adapter for Split Options.
  ///

  pub trait SplitOptionsAdapter< 'a, D >
  {
    /// A string to split.
    fn src( &self ) -> &'a str;
    /// A delimeter to split string.
    fn delimeter( &self ) -> D;
    /// Preserving or dropping empty splits.
    fn preserving_empty( &self ) -> bool;
    /// Preserving or dropping delimeters.
    fn preserving_delimeters( &self ) -> bool;
    /// Stripping.
    fn stripping( &self ) -> bool;
    /// Do splitting.
    fn split( self ) -> SplitIterator< 'a, D >
    where
      Self : Sized,
      D : Searcher + Default,
    {
      SplitIterator::new
      (
        self.src(),
        self.delimeter(),
        self.preserving_empty(),
        self.preserving_delimeters(),
        self.stripping()
      )
    }
  }

  //

  impl< 'a, D > SplitOptionsAdapter< 'a, D > for SplitOptions< 'a, D >
  where
    D : Searcher + Default + Clone,
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
    fn stripping( &self ) -> bool
    {
      self.stripping
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
            assert!( !self.formed, "Already formed" );
            self.$field = value;
            self
          }
        )*

        pub fn form( &mut self ) -> &mut $name< 'a >
        {
          assert!( !self.formed, "Already formed" );
          self.formed = true;
          self
        }
      }
    }
  }

  ///
  /// Former for SplitOptions.
  ///

  #[ derive( Debug ) ]
  pub struct SplitOptionsFormer< 'a >
  {
    src : &'a str,
    delimeter : OpType< &'a str >,
    preserving_empty : bool,
    preserving_delimeters : bool,
    stripping : bool,
    formed : bool,
  }
  builder_impls_from!
  (
    SplitOptionsFormer,
    ( src, &'a str ),
    ( preserving_empty, bool ),
    ( preserving_delimeters, bool ),
    ( stripping, bool )
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
        stripping : true,
        formed : false,
      }
    }

    pub fn delimeter< D : Into< OpType< &'a str > > >( &mut self, value : D ) -> &mut SplitOptionsFormer< 'a >
    {
      assert!( !self.formed, "Already formed" );
      let op_vec : OpType<&'a str> = OpType::Vector( vec![] );
      let op : OpType<&'a str> = value.into();
      self.delimeter = op_vec.append( op );
      self
    }

    pub fn perform( &mut self ) -> SplitIterator< 'a, Vec< &'a str > >
    {
      if !self.formed
      {
        self.formed = true;
      }

      let opts = SplitOptions
      {
        src : self.src,
        delimeter : self.delimeter.clone().vector().unwrap(),
        preserving_empty : self.preserving_empty,
        preserving_delimeters : self.preserving_delimeters,
        stripping : self.stripping,
      };
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
  ///   let iter = wstring_tools::string::split()
  ///   .src( "abc def" )
  ///   .delimeter( " " )
  ///   .perform();
  /// ```

  pub fn split< 'a >() -> SplitOptionsFormer< 'a >
  {
    SplitOptionsFormer::new( < &str >::default() )
  }
}

/// Owned namespace of the module.
pub mod own
{
  use super::internal as i;

  pub use i::Split;
  pub use i::SplitType;
  pub use i::SplitFastIterator;
  pub use i::SplitOptions;
  pub use i::SplitOptionsAdapter;
  pub use i::split;
}

pub use own::*;

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;

  pub use i::SplitOptionsAdapter;
  pub use i::split;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  use super::internal as i;

  pub use i::SplitOptionsAdapter;
}
