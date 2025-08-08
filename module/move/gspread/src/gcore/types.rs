//!
//! Google Sheets API types and data structures.
//!

mod private
{
  use serde_json;
  use ser::
  {
    Serialize,
    Deserialize
  };
  use crate::gcore::client::
  {
    SheetType,
    ColorStyle,
    DataExecutionState,
    DataExecutionErrorCode,
    Dimension,
    ValueRenderOption,
    DateTimeRenderOption,
    ValueInputOption,
    InsertDataOption
  };

  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct SheetCopyRequest
  {
    #[ serde( rename = "destinationSpreadsheetId" ) ]
    pub dest : Option< String >
  }

  /// Properties of a grid.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct GridProperties
  {
    /// The number of rows in the grid. 
    #[ serde( rename = "rowCount" ) ]
    row_count : Option< u64 >,

    /// The number of columns in the grid. 
    #[ serde( rename = "columnCount" ) ]
    column_count : Option< u32 >,

    /// The number of rows that are frozen in the grid. 
    #[ serde( rename = "frozenRowCount" ) ]
    frozen_row_count : Option< u64 >,

    /// The number of columns that are frozen in the grid. 
    #[ serde( rename = "frozenColumnCount" ) ]
    frozen_column_count : Option< u64 >,

    /// True if the grid isn't showing gridlines in the UI. 
    #[ serde( rename = "hideGridlines" ) ]
    hide_grid_lines : Option< bool >,

    /// True if the row grouping control toggle is shown after the group. 
    #[ serde( rename = "rowGroupControlAfter" ) ]
    row_group_control_after : Option< bool >,

    /// True if the column grouping control toggle is shown after the group. 
    #[ serde( rename = "columnGroupControlAfter" ) ]
    column_group_control_after : Option< bool >
  }

  /// Represents a color in the RGBA color space. 
  /// More information here [color google docs](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other#Color)
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct Color
  {
    /// The amount of red in the color as a value in the interval [0, 1]. 
    pub red : Option< f32 >,

    /// The amount of green in the color as a value in the interval [0, 1]. 
    pub green : Option< f32 >,

    /// The amount of blue in the color as a value in the interval [0, 1]. 
    pub blue : Option< f32 >,

    /// The fraction of this color that should be applied to the pixel.
    pub alpha : Option< f32 >
  }

  /// An unique identifier that references a data source column.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DataSourceColumnReference
  {
    /// The display name of the column. It should be unique within a data source. 
    pub name : Option< String >
  }

  /// A column in a data source.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DataSourceColumn
  {
    /// The column reference. 
    pub reference : Option< DataSourceColumnReference >,

    /// The formula of the calculated column. 
    pub formula : Option< String >
  }

  /// The data execution status.
  /// More information [here](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other#DataExecutionStatus)
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DataExecutinStatus
  {
    /// The state of the data execution.
    pub state : Option< DataExecutionState >,

    /// The error code
    #[ serde( rename = "errorCode" ) ]
    pub error_code : Option< DataExecutionErrorCode >,

    /// The error message, which may be empty. 
    #[ serde( rename = "errorMessage" ) ]
    pub error_message : Option< String >,

    /// lastRefreshTime
    #[ serde( rename = "lastRefreshTime" ) ]
    pub last_refresh_time : Option< String >
  }

  /// Additional properties of a [DATA_SOURCE](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/sheets#SheetType) sheet. 
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DataSourceSheetProperties
  {
    /// ID of the [DataSource](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets#DataSource) the sheet is connected to. 
    #[ serde( rename = "dataSourceId" ) ]
    pub data_source_id : Option< String >,

    /// The columns displayed on the sheet, corresponding to the values in [RowData](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/sheets#RowData). 
    pub columns : Option< Vec< DataSourceColumn > >,

    /// The data execution status.
    #[ serde( rename = "dataExecutionStatus" ) ]
    pub data_executin_status : Option< DataExecutinStatus >
  }

  /// Properties of a sheet. 
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct SheetProperties
  {
    /// The ID of the sheet. Must be non-negative. This field cannot be changed once set. 
    #[ serde( rename = "sheetId" ) ]
    pub sheet_id : Option< u64 >,

    /// The name of the sheet. 
    pub title : Option< String >,

    /// The index of the sheet within the spreadsheet. When adding or updating sheet properties, if this field is excluded then
    /// the sheet is added or moved to the end of the sheet list. When updating sheet indices or inserting sheets, movement 
    /// is considered in "before the move" indexes. For example, if there were three sheets (S1, S2, S3) in order to move S1
    /// ahead of S2 the index would have to be set to 2. A sheet index update request is ignored if the requested index is
    /// identical to the sheets current index or if the requested new index is equal to the current sheet index + 1. 
    pub index : Option< u64 >,

    #[ serde( rename = "sheetType" ) ]
    /// The type of sheet. Defaults to GRID. This field cannot be changed once set.
    pub sheet_type : Option< SheetType >,

    /// Additional properties of the sheet if this sheet is a grid. (If the sheet is an object sheet, containing a chart or image, then this field will be absent.) When writing it is an error to set any grid properties on non-grid sheets. 
    #[ serde( rename = "gridProperties" ) ]
    pub grid_properties : Option< GridProperties >,

    /// True if the sheet is hidden in the UI, false if it's visible. 
    pub hidden : Option< bool >,

    /// The color of the tab in the UI. Deprecated: Use tabColorStyle. 
    #[ serde( rename = "tabColor" ) ]
    pub tab_color : Option< Color >,

    /// The color of the tab in the UI. If tabColor is also set, this field takes precedence. 
    #[ serde( rename = "tabColorStyle" ) ]
    pub tab_color_style : Option< ColorStyle >,

    /// True if the sheet is an RTL sheet instead of an LTR sheet. 
    #[ serde( rename = "rightToLeft" ) ]
    pub right_to_left : Option< bool >,

    /// Output only. If present, the field contains DATA_SOURCE sheet specific properties. 
    #[ serde( rename = "dataSourceSheetProperties" ) ]
    pub data_source_sheet_properties : Option< DataSourceSheetProperties >
  }

