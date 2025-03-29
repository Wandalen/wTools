//! Table and columns info operations from Sled storage.

use crate::*;
use error_tools::untyped::Result;
use gluesql::
{
  core::executor::Payload,
  sled_storage::SledStorage,
};
use entity::table::TableStore;
use action::table::TablesReport;
use sled_adapter::FeedStorage;

#[ async_trait::async_trait( ?Send ) ]
impl TableStore for FeedStorage< SledStorage >
{
  async fn tables_list( &mut self ) -> Result< TablesReport >
  {
    let glue = &mut *self.0.lock().await;
    let payloads = glue.execute( "SELECT * FROM GLUE_TABLE_COLUMNS" ).await?;

    let report = TablesReport::new( payloads );

    Ok( report )
  }

  async fn table_list( &mut self, table_name : String ) -> Result< Vec< Payload > >
  {
    let glue = &mut *self.0.lock().await;
    let query_str = format!( "SELECT * FROM GLUE_TABLE_COLUMNS WHERE TABLE_NAME='{}'", table_name );
    let payloads = glue.execute( &query_str ).await?;

    Ok( payloads )
  }
}