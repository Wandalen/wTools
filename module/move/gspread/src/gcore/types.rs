

mod private
{
  use crate::*;
  use ser::
  { 
    self, 
    Serialize, 
    Deserialize 
  };

  /// Indicates which dimension an operation should apply to.
  #[ derive( Debug, Clone, Copy, Serialize, Deserialize ) ]
  pub enum Dimension 
  {
    /// Operates on the rows of a sheet.
    #[ serde( rename = "ROWS" ) ]
    Row,

    /// Operates on the columns of a sheet.
    #[ serde( rename = "COLUMNS" ) ]
    Column,
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

  /// Determines how input data should be interpreted.
  #[ derive( Debug, Clone, Copy, Default, Serialize ) ]
  pub enum ValueInputOption
  {
    /// The values the user has entered will not be parsed and will be stored as-is.
    #[ default ]
    #[ serde( rename = "RAW" ) ]
    Raw,

    /// The values will be parsed as if the user typed them into the UI. Numbers will stay as numbers, but strings may be converted to numbers, dates, etc. following the same rules that are applied when entering text into a cell via the Google Sheets UI.
    #[ serde( rename = "USER_ENTERED" ) ]
    UserEntered
  }

  /// Determines how values should be rendered in the output.
  #[ derive( Debug, Clone, Copy, Serialize ) ]
  pub enum ValueRenderOption
  {
    /// Values will be calculated & formatted in the response according to the cell's formatting. Formatting is based on the spreadsheet's locale, not the requesting user's locale. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return "$1.23".
    #[ serde( rename = "FORMATTED_VALUE" ) ]
    FormattedValue,

    /// Values will be calculated, but not formatted in the reply. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return the number 1.23.
    #[ serde( rename = "UNFORMATTED_VALUE" ) ]
    UnformattedValue,

    /// Values will not be calculated. The reply will include the formulas. For example, if A1 is 1.23 and A2 is =A1 and formatted as currency, then A2 would return "=A1".
    ///
    /// Sheets treats date and time values as decimal values. This lets you perform arithmetic on them in formulas. For more information on interpreting date and time values, see About date & time values.
    #[ serde( rename = "FORMULA" ) ]
    Formula
  }

  /// Determines how dates should be rendered in the output.
  #[ derive( Debug, Clone, Copy, Serialize ) ]
  pub enum DateTimeRenderOption
  {
    /// Instructs date, time, datetime, and duration fields to be output as doubles in "serial number" format, as popularized by Lotus 1-2-3. The whole number portion of the value (left of the decimal) counts the days since December 30th 1899. The fractional portion (right of the decimal) counts the time as a fraction of the day. For example, January 1st 1900 at noon would be 2.5, 2 because it's 2 days after December 30th 1899, and .5 because noon is half a day. February 1st 1900 at 3pm would be 33.625. This correctly treats the year 1900 as not a leap year.
    #[ serde( rename = "SERIAL_NUMBER" ) ]
    SerialNumber,

    /// Instructs date, time, datetime, and duration fields to be output as strings in their given number format (which depends on the spreadsheet locale).
    #[ serde( rename = "FORMATTED_STRING" ) ]
    FormattedString
  }

  /// Determines how existing data is changed when new data is input.
  #[ derive( Debug, Clone, Copy, Serialize, Deserialize ) ]
  pub enum InsertDataOption
  {
    /// The new data overwrites existing data in the areas it is written. (Note: adding data to the end of the sheet will still insert new rows or columns so the data can be written.)
    #[ serde( rename = "OVERWRITE" ) ]
    Overwrite,

    /// Rows are inserted for the new data.
    #[ serde( rename = "INSERT_ROWS" ) ]
    InsertRows
  }

  /// The kind of sheet.
  #[ derive( Debug, Serialize, Deserialize) ]
  pub enum SheetType
  {
    /// The sheet is a grid. 
    #[ serde( rename = "GRID" ) ]
    Grid,

    /// The sheet has no grid and instead has an object like a chart or image. 
    #[ serde( rename = "OBJECT" ) ]
    Object,

    /// The sheet connects with an external DataSource and shows the preview of data.
    #[ serde( rename = "DATA_SOURCE" ) ]
    DataSource
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

  /// Theme color types.  
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub enum ThemeColorType
  {
    /// Represents the primary text color 
    #[ serde( rename = "TEXT" ) ]
    Text,

    /// Represents the primary background color 
    #[ serde( rename = "BACKGROUND" ) ]
    Background,

    /// Represents the first accent color 
    #[ serde( rename = "ACCENT1" ) ]
    Accent1,

    /// Represents the second accent color 
    #[ serde( rename = "ACCENT2" ) ]
    Accent2,

    #[ serde( rename = "ACCENT3" ) ]
    /// Represents the third accent color 
    Accent3,

    #[ serde( rename = "ACCENT4" ) ]
    /// Represents the fourth accent color 
    Accent4,

    #[ serde( rename = "ACCENT5" ) ]
    /// Represents the fifth accent color
    Accent5,

    #[ serde( rename = "ACCENT6" ) ]
    /// Represents the sixth accent color
    Accent6,

    /// Represents the color to use for hyperlinks
    #[ serde( rename = "LINK" ) ]
    Link
  }

