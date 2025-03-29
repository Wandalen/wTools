

mod private
{
  use clap::Subcommand;
  use serde_json::json;
  use debug::
  {
    Report, RowWrapper
  };
  
  use crate::*;
  use gcore::Secret;
  use gcore::client::Client;
  use actions::
  {
    self,
    utils::get_spreadsheet_id_from_url
  };

  /// # Commands
  ///
  /// Subcommands for the `ROW` command.
  ///
  /// ## Variants:
  ///
  /// ### `Append`
  /// Appends a new row to at the end of Google Sheet.
  /// 
  /// **Arguments:**
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example:  
  ///   `--url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'`
  ///
  /// - `tab`:  
  ///   The name of the specific sheet (tab) in the Google Spreadsheet.  
  ///   Example:  
  ///   `--tab 'Sheet1'`
  ///
  /// - `json`:  
  ///   A string containing the key-value pairs for the new row.  
  ///   The keys are column names (only uppercase Latin letters, e.g. `"A"`, `"B"`, etc.),  
  ///   and the values are strings or other JSON-compatible data.  
  ///   Depending on the shell, you may need to escape quotes.  
  ///   Examples:  
  ///   1. `--json '{"A": "value1", "B": "value2"}'`  
  ///   2. `--json "{\\\"A\\\": \\\"value1\\\", \\\"B\\\": \\\"value2\\\"}"`
  ///
  /// **Example:**
  /// ```bash
  /// gspread row append \
  ///   --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
  ///   --tab 'tab1' \
  ///   --json '{"A": "Hello", "B": "World"}'
  /// ```
  /// 
  /// ### `Update`
  /// Updates a specific row.
  /// 
  /// **Arguments**
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example:  
  ///   `--url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'`
  /// 
  /// - `tab`:  
  ///   The name of the specific sheet (tab) in the Google Spreadsheet.  
  ///   Example:  
  ///   `--tab 'Sheet1'`
  /// 
  /// - `json`:  
  ///   A JSON string of column-value pairs that you want to update.  
  ///   The keys should be valid column names (uppercase letters only),  
  ///   and values are JSON-compatible.  
  ///   Example:  
  ///   `--json '{"id": 2, "A": 10, "B": "Some text"}'`
  /// 
  /// - `select_row_by_key`:  
  ///   A string specifying the identifier of the row to update.  
  ///   Example: `"id"`.
  /// 
  /// **Example:**
  /// ```bash
  /// gspread row update \
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
  /// --tab tab1 \
  /// --select-row-by-key "id" \
  /// --json '{"id": 2, "A": 1, "B": 2}'
  /// ```
  ///
  /// ### `UpdateCustom`
  /// Updates one or more rows in a Google Sheet based on a custom key (or condition),
  /// and offers control over what to do if no rows are found.
  ///
  /// **Arguments:**
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example:  
  ///   `--url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'`
  ///
  /// - `tab`:  
  ///   The name of the specific sheet (tab) in the Google Spreadsheet.  
  ///   Example:  
  ///   `--tab 'Sheet1'`
  ///
  /// - `json`:  
  ///   A JSON string of column-value pairs that you want to update.  
  ///   The keys should be valid column names (uppercase letters only),  
  ///   and values are JSON-compatible.  
  ///   Example:  
  ///   `--json '{"A": "10", "B": "Some text"}'`
  ///
  /// - `key_by`:  
  ///   An expression specifying **which** rows to match.  
  ///   Example:  
  ///   or  
  ///   `--key-by '["columnX", value_to_find]'`  
  ///
  /// - `on_fail`:  
  ///   What to do if **no rows are found** matching the key.  
  ///   Possible values might be `Error`, `AppendRow`, or `Nothing` (depending on your logic).  
  ///
  /// - `on_find`:  
  ///   What to do if **one or multiple** rows are found.  
  ///   Possible values might be `UpdateFirstMatchedRow`, `UpdateLastMatchedRow`, or `UpdateAllMatchedRow`.
  ///
  /// **Example:**
  /// ```bash
  /// gspread row update-custom \
  ///   --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
  ///   --tab tab1 \
  ///   --json '{"A": "newVal", "B": "updatedVal"}' \
  ///   --key-by '["C", 12]' \
  ///   --on_fail append \
  ///   --on_find all
  /// ```
  /// 
  /// ### `Get`
  /// Retrieves a specific row from a Google Sheet.
  /// 
  /// **Arguments:**
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example:  
  ///   `--url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'`
  ///
  /// - `tab`:  
  ///   The name of the specific sheet (tab) in the Google Spreadsheet.  
  ///   Example:  
  ///   `--tab 'Sheet1'`
  /// 
  /// - `row-key`:
  ///   Row key (id). The range starts from 1.
  ///   Example:
  ///   `row-key 2`
  /// 
  /// **Example:**
  /// 
  /// gspread row get
  /// --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
  /// --tab 'tab1'
  /// 
  /// ### `GetCustom`
  /// Retrieves one or more rows from a Google Sheet based on a custom key condition,
  /// specifying how to handle multiple matches.
  ///
  /// **Arguments:**
  /// - `url`:  
  ///   The full URL of the Google Sheet.  
  ///   Example:  
  ///   `--url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'`
  ///
  /// - `tab`:  
  ///   The name of the specific sheet (tab) in the Google Spreadsheet.  
  ///   Example:  
  ///   `--tab 'Sheet1'`
  ///
  /// - `key_by`:  
  ///   A JSON array of the form `[<column>, <value>]`, defining which rows to match.  
  ///   For instance, if you pass `["A", "Hello"]`, the function will look in column `A`  
  ///   for cells whose value equals `"Hello"`.  
  ///   Example:  
  ///   `--key-by '["C", 12]'`
  ///
  /// - `on_find`:  
  ///   Defines how to handle situations where multiple rows match the key.  
  ///   Possible values (depending on your logic):  
  ///   - `all`: Return **all** matched rows,  
  ///   - `first`: Return **only the first** matched row,  
  ///   - `last`: Return **only the last** matched row.
  ///
  /// **Example:**
  /// ```bash
  /// gspread row get-custom \
  ///   --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \
  ///   --tab 'Sheet1' \
  ///   --key-by '["C", 12]' \
  ///   --on-find all
  /// ```
  #[ derive( Debug, Subcommand ) ]
  #[ command( long_about = "\n\nSubcommands for `ROW` command" ) ]
  pub enum Commands
  {
    #[ command( name = "append", about = "Appends a new row at the end of Google Sheet.", long_about = r#"
---------------------------------------------------------------------------------------------------------------
                                              ROW APPEND                                                       
---------------------------------------------------------------------------------------------------------------
 ● Description:                                                                                                
   ↓ ↓ ↓ ↓ ↓ ↓                                                                                                 
                                                                                                               
