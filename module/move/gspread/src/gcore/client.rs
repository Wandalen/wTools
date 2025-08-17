//!
//! Main module coordinator for Google Sheets API client functionality.
//! 
//! This module serves as the main entry point and coordinator for all Google Sheets 
//! API functionality, re-exporting key types and components from specialized modules.
//!

mod private
{

}

crate::mod_interface!
{
  // Re-export from auth module
  exposed use crate::gcore::auth::
  {
    Auth,
    Client,
  };

  // Re-export from methods module  
  exposed use crate::gcore::methods::
  {
    SpreadSheetMethod,
    SheetCopyMethod, 
    SpreadSheetValuesMethod,
    ValuesGetMethod,
    ValuesBatchGetMethod,
    ValuesUpdateMethod,
    ValuesBatchUpdateMethod,
    ValuesAppendMethod,
    ValuesClearMethod,
    ValuesBatchClearMethod,
  };

  // Re-export from types module
  exposed use crate::gcore::types::
  {
    SheetCopyRequest,
    GridProperties, 
    Color,
    DataSourceColumnReference,
    DataSourceColumn,
    DataExecutinStatus,
    DataSourceSheetProperties,
    SheetProperties,
    GetValuesRequest,
    BatchGetValuesRequest,
    UpdateValuesRequest,
    BatchUpdateValuesRequest,
    ValuesAppendRequest,
    BatchClearValuesRequest,
    BatchGetValuesResponse,
    UpdateValuesResponse,
    BatchUpdateValuesResponse,
    ValuesAppendResponse,
    BatchClearValuesResponse,
    ValuesClearResponse,
    ValueRange,
  };

  // Re-export from enums module
  exposed use crate::gcore::enums::
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
    Dimension,
  };
}