  /// A color value.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub enum ColorStyle
  {
    #[ serde( rename = "rgbColor" ) ]
    RgbColor( Color ),

    #[ serde( rename = "themeColor" ) ]
    ThemeColor( ThemeColorType )
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

  /// An enumeration of data execution states. 
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub enum DataExecutionState
  {
    /// The data execution has not started. 
    #[ serde( rename = "NOT_STARTED" ) ]
    NotStarted,
    
    /// The data execution has started and is running.
    #[ serde( rename = "RUNNING" ) ]
    Running,

    /// The data execution is currently being cancelled.
    #[ serde( rename = "CANCELLING" ) ]
    Cancelling,

    /// The data execution has completed successfully. 
    #[ serde( rename = "SUCCEEDED" ) ]
    Succeeded,

    /// The data execution has completed with errors.
    #[ serde( rename = "FAILED" ) ]
    Failed
  }

  /// An enumeration of data execution error code.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub enum DataExecutionErrorCode
  {
    /// The data execution timed out. 
    #[ serde( rename = "TIMED_OUT" ) ]
    TimedOut,

    /// The data execution returns more rows than the limit.
    #[ serde( rename = "TOO_MANY_ROWS" ) ]
    TooManyRows,

    /// The data execution returns more columns than the limit.
    #[ serde( rename = "TOO_MANY_COLUMNS" ) ]
    TooManyColumns,

    /// The data execution returns more cells than the limit.
    #[ serde( rename = "TOO_MANY_CELLS" ) ]
    TooManyCells,

    /// Error is received from the backend data execution engine (e.g. BigQuery)
    #[ serde( rename = "ENGINE" ) ]
    Engine,

    /// One or some of the provided data source parameters are invalid. 
    #[ serde( rename = "PARAMETER_INVALID" ) ]
    ParameterInvalid,

    /// The data execution returns an unsupported data type. 
    #[ serde( rename = "UNSUPPORTED_DATA_TYPE" ) ]
    UnsupportedDataType,

    /// The data execution returns duplicate column names or aliases.
    #[ serde( rename = "DUPLICATE_COLUMN_NAMES" ) ]
    DuplicateColumnNames,

    /// The data execution is interrupted. Please refresh later.
    #[ serde( rename = "INTERRUPTED" ) ]
    Interrupted,

    /// The data execution is currently in progress, can not be refreshed until it completes. 
    #[ serde( rename = "CONCURRENT_QUERY" ) ]
    ConcurrentQuery,

    /// Other errors. 
    #[ serde( rename = "OTHER" ) ]
    Other,

    /// The data execution returns values that exceed the maximum characters allowed in a single cell.
    #[ serde( rename = "TOO_MANY_CHARS_PER_CELL" ) ]
    TooManyCharsPerCell,

    /// The database referenced by the data source is not found.
    #[ serde( rename = "DATA_NOT_FOUND" ) ]
    DataNotFound,

    /// The user does not have access to the database referenced by the data source. 
    #[ serde( rename = "PERMISSION_DENIED" ) ]
    PermissionDenied,

    /// The data execution returns columns with missing aliases. 
    #[ serde( rename = "MISSING_COLUMN_ALIAS" ) ]
    MissingColumnAlias,

    /// The data source object does not exist. 
    #[ serde( rename = "OBJECT_NOT_FOUND" ) ]
    ObjectNotFound,

    /// The data source object is currently in error state.
    #[ serde( rename = "OBJECT_IN_ERROR_STATE" ) ]
    ObjectInErrorState,

    /// The data source object specification is invalid. 
    #[ serde( rename = "OBJECT_SPEC_INVALID" ) ]
    ObjectSprecInvalid,

    /// The data execution has been cancelled. 
    #[ serde( rename = "DATA_EXECUTION_CANCELLED" ) ]
    DataExecutionCancelled
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

  #[ derive( Debug ) ]
  pub struct Range
  {
    start_index : usize,
    end_index   : usize
  }

  /// A range along a single dimension on a sheet. All indexes are zero-based. Indexes are half open: the start index is inclusive and the end index is exclusive. Missing indexes indicate the range is unbounded on that side.
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DimensionRange
  {
    /// The sheet this span is on.
    #[ serde( rename = "sheetId" ) ]
    pub sheet_id : String,
    /// The dimension of the span.
    pub dimension : Dimension,
    #[ serde( rename = "startIndex" ) ]
    /// The start (inclusive) of the span, or not set if unbounded.
    pub start_index : Option< usize >,
    #[ serde( rename = "endIndex" ) ]
    /// The end (exclusive) of the span, or not set if unbounded.
    pub end_index : Option< usize >
  }

  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DeleteDimensionRequest
  {
    pub range : DimensionRange
  }

  /// A group over an interval of rows or columns on a sheet, which can contain or be contained within other groups. A group can be collapsed or expanded as a unit on the sheet. 
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct DimensionGroup
  {
    range : Option< DimensionRange >,
    depth : Option< usize >,
    collapsed : Option< bool >
  }

    /// [The result of deleting a group.](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/response#DeleteDimensionGroupResponse)
    #[ derive( Debug, Serialize, Deserialize ) ]
    pub struct DeleteDimensionGroupResponse
    {
      #[ serde( rename = "dimensionGroups" ) ]
      dimension_groups : Option< Vec< DimensionGroup > >
    }
  
    #[ derive( Debug, Serialize, Deserialize ) ]
    pub enum Response
    {
      DeleteDimensionGroupResponse,
    }
}

crate::mod_interface!
{
  orphan use
  {
    Dimension,
    ValueRange,
    DeleteDimensionRequest,
    Response
  };

  own use
  {
    DateTimeRenderOption,
    InsertDataOption,
    ValueRenderOption,
    ValueInputOption,
    SheetProperties,
    DataSourceSheetProperties,
    ColorStyle,
    Color,
    GridProperties,
    SheetType,
    DimensionRange,
    DeleteDimensionRequest
  };
}