 Appends a new row at the end of the Google Sheet.                                                             
                                                                                                               
 The new row is generated by the `--json` argument, which should contain key-value pairs                       
 where the key is a column ID and the value is the data to insert. Column IDs can range from `A` to `ZZZ`.     
                                                                                                               
 Values are inserted according to their type:                                                                  
   • `{"A":1}` will parse the value as an integer.                                                             
   • `{"A":true}` or `{"A":false}` will parse the value as a boolean.                                          
   • Any string should be quoted, e.g. `"true"`, `"Hello"` or `"123"`.                                         
                                                                                                               
 If there is empty space between columns (for instance, providing values for columns C, D, and F),             
 then empty strings `("")` will be inserted into columns A, B, and E.                                          
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Command example:                                                                                            
   ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓                                                                                             
                                                                                                               
 cargo run gspread row append \                                                                                
         --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \  
         --tab 'tab1' \                                                                                        
         --json '{"A": "Hello", "B": "World"}'                                                                 
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Output:  Prints a message with the amount of updated cells:                                                 
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
 Row was successfully append at the end of the sheet! Amount of updated cells: 2                               
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Errors:                                                                                                     
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
  ◦ Error::ApiError:                                                                                           
    ----------------------------------------------------------------                                         
     Occurs if the Google Sheets API returns an error,                                                       
     such as an invalid spreadsheet ID, insufficient permissions                                             
     or invalid sheet name.                                                                                  
    ----------------------------------------------------------------                                         
                                                                                                               
  ◦ Error::ParseError:                                                                                         
    -----------------------------------------------------------                                              
     Occurs when serde_json can not parse an argument                                                        
    -----------------------------------------------------------                                              
                                                                                                               
