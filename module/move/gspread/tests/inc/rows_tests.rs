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
async fn test_get_rows()
{
  let ( hub, spreadsheet_id ) = setup().await;
  let table_name = "tab1";

  let result = actions::gspread_get_rows::action
  (
    &hub,
    spreadsheet_id,
    table_name
  )
  .await
  .expect( "Error getting rows" );

  assert_eq!
  (
    result,
    vec![
      vec![ "Vsevolod",	"Bakutov", "20" ],
      vec![ "Victor", "Ovsyanik", "85" ],
      vec![ "Olexandr", "Optimus", "28" ],
      vec![ "Ivan", "Optimus", "34" ],
      vec![ "Bogdan", "Optimus", "28" ],
    ]
  )
}

#[ tokio::test ]
async fn test_get_rows_with_spaces()
{
  let ( hub, spreadsheet_id ) = setup().await;
  let table_name = "tab2";

  let result = actions::gspread_get_rows::action
  (
    &hub,
    spreadsheet_id,
    table_name
  )
  .await
  .expect( "Error getting rows" );

  assert_eq!
  (
    result,
    vec![
      vec![ "Vsevolod",	"Bakutov" ],
      vec![ "Victor", "", "85" ],
      vec![ "", "Optimus", "28" ],
      vec![ ],
      vec![ "Bogdan", "Optimus", "28" ],
    ]
  )
}

#[ tokio::test ]
async fn test_get_rows_empty()
{
  let ( hub, spreadsheet_id ) = setup().await;
  let table_name = "tab3";

  let result = actions::gspread_get_rows::action
  (
    &hub,
    spreadsheet_id,
   table_name
  )
  .await
  .expect( "Error getting rows" );

  assert_eq!( result, Vec::< Vec< String > >::new() )
}