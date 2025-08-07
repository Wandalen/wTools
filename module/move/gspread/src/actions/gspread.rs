//!
//! Google Sheets API actions.
//!
//! This module also contains the definition of Google Sheets Error.
//!

mod private
{
  use regex::Regex;
  use serde_json::json;
  use once_cell::sync::Lazy;
  use std::collections::HashMap;

  use crate::gcore::client::InsertDataOption;
  use crate::*;
  use gcore::Secret;
  use gcore::error::
  { 
    Error, 
    Result 
  };
  use gcore::client::
  {
    Client, 
    Dimension, 
    ValueRange, 
    ValueInputOption, 
    ValueRenderOption, 
    UpdateValuesResponse, 
    // ValuesAppendResponse,
    BatchUpdateValuesRequest, 
    BatchUpdateValuesResponse,
    BatchClearValuesRequest,
    BatchClearValuesResponse, 
    SheetProperties, 
    ValuesClearResponse
  };

  static REGEX_ROW_INDEX : Lazy< Regex > = Lazy::new( || {
    Regex::new( r"^([A-Za-z]+)(\d+)$" ).unwrap()
  });

  /// # get_key_matches
  /// 
  /// Collect value matches in a column.
  /// 
  /// ## Params:
  ///  - `column`: A reference to Vec< serde_json::Value >, column.
  ///  - `key`: A reference to a serde_json::Value, value to find.
  /// 
  /// Return `Vec< usize >`
  fn get_key_matches
  ( 
    column : &Vec< serde_json::Value >,
    key : &serde_json::Value 
  ) -> Vec< usize >
  {
    column
    .iter()
    .enumerate()
    .filter( | &( _, val ) | { *val == *key } )
    .map( | ( i, _ ) | i )
    .collect()
  }

  /// Return row key depending on selected action.
  fn get_row_keys
  (
    key_matches : Vec< usize >,
    action : OnFind
  ) -> Vec< usize >
  {
    match action
    {
      OnFind::AllMatchedRow => key_matches,
      OnFind::FirstMatchedRow => vec![ *key_matches.first().unwrap() ],
      OnFind::LastMatchedRow => vec![ *key_matches.last().unwrap() ]
    }
  }

  /// Converts number to column label.
  fn number_to_column_label( mut num : usize ) -> String
  {
    let mut chars = Vec::new();
    while num > 0
    {
      let remainder = ( num - 1 ) % 26;
      let c = ( b'A' + remainder as u8 ) as char;
      chars.push( c );
      num = ( num - 1 ) / 26;
    }
    chars.reverse();
    chars.into_iter().collect()
  }
  /// Converts label to number.
  fn column_label_to_number( col : &str ) -> usize
  {
    let mut result = 0;
    for c in col.chars()
    {
      let digit = c as usize - 'A' as usize + 1;
      result = result * 26 + digit
    }
    result
  }