  ◦ Error::InvalidURL:                                                                                         
    ------------------------------------------------------------------------                                 
     Occurs when you passed url with an invalid format of your spreadsheet.                                  
    ------------------------------------------------------------------------                                 
                                                                                                               
---------------------------------------------------------------------------------------------------------------
    "# ) ]
    Append
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Value range. 
      The key is a column name (not a header name, but a column name, which can only contain Latin letters).
      Depending on the shell, different handling might be required.\n\
      Examples:\n\
      1. --json '{\"A\": 1, \"B\": \"Hello\"}'\n\
      2. --json '{\\\"A\\\": 1, \\\"B\\\": \\\"Hello\\\"}'\n" ) ]
      json : String
    },

    #[ command( name = "update", about = "Updates a single row.", long_about = r#"
---------------------------------------------------------------------------------------------------------------
                                             ROW UPDATE                                                        
---------------------------------------------------------------------------------------------------------------
 ● Description:                                                                                                
   ↓ ↓ ↓ ↓ ↓ ↓                                                                                                 
                                                                                                               
 This command performs a batch update of a row specified by the `--select_row_by_key` argument                 
 and its corresponding value in the `--json` argument.                                                         
                                                                                                               
 Essentially, you define which row to update by providing a key (e.g., "id") in `--select_row_by_key`,         
 and then within `--json`, you supply both the key-value pair for identifying the row (e.g., "id": 2)          
 and the columns to be updated with their new values (e.g., "A": 1, "B": 2).                                   
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Command example:                                                                                            
   ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓                                                                                             
                                                                                                               
 cargo run gspread row update \                                                                                
         --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \  
         --tab tab1 \                                                                                          
         --select-row-by-key "id" \                                                                            
         --json '{"id": 2, "A": 1, "B": 2}'                                                                    
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Output:  Prints a message with the amount of updated cells:                                                 
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
 2 cells were successfully updated!                                                                            
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Errors:                                                                                                     
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
  ◦ Error::ApiError:                                                                                           
    ----------------------------------------------------------------                                         
     Occurs if the Google Sheets API returns an error,                                                       
     such as an invalid spreadsheet ID, insufficient permissions                                             
     or invalid sheet name.                                                                                  
    ----------------------------------------------------------------                                         
                                                                                                               
  ◦ Error::InvalidURL:                                                                                         
    ------------------------------------------------------------------------                                 
     Occurs when you passed url with an invalid format of your spreadsheet.                                  
    ------------------------------------------------------------------------                                 
                                                                                                               
  ◦ Error::ParseError:                                                                                         
    ----------------------------------------------------------------------                                   
     Occurs when serde_json cannot parse the provided `--json` argument.                                     
     Or if you input wrong `--select_row_by_key`                                                             
    ----------------------------------------------------------------------                                   
                                                                                                              
---------------------------------------------------------------------------------------------------------------
    "# ) ]
    Update
    {
      #[ arg( long, help = "Identifier of a row. Available identifiers: id (row's unique identifier).\n\
      Example: --select_row_by_key \"id\"" ) ]
      select_row_by_key : String,
      
      #[ arg( long, help = "Value range. It must contain select_row_by_key.
      The key is a column name (not a header name, but a column name, which can only contain Latin letters).
      Every key and value must be a string.
      Depending on the shell, different handling might be required.\n\
      Examples:\n\
      1. --json '{\"id\": 3, \"A\": 1, \"B\": 2}'\n\
      3. --json '{\\\"id\\\": 3, \\\"A\\\": \\\"Hello\\\", \\\"B\\\": true}'\n" ) ]
      json : String,

      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String
    },

    #[ command( name = "update-custom", about = "Updates rows according to '--key-by', '--on-find' and '--on-fail' arguments.", long_about = r#"
---------------------------------------------------------------------------------------------------------------
                                         ROW UPDATE-CUSTOM                                                     
