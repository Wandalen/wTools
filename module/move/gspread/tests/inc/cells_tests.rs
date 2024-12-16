#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  hub,
  Secret,
  actions,
  SheetsType,
};

async fn setup() -> ( SheetsType, &'static str, &'static str, &'static str )
{
  let secret = Secret::load().expect( "Failed to load secret" );
  let hub = hub( &secret ).await.expect( "Failed to create a hub" );
  let select_row_by_key = "id";
  let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";
  let table_name = "tab7";

  ( hub, select_row_by_key, spreadsheet_id, table_name )
}

#[ tokio::test ]
async fn test_set_cells()
{
  let 
  ( 
    hub, 
    select_row_by_key,
    spreadsheet_id, 
    table_name 
  ) = setup().await;
  
  let json = r#"{ "id": "2", "A": "new_val1", "B": "new_val2"}"#;

  let result = actions::gspread_cells_set::action
  (
    &hub,
    select_row_by_key,
    json,
    spreadsheet_id,
    table_name,
  )
  .await
  .expect( "Error while updating" );

  assert_eq!( result, "Cells were sucsessfully updated!" )
}

#[ tokio::test ]
async fn test_set_cells_wrong_row()
{
  let 
  ( 
    hub, 
    select_row_by_key,
    spreadsheet_id, 
    table_name 
  ) = setup().await;
  
  let json = r#"{ "id": "a", "A": "new_val1", "B": "new_val2"}"#;

  let result = actions::gspread_cells_set::action
  (
    &hub,
    select_row_by_key,
    json,
    spreadsheet_id,
    table_name,
  )
  .await
  .expect( "Error while updating" );

  assert_eq!
  ( 
    result, 
    r#"Bad Request: {"error":{"code":400,"message":"Invalid data[0]: Unable to parse range: tab7!Aa","status":"INVALID_ARGUMENT"}}"# 
  )
}