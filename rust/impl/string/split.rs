// #![ warn( missing_docs ) ]
// #![ warn( missing_debug_implementations ) ]

use former::Former;

///
/// Either delimeter or delimeted with the slice on its string.
///

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
/// Split iterator.
///

#[ derive( Debug ) ]
pub struct SplitIterator< 'a >
{
  iterator : std::iter::Peekable< std::str::Split< 'a, &'a str > >,
  counter : i32,
  delimeter : &'a str,
}

//

impl< 'a > SplitIterator< 'a >
{
  fn new( src : &'a str, delimeter : &'a str ) -> Self
  {
    let counter = 0;
    // let delimeter = delimeter.clone();
    let delimeter_slice = unsafe
    {
      let slice = core::slice::from_raw_parts( delimeter.as_ptr(), delimeter.len() );
      core::str::from_utf8_unchecked( slice )
    };
    let iterator = src.split( delimeter_slice ).peekable();
    Self
    {
      iterator,
      delimeter,
      counter,
    }
  }
}

//

impl< 'a > Iterator for SplitIterator< 'a >
{
  type Item = Split< 'a >;

  fn next( &mut self ) -> Option< Self::Item >
  {
    self.counter += 1;
    if self.counter % 2 == 1
    {
      let next = self.iterator.next();
      if let Some( next ) = next
      {
        Some( Split { string : next, typ : SplitType::Delimeted } )
      }
      else
      {
        None
      }
    }
    else
    {
      if self.iterator.peek().is_none()
      {
        self.iterator.next();
        return None;
      }
      Some( Split { string : self.delimeter, typ : SplitType::Delimeter } )
      // Some( Split::Delimeter( self.delimeter.clone() ) )
    }
  }
}

///
/// Options of function split.
///

#[ derive( Debug ) ]
#[ derive( Former ) ]
#[ form_after( fn split( self ) -> SplitIterator< 'a > ) ]
pub struct SplitOptions< 'a >
{

  #[ default( "" ) ]
  src : &'a str,
  #[ default( "" ) ]
  delimeter : &'a str,
  #[ default( true ) ]
  preserving_empty : bool,
  #[ default( true ) ]
  preserving_delimeters : bool,

  // #[ method ]
  // fn split( self ) -> SplitIterator< 'a >
  // where
  //   Self : Sized,
  // {
  //   SplitIterator::new( self.src(), self.delimeter() )
  // }

  // result : HashMap< Box< str >, Box< str > >,
}

///
/// Adapter for Split Options.
///

pub trait SplitOptionsAdapter< 'a >
{
  /// A string to split.
  fn src( &self ) -> &'a str;
  /// A delimeter to split string.
  fn delimeter( &self ) -> &'a str;
  /// Preserving or dropping empty splits.
  fn preserving_empty( &self ) -> bool;
  /// Preserving or dropping delimeters.
  fn preserving_delimeters( &self ) -> bool;
  /// Do splitting.
  fn split( self ) -> SplitIterator< 'a >
  where
    Self : Sized,
  {
    SplitIterator::new( self.src(), self.delimeter() )
  }
}

//

impl< 'a > SplitOptionsAdapter< 'a > for SplitOptions< 'a >
{
  fn src( &self ) -> &'a str
  {
    self.src
  }
  fn delimeter( &self ) -> &'a str
  {
    self.delimeter
  }
  fn preserving_empty( &self ) -> bool
  {
    self.preserving_empty
  }
  fn preserving_delimeters( &self ) -> bool
  {
    self.preserving_delimeters
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
///   .form();
/// ```

pub fn split< 'a >() -> SplitOptionsFormer< 'a >
{
  SplitOptions::former()
}
