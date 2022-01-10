
// ///
// /// Split iterator.
// ///
//
// #[ derive( Debug ) ]
// pub struct IteratorJunction< 'a >
// {
//   iterator : std::iter::Peekable< std::str::Split< 'a, &'a str > >,
//   counter : i32,
//   delimeter : &'a str,
// }
//
// //
//
// impl< 'a > IteratorJunction< 'a >
// {
//   fn new( src : &'a str, delimeter : &'a str ) -> Self
//   {
//     let counter = 0;
//     // let delimeter = delimeter.clone();
//     let delimeter_slice = unsafe
//     {
//       let slice = core::slice::from_raw_parts( delimeter.as_ptr(), delimeter.len() );
//       core::str::from_utf8_unchecked( slice )
//     };
//     let iterator = src.split( delimeter_slice ).peekable();
//     Self
//     {
//       iterator,
//       delimeter,
//       counter,
//     }
//   }
// }
//
// //
//
// impl< 'a > Iterator for IteratorJunction< 'a >
// {
//   type Item = Split< 'a >;
//
//   fn next( &mut self ) -> Option< Self::Item >
//   {
//     self.counter += 1;
//     if self.counter % 2 == 1
//     {
//       let next = self.iterator.next();
//       if let Some( next ) = next
//       {
//         Some( Split { string : next, typ : SplitType::Delimeted } )
//       }
//       else
//       {
//         None
//       }
//     }
//     else
//     {
//       if self.iterator.peek().is_none()
//       {
//         self.iterator.next();
//         return None;
//       }
//       Some( Split { string : self.delimeter, typ : SplitType::Delimeter } )
//       // Some( Split::Delimeter( self.delimeter.clone() ) )
//     }
//   }
// }

#[ derive( Debug ) ]
pub struct IteratorJunction< IteratorOriginal : Iterator >
// pub struct IteratorJunction< T >
{
  iterator : IteratorOriginal,
  // iterator : dyn Iterator< Item = T >,
}

//

impl< IteratorOriginal : Iterator > IteratorJunction< IteratorOriginal >
{
  // fn new( src : &'a str, delimeter : &'a str ) -> Self
  // {
  // }
}

//

// impl< IteratorOriginal : Iterator > Iterator for IteratorJunction< IteratorOriginal::Item >
impl< IteratorOriginal : Iterator > Iterator for IteratorJunction< IteratorOriginal >
// impl Iterator for IteratorJunction<  >
{
  type Item = IteratorOriginal::Item;

  fn next( &mut self ) -> Option< Self::Item >
  {
    self.iterator.next()
  }
}

mod split
{

  use former::Former;

  #[ derive( PartialOrd ) ]
  #[ derive( Former, PartialEq, Debug ) ]
  #[ form_after( fn perform( self ) -> std::str::Split< 'a, &'a str > ) ]
  pub struct Options< 'a >
  {
    pub src : &'a str,
    pub delimeter : &'a str,
    #[ default( true ) ]
    pub left : bool,
  }

  pub trait OptionsAdapter< 'a >
  {
    fn src( &self ) -> &'a str;
    fn delimeter( &self ) -> &'a str;
    fn left( &self ) -> &bool;
    // #[ inline ]
    fn perform( self ) -> std::str::Split< 'a, &'a str >
    // fn perform( self ) -> impl std::iter::Iterator< Item = &'a str >
    where
      Self : Sized,
    {
      // if *self.left()
      // {
        self.src().split( self.delimeter() )
      // }
      // else
      // {
      //   self.src().rsplit( self.delimeter() ).map( | e | e )
      // }
    }
  }

  impl< 'a > OptionsAdapter< 'a > for Options< 'a >
  {
    #[ inline ]
    fn src( &self ) -> &'a str
    {
      &self.src
    }
    #[ inline ]
    fn delimeter( &self ) -> &'a str
    {
      &self.delimeter
    }
    #[ inline ]
    fn left( &self ) -> &bool
    {
      &self.left
    }
  }

  #[ inline ]
  pub fn former< 'a >() -> OptionsFormer< 'a >
  {
    Options::< 'a >::former()
  }

}

#[ inline ]
fn split< 'a >() -> split::OptionsFormer< 'a >
{
  split::former::< 'a >()
}

//

include!( "./basic_only_test.rs" );
