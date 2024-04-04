//! Functionality for storage tables information.

use crate::*;
use error_tools::Result;
use gluesql::
{
  sled_storage::SledStorage,
  prelude::Payload,
};

use action::table::TablesReport;
use storage::FeedStorage;

/// Functions for tables informantion.
#[ async_trait::async_trait( ?Send ) ]
pub trait TableStore
{
  /// List tables in storage.
  async fn tables_list( &mut self ) -> Result< TablesReport >;

  /// List columns of table.
  async fn table_list( &mut self, table_name : String ) -> Result< Vec< Payload > >;
}

#[ async_trait::async_trait( ?Send ) ]
impl TableStore for FeedStorage< SledStorage >
{
  async fn tables_list( &mut self ) -> Result< TablesReport >
  {
    let glue = &mut *self.storage.lock().await;
    let payloads = glue.execute( "SELECT * FROM GLUE_TABLE_COLUMNS" ).await?;

    let report = TablesReport::new( payloads );

    Ok( report )
  }

  async fn table_list( &mut self, table_name : String ) -> Result< Vec< Payload > >
  {
    let glue = &mut *self.storage.lock().await;
    let query_str = format!( "SELECT * FROM GLUE_TABLE_COLUMNS WHERE TABLE_NAME='{}'", table_name );
    let payloads = glue.execute( &query_str ).await?;

    Ok( payloads )
  }
}