  #[ derive( Debug, Serialize ) ]
  pub struct GetValuesRequest
  {
    #[ serde( rename = "majorDimension" ) ]
    major_dimension : Option< Dimension >,

    #[ serde( rename = "valueRenderOption" ) ]
    value_render_option : Option< ValueRenderOption >,

    #[ serde( rename = "dateTimeRenderOption" ) ]
    date_time_render_option : Option< DateTimeRenderOption >
  }

  #[ derive( Debug, Serialize ) ]
  pub struct BatchGetValuesRequest
  {
    ranges : Vec< String >,

    #[ serde( rename = "majorDimension" ) ]
    major_dimension : Option< Dimension >,

    #[ serde( rename = "valueRenderOption" ) ]
    value_render_option : Option< ValueRenderOption >,

    #[ serde( rename = "dateTimeRenderOption" ) ]
    date_time_render_option : Option< DateTimeRenderOption >
  }

  #[ derive( Debug, Serialize ) ]
  pub struct UpdateValuesRequest
  {
    #[ serde( rename = "valueInputOption" )]
    value_input_option : ValueInputOption,

    #[ serde( rename = "includeValuesInResponse" ) ]
    include_values_in_response : Option< bool >,

    #[ serde( rename = "responseValueRenderOption" ) ]
    response_value_render_option : Option< ValueRenderOption >,

    #[ serde( rename = "responseDateTimeRenderOption" ) ]
    response_date_time_render_option : Option< DateTimeRenderOption >
  }

  /// The request body.
  #[ derive( Debug, Serialize, Clone ) ]
  pub struct BatchUpdateValuesRequest 
  {
    /// The new values to apply to the spreadsheet.
    pub data : Vec< ValueRange >,

    #[ serde( rename = "valueInputOption" ) ]
    /// How the input data should be interpreted.
    pub value_input_option : ValueInputOption,

    /// Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The updatedData field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).
    #[ serde( rename = "includeValuesInResponse" ) ]
    pub include_values_in_response : Option< bool >,

    /// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
    #[ serde( rename = "responseValueRenderOption" ) ]
    pub response_value_render_option : Option< ValueRenderOption >,

    /// Determines how dates, times, and durations in the response should be rendered. This is ignored if responseValueRenderOption is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    #[ serde( rename = "responseDateTimeRenderOption" ) ]
    pub response_date_time_render_option : Option< DateTimeRenderOption >,
  }

  #[ derive( Debug, Serialize ) ]
  pub struct ValuesAppendRequest
  {
    #[ serde( rename = "valueInputOption" ) ]
    pub value_input_option : ValueInputOption,
    
    #[ serde( rename = "insertDataOption" ) ]
    pub insert_data_option : Option< InsertDataOption >,

    #[ serde( rename = "includeValuesInResponse" ) ]
    pub include_values_in_response : bool,

    #[ serde( rename = "responseValueRenderOption" ) ]
    pub response_value_render_option : Option< ValueRenderOption >,

    #[ serde( rename = "responseDateTimeRenderOption" ) ]
    pub response_date_time_render_option : Option< DateTimeRenderOption >
  }

  /// The request body.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct BatchClearValuesRequest
  {
    /// The ranges to clear, in A1 notation or R1C1 notation.
    pub ranges : Vec< String >
  }

  /// Response from [`values.batchGet`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchGet).
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct BatchGetValuesResponse 
  {
    /// The ID of the spreadsheet.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// A list of ValueRange objects with data for each requested range.
    #[ serde( rename = "valueRanges" ) ]
    pub value_ranges : Option< Vec< ValueRange > >,
  }

  /// Response from [`values.update`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/update).
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
  pub struct UpdateValuesResponse 
  {
    /// The ID of the spreadsheet that was updated.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// The range (A1 notation) that was updated.
    #[ serde( rename = "updatedRange" ) ]
    pub updated_range : Option< String >,

    /// How many rows were updated.
    #[ serde( rename = "updatedRows" ) ]
    pub updated_rows : Option< u32 >,

    /// How many columns were updated.
    #[ serde( rename = "updatedColumns" ) ]
    pub updated_columns : Option< u32 >,

