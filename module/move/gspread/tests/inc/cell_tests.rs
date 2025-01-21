#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  hub,
  Secret,
  actions,
  SheetsType,
  ser::JsonValue
};

async fn setup() -> ( SheetsType, &'static str, &'static str )
{
  let secret = Secret::load().expect( "Failed to load secret" );
  let hub = hub( &secret ).await.expect( "Failed to create a hub" );
  let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";
  let table_name = "tab5";

  ( hub, spreadsheet_id, table_name )
}

#[ tokio::test ]
async fn test_get_cell()
{
  let ( hub, spreadsheet_id, table_name ) = setup().await;
  let cell_id = "R2C1";

  let result = actions::gspread_cell_get::action
  (
    &hub,
    spreadsheet_id,
    table_name,
    cell_id
  )
  .await
  .expect( "Error getting cell" );

  assert_eq!( result, "Vsevolod" )
}

#[ tokio::test ]
async fn test_get_cell_empty()
{
  let ( hub, spreadsheet_id, table_name ) = setup().await;
  let cell_id = "R4C1";

  let result = actions::gspread_cell_get::action
  (
    &hub,
    spreadsheet_id,
    table_name,
    cell_id
  )
  .await
  .expect( "Error getting cell" );

  assert_eq!( result, JsonValue::Null )
}

#[ tokio::test ]
async fn test_set_cell()
{
  let ( hub, spreadsheet_id, table_name ) = setup().await;
  let cell_id = "R2C1";
  let value = "Seva";

  let result = actions::gspread_cell_set::action
  (
    &hub,
    spreadsheet_id,
    table_name,
    cell_id,
    value
  )
  .await;

  assert!( result.is_ok() );
}

#[ tokio::test ]
async fn test_set_empty_cell()
{
  let ( hub, spreadsheet_id, table_name ) = setup().await;
  let cell_id = "R4C1";
  let value = "Stanislav";

  let result = actions::gspread_cell_set::action
  (
    &hub,
    spreadsheet_id,
    table_name,
    cell_id,
    value
  )
  .await;

  assert!( result.is_ok() );
}