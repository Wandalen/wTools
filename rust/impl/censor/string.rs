//
// /* xxx : implement no_std feature */
// use std::rc::Rc;
// use wtools::former::Former;
//
// //
//
// pub enum Split< 'a >
// {
//   Split( &'a str ),
//   Delimeter( Rc< str > ),
// }
//
// impl< 'a > From< Split< 'a > > for String
// {
//   fn from( src : Split ) -> Self
//   {
//     match src
//     {
//       Split::Split( e ) => e.into(),
//       Split::Delimeter( e ) => e.to_string(),
//     }
//   }
// }
//
// //
//
// pub struct SplitIterator< 'a >
// {
//   iterator : std::iter::Peekable< std::str::Split< 'a, &'a str > >,
//   counter : i32,
//   delimeter : Rc< str >,
// }
//
// //
//
// impl< 'a > SplitIterator< 'a >
// {
//   fn new( src : &'a str, delimeter : Rc< str > ) -> Self
//   {
//     let counter = 0;
//     let delimeter = delimeter.clone();
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
// impl< 'a > Iterator for SplitIterator< 'a >
// {
//   // type Item = &'a str;
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
//         Some( Split::Split( next ) )
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
//       Some( Split::Delimeter( self.delimeter.clone() ) )
//     }
//   }
// }
//
// //
//
// #[ derive( Former ) ]
// #[ form_after( fn split( self ) -> SplitIterator< 'a > ) ]
// pub struct SplitOptions< 'a >
// {
//   #[ default( "" ) ]
//   src : &'a str,
//   #[ default( "" ) ]
//   delimeter : Rc< str >,
//   #[ default( true ) ]
//   preserving_empty : bool,
//   #[ default( true ) ]
//   preserving_delimeters : bool,
//   // result : HashMap< Box< str >, Box< str > >,
// }
//
// impl< 'a > SplitOptions< 'a >
// {
//   pub fn new() -> Self
//   {
//     Self
//     {
//       src : "".into(),
//       delimeter : " ".into(),
//       preserving_empty : true,
//       preserving_delimeters : true,
//     }
//   }
// }
//
// //
//
// pub trait SplitOptionsAdapter< 'a >
// {
//   fn delimeter( &self ) -> Rc< str >;
//   fn src( &self ) -> &'a str;
//   fn split( self ) -> SplitIterator< 'a >
//   where
//     Self : Sized,
//   {
//     SplitIterator::new( self.src(), self.delimeter() )
//   }
// }
//
// //
//
// impl< 'a > SplitOptionsAdapter< 'a > for SplitOptions< 'a >
// {
//   fn delimeter( &self ) -> Rc< str >
//   {
//     self.delimeter.clone()
//   }
//   fn src( &self ) -> &'a str
//   {
//     self.src
//   }
// }
//
// //
//
// // pub trait SplitOptionSrcAdapter< 'a > : SplitOptionsAdapter
// // {
// //   fn src( &self ) -> &'a str;
// //   fn split( &self ) -> SplitIterator
// //   {
// //     SplitIterator::new( self.src(), self.delimeter().clone() )
// //   }
// // }
// //
// // //
// //
// // impl< 'a > SplitOptionSrcAdapter< 'a > for SplitOptions< 'a >
// // {
// //   fn src( &self ) -> &'a str
// //   {
// //     self.src
// //   }
// // }
//
// //
//
// // pub fn split_default< 'a >( src : &'a str ) -> SplitIterator< 'a >
// // {
// //   let mut options = SplitOptions::new();
// //   // options.src = src;
// //   options.split()
// // }
//
// //
//
// pub fn split< 'a >() -> SplitOptionsFormer< 'a >
// {
//   SplitOptions::former()
// }
