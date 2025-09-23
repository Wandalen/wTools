//!
//! Enum definitions for Google Sheets API types.
//! 

mod private
{
  use ser ::
  { 
  Serialize, 
  Deserialize 
 };

  use crate :: *;
  use gcore ::types ::
  {
  Color
 };

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

}

crate ::mod_interface!
{
  exposed use
  {
  SheetType,
  ThemeColorType,
  ColorStyle,
  DataExecutionState,
  DataExecutionErrorCode,
  InsertDataOption,
  DateTimeRenderOption,
  ValueRenderOption,
  ValueInputOption,
  Dimension
 };
}