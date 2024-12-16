#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  hub,
  Secret,
  actions,
  SheetsType
};

async fn setup() -> ( SheetsType, &'static str )
{
  let secret = Secret::load().expect( "Failed to load secret" );
  let hub = hub( &secret ).await.expect( "Failed to create a hub" );
  let spreadsheet_id = "1EAEdegMpitv-sTuxt8mV8xQxzJE7h_J0MxQoyLH7xxU";

  ( hub, spreadsheet_id )
}
#[ tokio::test ]
async fn test_get_header()
{
  let ( hub, spreadsheet_id ) = setup().await;
  let table_name = "tab1";

  let result = actions::gspread_get_header::action
  (
    &hub,
    spreadsheet_id,
    table_name
  )
  .await
  .expect( "Error getting header" );

  assert_eq!( result, vec![ vec![ "Name", "Surname", "Age" ] ] );
}

#[ tokio::test ]
async fn test_get_header_with_spaces()
{
  let ( hub, spreadsheet_id ) = setup().await;
  let table_name = "tab2";

  let result = actions::gspread_get_header::action
  (
    &hub,
    spreadsheet_id,
    table_name
  )
  .await
  .expect( "Error getting header" );

  assert_eq!( result, vec![ vec![ "Name", "", "Age" ] ] );
}

#[ tokio::test ]
async fn test_get_header_empty()
{
  let ( hub, spreadsheet_id ) = setup().await;
  let table_name = "tab3";

  let result = actions::gspread_get_header::action
  (
    &hub,
    spreadsheet_id,
    table_name
  )
  .await
  .expect( "Error getting header" );

  assert_eq!( result, Vec::< Vec< String > >::new() );
}

#[ tokio::test ]
async fn test_get_header_with_empty_end()
{
  let ( hub, spreadsheet_id ) = setup().await;
  let table_name = "tab4";

  let result = actions::gspread_get_header::action
  (
    &hub,
    spreadsheet_id,
    table_name
  )
  .await
  .expect( "Error getting header" );

  assert_eq!( result, vec![ vec![ "Name", "Surname" ] ] );
}