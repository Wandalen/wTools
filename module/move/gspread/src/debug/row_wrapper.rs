//!
//! Gspread wrapper for outputting data to console
//!
//! It is used for "header" and "rows" commands
//!

use super::*;
use crate::*;
use ser::JsonValue;


#[ derive( Debug ) ]
pub struct RowWrapper
{
  pub row: Vec< JsonValue >,
  pub max_len: usize
}

impl Clone for RowWrapper
{
  fn clone( &self ) -> Self
  {
    Self
    {
      row: self.row.clone(),
      max_len: self.max_len.clone()
    }
  }
}

impl TableWithFields for RowWrapper {}
impl Fields< &'_ str, Option< Cow< '_, str > > >
for RowWrapper
{
  type Key< 'k > = &'k str;
  type Val< 'v > = Option< Cow< 'v, str > >;

  fn fields( &self ) -> impl IteratorTrait< Item= ( &'_ str, Option<Cow<'_, str > > ) >
  {
    let mut dst = Vec::new();

    for ( index, value ) in self.row.iter().enumerate()
    {
      let column_name = format!( "Column{}", index );
      let title = Box::leak( column_name.into_boxed_str() ) as &str;
      dst.push( ( title, Some( Cow::Owned( value.to_string() ) ) ) )
    }

    //adding empty values for missing cells
    for index in self.row.len()..self.max_len
    {
      let column_name = format!( "Column{}", index );
      let title = Box::leak( column_name.into_boxed_str() ) as &str;
      dst.push( ( title, Some( Cow::Owned( "".to_string() ) ) ) );
    }

    dst.into_iter()
  }
}