  /// # `update_row`
  ///
  /// Updates a specific row in a Google Sheet with the provided values.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet.
  /// - `row_key`:  
  ///   A `serde_json::Value` representing the row's key (e.g., the row index).
  /// - `row_key_val`:  
  ///   A `HashMap< String, serde_json::Value >` where:  
  ///   - Key: The column name (e.g., "A", "B").  
  ///   - Value: The new value to set in the corresponding cell.
  ///
  /// ## Returns:
  /// - Result< [`BatchUpdateValuesResponse`] >
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, e.g., due to invalid input or insufficient permissions.
  pub async fn update_row< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key : serde_json::Value,
    row_key_val : HashMap< String, serde_json::Value >
  ) -> Result< BatchUpdateValuesResponse >
  {
    let mut value_ranges = Vec::with_capacity( row_key_val.len() );

    for ( col_name, value ) in row_key_val 
    {
      value_ranges.push
      (
        ValueRange
        { 
          major_dimension : Some( Dimension::Row ),
          values : Some( vec![ vec![ value ] ] ),
          range : Some( format!( "{}!{}{}", sheet_name, col_name, row_key ) ),
        }
      )
    }

    let request = BatchUpdateValuesRequest
    {
      data : value_ranges,
      value_input_option : ValueInputOption::UserEntered,
      include_values_in_response : Some( true ),
      response_value_render_option : Some( ValueRenderOption::FormattedValue ),
      response_date_time_render_option : Default::default()
    };

    match client
    .spreadsheet()
    .values_batch_update( spreadsheet_id, request )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( error )
    }
  }

  /// # get_column
  /// 
  /// Retrive a specific column from a Google Sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet.
  /// - `column_id`:
  ///   `&str` specifying the sheet's column id (e. g. A, B, C, ..., ZZZ)
  ///
  /// ## Returns:
  /// - Result< Vec< serde_json::Value > >
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, e.g., due to invalid input or insufficient permissions.
  pub async fn get_column< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    column_id : &str
  ) -> Result< Vec< serde_json::Value > >
  {
    let range = format!( "{}!{}:{}", sheet_name, column_id, column_id );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .major_dimension( Dimension::Column )
    .value_render_option( ValueRenderOption::UnformattedValue )
    .doit()
    .await
    {
      Ok( response ) => 
      {
        match response.values
        {
          Some( values ) => 
          {
            let column = values
            .into_iter()
            .next()
            .unwrap_or_default();

            Ok( column )
          }
          None => Ok( Vec::new() )
        }
      },
      Err( error ) => Err( Error::ApiError( error.to_string() ) )
    }
  }

  /// # `update_rows_by_custom_row_key`
  ///
  /// Updates a specific row or rows in a Google Sheet with the provided values.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet.
  /// - `key_by`:  
  ///   A `( &str, serde_json::Value )` a pair of column key and its value.
  /// - `row_key_val`:  
  ///   A `HashMap< String, serde_json::Value >` where:  
  ///   - Key: The column name (e.g., "A", "B").  
  ///   - Value: The new value to set in the corresponding cell.
  /// - `update_range_at_all_match_cells`
  ///   A `bool` If true, updates the rows with all match cells. Otherwise updates row with the first match cell.
  /// - `raise_error_on_fail`
  ///   Returns an error if there were not found any matches.
  ///
  /// ## Returns:
  /// - Result< [`BatchUpdateValuesResponse`] >
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, e.g., due to invalid input or insufficient permissions.
  pub async fn update_rows_by_custom_row_key< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    key_by : ( &str, serde_json::Value ), 
    row_key_val : HashMap< String, serde_json::Value >,
    on_find : OnFind,
    on_fail : OnFail
  ) -> Result< BatchUpdateValuesResponse >
  {
    // Getting provided column.
    let range = format!( "{}!{}:{}", sheet_name, key_by.0, key_by.0 );

    // Get column
    let value_range = client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .major_dimension( Dimension::Column )
    .value_render_option( ValueRenderOption::UnformattedValue )
    .doit()
    .await
    .map_err( | err | Error::ApiError( err.to_string() ) )?;

    let values = match value_range.values
    {
      Some( values ) => values,
      None =>
      {
        match on_fail
        {
          OnFail::Nothing => return Ok( BatchUpdateValuesResponse::default() ),
          OnFail::AppendRow =>
          {
            let _ = append_row( client, spreadsheet_id, sheet_name, &row_key_val ).await?;
            let response = BatchUpdateValuesResponse
            {
              spreadsheet_id : Some( spreadsheet_id.to_string() ),
              total_updated_rows : Some( 1 ),
              total_updated_sheets : Some( 1 ),
              total_updated_cells : Some( row_key_val.len() as u32 ),
              total_updated_columns : Some( row_key_val.len() as u32 ),
              responses : None
            };

            return Ok( response );
          }
          OnFail::Error => return Err( Error::ApiError( "Not such value in the sheet.".to_string() ) )
        }
      }
    };

    // Counting mathces.
    let row_keys : Vec< usize > = values[0]
    .iter()
    .enumerate()
    .filter( | &( _, val ) | { *val == key_by.1 } )
    .map( | ( i, _ ) | i )
    .collect();

    if row_keys.is_empty()
    {
      match on_fail
      {
        OnFail::Nothing => return Ok( BatchUpdateValuesResponse::default() ),
        OnFail::AppendRow =>
        {
          let _ = append_row( client, spreadsheet_id, sheet_name, &row_key_val ).await?;
          let response = BatchUpdateValuesResponse
          {
            spreadsheet_id : Some( spreadsheet_id.to_string() ),
            total_updated_rows : Some( 1 ),
            total_updated_sheets : Some( 1 ),
            total_updated_cells : Some( row_key_val.len() as u32 ),
            total_updated_columns : Some( row_key_val.len() as u32 ),
            responses : None
          };

          return Ok( response );
        }
        OnFail::Error => return Err( Error::ApiError( "Not such value in the sheet.".to_string() ) )
      }
    }

    // Preparing value ranges.
    let mut value_ranges = Vec::with_capacity( row_key_val.len() );
    let range = match on_find
    {
      OnFind::AllMatchedRow => row_keys,
      OnFind::FirstMatchedRow => vec![ *row_keys.first().unwrap() ],
      OnFind::LastMatchedRow => vec![ *row_keys.last().unwrap() ]
    };

    for row_key in range
    {
      for ( col_name, value ) in &row_key_val 
      {
        value_ranges.push
        (
          ValueRange
          { 
            major_dimension : Some( Dimension::Row ),
            values : Some( vec![ vec![ value.clone() ] ] ),
            range : Some( format!( "{}!{}{}", sheet_name, col_name, row_key + 1 ) ),
          }
        );
      }
    }

    // Making HTTP request.
    let request = BatchUpdateValuesRequest
    {
      data : value_ranges,
      value_input_option : ValueInputOption::UserEntered,
      include_values_in_response : Some( true ),
      response_value_render_option : Some( ValueRenderOption::FormattedValue ),
      response_date_time_render_option : Default::default()
    };

    match client
    .spreadsheet()
    .values_batch_update( spreadsheet_id, request )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( error )
    }

  }

  /// # `append_row`
  ///
  /// Append a new row at the end of the sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose header is to be retrieved.
  /// - `row_key_val`: 
  ///   A `HashMap< String, serde_json::Value >` where:  
  ///   - Key: The column name (e.g., "A", "B").  
  ///   - Value: The new value to set in the corresponding cell.
  /// 
  /// ## Returns:
  /// - `Result< ValuesAppendResponse >`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn append_row< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key_val : &HashMap< String, serde_json::Value >
  ) -> Result< BatchUpdateValuesResponse >
  {
    // Sort column indexes, from A -> ZZZ
    let mut columns : Vec< ( String, usize, serde_json::Value ) > = row_key_val
    .iter()
    .map( | ( k, v ) | ( k.clone(), column_label_to_number( k ), v.clone() ) )
    .collect();

    columns.sort_by_key( | ( _, col_idx, _ ) | *col_idx );

    let min_idx = 1;
    let max_idx = columns.last().unwrap().1;
    
    let empty_row_size = max_idx - min_idx + 1;
    let empty_row = vec![ json!( "" ); empty_row_size ];

    let range = format!( "{}!A1", sheet_name );
    let empty_value_range = ValueRange
    {
      major_dimension : Some( Dimension::Row ),
      values : Some( vec![ empty_row ] ),
      range : None
    };

    let append_response = client
    .spreadsheet()
    .append( spreadsheet_id, &range, empty_value_range )
    .insert_data_option( InsertDataOption::InsertRows )
    .doit()
    .await;

    let row_index = match append_response
    {
      Ok( ref response ) => parse_row_index
      ( 
        &response
        .updates
        .clone()
        .unwrap()
        .updated_range
        .unwrap()
      )?,
      Err( error ) => return Err( Error::ApiError( error.to_string() ) )
    };

    let total_colspan = max_idx - min_idx + 1;
    let max_subrequests = 100;
    let chunk_size = ( total_colspan + max_subrequests - 1 ) / max_subrequests;

    let mut batch_ranges = Vec::new();
    
    let mut start_col = min_idx;
    let mut idx_cols = 0;
    let col_count = columns.len();

    while start_col <= max_idx 
    {
      let end_col = ( start_col + chunk_size - 1 ).min( max_idx );
      let subrange_len = end_col - start_col + 1;

      let mut row_values = vec![ json!( "" ); subrange_len ];
      while idx_cols < col_count 
      {
        let col_idx = columns[ idx_cols ].1;
        if col_idx < start_col
        {
          idx_cols += 1;
          continue;
        }
        if col_idx > end_col
        {
          break;
        }

        let offset = col_idx - start_col;
        row_values[ offset ] = columns[ idx_cols ].2.clone();
        idx_cols += 1;
      }

      let start_col_label = number_to_column_label( start_col );
      let end_col_label = number_to_column_label( end_col );

      let range_str = if start_col == end_col {
        format!( "{}!{}{}", sheet_name, start_col_label, row_index )
      } else {
        format!
        (
          "{}!{}{}:{}{}",
          sheet_name, start_col_label, row_index, end_col_label, row_index
        )
      };

      let value_range = ValueRange 
      {
        major_dimension : Some( Dimension::Row ),
        values : Some( vec![ row_values ] ),
        range : Some( range_str ),
      };
      batch_ranges.push( value_range );

      // Next chunck;
      start_col = end_col + 1;
    }

    let request = BatchUpdateValuesRequest 
    {
      data : batch_ranges,
      value_input_option : ValueInputOption::UserEntered,
      include_values_in_response : Some( true ),
      response_value_render_option : Some( ValueRenderOption::FormattedValue ),
      response_date_time_render_option : Default::default(),
    };

    match client
    .spreadsheet()
    .values_batch_update( spreadsheet_id, request )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => {
        println!( "{error}" );
        Err( Error::ApiError( error.to_string() ) )
      }
    }
  }

  fn parse_row_index( range_str : &str ) -> Result< u32 >
  {
    let parts : Vec< &str > = range_str.split( '!' ).collect();
    
    let second_part = parts[ 1 ];
    
    let sub_parts : Vec< &str > = second_part.split( ':' ).collect();
    
    let left_part = sub_parts[ 0 ];

    if let Some( caps ) = REGEX_ROW_INDEX.captures( left_part ) 
    {
      let row_str = &caps[ 2 ];
      let row_index = row_str
      .parse::< u32 >()
      .map_err( | err | Error::ParseError( err.to_string() ) )?;
      
      Ok( row_index )
    } 
    else 
    {
      Err( Error::ParseError( format!( "Could not parse column+row from '{left_part}'" ) ) )
    }
  }

  /// # `get_row_by_custom_row_key`
  /// 
  /// Retrieves rows from the specified sheet that match a given "custom row key" value.
  /// [batchGet docs](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchGet).
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet from which rows are to be retrieved.
  /// - `key_by`:  
  ///   A tuple `( column_id, value )` where:
  ///   - `column_letter`: The column identifier (e.g., `"A"`, `"B"`).
  ///   - `value`: A `serde_json::Value` to match in the given column.
  /// - `on_find`:  
  ///   An enum [`OnFind`] defining how to handle multiple matches 
  ///   (e.g., return the first match, last match, or all matches).
  ///
  /// ## Returns:
  /// - `Result< Vec< Vec< serde_json::Value > > >`  
  ///   On success, returns a list of rows, where each row is a `Vec< serde_json::Value >`.
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, 
  ///   such as an invalid spreadsheet ID, insufficient permissions, 
  ///   or any issues during the request/response cycle.
  pub async fn get_row_by_custom_row_key< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    key_by : ( &str, serde_json::Value ),
    on_find : OnFind,
  ) -> Result< Vec< Vec< serde_json::Value > > >
  {
    match get_column
    (
      client, 
      spreadsheet_id, 
      sheet_name, 
      key_by.0
    )
    .await
    {
      Ok( column ) => 
      {
        if column.is_empty()
        {
          return Ok( Vec::new() );
        }
        else 
        {
          let key_matches = get_key_matches( &column, &key_by.1 );
          let row_keys = get_row_keys( key_matches, on_find );

          let mut ranges = Vec::with_capacity( row_keys.len() );
          for row_key in row_keys
          {
            let range = format!( "{}!A{}:ZZZ{}", sheet_name, row_key + 1, row_key + 1 );
            ranges.push( range );
          }

          match client
          .spreadsheet()
          .values_get_batch( spreadsheet_id )
          .ranges( ranges )
          .doit()
          .await
          {
            Ok( response ) =>
            {
              let values : Vec< Vec< serde_json::Value > > = response
              .value_ranges
              .unwrap_or_default()
              .into_iter()
              .flat_map( | range | range.values.unwrap_or_default() )
              .collect();
              
              Ok( values )
            }
            Err( error ) => Err( Error::ApiError( error.to_string() ) )
          }
        }
      },

      Err( error ) => Err( Error::ApiError( error.to_string() ) )
    }

  }



  /// # `get_header`
  ///
  /// Retrieves the header row of a specific sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose header is to be retrieved.
  ///
  /// ## Returns:
  /// - `Result< Vec< Vec< serde_json::Value > > >`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_header< S : Secret >
  (

    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< serde_json::Value > >
  {
    let range = format!( "{}!A1:ZZZ1", sheet_name );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) =>
      {
        match response.values
        {
          Some( values ) => Ok( values[0].clone() ),
          None => Ok( Vec::new() )
        }
      } 
      Err( error ) => Err( error )
    }
    
  }

  /// # get_row
  /// 
  /// Retreive a specific row by its key for a Google Sheet.
  /// 
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the `Client` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose rows are to be retrieved.
  /// - `row_key`:
  ///   A `serde_json::Value` represents row's key. Key starts from 1.
  pub async fn get_row< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    row_key : serde_json::Value
  ) -> Result< Vec< serde_json::Value > >
  {
    let range = format!( "{}!A{}:ZZZ{}", sheet_name, row_key, row_key );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .value_render_option( ValueRenderOption::UnformattedValue )
    .doit()
    .await
    {
      Ok( response ) => 
      {
        match response.values
        {
          Some( values ) =>
          {
            let row = values
            .into_iter()
            .next()
            .unwrap_or_default();

            Ok( row )
          },
          None => Ok( Vec::new() )
        }
      }
      Err( error ) => Err( error )
    }
  }

  /// # `get_rows`
  ///
  /// Retrieves all rows (excluding the header) from a specific sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the `Client` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet whose rows are to be retrieved.
  ///
  /// ## Returns:
  /// - `Result< Vec< Vec< serde_json::Value > > >`
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_rows< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str, 
  ) -> Result< Vec< Vec< serde_json::Value > > >
  {
    let range = format!( "{}!A2:ZZZ", sheet_name );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .value_render_option( ValueRenderOption::UnformattedValue )
    .doit()
    .await
    {
      Ok( response ) => 
      {
        match response.values
        {
          Some( values ) => Ok( values ),
          None => Ok( Vec::new() )
        }
      }
      Err( error ) => Err( error )
    }
    
  }

  /// # `get_cell`
  ///
  /// Retrieves the value of a specific cell from a Google Sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the [`Client`] client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet where the cell is located.
  /// - `cell_id`:  
  ///   A `&str` representing the cell ID in the format `A1`, where `A` is the column and `1` is the row.
  ///
  /// ## Returns:
  /// - `Result< serde_json::Value >`:
  ///
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as an invalid spreadsheet ID
  ///   or insufficient permissions.
  pub async fn get_cell< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str
  ) -> Result< serde_json::Value >
  {
    let range = format!( "{}!{}", sheet_name, cell_id );

    match client
    .spreadsheet()
    .values_get( spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) =>
      {
        match response.values
        {
          Some( values ) => Ok( values[0][0].clone() ),
          None => Ok( json!( "" ) )
        }
      }
      Err( error ) => Err( error )
    }
  }

  /// # `set_cell`
  ///
  /// Updates the value of a specific cell in a Google Sheet.
  ///
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the `Client` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:  
  ///   A `&str` specifying the name of the sheet where the cell is located.
  /// - `cell_id`:  
  ///   A `&str` representing the cell ID in the format `A1`, where `A` is the column and `1` is the row.
  /// - `value`:  
  ///   A `serde_json::Value` containing the new value to update in the cell.
  ///
  /// ## Returns:
  /// - Result< [`UpdateValuesResponse`] >
  /// 
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as invalid input or insufficient permissions.
  pub async fn set_cell< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    cell_id : &str,
    value : serde_json::Value
  ) -> Result< UpdateValuesResponse >
  {
    let range = format!( "{}!{}", sheet_name, cell_id );

    let value_range = ValueRange
    {
      values : Some( vec![ vec![ value ] ] ),
      ..ValueRange::default()
    };

    match client
    .spreadsheet()
    .values_update( value_range, spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( error )
    }
  }

  /// # clear
  /// 
  /// Clears a provided sheet.
  /// 
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the `Client` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:
  ///   A `&str` specifying the name of the sheet where the cell is located.
  /// 
  /// ## Returns:
  /// - Result< [`ValuesClearResponse`] >
  /// 
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as invalid input or insufficient permissions.
  pub async fn clear< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str
  ) -> Result< ValuesClearResponse >
  {
    let range = format!( "{sheet_name}!A:ZZZ" );
    match client
    .spreadsheet()
    .clear( spreadsheet_id, &range )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( error )
    }
  }

  /// # clear_by_custom_row_key
  /// 
  /// Clears matched rows by doing action provided by `on_find`.
  /// 
  /// ## Parameters:
  /// - `client`:  
  ///   A reference to the `Client` client configured for the Google Sheets API.
  /// - `spreadsheet_id`:  
  ///   A `&str` representing the unique identifier of the spreadsheet.
  /// - `sheet_name`:
  ///   A `&str` specifying the name of the sheet where the cell is located.
  /// - `key_by`:
  ///   A tuple representing a column id and value to find in that column.
  /// - `on_find`:
  ///   Action to do on finded matches.
  /// 
  /// ## Returns:
  /// - Result< [`BatchClearValuesResponse`] >
  /// 
  /// ## Errors:
  /// - `Error::ApiError`:  
  ///   Occurs if the Google Sheets API returns an error, such as invalid input or insufficient permissions.
  pub async fn clear_by_custom_row_key< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_name : &str,
    key_by : ( &str, serde_json::Value ),
    on_find : OnFind,
  ) -> Result< BatchClearValuesResponse >
  {
    match get_column
    (
      client, 
      spreadsheet_id, 
      sheet_name, 
      key_by.0
    )
    .await
    {
      Ok( column ) =>
      {
        if column.is_empty()
        {
          return Ok( BatchClearValuesResponse::default() );
        }

        let key_matches = get_key_matches( &column, &key_by.1 );
        let row_keys = get_row_keys( key_matches, on_find );

        let mut ranges = Vec::with_capacity( row_keys.len() );
        for row_key in row_keys
        {
          let range = format!( "{}!A{}:ZZZ{}", sheet_name, row_key + 1, row_key + 1 );
          ranges.push( range );
        }

        let request = BatchClearValuesRequest
        {
          ranges : ranges
        };

        match client
        .spreadsheet()
        .clear_batch( spreadsheet_id, request )
        .doit()
        .await
        {
          Ok( response ) => Ok( response ),
          Err( error ) => Err( error )
        }
      },
      Err( error ) => Err( error )
    }
  }

  /// # copy_to
  /// 
  /// Copies a spreadsheet's sheet to the other spreadsheet.
  /// 
  /// ## Prameters:
  ///  - `client`
  ///   A referebce to a [`Client`] object.
  /// - `spreadsheet_id`
  ///   A reference to string slice which represents a spreadsheet id.
  /// - `sheet_id`
  ///   A reference to a string slice which represents a source sheet's id.
  /// - `dest`
  ///   A reference to a string slice which represents a destination spreadsheet's id.
  /// 
  /// ## Returns:
  ///   - `Result< `[SheetProperties]` >`
  /// 
  /// ## Errors:
  ///   - [`Error::ApiError`]
  ///   - [`Error::ParseError`]
  pub async fn copy_to< S : Secret >
  (
    client : &Client< '_, S >,
    spreadsheet_id : &str,
    sheet_id : &str,
    dest : &str
  ) -> Result< SheetProperties >
  {
    match client
    .sheet()
    .copy_to( spreadsheet_id, sheet_id, dest )
    .doit()
    .await
    {
      Ok( response ) => Ok( response ),
      Err( error ) => Err( error )
    }
  }

  /// Action to do if one or more rows were found.
  pub enum OnFind
  {
    /// Update first matched row.
    FirstMatchedRow,
    /// Update last matched row.
    LastMatchedRow,
    /// Update all matched rows.
    AllMatchedRow,
  }

  /// Action to do if row was not find.
  pub enum OnFail
  {
    /// Returns error.
    Error,
    /// Does nothing.
    Nothing,
    /// Append provided row at the and of sheet.
    AppendRow,
  }
  
}

crate::mod_interface!
{
  own use
  {
    OnFind,
    OnFail,
    set_cell,
    get_cell,
    get_row,
    get_rows,
    update_row,
    get_header,
    append_row,
    update_rows_by_custom_row_key,
    get_row_by_custom_row_key,
    get_column,
    clear,
    clear_by_custom_row_key,
    copy_to
  };
}