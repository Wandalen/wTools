//!
//! Gspread wrapper for outputting data to console
//!
//! It is used for "header" and "rows" commands
//!

use super::*;
use crate::*;
use ser::JsonValue;


#[ derive( Debug ) ]
pub struct RowWrapper( pub Vec< JsonValue >);

impl Clone for RowWrapper
{
  fn clone( &self ) -> Self
  {
    RowWrapper( self.0.clone() )
  }
}
impl TableWithFields for RowWrapper {}
impl Fields< &'_ str, Option< Cow< '_, str > > >
for RowWrapper
{
  type Key< 'k > = &'k str;
  type Val< 'v > = Option< Cow< 'v, str > >;

  fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
  {
    let mut dst = Vec::new();

    let start = 'A' as u8;
    let end = 'A' as u8 + self.0.len() as u8 - 1;

    for c in start..=end
    {
      for val in self.0.iter()
      {
        let title = Box::leak( c.to_string().into_boxed_str() ) as &str;
        dst.push( ( title, Some( Cow::Owned( val.to_string() ) ) ) )
      }
    }

    dst.into_iter()
  }
}