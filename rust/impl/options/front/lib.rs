#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Mechanism to define map of options for a function and its defaults laconically.
//!
// //! # Sample
// //! ```
// //! mod splitter
// //! {
// //!   use former::Former;
// //!
// //!   #[ derive( PartialOrd ) ]
// //!   #[ derive( Former, PartialEq, Debug ) ]
// //!   #[ form_after( fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) > ) ]
// //!   pub struct Options< 'a >
// //!   {
// //!     pub src : &'a str,
// //!     pub delimeter : &'a str,
// //!     #[ default( true ) ]
// //!     pub left : bool,
// //!   }
// //!
// //!   pub trait OptionsAdapter< 'a >
// //!   {
// //!     fn src( &self ) -> &'a str;
// //!     fn delimeter( &self ) -> &'a str;
// //!     fn left( &self ) -> &bool;
// //!     #[ inline ]
// //!     fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) >
// //!     where
// //!       Self : Sized,
// //!     {
// //!       if *self.left()
// //!       {
// //!         Box::new( self.src().split( self.delimeter() ) )
// //!       }
// //!       else
// //!       {
// //!         Box::new( self.src().rsplit( self.delimeter() ) )
// //!       }
// //!     }
// //!   }
// //!
// //!   impl< 'a > OptionsAdapter< 'a > for Options< 'a >
// //!   {
// //!     #[ inline ]
// //!     fn src( &self ) -> &'a str
// //!     {
// //!       &self.src
// //!     }
// //!     #[ inline ]
// //!     fn delimeter( &self ) -> &'a str
// //!     {
// //!       &self.delimeter
// //!     }
// //!     #[ inline ]
// //!     fn left( &self ) -> &bool
// //!     {
// //!       &self.left
// //!     }
// //!   }
// //!
// //!   #[ inline ]
// //!   pub fn former< 'a >() -> OptionsFormer< 'a >
// //!   {
// //!     Options::< 'a >::former()
// //!   }
// //!
// //! }
// //!
// //! #[ inline ]
// //! fn splitter< 'a >() -> splitter::OptionsFormer< 'a >
// //! {
// //!   splitter::former::< 'a >()
// //! }
// //!
// //! //
// //!
// //! fn main()
// //! {
// //!   /* form options */
// //!   let from_former = splitter().src( "abc" ).delimeter( "b" )._form();
// //!   let from_options = splitter::Options
// //!   {
// //!     src : "abc",
// //!     delimeter : "b",
// //!     left : true,
// //!   };
// //!   assert_eq!( from_former, from_options );
// //!
// //!   /* perform methods from autotrait */
// //!   use splitter::OptionsAdapter;
// //!   let splitted = from_former.perform().map( | e | String::from( e ) ).collect::< Vec< _ > >();
// //!   assert_eq!( splitted, vec![ "a", "c" ] );
// //! }
// //! ```
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

pub use woptions_runtime as runtime;
pub use woptions_meta as meta;
pub use meta::Options;
// pub use meta::options;

pub use former::derive::Former;