    /// How many cells were updated.
    #[ serde( rename = "updatedCells" ) ]
    pub updated_cells : Option< u32 >,

    /// If `includeValuesInResponse` was `true`, this field contains the updated data.
    #[ serde( rename = "updatedData" ) ]
    pub updated_data : Option< ValueRange >,
  }

  /// Response from [`values.batchUpdate`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchUpdate).
  #[ derive( Debug, Default, Serialize, Deserialize, Clone ) ]
  pub struct BatchUpdateValuesResponse 
  {
    /// The ID of the spreadsheet that was updated.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// Total number of rows updated.
    #[ serde( rename = "totalUpdatedRows" ) ]
    pub total_updated_rows : Option< u32 >,

    /// Total number of columns updated.
    #[ serde( rename = "totalUpdatedColumns" ) ]
    pub total_updated_columns : Option< u32 >,

    /// Total number of cells updated.
    #[ serde( rename = "totalUpdatedCells" ) ]
    pub total_updated_cells : Option< u32 >,

    /// Total number of sheets with updates.
    #[ serde( rename = "totalUpdatedSheets" ) ]
    pub total_updated_sheets : Option< u32 >,

    /// The response for each range updated (if `includeValuesInResponse` was `true`).
    pub responses : Option< Vec< ValueRange > >,
  }

  /// Response from [`values.append`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/append).
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
  pub struct ValuesAppendResponse 
  {
    /// The ID of the spreadsheet to which data was appended.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// The range (A1 notation) that covered the appended data before the append.
    #[ serde( rename = "tableRange" ) ]
    pub table_range : Option< String >,

    /// If `includeValuesInResponse` was `true`, this field contains metadata about the update.
    pub updates : Option< UpdateValuesResponse >,
  }

  /// Response from [values.clearBatch](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/batchClear)
  #[ derive( Debug, Default, Serialize, Deserialize ) ]
  pub struct BatchClearValuesResponse
  {
    /// The spreadsheet the updates were applied to.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// The ranges that were cleared, in A1 notation. If the requests are for an unbounded range or a ranger larger than the bounds of the sheet, this is the actual ranges that were cleared, bounded to the sheet's limits.
    #[ serde( rename = "clearedRanges" ) ]
    pub cleared_ranges : Option< Vec< String > >
  }

  /// Response from [`values.clear`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets.values/clear)
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct ValuesClearResponse
  {
    /// The spreadsheet the updates were applied to.
    #[ serde( rename = "spreadsheetId" ) ]
    pub spreadsheet_id : Option< String >,

    /// The range (in A1 notation) that was cleared. (If the request was for an unbounded range or a ranger larger than the bounds of the sheet, this will be the actual range that was cleared, bounded to the sheet's limits.)
    #[ serde( rename = "clearedRange" ) ]
    pub cleared_range : Option< String >
  }

  /// Data within a range of the spreadsheet.
  #[ derive( Debug, Clone, Default, serde::Serialize, serde::Deserialize ) ]
  pub struct ValueRange
  {
    /// The range the values cover, in A1 notation. For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended.
    pub range : Option< String >,

    /// The major dimension of the values.
    /// For output, if the spreadsheet data is: A1=1,B1=2,A2=3,B2=4, then requesting range=A1:B2,majorDimension=ROWS will return [[1,2],[3,4]], whereas requesting range=A1:B2,majorDimension=COLUMNS will return [[1,3],[2,4]].
    ///
    /// For input, with range=A1:B2,majorDimension=ROWS then [[1,2],[3,4]] will set A1=1,B1=2,A2=3,B2=4. With range=A1:B2,majorDimension=COLUMNS then [[1,2],[3,4]] will set A1=1,B1=3,A2=2,B2=4.
    ///
    /// When writing, if this field is not set, it defaults to ROWS.
    #[ serde( rename = "majorDimension" ) ]
    pub major_dimension : Option< Dimension >,

    /// The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell.
    ///
    /// For output, empty trailing rows and columns will not be included.
    /// 
    /// For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string.
    pub values : Option< Vec< Vec< serde_json::Value > > >
  }

}

crate::mod_interface!
{
  exposed use private::SheetCopyRequest;
  exposed use private::GridProperties;
  exposed use private::Color;
  exposed use private::DataSourceColumnReference;
  exposed use private::DataSourceColumn;
  exposed use private::DataExecutinStatus;
  exposed use private::DataSourceSheetProperties;
  exposed use private::SheetProperties;
  exposed use private::GetValuesRequest;
  exposed use private::BatchGetValuesRequest;
  exposed use private::UpdateValuesRequest;
  exposed use private::BatchUpdateValuesRequest;
  exposed use private::ValuesAppendRequest;
  exposed use private::BatchClearValuesRequest;
  exposed use private::BatchGetValuesResponse;
  exposed use private::UpdateValuesResponse;
  exposed use private::BatchUpdateValuesResponse;
  exposed use private::ValuesAppendResponse;
  exposed use private::BatchClearValuesResponse;
  exposed use private::ValuesClearResponse;
  exposed use private::ValueRange;
}