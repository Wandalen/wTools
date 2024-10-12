#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/assistant/latest/assistant/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use std::
{
  env,
  error::Error,
};

use format_tools::
{
  AsTable,
  TableFormatter,
  output_format,
};
use dotenv::dotenv;

use assistant::
{
  client,
};

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn Error > >
{
  dotenv().ok();

  let client = client()?;

  let response = client.file_list().await?;
  // println!( "Files: {:?}", response.data );
  let files : Vec< _ > = response.data.into_iter().map( | e | assistant::FileDataWrap( e ) ).collect();
  println!
  (
    "Files:\n{}",
    AsTable::new( &files ).table_to_string_with_format( &output_format::Table::default() ),
  );

  let response = client.list_assistant( None, None, None, None ).await?;

  // println!( "Assistants: {:?}", assistants.data );
  let assistants : Vec< _ > = response.data.into_iter().map( | e | assistant::AssistantObjectWrap( e ) ).collect();
  println!
  (
    "Assistants:\n{}",
    AsTable::new( &assistants ).table_to_string_with_format( &output_format::Records::default() ),
  );

  Ok( () )
}