---------------------------------------------------------------------------------------------------------------
 ● Description:                                                                                                
   ↓ ↓ ↓ ↓ ↓ ↓                                                                                                 
                                                                                                               
 Updates range specified by `key-by`, `on-find` and `on-fail` actions.                                         
                                                                                                               
 • `key-by` is a tuple of column ID and a value to find in that column.                                        
   For example, `--key-by ["A", 2]` means "We are looking for the value `2` in the column with ID `A`."        
                                                                                                               
 • `on-find` is the action performed upon finding that value. There are 3 variants:                            
   1. Update only the first matched row.                                                                       
   2. Update only the last matched row.                                                                        
   3. Update all matched rows.                                                                                 
                                                                                                               
 • `on-fail` is the action performed if no match is found. There are 3 variants:                               
   1. Do nothing.                                                                                              
   2. Return an error.                                                                                         
   3. Append a new row (using `--json` data) at the end of the sheet.                                          
                                                                                                               
 For example, consider the following table:                                                                    
 |-----------|                                                                                                 
 | A | B | C |                                                                                                 
 |-----------|                                                                                                 
 | 1 | . | . |                                                                                                 
 | 1 | . | . |                                                                                                 
 | 2 | . | . |                                                                                                 
 | 3 | . | . |                                                                                                 
 | 1 | . | . |                                                                                                 
 |-----------|                                                                                                 
                                                                                                               
 If we run: `cargo run row update-custom ... --key-by ["A", 1] --on-find (action) --on-fail (action)`,         
 the program will find all rows which contain the value `1` in column `A`                                      
 and update them according to the specified `on-find` action.                                                  
                                                                                                               
 If there are no matches, the `--on-fail` action takes place.                                                  
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Command example:                                                                                            
   ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓                                                                                             
                                                                                                               
 cargo run gspread row update-custom \                                                                         
         --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \  
         --tab 'tab1' \                                                                                        
         --json '{"A": "newVal", "B": "updatedVal"}' \                                                         
         --key-by '["C", 12]' \                                                                                
         --on-fail error \                                                                                     
         --on-find first                                                                                       
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Output:  Depending on whether the value is found:                                                           
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
 • If value was found:                                                                                         
   2 cells were successfully updated!                                                                          
                                                                                                               
 • Otherwise (no match):                                                                                       
   Row key was not found, provided action has worked.                                                          
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Errors:                                                                                                     
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
  ◦ Error::ApiError:                                                                                           
    ----------------------------------------------------------------                                         
     Occurs if the Google Sheets API returns an error,                                                       
     such as an invalid spreadsheet ID, insufficient permissions                                             
     or invalid sheet name.                                                                                  
    ----------------------------------------------------------------                                         
                                                                                                               
  ◦ Error::ParseError:                                                                                         
    ----------------------------------------------------------------                                         
     Occurs when serde_json cannot parse the provided `--json`.                                              
    ----------------------------------------------------------------                                         
                                                                                                               
  ◦ Error::InvalidURL:                                                                                         
    ----------------------------------------------------------------------                                   
     Occurs when you pass a URL with an invalid spreadsheet format.                                          
    ----------------------------------------------------------------------                                   
                                                                                                               
---------------------------------------------------------------------------------------------------------------
    "# ) ]
    UpdateCustom
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "Value range. 
      The key is a column name (not a header name, but a column name, which can only contain Latin letters).
      Depending on the shell, different handling might be required.\n\
      Examples:\n\
      1. --json '{\"A\": 1, \"B\": 2}'\n\
      2. --json '{\\\"A\\\": \\\"Hello\\\", \\\"B\\\": \\\"World\\\"}'\n" ) ]
      json : String,

      #[ arg( long, help = "A string with key pair view, like [\"A\", \"new_val\"], where A is a column index." ) ]
      key_by : String,

      #[ arg( long, help = "Action to take if no rows are found.
      Available: 
        - none - Does nothing.
        - error - Return an error.
        - append - Append a new row at the end of sheet." ) ]
      on_fail : String,

      #[ arg( long, help = "Action to take if one or more rows are found.
      Available: 
        - all - Update all matched rows, with provided values.
        - first - Update first matched row with provided values.
        - last - Update last matched row with provided data." ) ]
      on_find : String
    },

    #[ command( name = "get", about = "Retrieves a single row.", long_about = r#"
---------------------------------------------------------------------------------------------------------------
                                                 ROW GET                                                       
