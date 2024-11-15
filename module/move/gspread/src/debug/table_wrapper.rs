//!
//! Simple custom wrapper for outputting data to console
//!
//! It is used for "header" and "rows" commands
//!


mod private
{

  use crate::*;
  use ser::JsonValue;
  use actions::gspread::Value;

  pub struct Table
  {
    pub data: Value,
  }

  impl Table
  {
    pub fn new( data: Value ) -> Self
    {
      Table { data }
    }

    pub fn display(&self)
    {
      for ( i, row ) in self.data.iter().enumerate()
      {
        print!( "row {}: ", i + 1 );
        for cell in row
        {
          print!( "{} ", Self::format_value( cell ) );
        }
        println!();
      }
    }

    fn format_value( value: &JsonValue ) -> String
    {
      match value
      {
        JsonValue::String( s ) => s.clone(),
        JsonValue::Number( n ) => n.to_string(),
        JsonValue::Bool( b ) => b.to_string(),
        JsonValue::Null => "".to_string(),
        _ => "unsupported".to_string(),
      }
    }
  }

}

pub use private::
{
  Table,
};