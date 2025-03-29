

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
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
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
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
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
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
  pub enum ColorStyle
  {
    #[ serde( rename = "rgbColor" ) ]
    RgbColor( Color ),

    #[ serde( rename = "themeColor" ) ]
    ThemeColor( ThemeColorType )
  }

  /// An unique identifier that references a data source column.
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
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
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
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
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
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
  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
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

  #[ derive( Debug, Serialize, Deserialize, Clone ) ]
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

    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum Request
    {
      #[ serde( rename = "deleteDimension" ) ]
      DeleteDimension( DeleteDimensionRequest )
    }

    /// An enumeration of the possible recalculation interval options.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum RecalculationInterval
    {
      /// Volatile functions are updated on every change.
      #[ serde( rename = "ON_CHANGE" ) ]
      OnChange,
      /// Volatile functions are updated on every change and every minute.
      #[ serde( rename = "MINUTE" ) ] 
      Minute,
      /// Volatile functions are updated on every change and hourly.
      #[ serde( rename = "HOUR" ) ]
      Hour
    }

    /// The number format of the cell. In this documentation the locale is assumed to be en_US, but the actual format depends on the locale of the spreadsheet. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum NumberFormatType
    {
      /// Text formatting, e.g 1000.12
      #[ serde( rename = "TEXT" ) ]
      Text,
      /// Number formatting, e.g, 1,000.12
      #[ serde( rename = "NUMBER" ) ]
      Number,
      /// Percent formatting, e.g 10.12%
      #[ serde( rename = "PERCENT" ) ]
      Percent,
      /// Currency formatting, e.g $1,000.12
      #[ serde( rename = "CURRENCY" ) ]
      Currency,
      /// Date formatting, e.g 9/26/2008
      #[ serde( rename = "DATE" ) ]
      Date,
      /// Time formatting, e.g 3:59:00 PM
      #[ serde( rename = "TIME" ) ]
      Time,
      /// Date+Time formatting, e.g 9/26/08 15:59:00
      #[ serde( rename = "DATE_TIME" ) ]
      DateTime,
      /// Scientific number formatting, e.g 1.01E+03
      #[ serde( rename = "SCIENTIFIC" ) ]
      Scientific
    }

    /// The number format of a cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct NumberFormat
    {
      /// The type of the number format. When writing, this field must be set.
      #[ serde( rename = "type" ) ]
      t : Option< NumberFormatType >,
      /// Pattern string used for formatting. If not set, a default pattern based on the user's locale will be used if necessary for the given type. See the Date and [Number Formats guide](https://developers.google.com/sheets/api/guides/formats) for more information about the supported patterns.
      pattern : Option< String >
    }

    /// The style of a border. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum Style
    {
      /// The border is dotted.
      #[ serde( rename = "DOTTED" ) ]
      Dotted,
      /// The border is dashed. 
      #[ serde( rename = "DASHED" ) ]
      Dashed,
      /// The border is a thin solid line.
      #[ serde( rename = "SOLID" ) ]
      Solid,
      /// The border is a medium solid line.
      #[ serde( rename = "SOLID_MEDIUM" ) ]
      SolidMedium,
      /// The border is a thick solid line. 
      #[ serde( rename = "SOLID_THICK" ) ]
      SolidThick,
      /// No border. Used only when updating a border in order to erase it.
      #[ serde( rename = "NONE" ) ]
      None,
      /// The border is two solid lines.
      #[ serde( rename = "DOUBLE" ) ]
      Double
    }

    /// A border along a cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct Border
    {
      /// The style of the border. 
      style : Option< Style >,
      /// This item is deprecated! 
      /// The width of the border, in pixels. Deprecated; the width is determined by the "style" field. 
      width : Option< i32 >,
      /// This item is deprecated! 
      /// The color of the border. Deprecated: Use color_style. 
      color : Option< Color >,
      /// The color of the border. If color is also set, this field takes precedence. 
      #[ serde( rename = "colorStyle" ) ]
      color_style : Option< ColorStyle >
    }

    /// The borders of the cell.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct Borders
    {
      /// The top border of the cell.
      top : Option< Border >,
      /// The bottom border of the cell. 
      bottom : Option< Border >,
      /// The left border of the cell.
      left : Option< Border >,
      /// The right border of the cell. 
      right : Option< Border >,
    }

    /// The amount of padding around the cell, in pixels. When updating padding, every field must be specified.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct Padding
    {
      /// The top padding of the cell. 
      top : Option< i32 >,
      /// The right padding of the cell.
      bottom : Option< i32 >,
      /// The bottom padding of the cell.
      left : Option< i32 >,
      /// The left padding of the cell.
      right : Option< i32 >,
    }

    /// The horizontal alignment of text in a cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum HorizontalAlign
    {
      /// The text is explicitly aligned to the left of the cell.
      #[ serde( rename = "LEFT" ) ]
      Left,
      /// The text is explicitly aligned to the center of the cell. 
      #[ serde( rename = "CENTER" ) ]
      Center,
      /// The text is explicitly aligned to the right of the cell.
      #[ serde( rename = "RIGHT" ) ]
      Right
    }

    /// The vertical alignment of text in a cell.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum VerticalAlign
    {
      /// The text is explicitly aligned to the top of the cell.
      #[ serde( rename = "TOP" ) ]
      Top,
      /// The text is explicitly aligned to the middle of the cell. 
      #[ serde( rename = "MIDDLE" ) ]
      Middle,
      /// The text is explicitly aligned to the bottom of the cell.
      #[ serde( rename = "BOTTOM" ) ]
      Bottom
    }

    /// How to wrap text in a cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum WrapStrategy
    {
      /// Lines that are longer than the cell width will be written in the next cell over, so long as that cell is empty. If the next cell over is non-empty, this behaves the same as CLIP. The text will never wrap to the next line unless the user manually inserts a new line.
      /// Example:
      /// 
      /// ```
      /// | First sentence. |
      /// | Manual newline that is very long. <- Text continues into next cell
      /// | Next newline.   |
      /// ```
      #[ serde( rename = "OVERFLOW_CELL" ) ]
      OverflowCell,
      /// This wrap strategy represents the old Google Sheets wrap strategy where words that are longer than a line are clipped rather than broken. This strategy is not supported on all platforms and is being phased out. 
      /// Example:
      /// ```
      /// | Cell has a |
      /// | loooooooooo| <- Word is clipped.
      /// | word.      |
      /// ``` 
      #[ serde( rename = "LEGACY_WRAP" ) ]
      LegacyWrap,
      /// Lines that are longer than the cell width will be clipped. The text will never wrap to the next line unless the user manually inserts a new line. 
      /// Example: 
      /// 
      /// ```
      /// | First sentence. |
      /// | Manual newline t| <- Text is clipped
      /// | Next newline.   |
      /// ```
      #[ serde( rename = "CLIP" ) ]
      Clip,
      /// Words that are longer than a line are wrapped at the character level rather than clipped. 
      /// Example:
      /// 
      /// ```
      /// | Cell has a |
      /// | loooooooooo| <- Word is broken.
      /// | ong word.  |
      /// ``` 
      #[ serde( rename = "WRAP" ) ]
      Wrap,
    }

    /// The direction of text in a cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum TextDirection
    {
      /// The text direction of left-to-right was set by the user. 
      #[ serde( rename = "LEFT_TO_RIGHT" ) ]
      LeftToRight,
      /// The text direction of right-to-left was set by the user. 
      #[ serde( rename = "RIGHT_TO_LEFT" ) ]
      RigthToLeft
    }

    /// An external or local reference. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct Link
    {
      /// The link identifier.
      uri : Option< String >
    }

    /// The format of a run of text in a cell. Absent values indicate that the field isn't specified. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct TextFormat
    {
      /// This item is deprecated! 
      /// The foreground color of the text. Deprecated: Use foreground_color_style. 
      #[ serde( rename = "foregroundColor" ) ]
      foreground_color : Option< Color >,
      /// The foreground color of the text. If foreground_color is also set, this field takes precedence. 
      #[ serde( rename = "foregroundColorStyle" ) ]
      foreground_color_style : Option< ColorStyle >,
      /// The font family.
      #[ serde( rename = "fontFamily" ) ]
      font_family : Option< String >,
      /// The size of the font.
      #[ serde( rename = "fontSize" ) ]
      font_szie : Option< i32 >,
      /// True if the text is bold. 
      bold : Option< bool >,
      /// True if the text is italicized. 
      italic : Option< bool >,
      /// True if the text has a strikethrough. 
      strikethrough : Option< bool >,
      /// True if the text is underlined.
      underline : Option< bool >,
      /// The link destination of the text, if any.
      link : Option< Link >
    }

    /// Whether to explicitly render a hyperlink. If not specified, the hyperlink is linked. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum HyperlinkDisplayType
    {
      /// A hyperlink should be explicitly rendered.
      #[ serde( rename = "LINKED" ) ]
      Linked,
      /// A hyperlink should not be rendered.
      #[ serde( rename = "PLAIN_TEXT" ) ]
      PlainText
    }

    /// The rotation applied to text in a cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum TextRotation
    {
      /// The angle between the standard orientation and the desired orientation. Measured in degrees. Valid values are between -90 and 90. Positive angles are angled upwards, negative are angled downwards.
      #[ serde( rename = "angle" ) ]
      Angle { angle : i32 },
      /// If true, text reads top to bottom, but the orientation of individual characters is unchanged
      /// For example:
      /// 
      /// ```
      /// | V |
      /// | e |
      /// | r |
      /// | t |
      /// | i |
      /// | c |
      /// | a |
      /// | l |
      /// ```
      #[ serde( rename = "vertical" ) ]
      Vertical { vertical : bool }
    }

    /// The format of a cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct CellFormat
    {
      ///A format describing how number values should be represented to the user. 
      #[ serde( rename = "numberFormat" ) ]
      number_format : Option< NumberFormat >,
      /// This item is deprecated! 
      /// The background color of the cell. Deprecated: Use background_color_style. 
      #[ serde( rename = "backgroundColor" ) ]
      background_color : Option< Color >,
      /// The background color of the cell. If background_color is also set, this field takes precedence. 
      #[ serde( rename = "backgroundColorStyle" ) ]
      background_color_style : Option< ColorStyle >,
      /// The borders of the cell.
      borders : Option< Borders >,
      /// The padding of the cell. 
      padding : Option< Padding >,
      /// The horizontal alignment of the value in the cell. 
      #[ serde( rename = "horizontalAlignment" ) ]
      horizontal_alignment : Option< HorizontalAlign >,
      /// The vertical alignment of the value in the cell. 
      #[ serde( rename = "verticalAlignment" ) ]
      vertical_alignment : Option< VerticalAlign >,
      /// The wrap strategy for the value in the cell. 
      #[ serde( rename = "wrapStrategy" ) ]
      wrap_strategy : Option< WrapStrategy >,
      /// The direction of the text in the cell.
      #[ serde( rename = "textDirection" ) ]
      text_direction : Option< TextDirection >,
      /// The format of the text in the cell (unless overridden by a format run)
      #[ serde( rename = "textFormat" ) ]
      text_format : Option< TextFormat >,
      /// If one exists, how a hyperlink should be displayed in the cell. 
      #[ serde( rename = "hyperlinkDisplayType" ) ]
      hyperlink_display_type : Option< HyperlinkDisplayType >,
      /// The rotation applied to text in the cell. 
      #[ serde( rename = "textRotation" ) ]
      text_rotation : Option< TextRotation >
    }

    /// Settings to control how circular dependencies are resolved with iterative calculation.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct IterativeCalculationSettings
    {
      /// When iterative calculation is enabled, the maximum number of calculation rounds to perform.
      #[ serde( rename = "maxIterations" ) ]
      max_iterations : Option< i32 >,
      /// When iterative calculation is enabled and successive results differ by less than this threshold value, the calculation rounds stop.
      #[ serde( rename = "convergenceThreshold" ) ]
      convergence_threshold : Option< f32 >  
    }

    /// Represents spreadsheet theme
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct SpreadsheetTheme
    {
      /// Name of the primary font family.
      #[ serde( rename = "primaryFontFamily" ) ]
      primary_font_family : Option< String >,
      /// The spreadsheet theme color pairs. To update you must provide all theme color pairs.
      #[ serde( rename = "themeColors" ) ]
      theme_colors : Option< ThemeColorType >
    }

    /// Properties of a spreadsheet.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct SpreadsheetProperties
    {
      /// The title of the spreadsheet.
      title : Option< String >,
      /// The locale of the spreadsheet in one of the following formats:
      ///  - an ISO 639-1 language code such as **en**
      ///  - an ISO 639-2 language code such as **fil**, if no 639-1 code exists
      ///  - a combination of the ISO language code and country code, such as **en_US**
      /// 
      /// Note: when updating this field, not all locales/languages are supported.
      locale : Option< String >,
      /// The amount of time to wait before volatile functions are recalculated.
      #[ serde( rename = "autoRecalc" ) ]
      auto_recalc : Option< RecalculationInterval >,
      /// The time zone of the spreadsheet, in CLDR format such as **America/New_York**. If the time zone isn't recognized, this may be a custom time zone such as **GMT-07:00**.
      #[ serde( rename = "timeZone" ) ]
      time_zone : Option< String >,
      /// The default format of all cells in the spreadsheet.
      #[ serde( rename = "defaultFormat" ) ]
      default_format : Option< CellFormat >,
      /// Determines whether and how circular references are resolved with iterative calculation. Absence of this field means that circular references result in calculation errors.
      #[ serde( rename = "iterativeCalculationSettings" ) ]
      iterative_calculation_settings : Option< IterativeCalculationSettings >,
      /// Theme applied to the spreadsheet.
      #[ serde( rename = "spreadsheetTheme" ) ]
      spreadsheet_theme : Option< SpreadsheetTheme >,
      /// Whether to allow external URL access for image and import functions. Read only when true. When false, you can set to true. This value will be bypassed and always return true if the admin has enabled the allowlisting feature.
      #[ serde( rename = "importFunctionsExternalUrlAccessAllowed" ) ]
      import_functions_external_url_access_allowed : Option< bool >
    }

    /// The type of error. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum ErrorType
    {
      /// Corresponds to the **#ERROR!** error
      #[ serde( rename = "ERROR" ) ]
      Error,
      /// Corresponds to the **#NULL!** error. 
      #[ serde( rename = "NULL_VALUE" ) ]
      NullValue,
      /// Corresponds to the **#DIV/0** error.
      #[ serde( rename = "DIVIDE_BY_ZERO" ) ]
      DivideByZero,
      /// Corresponds to the **#VALUE!** error.
      #[ serde( rename = "VALUE" ) ]
      Value,
      /// Corresponds to the **#REF!** error.
      #[ serde( rename = "REF" ) ]
      Ref,
      /// Corresponds to the **#NAME?** error. 
      #[ serde( rename = "NAME" ) ]
      Name,
      /// Corresponds to the **#NUM!** error.
      #[ serde( rename = "NUM" ) ]
      Num,
      /// Corresponds to the **#N/A** error.
      #[ serde( rename = "NA" ) ]
      Na,
      /// Corresponds to the **Loading...** state. 
      #[ serde( rename = "LOADING" ) ]
      Loading
    }

    /// An error in a cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct ErrorValue
    {
      /// The type of error.
      #[ serde( rename = "type" ) ]
      t : Option< ErrorType >,
      /// A message with more information about the error (in the spreadsheet's locale).
      message : Option< String >
    }

    /// The kinds of value that a cell in a spreadsheet can have. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum ExtendedValue
    {
      /// Represents a double value. Note: Dates, Times and DateTimes are represented as doubles in SERIAL_NUMBER format. 
      #[ serde( rename = "numberValue" ) ]
      NumberValue( f64 ),
      /// Represents a string value. Leading single quotes are not included. For example, if the user typed **'123** into the UI, this would be represented as a **stringValue** of **"123"**.
      #[ serde( rename = "stringValue" ) ]
      StringValue( String ),
      /// Represents a boolean value. 
      #[ serde( rename = "boolValue" ) ]
      BoolValue( bool ),
      /// Represents a formula. 
      #[ serde( rename = "formulaValue" ) ]
      FormulaValue( String ),
      /// Represents an error. This field is read-only. 
      #[ serde( rename = "errorValue" ) ]
      ErrorValue( ErrorValue ),
    }
    
    /// A run of a text format. The format of this run continues until the start index of the next run. When updating, all fields must be set. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct TextFormatRun
    {
      /// The zero-based character index where this run starts, in UTF-16 code units.
      #[ serde( rename = "startIndex" ) ]
      start_index : Option< i32 >,
      /// The format of this run. Absent values inherit the cell's format. 
      format : Option< TextFormat >
    }

    /// Controls how a date condition is evaluated. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum RealtiveDate
    {
      /// The value is one year before today. 
      #[ serde( rename = "PAST_YEAR" ) ]
      PastYear,
      /// The value is one month before today. 
      #[ serde( rename = "PAST_MONTH" ) ]
      PastMonth,
      /// The value is one week before today. 
      #[ serde( rename = "PAST_WEEK" ) ]
      PastWeek,
      /// The value is yesterday. 
      #[ serde( rename = "YESTERDAY" ) ]
      Yesterday,
      /// The value is today. 
      #[ serde( rename = "TODAY" ) ]
      Today,
      /// The value is tomorrow.
      #[ serde( rename = "TOMORROW" ) ]
      Tomorrow
    }

    /// The value of the condition. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum ConditionValue
    {
      /// A relative date (based on the current date). Valid only if the type is DATE_BEFORE, DATE_AFTER, DATE_ON_OR_BEFORE or DATE_ON_OR_AFTER. 
      /// 
      /// Relative dates are not supported in data validation. They are supported only in conditional formatting and conditional filters. 
      #[ serde( rename = "relativeDate" ) ]
      RelativeDate( RealtiveDate ),
      /// A value the condition is based on. The value is parsed as if the user typed into a cell. Formulas are supported (and must begin with an = or a '+'). 
      #[ serde( rename = "userEnteredValue" ) ]
      UserEnteredValue( String )
    }

    /// The type of condition.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum ConditionType {
      /// The cell's value must be greater than the condition's value. Supported by data validation, conditional formatting and filters.
      NumberGreater( ConditionValue ),
      /// The cell's value must be greater than or equal to the condition's value. Supported by data validation, conditional formatting and filters
      NumberGreaterThanEq( ConditionValue ),
      /// The cell's value must be less than the condition's value. Supported by data validation, conditional formatting and filters.
      NumberLess( ConditionValue ),
      /// The cell's value must be less than or equal to the condition's value. Supported by data validation, conditional formatting and filters
      NumberLessThanEq( ConditionValue ),
      /// The cell's value must be equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects. 
      NumberEq( Vec< ConditionValue > ),
      /// The cell's value must be not equal to the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects. 
      NumberNotEq( Vec< ConditionValue > ),
      /// The cell's value must be between the two condition values. Supported by data validation, conditional formatting and filters. Requires exactly two ConditionValues. 
      NumberBetween( Vec< ConditionValue > ),
      /// The cell's value must not be between the two condition values. Supported by data validation, conditional formatting and filters. Requires exactly two ConditionValues. 
      NumberNotBetween( Vec< ConditionValue > ),
      /// The cell's value must contain the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue. 
      TextContains( ConditionValue ),
      /// The cell's value must not contain the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue.
      TextNotContains( ConditionValue ),
      /// The cell's value must start with the condition's value. Supported by conditional formatting and filters. Requires a single ConditionValue. 
      TextStartsWith( ConditionValue ),
      /// The cell's value must end with the condition's value. Supported by conditional formatting and filters. Requires a single ConditionValue. 
      TextEndsWith( ConditionValue ),
      /// The cell's value must be exactly the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects. 
      TextEq( Vec< ConditionValue > ),
      /// The cell's value must be a valid email address. Supported by data validation. Requires no ConditionValues. 
      TextIsEmail,
      /// The cell's value must be a valid URL. Supported by data validation. Requires no ConditionValues. 
      TextIsUrl,
      /// The cell's value must be the same date as the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue for data validation, conditional formatting, and filters on non-data source objects and at least one ConditionValue for filters on data source objects. 
      DateEq( Vec< ConditionValue > ),
      /// The cell's value must be before the date of the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue that may be a relative date. 
      DateBefore( ConditionValue ),
      /// The cell's value must be after the date of the condition's value. Supported by data validation, conditional formatting and filters. Requires a single ConditionValue that may be a relative date. 
      DateAfter( ConditionValue ),
      /// The cell's value must be on or before the date of the condition's value. Supported by data validation. Requires a single ConditionValue that may be a relative date.
      DateOnOrBefore( ConditionValue ),
      ///  The cell's value must be on or after the date of the condition's value. Supported by data validation. Requires a single ConditionValue that may be a relative date. 
      DateOnOrAfter( ConditionValue ),
      /// The cell's value must be between the dates of the two condition values. Supported by data validation. Requires exactly two ConditionValues. 
      DateBetween( Vec< ConditionValue > ),
      /// The cell's value must be outside the dates of the two condition values. Supported by data validation. Requires exactly two ConditionValues. 
      DateNotBetween( Vec< ConditionValue > ),
      /// The cell's value must be a date. Supported by data validation. Requires no ConditionValues. 
      DateIsValid,
      /// The cell's value must be listed in the grid in condition value's range. Supported by data validation. Requires a single ConditionValue, and the value must be a valid range in A1 notation. 
      OneOfRange( ConditionValue ),
      ///  	The cell's value must be in the list of condition values. Supported by data validation. Supports any number of condition values, one per item in the list. Formulas are not supported in the values. 
      OneOfList( Vec< ConditionValue > ), 
      /// The cell's value must be empty. Supported by conditional formatting and filters. Requires no ConditionValues. 
      Blank,
      /// The cell's value must not be empty. Supported by conditional formatting and filters. Requires no ConditionValues. 
      NotBlank,
      /// The condition's formula must evaluate to true. Supported by data validation, conditional formatting and filters. Not supported by data source sheet filters. Requires a single ConditionValue. 
      CustomFormula( ConditionValue ),
      /// The cell's value must be TRUE/FALSE or in the list of condition values. Supported by data validation. Renders as a cell checkbox. Supports zero, one or two ConditionValues. No values indicates the cell must be TRUE or FALSE, where TRUE renders as checked and FALSE renders as unchecked. One value indicates the cell will render as checked when it contains that value and unchecked when it is blank. Two values indicate that the cell will render as checked when it contains the first value and unchecked when it contains the second value. For example, ["Yes","No"] indicates that the cell will render a checked box when it has the value "Yes" and an unchecked box when it has the value "No".
      Boolean( Vec< ConditionValue > ),
      /// The cell's value must be exactly not the condition's value. Supported by filters on data source objects. Requires at least one ConditionValue. 
      TextNotEq( Vec< ConditionValue > ),
      /// The cell's value must be exactly not the condition's value. Supported by filters on data source objects. Requires at least one ConditionValue. 
      DateNotEq( Vec< ConditionValue > ),
      /// The cell's value must follow the pattern specified. Requires a single ConditionValue. 
      FilterExpression( ConditionValue ),
    }

    /// A condition that can evaluate to true or false. BooleanConditions are used by conditional formatting, data validation, and the criteria in filters.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct BooleanCondition
    {
      /// The type of condition. 
      #[ serde( rename = "type" ) ]
      t : Option< ConditionType >,
      /// The values of the condition. The number of supported values depends on the condition type. Some support zero values, others one or two values, and ConditionType.ONE_OF_LIST supports an arbitrary number of values. 
      values : Option< Vec< ConditionValue > >
    }

    /// A data validation rule.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct DataValidationRule
    {
      /// The condition that data in the cell must match. 
      condition : Option< BooleanCondition >,
      /// A message to show the user when adding data to the cell.
      #[ serde( rename = "inputMessage" ) ]
      input_message : Option< String >,
      /// True if invalid data should be rejected.
      strict : Option< bool >,
      /// True if the UI should be customized based on the kind of condition. If true, "List" conditions will show a dropdown.
      #[ serde( rename = "showCustomUi" ) ]
      show_custom_ui : Option< bool >
    }

    /// Metadata about a value in a pivot grouping. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct PivotGroupValueMetadata
    {
      /// The calculated value the metadata corresponds to. (Note that formulaValue is not valid, because the values will be calculated.) 
      value : Option< ExtendedValue >,
      /// True if the data corresponding to the value is collapsed. 
      collapsed : Option< bool >
    }

    /// A sort order. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    #[ serde( untagged ) ]
    pub enum SortOrder
    {
      /// Sort ascending. 
      #[ serde( rename = "ASCENDING" ) ]
      Ascending,
      /// Sort descending.
      #[ serde( rename = "DESCENDING" ) ]
      Descending
    }

    /// Information about which values in a pivot group should be used for sorting. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct PivotGroupSortValueBucket
    {
      /// The offset in the PivotTable.values list which the values in this grouping should be sorted by. 
      #[ serde( rename = "valuesIndex" ) ]
      values_index : Option< i32 >,
      /// Determines the bucket from which values are chosen to sort. 
      /// 
      /// For example, in a pivot table with one row group & two column groups, the row group can list up to two values. The first value corresponds to a value within the first column group, and the second value corresponds to a value in the second column group. If no values are listed, this would indicate that the row should be sorted according to the "Grand Total" over the column groups. If a single value is listed, this would correspond to using the "Total" of that bucket. 
      buckets : Option< ExtendedValue >
    }

    /// A group name and a list of items from the source data that should be placed in the group with this name. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct ManualRuleGroup
    {
      /// The group name, which must be a string. Each group in a given ManualRule must have a unique group name. 
      #[ serde( rename = "groupName" ) ]
      group_name : Option< ExtendedValue >,
      /// The items in the source data that should be placed into this group. Each item may be a string, number, or boolean. Items may appear in at most one group within a given ManualRule. Items that do not appear in any group will appear on their own. 
      items : Option< Vec< ExtendedValue > >
    }

    /// Allows you to manually organize the values in a source data column into buckets with names of your choosing. For example, a pivot table that aggregates population by state:
    /// 
    /// +-------+-------------------+
    /// | State | SUM of Population |
    /// +-------+-------------------+
    /// | AK    |               0.7 |
    /// | AL    |               4.8 |
    /// | AR    |               2.9 |
    /// ...
    /// +-------+-------------------+
    /// 
    /// could be turned into a pivot table that aggregates population by time zone by providing a list of groups (for example, groupName = 'Central', items = ['AL', 'AR', 'IA', ...]) to a manual group rule. Note that a similar effect could be achieved by adding a time zone column to the source data and adjusting the pivot table. 
    /// 
    /// +-----------+-------------------+
    /// | Time Zone | SUM of Population |
    /// +-----------+-------------------+
    /// | Central   |             106.3 |
    /// | Eastern   |             151.9 |
    /// | Mountain  |              17.4 |
    /// ...
    /// +-----------+-------------------+
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct ManualRule
    {
      /// The list of group names and the corresponding items from the source data that map to each group name. 
      groups : Option< Vec< ManualRuleGroup > >
    }

    /// Allows you to organize the numeric values in a source data column into buckets of a constant size. All values from HistogramRule.start to HistogramRule.end are placed into groups of size HistogramRule.interval. In addition, all values below HistogramRule.start are placed in one group, and all values above HistogramRule.end are placed in another. Only HistogramRule.interval is required, though if HistogramRule.start and HistogramRule.end are both provided, HistogramRule.start must be less than HistogramRule.end. For example, a pivot table showing average purchase amount by age that has 50+ rows: 
    /// 
    /// +-----+-------------------+
    /// | Age | AVERAGE of Amount |
    /// +-----+-------------------+
    /// | 16  |            $27.13 |
    /// | 17  |             $5.24 |
    /// | 18  |            $20.15 |
    /// ...
    /// +-----+-------------------+
    /// 
    /// could be turned into a pivot table that looks like the one below by applying a histogram group rule with a HistogramRule.start of 25, an HistogramRule.interval of 20, and an HistogramRule.end of 65. 
    /// 
    /// +-------------+-------------------+
    /// | Grouped Age | AVERAGE of Amount |
    /// +-------------+-------------------+
    /// | < 25        |            $19.34 |
    /// | 25-45       |            $31.43 |
    /// | 45-65       |            $35.87 |
    /// | > 65        |            $27.55 |
    /// +-------------+-------------------+
    /// | Grand Total |            $29.12 |
    /// +-------------+-------------------+
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct HistogramRule
    {
      /// The size of the buckets that are created. Must be positive. 
      interval : Option< f64 >,
      /// The minimum value at which items are placed into buckets of constant size. Values below start are lumped into a single bucket. This field is optional. 
      start : Option< f64 >,
      /// The maximum value at which items are placed into buckets of constant size. Values above end are lumped into a single bucket. This field is optional. 
      end : Option< f64 >
    }

    /// The available types of date-time grouping rules. This documentation assumes the spreadsheet locale is "en-US", though the actual rendering of the dates and times uses the locale of the spreadsheet for some rule types. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum DateTimeRuleType 
    {
      /// Group dates by second, from 0 to 59.
      #[ serde( rename = "SECOND" ) ]
      Second,
      /// Group dates by minute, from 0 to 59. 
      #[ serde( rename = "MINUTE" ) ]
      Minute,
      /// Group dates by hour using a 24-hour system, from 0 to 23. 
      #[ serde( rename = "HOUR" ) ]
      Hour,
      /// Group dates by hour and minute using a 24-hour system, for example 19:45.
      #[ serde( rename = "HOUR_MINUTE" ) ]
      HourMinute,
      /// Group dates by hour and minute using a 12-hour system, for example 7:45 PM. The AM/PM designation is translated based on the spreadsheet locale.
      #[ serde( rename = "HOUR_MINUTE_AMPM" ) ]
      HourMinuteAmpm,
      /// Group dates by day of week, for example Sunday. The days of the week will be translated based on the spreadsheet locale. 
      #[ serde( rename = "DAY_OF_WEEK" ) ]
      DayOfWeek,
      /// Group dates by day of year, from 1 to 366. Note that dates after Feb. 29 fall in different buckets in leap years than in non-leap years. 
      #[ serde( rename = "DAY_OF_YEAR" ) ]
      DayOfYear,
      /// Group dates by day of month, from 1 to 31. 
      #[ serde( rename = "DAY_OF_MONTH" ) ]
      DayOfMonth,
      /// Group dates by day and month, for example 22-Nov. The month is translated based on the spreadsheet locale. 
      #[ serde( rename = "DAY_MONTH" ) ]
      DayMonth,
      /// Group dates by month, for example Nov. The month is translated based on the spreadsheet locale.
      #[ serde( rename = "MONTH" ) ]
      Month,
      /// Group dates by quarter, for example Q1 (which represents Jan-Mar). 
      #[ serde( rename = "QUARTER" ) ]
      Quarter,
      /// Group dates by year, for example 2008. 
      #[ serde( rename = "YEAR" ) ]
      Year,
      /// Group dates by year and month, for example 2008-Nov. The month is translated based on the spreadsheet locale. 
      #[ serde( rename = "YEAR_MONTH" ) ]
      YearMonth,
      /// Group dates by year and quarter, for example 2008 Q4. 
      #[ serde( rename = "YEAR_QUARTER" ) ]
      YearQuarter,
      /// Group dates by year, month, and day, for example 2008-11-22. 
      #[ serde( rename = "YEAR_MONTH_DAY" ) ]
      YearMonthDay,
    }

    /// Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values. For example, consider a pivot table showing sales transactions by date: 
    /// 
    /// +----------+--------------+
    /// | Date     | SUM of Sales |
    /// +----------+--------------+
    /// | 1/1/2017 |      $621.14 |
    /// | 2/3/2017 |      $708.84 |
    /// | 5/8/2017 |      $326.84 |
    /// ...
    /// +----------+--------------+
    /// 
    /// Applying a date-time group rule with a DateTimeRuleType of YEAR_MONTH results in the following pivot table. 
    /// 
    /// +--------------+--------------+
    /// | Grouped Date | SUM of Sales |
    /// +--------------+--------------+
    /// | 2017-Jan     |   $53,731.78 |
    /// | 2017-Feb     |   $83,475.32 |
    /// | 2017-Mar     |   $94,385.05 |
    /// ...
    /// +--------------+--------------+
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct DateTimeRule
    {
      /// The type of date-time grouping to apply. 
      #[ serde( rename = "type" ) ]
      t : Option< DateTimeRuleType >
    }

    /// An optional setting on a PivotGroup that defines buckets for the values in the source data column rather than breaking out each individual value. Only one PivotGroup with a group rule may be added for each column in the source data, though on any given column you may add both a PivotGroup that has a rule and a PivotGroup that does not. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    #[ serde( untagged ) ]
    pub enum PivotGroupRule
    {
      #[ serde( rename = "manualRule" ) ]
      Manual( ManualRule ),
      #[ serde( rename = "histogramRule" ) ]
      Histogram( HistogramRule ),
      #[ serde( rename = "dateTimeRule" ) ]
      DateTime( DateTimeRule )
    }

    /// The count limit on rows or columns in the pivot group. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct PivotGroupLimit
    {
      /// The count limit. 
      #[ serde( rename = "countLimit" ) ]
      count_limit : Option< i64 >,
      /// The order in which the group limit is applied to the pivot table. 
      /// 
      /// Pivot group limits are applied from lower to higher order number. Order numbers are normalized to consecutive integers from 0. 
      /// 
      /// For write request, to fully customize the applying orders, all pivot group limits should have this field set with an unique number. Otherwise, the order is determined by the index in the PivotTable.rows list and then the PivotTable.columns list. 
      #[ serde( rename = "applyOrder" ) ]
      apply_order : Option< i64 >
    }

    /// Union field source. The data source of the pivot group.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    #[ serde( untagged ) ]
    pub enum PivotGroupSource
    {
      /// The column offset of the source range that this grouping is based on. 
      /// 
      /// For example, if the source was C10:E15, a sourceColumnOffset of 0 means this group refers to column C, whereas the offset 1 would refer to column D. 
      #[ serde( rename = "sourceColumnOffset" ) ]
      SourceColumnOffset( i64 ),
      /// The reference to the data source column this grouping is based on. 
      #[ serde( rename = "dataSourceColumnReference" ) ]
      DataSourceColumnReference( DataSourceColumnReference )
    }

    /// A single grouping (either row or column) in a pivot table. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct PivotGroup
    {
      /// True if the pivot table should include the totals for this grouping. 
      #[ serde( rename = "showTotals" ) ]
      show_totals : bool,
      /// Metadata about values in the grouping. 
      #[ serde( rename = "valueMetadata" ) ]
      value_metadata : Vec< PivotGroupValueMetadata >,
      /// The order the values in this group should be sorted. 
      #[ serde( rename = "sortOrder" ) ]
      sort_order : SortOrder,
      /// The bucket of the opposite pivot group to sort by. If not specified, sorting is alphabetical by this group's values. 
      #[ serde( rename = "valueBucket" ) ]
      value_bucket : Option< PivotGroupSortValueBucket >,
      /// True if the headings in this pivot group should be repeated. This is only valid for row groupings and is ignored by columns. 
      /// 
      /// By default, we minimize repetition of headings by not showing higher level headings where they are the same. For example, even though the third row below corresponds to "Q1 Mar", "Q1" is not shown because it is redundant with previous rows. Setting repeatHeadings to true would cause "Q1" to be repeated for "Feb" and "Mar". 
      /// 
      /// +--------------+
      /// | Q1     | Jan |
      /// |        | Feb |
      /// |        | Mar |
      /// +--------+-----+
      /// | Q1 Total     |
      /// +--------------+
      #[ serde( rename = "repeatHeadings" ) ]
      repeat_headings : bool,
      /// The labels to use for the row/column groups which can be customized. For example, in the following pivot table, the row label is Region (which could be renamed to State) and the column label is Product (which could be renamed Item). Pivot tables created before December 2017 do not have header labels. If you'd like to add header labels to an existing pivot table, please delete the existing pivot table and then create a new pivot table with same parameters. 
      /// 
      /// +--------------+---------+-------+
      /// | SUM of Units | Product |       |
      /// | Region       | Pen     | Paper |
      /// +--------------+---------+-------+
      /// | New York     |     345 |    98 |
      /// | Oregon       |     234 |   123 |
      /// | Tennessee    |     531 |   415 |
      /// +--------------+---------+-------+
      /// | Grand Total  |    1110 |   636 |
      /// +--------------+---------+-------+
      label : Option< String >,
      /// The group rule to apply to this row/column group. 
      #[ serde( rename = "groupRule" ) ]
      group_rule : Option< PivotGroupRule >,
      /// The count limit on rows or columns to apply to this pivot group. 
      #[ serde( rename = "groupLimit" ) ]
      group_limit : Option< PivotGroupLimit >,
      /// Union field source. The data source of the pivot group.
      source : Option< PivotGroupSource >
    }

    /// Criteria for showing/hiding rows in a pivot table. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct PivotFilterCriteria
    {
      /// Values that should be included. Values not listed here are excluded. 
      #[ serde( rename = "visibleValues" ) ]
      visible_values : Option< Vec< String > >,
      /// A condition that must be true for values to be shown. ( visibleValues does not override this -- even if a value is listed there, it is still hidden if it does not meet the condition.) 
      /// 
      /// Condition values that refer to ranges in A1-notation are evaluated relative to the pivot table sheet. References are treated absolutely, so are not filled down the pivot table. For example, a condition value of =A1 on "Pivot Table 1" is treated as 'Pivot Table 1'!$A$1. 
      /// 
      /// The source data of the pivot table can be referenced by column header name. For example, if the source data has columns named "Revenue" and "Cost" and a condition is applied to the "Revenue" column with type NUMBER_GREATER and value =Cost, then only columns where "Revenue" > "Cost" are included. 
      condition : Option< BooleanCondition >,
      /// Whether values are visible by default. If true, the visibleValues are ignored, all values that meet condition (if specified) are shown. If false, values that are both in visibleValues and meet condition are shown. 
      #[ serde( rename = "visibleByDefault" ) ]
      visible_by_default : Option< bool >
    }

    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct Criteria
    {
      integer : Option< PivotFilterCriteria >
    }

    /// The pivot table filter criteria associated with a specific source column offset.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct PivotFilterSpec
    {
      /// The criteria for the column.
      #[ serde( rename = "filterCriteria" ) ]
      filter_criteria : Option< PivotFilterCriteria >,
      /// Union field source. The source column that this filter applies to.
      source : Option< PivotGroupSource >
    }

    /// A function to summarize a pivot value.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum PivotValueSummarizeFunction
    {
      /// Corresponds to the SUM function. 
      #[ serde( rename = "SUM" ) ]
      Sum,
      /// Corresponds to the COUNTA function. 
      #[ serde( rename = "COUNTA" ) ]
      Counta,
      /// Corresponds to the COUNT function. 
      #[ serde( rename = "COUNT" ) ]
      Count,
      /// Corresponds to the COUNTUNIQUE function. 
      #[ serde( rename = "COUNTUNIQUE" ) ]
      Countunique,
      /// Corresponds to the AVERAGE function.
      #[ serde( rename = "AVERAGE" ) ]
      Average,
      /// Corresponds to the MAX function. 
      #[ serde( rename = "MAX" ) ]
      Max,
      /// Corresponds to the MIN function. 
      #[ serde( rename = "MIN" ) ]
      Min,
      /// Corresponds to the MEDIAN function. 
      #[ serde( rename = "MEDIAN" ) ]
      Median,
      /// Corresponds to the PRODUCT function. 
      #[ serde( rename = "PRODUCT" ) ]      
      Product,
      /// Corresponds to the STDEV function. 
      #[ serde( rename = "STDEV" ) ]
      Stdev,
      /// Corresponds to the STDEVP function. 
      #[ serde( rename = "STDEVP" ) ]
      Stdevp,
      /// Corresponds to the VAR function. 
      #[ serde( rename = "VAR" ) ]
      Var,
      /// Corresponds to the VARP function. 
      #[ serde( rename = "VARP" ) ]
      Varp,
      // Indicates the formula should be used as-is. Only valid if PivotValue.formula was set.
      #[ serde( rename = "CUSTOM" ) ]
      Custom,
      /// Indicates that the value is already summarized, and the summarization function is not explicitly specified. Used for Looker data source pivot tables where the value is already summarized. 
      #[ serde( rename = "NONE" ) ]
      None
    }

    /// The possible ways that pivot values may be calculated for display. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum PivotValueCalculatedDisplayType
    {
      /// Shows the pivot values as percentage of the row total values. 
      #[ serde( rename = "PERCENT_OF_ROW_TOTAL" ) ]
      PercentOfRowTotal,
      /// Shows the pivot values as percentage of the column total values. 
      #[ serde( rename = "PERCENT_OF_COLUMN_TOTAL" ) ]
      PercentOfColumnTotal,
      /// Shows the pivot values as percentage of the grand total values. 
      #[ serde( rename = "PERCENT_OF_GRAND_TOTAL" ) ]
      PercentOfGrandTotal
    }

    /// The definition of how a value in a pivot table should be calculated. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    #[ serde( untagged ) ]
    pub enum ValuePivotValue
    {
      /// The column offset of the source range that this value reads from. 
      /// 
      /// For example, if the source was C10:E15, a sourceColumnOffset of 0 means this value refers to column C, whereas the offset 1 would refer to column D. 
      SourceColumnOffset( i64 ),
      /// A custom formula to calculate the value. The formula must start with an = character. 
      Formula( String ),
      /// The reference to the data source column that this value reads from. 
      DataSourceColumnReference( DataSourceColumnReference )
    }

    /// The definition of how a value in a pivot table should be calculated. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct PivotValue
    {
      /// A function to summarize the value. If formula is set, the only supported values are SUM and CUSTOM. If sourceColumnOffset is set, then CUSTOM is not supported. 
      #[ serde( rename = "summarizeFunction" ) ]
      summarize_function : Option< PivotValueSummarizeFunction >,
      /// A name to use for the value. 
      name : Option< String >,
      /// If specified, indicates that pivot values should be displayed as the result of a calculation with another pivot value. For example, if calculatedDisplayType is specified as PERCENT_OF_GRAND_TOTAL, all the pivot values are displayed as the percentage of the grand total. In the Sheets editor, this is referred to as "Show As" in the value section of a pivot table. 
      #[ serde( rename = "calculatedDisplayType" ) ]
      calculated_display_type : Option< PivotValueCalculatedDisplayType >,
      /// Union field value. The data to use for the values in the pivot table. Exactly one value must be set.
      value : Option< ValuePivotValue >
    }

    /// The layout of pivot values. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum PivotValueLayout
    {
      /// Values are laid out horizontally (as columns). 
      #[ serde( rename = "HORIZONTAL" ) ]
      Horizontal,
      /// Values are laid out vertically (as rows). 
      #[ serde( rename = "VERTICAL" ) ]
      Vertical
    }

    /// A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [startIndex, endIndex). Missing indexes indicate the range is unbounded on that side. 
    /// 
    /// For example, if "Sheet1" is sheet ID 123456, then: 
    /// 
    /// `Sheet1!A1:A1 == sheetId: 123456, startRowIndex: 0, endRowIndex: 1, startColumnIndex: 0, endColumnIndex: 1`
    /// 
    /// `Sheet1!A3:B4 == sheetId: 123456, startRowIndex: 2, endRowIndex: 4, startColumnIndex: 0, endColumnIndex: 2`
    /// 
    /// `Sheet1!A:B == sheetId: 123456, startColumnIndex: 0, endColumnIndex: 2`
    /// 
    /// `Sheet1!A5:B == sheetId: 123456, startRowIndex: 4, startColumnIndex: 0, endColumnIndex: 2`
    /// 
    /// `Sheet1 == sheetId: 123456`
    /// 
    /// The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct GridRange
    {
      /// The sheet this range is on. 
      #[ serde( rename = "sheetId" ) ]
      sheet_id : Option< i64 >,
      /// The start row (inclusive) of the range, or not set if unbounded. 
      #[ serde( rename = "startRowIndex" ) ]
      start_row_index : Option< i64 >,
      /// The end row (exclusive) of the range, or not set if unbounded. 
      #[ serde( rename = "endRowIndex" ) ]
      end_row_index : Option< i64 >,
      /// The start column (inclusive) of the range, or not set if unbounded. 
      #[ serde( rename = "startColumnIndex" ) ]
      start_column_index : Option< i64 >,
      /// The end column (exclusive) of the range, or not set if unbounded.
      #[ serde( rename = "endColumnIndex" ) ]
      end_column_index : Option< i64 >,
    }

    /// Union field source_data. The source of the pivot table data.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum SourceData
    {
      /// The range the pivot table is reading data from. 
      #[ serde( rename = "source" ) ]
      Source( GridRange ),
      /// The ID of the data source the pivot table is reading data from. 
      #[ serde( rename = "dataSourceId" ) ]
      DataSourceId( String )
    }

    /// A pivot table. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct PivotTable
    {
      /// Each row grouping in the pivot table. 
      rows : Option< Vec< PivotGroup > >,
      /// Each column grouping in the pivot table. 
      columns : Option< Vec< PivotGroup > >,
      /// This item is deprecated! 
      /// 
      /// An optional mapping of filters per source column offset. 
      /// 
      /// The filters are applied before aggregating data into the pivot table. The map's key is the column offset of the source range that you want to filter, and the value is the criteria for that column. 
      /// 
      /// For example, if the source was C10:E15, a key of 0 will have the filter for column C, whereas the key 1 is for column D. 
      /// 
      /// This field is deprecated in favor of `filter_specs`. 
      criteria : Option< Criteria >,
      #[ serde( rename = "filterSpecs" ) ]
      /// The filters applied to the source columns before aggregating data for the pivot table. 
      /// 
      /// Both criteria and filter_specs are populated in responses. If both fields are specified in an update request, this field takes precedence.
      filter_specs : Option< Vec< PivotFilterSpec > >,
      /// A list of values to include in the pivot table. 
      values : Option< Vec< PivotValue > >,
      /// Whether values should be listed horizontally (as columns) or vertically (as rows). 
      #[ serde( rename = "valueLayout" ) ]
      value_layout : Option< PivotValueLayout >,
      /// Output only. The data execution status for data source pivot tables. 
      #[ serde( rename = "dataExecutionStatus" ) ]
      data_execution_status : Option< DataExecutinStatus >,
      /// Union field source_data. The source of the pivot table data.
      source_data : Option< SourceData >
    }

    /// The data source table column selection types. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum DataSourceTableColumnSelectionType
    {
      /// Select columns specified by columns field. 
      #[ serde( rename = "SELECTED" ) ]
      Selected,
      /// Sync all current and future columns in the data source. 
      /// 
      /// If set, the data source table fetches all the columns in the data source at the time of refresh. 
      #[ serde( rename = "SYNC_ALL" ) ]
      SyncAll
    }

    /// Criteria for showing/hiding rows in a filter or filter view. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct FilterCriteria
    {
      /// Values that should be hidden. 
      #[ serde( rename = "hiddenValues" ) ]
      hidden_values : Option< String >,
      /// A condition that must be true for values to be shown. (This does not override hiddenValues -- if a value is listed there, it will still be hidden.) 
      condition : Option< BooleanCondition >,
      /// This item is deprecated! 
      /// 
      /// The background fill color to filter by; only cells with this fill color are shown. Mutually exclusive with visibleForegroundColor. Deprecated: Use visibleBackgroundColorStyle. 
      #[ serde( rename = "visibleBackgroundColor" ) ]
      visible_background_color : Option< Color >,
      /// The background fill color to filter by; only cells with this fill color are shown. This field is mutually exclusive with visibleForegroundColor, and must be set to an RGB-type color. If visibleBackgroundColor is also set, this field takes precedence. 
      #[ serde( rename = "visibleBackgroundColorStyle" ) ]
      visible_background_color_style : Option< ColorStyle >,
      /// This item is deprecated! 
      /// 
      /// The foreground color to filter by; only cells with this foreground color are shown. Mutually exclusive with visibleBackgroundColor. Deprecated: Use visibleForegroundColorStyle. 
      #[ serde( rename = "visibleForegroundColor" ) ]
      visible_foreground_color : Option< Color >,
      /// The foreground color to filter by; only cells with this foreground color are shown. This field is mutually exclusive with visibleBackgroundColor, and must be set to an RGB-type color. If visibleForegroundColor is also set, this field takes precedence. 
      #[ serde( rename = "visibleForegroundColorStyle" ) ]
      visible_foreground_color_style : Option< ColorStyle >,
    }

    /// Union field reference. Reference to the filtered column.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    #[ serde( untagged ) ]
    pub enum Reference
    {
      /// The zero-based column index. 
      #[ serde( rename = "columnIndex" ) ]
      ColumnIndex( i64 ),
      /// Reference to a data source column. 
      #[ serde( rename = "dataSourceColumnReference" ) ]
      DataSourceColumnReference( DataSourceColumnReference )
    }

    /// The filter criteria associated with a specific column. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct FilterSpec
    {
      /// The criteria for the column. 
      #[ serde( rename = "filterCriteria" ) ]
      filter_criteria : Option< FilterCriteria >,
      /// Union field reference. Reference to the filtered column
      reference : Option< Reference >
    }

    /// A sort order associated with a specific column or row. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct SortSpec
    {
      /// The order data should be sorted. 
      #[ serde( rename = "sortOrder" ) ]
      sort_order : Option< SortOrder >,
      /// This item is deprecated! 
      /// 
      /// The foreground color to sort by; cells with this foreground color are sorted to the top. Mutually exclusive with backgroundColor. Deprecated: Use foregroundColorStyle. 
      #[ serde( rename = "foregroundColor" ) ]
      foreground_color : Option< Color >,
      /// The foreground color to sort by; cells with this foreground color are sorted to the top. Mutually exclusive with backgroundColor, and must be an RGB-type color. If foregroundColor is also set, this field takes precedence. 
      #[ serde( rename = "foregroundColorStyle" ) ]
      foreground_color_style : Option< ColorStyle >,
      /// This item is deprecated! 
      /// 
      /// The background fill color to sort by; cells with this fill color are sorted to the top. Mutually exclusive with foregroundColor. Deprecated: Use backgroundColorStyle. 
      #[ serde( rename = "backgroundColor" ) ]
      background_color : Option< Color >,
      /// The background fill color to sort by; cells with this fill color are sorted to the top. Mutually exclusive with foregroundColor, and must be an RGB-type color. If backgroundColor is also set, this field takes precedence. 
      #[ serde( rename = "backgroundColorStyle" ) ]
      background_color_style : Option< ColorStyle >,
    }

    /// A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct DataSourceTable
    {
      /// The ID of the data source the data source table is associated with. 
      #[ serde( rename = "dataSourceId" )]
      data_source_id : Option< String >,
      /// The type to select columns for the data source table. Defaults to SELECTED. 
      #[ serde( rename = "columnSelectionType" ) ]
      column_selection_type : Option< DataSourceTableColumnSelectionType >,
      /// Columns selected for the data source table. The columnSelectionType must be SELECTED. 
      columns : Option< Vec< DataSourceColumnReference > >,
      /// Filter specifications in the data source table. 
      #[ serde( rename = "filterSpecs" ) ]
      filter_specs : Option< FilterSpec >,
      /// Sort specifications in the data source table. The result of the data source table is sorted based on the sort specifications in order. 
      #[ serde( rename = "sortSpecs" ) ]
      sort_specs : Option< Vec< SortSpec > >,
      /// The limit of rows to return. If not set, a default limit is applied. Please refer to the Sheets editor for the default and max limit. 
      #[ serde( rename = "rowLimit" ) ]
      row_limit : Option< i64 >,
      /// Output only. The data execution status.
      #[ serde( rename = "dataExecutionStatus" ) ]
      data_execution_status : Option< DataExecutinStatus >
    }

    /// A data source formula. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct DataSourceFormula
    {
      /// The ID of the data source the formula is associated with. 
      #[ serde( rename = "dataSourceId" ) ]
      data_source_id : Option< String >,
      /// Output only. The data execution status. 
      #[ serde( rename = "dataExecutionStatus" ) ]
      data_execution_status : Option< DataExecutinStatus >
    }

    /// Data about a specific cell. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct CellData
    {
      /// The value the user entered in the cell. e.g., 1234, 'Hello', or =NOW() Note: Dates, Times and DateTimes are represented as doubles in serial number format. 
      #[ serde( rename = "userEnteredValue" ) ]
      user_entered_value : Option< ExtendedValue >,
      /// The effective value of the cell. For cells with formulas, this is the calculated value. For cells with literals, this is the same as the userEnteredValue. This field is read-only. 
      #[ serde( rename = "effectiveValue" ) ]
      effective_value : Option< ExtendedValue >,
      /// The formatted value of the cell. This is the value as it's shown to the user. This field is read-only.
      #[ serde( rename = "formattedValue" ) ]
      formatted_value : Option< String >,
      /// The format the user entered for the cell. 
      /// 
      /// When writing, the new format will be merged with the existing format. 
      #[ serde( rename = "userEnteredFormat" ) ]
      user_entered_format : Option< CellFormat >,
      /// The effective format being used by the cell. This includes the results of applying any conditional formatting and, if the cell contains a formula, the computed number format. If the effective format is the default format, effective format will not be written. This field is read-only. 
      #[ serde( rename = "effectiveFormat" ) ]
      effective_format : Option< CellFormat >,
      /// A hyperlink this cell points to, if any. If the cell contains multiple hyperlinks, this field will be empty. This field is read-only. To set it, use a =HYPERLINK formula in the userEnteredValue.formulaValue field. A cell-level link can also be set from the userEnteredFormat.textFormat field. Alternatively, set a hyperlink in the textFormatRun.format.link field that spans the entire cell. 
      #[ serde( rename = "hyperLink" ) ]
      hyper_link : Option< String >,
      /// Any note on the cell. 
      note : Option< String >,
      /// Runs of rich text applied to subsections of the cell. Runs are only valid on user entered strings, not formulas, bools, or numbers. Properties of a run start at a specific index in the text and continue until the next run. Runs will inherit the properties of the cell unless explicitly changed. 
      /// 
      /// When writing, the new runs will overwrite any prior runs. When writing a new userEnteredValue, previous runs are erased. 
      #[ serde( rename = "textFormatRuns" ) ]
      text_format_runs : Option< Vec< TextFormatRun > >,
      /// A data validation rule on the cell, if any. 
      /// 
      /// When writing, the new data validation rule will overwrite any prior rule. 
      #[ serde( rename = "dataValidation" ) ]
      data_validation : Option< DataValidationRule >,
      /// A pivot table anchored at this cell. The size of pivot table itself is computed dynamically based on its data, grouping, filters, values, etc. Only the top-left cell of the pivot table contains the pivot table definition. The other cells will contain the calculated values of the results of the pivot in their effectiveValue fields. 
      #[ serde( rename = "pivotTable" ) ]
      pivot_table : Option< PivotTable >,
      /// A data source table anchored at this cell. The size of data source table itself is computed dynamically based on its configuration. Only the first cell of the data source table contains the data source table definition. The other cells will contain the display values of the data source table result in their effectiveValue fields. 
      #[ serde( rename = "dataSourceTable" ) ]
      data_source_table : Option< DataSourceTable >,
      /// Output only. Information about a data source formula on the cell. The field is set if userEnteredValue is a formula referencing some DATA_SOURCE sheet, e.g. =SUM(DataSheet!Column). 
      #[ serde( rename = "dataSourceFormula" ) ]
      data_source_formula : Option< DataSourceFormula >
    }

    /// Data about each cell in a row. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct RowData
    {
      /// The values in the row, one per column. 
      values : Option< Vec< CellData > >
    }
    
    /// An enumeration of the types of locations on which developer metadata may be associated.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum DeveloperMetadataLocationType
    {
      /// Developer metadata associated on an entire row dimension.
      #[ serde( rename = "ROW" ) ]
      Row,
      /// Developer metadata associated on an entire column dimension.
      #[ serde( rename = "COLUMN" ) ]
      Column,
      /// Developer metadata associated on an entire sheet.
      #[ serde( rename = "SHEET" ) ]
      Sheet,
      /// Developer metadata associated on the entire spreadsheet.
      #[ serde( rename = "SPREADSHEET" ) ]
      Spreadsheet
    }

    /// Union field location. The location where metadata is associated.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    #[ serde( untagged ) ]
    pub enum Location 
    {
      /// True when metadata is associated with an entire spreadsheet.
      #[ serde( rename = "spreadsheet" ) ]
      Spreadsheet( bool ),
      /// The ID of the sheet when metadata is associated with an entire sheet.
      #[ serde( rename = "sheetId" ) ]
      SheetId( i64 ),
      /// Represents the row or column when metadata is associated with a dimension. The specified DimensionRange must represent a single row or column; it cannot be unbounded or span multiple rows or columns.
      #[ serde( rename = "dimensionRange" ) ]
      DimensionRange( DimensionRange )
    }

    /// A location where metadata may be associated in a spreadsheet.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct DeveloperMetadataLocation
    {
      /// The type of location this object represents. This field is read-only.
      #[ serde( rename = "locationType" ) ]
      location_type : Option< DeveloperMetadataLocationType >,
      /// Union field location. The location where metadata is associated.
      location : Option< Location >
    }

    /// An enumeration of possible metadata visibilities.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum DeveloperMetadataVisibility
    {
      /// Document-visible metadata is accessible from any developer project with access to the document.
      #[ serde( rename = "DOCUMENT" ) ]
      Document,
      /// Project-visible metadata is only visible to and accessible by the developer project that created the metadata.
      #[ serde( rename = "PROJECT" ) ]
      Project
    }

    /// Developer metadata associated with a location or object in a spreadsheet. 
    /// Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet 
    /// and will remain associated at those locations as they move around and the spreadsheet is edited. 
    /// For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, 
    /// that original metadata will still be associated with the row it was first associated with (what is now row 6). 
    /// If the associated object is deleted its metadata is deleted too.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct DeveloperMetadata
    {
      /// The spreadsheet-scoped unique ID that identifies the metadata. 
      /// IDs may be specified when metadata is created, otherwise one will be randomly generated and assigned.
      ///  Must be positive.
      #[ serde( rename = "metadataId" ) ]
      metadata_id : Option< u64 >,
      /// The metadata key. 
      /// There may be multiple metadata in a spreadsheet with the same key. 
      /// Developer metadata must always have a key specified.
      #[ serde( rename = "metadataKey" ) ]
      metadata_key : Option< String >,
      /// Data associated with the metadata's key.
      #[ serde( rename = "metadataValue" ) ]
      metadata_value : Option< String >,
      /// The location where the metadata is associated.
      location : Option< DeveloperMetadataLocation >,
      /// The metadata visibility. Developer metadata must always have a visibility specified.
      visibility : Option< DeveloperMetadataVisibility >
    }

    /// Properties about a dimension. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct DimensionProperties
    {
      /// True if this dimension is being filtered. This field is read-only. 
      #[ serde( rename = "hiddenByFilter" ) ]
      hidden_by_filter : Option< bool >,
      /// True if this dimension is explicitly hidden. 
      #[ serde( rename = "hiddenByUser" ) ]
      hidden_by_user : Option< bool >,
      /// The height (if a row) or width (if a column) of the dimension in pixels. 
      #[ serde( rename = "pixelSize" ) ]
      pixel_size : Option< i32 >,
      /// The developer metadata associated with a single row or column. 
      #[ serde( rename = "developerMetadata" ) ]
      developer_metadata : Option< Vec< DeveloperMetadata > >,
      /// Output only. If set, this is a column in a data source sheet. 
      #[ serde( rename = "dataSourceColumnReference" ) ]
      data_source_column_reference : Option< DataSourceColumnReference >
    }

    /// Data in the grid, as well as metadata about the dimensions. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct GridData
    {
      /// The first row this GridData refers to, zero-based. 
      #[ serde( rename = "startRow" ) ]
      start_row : Option< i32 >,
      /// The first column this GridData refers to, zero-based. 
      #[ serde( rename ="startColumn" ) ]
      start_column : Option< i32 >,
      /// The data in the grid, one entry per row, starting with the row in startRow. The values in RowData will correspond to columns starting at startColumn. 
      #[ serde( rename = "rowData" ) ]
      row_data : Option< Vec< RowData > >,
      /// Metadata about the requested rows in the grid, starting with the row in startRow. 
      #[ serde( rename = "rowMetadata" ) ]
      row_metadata : Option< Vec< DimensionProperties > >,
      /// Metadata about the requested columns in the grid, starting with the column in startColumn. 
      #[ serde( rename = "columnMetadata" ) ]
      column_metadata : Option< Vec< DimensionProperties > >,
    }

    /// A rule that may or may not match, depending on the condition. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct BooleanRule
    {
      /// The condition of the rule. If the condition evaluates to true, the format is applied. 
      condition : Option< BooleanCondition >,
      /// The format to apply. 
      /// Conditional formatting can only apply a subset of formatting: bold, italic, strikethrough, foreground color and, background color.
      format : Option< CellFormat >
    }

    /// The kind of interpolation point
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub enum InterpolationPointType
    {
      /// The interpolation point uses the minimum value in the cells over the range of the conditional format. 
      #[ serde( rename = "MIN" ) ]
      Min,
      /// The interpolation point uses the maximum value in the cells over the range of the conditional format. 
      #[ serde( rename = "MAX" ) ]
      Max,
      /// The interpolation point uses exactly the value in InterpolationPoint.value. 
      #[ serde( rename = "NUMBER" ) ]
      Number,
      /// The interpolation point is the given percentage over all the cells in the range of the conditional format. 
      /// This is equivalent to NUMBER if the value was: 
      /// **=(MAX(FLATTEN(range)) * (value / 100)) + (MIN(FLATTEN(range)) * (1 - (value / 100)))** 
      /// (where errors in the range are ignored when flattening). 
      #[ serde( rename = "PERCENT" ) ]
      Percent,
      /// The interpolation point is the given percentile over all the cells in the range of the conditional format. 
      /// This is equivalent to **NUMBER** if the value was: 
      /// **=PERCENTILE(FLATTEN(range), value / 100)** (where errors in the range are ignored when flattening). 
      #[ serde( rename = "PERCENTILE" ) ]
      Percentile
    }

    /// A single interpolation point on a gradient conditional format. 
    /// These pin the gradient color scale according to the color, type and value chosen. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct InterpolationPoint
    {
      /// This item is deprecated! 
      /// 
      /// The color this interpolation point should use. Deprecated: Use color_style. 
      color : Option< Color >,
      /// The color this interpolation point should use. If color is also set, this field takes precedence. 
      #[ serde( rename = "colorStyle" ) ]
      color_style : Option< ColorStyle >,
      /// How the value should be interpreted. 
      #[ serde( rename = "type" ) ]
      t : Option< InterpolationPointType >,
      /// The value this interpolation point uses. May be a formula. Unused if type is MIN or MAX. 
      value : Option< String >
    }

    /// A rule that applies a gradient color scale format, based on the interpolation points listed. 
    /// The format of a cell will vary based on its contents as compared to the values of the interpolation points. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct GradientRule
    {
      /// The starting interpolation point
      minpoint : Option< InterpolationPoint >,
      /// An optional midway interpolation point. 
      midpoint : Option< InterpolationPoint >,
      /// The final interpolation point. 
      maxpoint : Option< InterpolationPoint >
    }

    /// Union field **rule**. The rule controlling this conditional format, exactly one must be set.
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    #[ serde( untagged ) ]
    pub enum Rule
    {
      /// The formatting is either "on" or "off" according to the rule.
      #[ serde( rename = "booleanRule" ) ]
      BooleanRule( BooleanRule ),
      /// The formatting will vary based on the gradients in the rule. 
      #[ serde( rename = "gradientRule" ) ]
      GradientRule( GradientRule )
    }

    /// A rule describing a conditional format. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct ConditionalFormatRule
    {
      /// The ranges that are formatted if the condition is true. All the ranges must be on the same grid. 
      ranges : Option< Vec< GridRange > >,
      /// Union field **rule**. The rule controlling this conditional format, exactly one must be set.
      rule : Option< Rule >
    }

    /// A filter view. 
    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct FilterView
    {
      /// The ID of the filter view. 
      #[ serde( rename = "filterViewId" ) ]
      filter_view_id : Option< String >,
      /// The name of the filter view. 
      title : Option< String >,
      /// The range this filter view covers. 
      /// 
      /// When writing, only one of range or namedRangeId may be set. 
      range : Option< GridRange >,
      /// The named range this filter view is backed by, if any. 
      /// 
      /// When writing, only one of range or namedRangeId may be set. 
      #[ serde( rename = "namedRangeId" ) ]
      named_range_id : Option< String >,
      /// The sort order per column. Later specifications are used when values are equal in the earlier specifications. 
      #[ serde( rename = "sortSpecs" ) ]
      sort_specs : Option< Vec< SortSpec > >,
      /// This item is deprecated! 
      /// 
      /// The criteria for showing/hiding values per column. 
      /// The map's key is the column index, and the value is the criteria for that column.
      /// 
      /// This field is deprecated in favor of filterSpecs.  
      criteria : Option< Criteria >,
      /// The filter criteria for showing/hiding values per column. 
      /// 
      /// Both criteria and filterSpecs are populated in responses. 
      /// If both fields are specified in an update request, this field takes precedence. 
      #[ serde( rename = "filterSpecs" ) ]
      filter_specs : Option< Vec< FilterSpec >  >
    }

    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct Sheet
    {
      properties : Option< SheetProperties >,
      data : Option< Vec< GridData > >,
      merges : Option< Vec< GridRange > >,
      #[ serde( rename = "conditionalFormats" ) ]
      conditional_formats : Option< ConditionalFormatRule >,
      filter_views : Option< FilterView >
    }

    #[ derive( Debug, ser::Serialize, ser::Deserialize, Clone ) ]
    pub struct Spreadsheet
    {
      #[ serde( rename = "spreadsheetId" ) ]
      spreadsheet_id : Option< String >,
      properties : Option< SpreadsheetProperties >,
      sheets : Option< Sheet >

    }
}

crate::mod_interface!
{
  orphan use
  {
    Dimension,
    ValueRange,
    DeleteDimensionRequest,
    Response,
    Request
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