---------------------------------------------------------------------------------------------------------------
 ● Description:                                                                                                
   ↓ ↓ ↓ ↓ ↓ ↓                                                                                                 
                                                                                                               
 Retrieves a specific row from a Google Sheet, identified by the `--row-key` argument.                         
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Command example:                                                                                            
   ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓                                                                                             
                                                                                                               
 gspread row get \                                                                                             
         --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \  
         --tab 'tab1' \                                                                                        
         --row-key 2                                                                                           
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Output:  Prints the retrieved row:                                                                          
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
 Row:                                                                                                          
 │ 0  │      1       │ 2  │                                                                                    
 ───────────────────────────                                                                                   
 │ 1  │ "updatedVal" │ 20 │                                                                                    
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Errors:                                                                                                     
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
  ◦ Error::ApiError:                                                                                          
    ----------------------------------------------------------------                                         
     Occurs if the Google Sheets API returns an error,                                                       
     such as an invalid spreadsheet ID, insufficient permissions                                             
     or invalid sheet name.                                                                                  
    ----------------------------------------------------------------                                         
                                                                                                               
  ◦ Error::ParseError:                                                                                         
    ---------------------------------------------------------                                                
     Occurs when serde_json::Value parse error.                                                              
    ---------------------------------------------------------                                                
                                                                                                               
  ◦ Error::InvalidURL:                                                                                         
    ----------------------------------------------------------------------                                   
     Occurs when you passed url with invalid format of your spreadsheet.                                     
    ----------------------------------------------------------------------                                   
                                                                                                               
---------------------------------------------------------------------------------------------------------------
    "# ) ]
    Get
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "A row key. Example: row_key=2" ) ]
      row_key : u32,
    },

    #[ command( name = "get-custom", about = "Retrieves rows according to `--key-by` and `--on-find` arguments.", long_about = r#"
---------------------------------------------------------------------------------------------------------------
                                           ROW GET-CUSTOM                                                      
---------------------------------------------------------------------------------------------------------------
 ● Description:                                                                                                
   ↓ ↓ ↓ ↓ ↓ ↓                                                                                                 
                                                                                                               
 Gets a range of rows specified by `key-by` and `on-find` actions.                                             
                                                                                                               
 • `key-by` is a tuple of column ID and a value to find in that column.                                        
   For example, `--key-by ["A", 2]` means “We are looking for the value `2` in the column with ID `A`.”        
                                                                                                               
 • `on-find` is the action to perform upon finding that value. There are 3 variants:                           
   1. Get only the first matched row.                                                                          
   2. Get only the last matched row.                                                                           
   3. Get all matched rows.                                                                                    
                                                                                                               
 For example, consider the following table:                                                                    
 |-----------|                                                                                                 
 | A | B | C |                                                                                                 
 |-----------|                                                                                                 
 | 1 | . | . |                                                                                                 
 | 1 | . | . |                                                                                                 
 | 2 | . | . |                                                                                                 
 | 3 | . | . |                                                                                                 
 | 1 | . | . |                                                                                                 
 |-----------|                                                                                                 
                                                                                                               
 If we run: `cargo run row get-custom ... --key-by ["A", 1] --on-find (action)`                                
 the program will find all rows which contain the value `1` in column `A`                                      
 and retrieve them according to the specified `on-find` action.                                                
                                                                                                               
 If there are no matches, nothing happens.                                                                     
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Command example:                                                                                            
   ↓ ↓ ↓ ↓ ↓ ↓ ↓ ↓                                                                                             
                                                                                                               
 cargo run gspread row get-custom \                                                                            
         --url 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}' \  
         --tab tab1 \                                                                                          
         --key-by '["A", 1]' \                                                                                 
         --on-find all                                                                                         
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Output:  Prints the retrieved rows:                                                                         
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
 Rows:                                                                                                         
 │ 0   │ 1  │ 2  │ 3  │ 4  │ 5   │                                                                             
 ─────────────────────────────────                                                                             
 │ "1" │ "" │ "" │ "" │ "" │ "a" │                                                                             
 │ "1" │ "" │ "" │ "" │ "" │ "a" │                                                                             
 │ "1" │ "" │ "" │ "" │ "" │ "a" │                                                                             
 │ "1" │ "" │ "" │ "" │ "" │ "a" │                                                                             
 │ "1" │ "" │ "" │ "" │ "" │ "a" │                                                                             
 │ "1" │ "" │ "" │ "" │ "" │ "a" │                                                                             
 │ "1" │ "" │ "" │ "" │ "" │ "a" │                                                                             
 │ "1" │ "" │ "" │ "" │ "" │ "a" │                                                                             
                                                                                                               
