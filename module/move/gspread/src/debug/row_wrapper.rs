//!
//! Gspread wrapper for outputting data to console
//!
//! It is used for "header" and "rows" commands
//!

mod private
{
  use std::borrow::Cow;
  use format_tools::
  {
    Fields, 
    IteratorTrait, 
    TableWithFields
  };

  /// # RowWrapper
  ///
  /// A structure used to display a row in the console in a table format.
  ///
  /// This structure is designed for displaying the results of HTTP requests in a tabular format 
  /// using the `format_tools` crate. It implements the `TableWithFields` and `Fields` traits 
  /// to enable this functionality.
  ///
  /// ## Fields:
  /// - `row`:  
  ///   A `Vec< JsonValue >` representing a single row of the table. This can include headers or data rows.
  /// - `max_len`:  
  ///   An `usize` specifying the maximum number of columns in the table.  
  ///   This ensures proper alignment and display of the table in the console.
  ///
  /// ## Traits Implemented:
  /// - `TableWithFields`:  
  /// - `Fields< &'_ str, Option< Cow< '_, str > > >`:  
  ///
  /// ## Implementation Details:
  /// - Missing cells in a row are filled with empty strings ( `""` ) to ensure all rows have `max_len` columns.
  /// - Keys (column names) are dynamically generated based on the column index.
  /// - Values are sanitized to remove unnecessary characters such as leading/trailing quotes.
  #[ derive( Debug, Clone ) ]
  pub struct RowWrapper
  {
    pub row : Vec< serde_json::Value >,
    pub max_len : usize
  }

  impl TableWithFields for RowWrapper {}
  impl Fields< &'_ str, Option< Cow< '_, str > > >
  for RowWrapper
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = Option< Cow< 'v, str > >;
    fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option<Cow<'_, str > > ) >
    {
      let mut dst = Vec::new();

      for ( index, value ) in self.row.iter().enumerate()
      {
        let column_name = format!( "{} ", index );
        let title = Box::leak( column_name.into_boxed_str() ) as &str;

        dst.push( ( title, Some( Cow::Owned( value.to_string() ) ) ) )
      }

      // adding empty values for missing cells
      for index in self.row.len()..self.max_len
      {
        let column_name = format!( "{}", index );
        let title = Box::leak( column_name.into_boxed_str() ) as &str;
        dst.push( ( title, Some( Cow::Owned( "".to_string() ) ) ) );
      }
      dst.into_iter()
    }
  }

}

crate::mod_interface!
{
  orphan use
  {
    RowWrapper
  };
}