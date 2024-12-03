//!
//! CLI actions of the tool.
//!

mod private {}

crate::mod_interface!
{
  layer gspread;
  layer gspread_get_header;
  layer gspread_get_rows;
  layer gspread_cell_get;
  layer gspread_cell_set;
  layer gspread_cells_set;
}

