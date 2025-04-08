// //! Content of a message which can be not only text, but also other media type.
//
// mod private
// {
//
//   pub enum ContentType
//   {
//     Text,
//     Image,
//     Sound,
//     Video,
//   }
//
//   pub trait Content
//   {
//     fn content_type( &self ) -> ContentType;
//     fn content_to_string( self ) -> String;
//     fn content_to_bytes( self ) -> Vec< u8 >;
//   }
//
//   /// Image
//   #[ derive( Debug ) ]
//   pub struct Image
//   {
//     pub source : Source,
//   }
//
//   /// Source
//   #[ derive( Debug ) ]
//   pub struct Source
//   {
//     pub media_type : String,
//     pub encoding : String,
//     pub data : AsBytes,
//   }
//
// }
//
// crate::mod_interface!
// {
//   orphan use private::
//   {
//     Content,
//   };
// }
