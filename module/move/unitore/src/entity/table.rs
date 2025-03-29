//! Functionality for storage tables information.

use crate::*;
use error_tools::untyped::Result;
use gluesql::prelude::Payload;

use action::table::TablesReport;

/// Functions for tables informantion.
#[ async_trait::async_trait( ?Send ) ]
pub trait TableStore
{
  /// List tables in storage.
  async fn tables_list( &mut self ) -> Result< TablesReport >;

  /// List columns of table.
  async fn table_list( &mut self, table_name : String ) -> Result< Vec< Payload > >;
}
