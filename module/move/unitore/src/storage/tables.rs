//! Tables sroring functions.

use crate::*;
use error_tools::Result;
use gluesql::
{
  sled_storage::SledStorage,
};

use executor::endpoints::
{
  table::TablesReport,
  list_fields::FieldsReport,
};
use storage::FeedStorage;

/// Functions for tables informantion.
#[ async_trait::async_trait( ?Send ) ]
pub trait TableStore
{
  /// Get list of column titles of feed table.
  fn columns_titles( &mut self ) -> FieldsReport;

  /// List tables in storage.
  async fn list_tables( &mut self ) -> Result< TablesReport >;

  /// List columns of table.
  async fn list_columns( &mut self, table_name : String ) -> Result< TablesReport >;
}

#[ async_trait::async_trait( ?Send ) ]
impl TableStore for FeedStorage< SledStorage >
{
  fn columns_titles( &mut self ) -> FieldsReport
  {
    FieldsReport
    {
      fields_list : self.frame_fields.clone()
    }
  }

  async fn list_tables( &mut self ) -> Result< TablesReport >
  {
    let glue = &mut *self.storage.lock().await;
    let payloads = glue.execute( "SELECT * FROM GLUE_TABLE_COLUMNS" ).await?;

    let report = TablesReport::new( payloads );

    Ok( report )
  }

  async fn list_columns( &mut self, table_name : String ) -> Result< TablesReport >
  {
    let glue = &mut *self.storage.lock().await;
    let query_str = format!( "SELECT * FROM GLUE_TABLE_COLUMNS WHERE TABLE_NAME='{}'", table_name );
    let payloads = glue.execute( &query_str ).await?;

    let report = TablesReport::new( payloads );

    Ok( report )
  }

}