---------------------------------------------------------------------------------------------------------------
 ● Errors:                                                                                                     
   ↓ ↓ ↓ ↓                                                                                                     
                                                                                                               
  ◦ Error::ApiError:                                                                                           
    ----------------------------------------------------------------                                         
     Occurs if the Google Sheets API returns an error,                                                       
     such as an invalid spreadsheet ID, insufficient permissions                                             
     or invalid sheet name.                                                                                  
    ----------------------------------------------------------------                                         
                                                                                                               
  ◦ Error::ParseError:                                                                                        
    ---------------------------------------------------------                                                
     Occurs when serde_json::Value parse error.                                                              
    ---------------------------------------------------------                                                
                                                                                                               
  ◦ Error::InvalidURL:                                                                                         
    ----------------------------------------------------------------------                                   
     Occurs when you pass a URL with an invalid spreadsheet format.                                          
    ----------------------------------------------------------------------                                   
                                                                                                               
---------------------------------------------------------------------------------------------------------------
    "# ) ]
    GetCustom
    {
      #[ arg( long, help = "Full URL of Google Sheet.\n\
      It has to be inside of '' to avoid parse errors.\n\
      Example: 'https://docs.google.com/spreadsheets/d/{spreadsheet_id}/edit?gid={sheet_id}#gid={sheet_id}'" ) ]
      url : String,

      #[ arg( long, help = "Sheet name.\nExample: Sheet1" ) ]
      tab : String,

      #[ arg( long, help = "A string with key pair view, like [\"A\", \"val\"], where A is a column index." ) ]
      key_by : String,

      #[ arg( long, help = "Action to take if one or more rows are found.
      Available: 
        - all - Retreive all matched rows.
        - first - Retreive first matched row.
        - last - Retreive last matched row." ) ]
      on_find : String
    }
  }

  pub async fn command< S : Secret >
  (
    client : &Client< '_, S >,
    commands : Commands
  )
  {
    match commands
    {
      Commands::Append { url, tab, json } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( &url ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_row_append::action( client, spreadsheet_id, &tab, &json ).await
        {
          Ok( updated_cells ) => println!
          ( 
            "Row was successfully append at the end of the sheet! Amount of updated cells: {} ",
            updated_cells
          ),

          Err( error ) => eprintln!( "Error\n{}", error )
        }
      },

      Commands::UpdateCustom { url, tab, json, key_by, on_fail, on_find } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( &url ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_row_update_custom::action
        ( 
          client, 
          spreadsheet_id, 
          &tab, 
          &key_by, 
          &json, 
          &on_find, 
          &on_fail 
        ).await
        {
          Ok( val ) => 
          {
            match val
            {
              0 => println!( "Row key was not found, provided action has worked." ),
              _ => println!( "{} cells were sucsessfully updated!", val )
            }
          },
          Err( error ) => eprintln!( "Error\n{}", error )
        }
      },

      Commands::Update { select_row_by_key, json, url, tab } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( &url ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_row_update::action
        (
          client,
          &select_row_by_key,
          &json,
          spreadsheet_id,
          &tab
        )
        .await
        {
          Ok( val ) => println!( "{} cells were sucsessfully updated!", val ),
          Err( error ) => println!( "Error:\n{}", error )
        }
      },

      Commands::Get { url, tab, row_key } => 
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( &url ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_row_get::action
        (
          client, 
          spreadsheet_id, 
          &tab,
          json!( row_key )
        )
        .await
        {
          Ok( row ) => 
          {
            let row_wrapped = RowWrapper
            {
              max_len : row.len(), 
              row : row
            };

            println!( "Row:\n{}", Report{ rows: vec![ row_wrapped ] } );
          },
          Err( error ) => eprintln!( "Error:\n{}", error ),
        }
      }

      Commands::GetCustom { url, tab, key_by, on_find } =>
      {
        let spreadsheet_id = match get_spreadsheet_id_from_url( &url ) 
        {
          Ok( id ) => id,
          Err( error ) => 
          {
            eprintln!( "Error extracting spreadsheet ID: {}", error );
            return;
          }
        };

        match actions::gspread_row_get_custom::action
        (
          client, 
          spreadsheet_id, 
          &tab, 
          &key_by, 
          &on_find
        )
        .await
        {
          Ok( rows ) =>
          {
            let max_len = rows
            .iter()
            .map( | row | row.len() )
            .max()
            .unwrap_or( 0 );

            let rows_wrapped: Vec< RowWrapper > = rows
            .into_iter()
            .map( | row | RowWrapper { row, max_len } )
            .collect();

            println!( "Rows:\n{}", Report{ rows : rows_wrapped } );
          }
          Err( error ) => eprintln!( "Error:\n{}", error ),
        }
      }
    }
  }
}

crate::mod_interface!
{
  own use
  {
    Commands,
    command
  };
}