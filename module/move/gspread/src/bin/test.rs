use std::error::Error;
use dotenv::dotenv;
use gspread::*;
use gcore::ApplicationSecret;
use gcore::client::
{
  Auth,
  Client
};

use std::collections::HashMap;
use serde_json::json;
use rand::Rng;
use rand::rngs::OsRng;


#[ tokio::main ]
async fn main() -> Result< (), Box< dyn Error > >
{
  dotenv().ok();

  let secret = ApplicationSecret::read();
  
  let auth = Auth::new( &secret );
  
  let client = Client::former()
  .auth( auth )
  .form();

  let spreadsheet_ids = vec![
    "172krpHTo_BI8Bwm9-9aGc5Bt9tm6P3nbiwkveVbO81k",
  ];
  let tables = vec!["t1"];
  let mut row_key_val = generate_truly_random_key_val(5000, 100);

  for &spreadsheet_id in &spreadsheet_ids {
    for i in 0..5 {
      for &sheet_name in &tables {
        row_key_val.insert("A".to_string(), json!(i));
        _ = gspread::actions::gspread::append_row(&client, spreadsheet_id, sheet_name, &row_key_val).await;
      }
    }
  }

  Ok( () )
}


fn generate_truly_random_key_val(n: usize, str_len: usize) -> HashMap<String, serde_json::Value> {
  let all_cols = generate_all_columns();
  let total = all_cols.len();

  let mut rng = OsRng;
  let mut indices: Vec<usize> = (0..total).collect();

  for i in 0..total {
    let j = i + (rng.gen_range(0..(total - i)));
    indices.swap(i, j);
  }

  let chosen_indices = &indices[0..n.min(total)];

  let mut result = HashMap::new();
  for &idx in chosen_indices {
    let col = &all_cols[idx];
    let val = random_string(&mut rng, str_len);
    result.insert(col.clone(), json!(val));
  }
  result
}

fn random_string(rng: &mut OsRng, length: usize) -> String {
  let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                  abcdefghijklmnopqrstuvwxyz\
                  0123456789";
  (0..length)
  .map(|_| {
    let idx = rng.gen_range(0..charset.len());
    charset[idx] as char
  })
  .collect()
}

fn generate_all_columns() -> Vec<String> {
  let mut columns = Vec::new();
  for c1 in b'A'..=b'Z' {
    columns.push((c1 as char).to_string());
  }
  for c1 in b'A'..=b'Z' {
    for c2 in b'A'..=b'Z' {
      columns.push(format!("{}{}", c1 as char, c2 as char));
    }
  }
  for c1 in b'A'..=b'Z' {
    for c2 in b'A'..=b'Z' {
      for c3 in b'A'..=b'Z' {
        columns.push(format!("{}{}{}", c1 as char, c2 as char, c3 as char));
      }
    }
  }
  columns
}