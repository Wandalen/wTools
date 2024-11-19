//! Implement classic table output format.
//!
//! # Example
//!
//! ```text
//!  sid | sname | gap
//! -----+-------+-----
//!    3 | Alice |   5
//!    6 | Joe   |   1
//!   10 | Boris |   5
//! ```

use crate::*;
use print::
{
  InputExtract,
  Context,
};
use std::borrow::Cow;
use core::
{
  fmt,
};
use std::sync::OnceLock;

/// A struct representing the classic table output format.
///
/// `Table` provides a standard implementation for table formatting,
/// supporting a classic style with default settings.
///
/// # Example
///
/// ```text
///  sid | sname | gap
/// -----+-------+-----
///    3 | Alice |   5
///    6 | Joe   |   1
///   10 | Boris |   5
/// ```
#[ derive( Debug ) ]
pub struct Table
{
  /// Delimitting header with grid line or not.
  pub delimitting_header : bool,
  /// Prefix added to each cell.
  pub cell_prefix : String,
  /// Postfix added to each cell.
  pub cell_postfix : String,
  /// Separator used between table columns.
  pub cell_separator : String,
  /// Prefix added to each row.
  pub row_prefix : String,
  /// Postfix added to each row.
  pub row_postfix : String,
  /// Separator used between rows.
  pub row_separator : String,
  /// Horizontal line character.
  pub h : char,
  /// Vertical line character.
  pub v : char,
  /// Left T-junction character.
  pub t_l : char,
  /// Right T-junction character.
  pub t_r : char,
  /// Top T-junction character.
  pub t_t : char,
  /// Bottom T-junction character.
  pub t_b : char,
  /// Cross junction character.
  pub cross : char,
  /// Top-left corner character.
  pub corner_lt : char,
  /// Top-right corner character.
  pub corner_rt : char,
  /// Bottom-left corner character.
  pub corner_lb : char,
  /// Bottom-right corner character.
  pub corner_rb : char,
  /// Limit table width. If the value is zero, then no limitation.
  pub max_width: usize,
}

impl Default for Table
{
  fn default() -> Self
  {

    let delimitting_header = true;

    let cell_prefix = "".to_string();
    let cell_postfix = "".to_string();
    let cell_separator = " │ ".to_string();
    let row_prefix = "│ ".to_string();
    let row_postfix = " │".to_string();
    let row_separator = "\n".to_string();

    let h = '─';
    let v = '|';
    let t_l = '├';
    let t_r = '┤';
    let t_t = '┬';
    let t_b = '┴';
    let cross = '┼';
    let corner_lt = '┌';
    let corner_rt = '┐';
    let corner_lb = '└';
    let corner_rb = '┘';
    let max_width = 0;

    Self
    {
      delimitting_header,
      cell_prefix,
      cell_postfix,
      cell_separator,
      row_prefix,
      row_postfix,
      row_separator,
      h,
      v,
      t_l,
      t_r,
      t_t,
      t_b,
      cross,
      corner_lt,
      corner_rt,
      corner_lb,
      corner_rb,
      max_width
    }
  }
}

impl Default for &'static Table
{
  fn default() -> Self
  {
    // qqq : find a better solution
    static STYLES : OnceLock< Table > = OnceLock::new();
    STYLES.get_or_init( ||
    {
      Table::default()
    })
  }
}

impl Table
{

  /// Returns a reference to a static instance of `Table`.
  ///
  /// This method provides access to a single shared instance of `Table`,
  /// ensuring efficient reuse of the classic table output format.
  pub fn instance() -> & 'static dyn TableOutputFormat
  {

    static INSTANCE : OnceLock< Table > = OnceLock::new();
    INSTANCE.get_or_init( ||
    {
      Self::default()
    })

  }

  /// Calculate how much space is needed in order to generate a table output with the specified
  /// number of columns. It will be impossible to render table smaller than the result of
  /// `min_width`.
  ///
  /// Is is the sum of:
  /// - Length of `row_prefix`.
  /// - Length of `row_postfix`.
  /// - Length of `cell_prefix` and `cell_postfix` multiplied by column count.
  /// - Length of `cell_separator` multiplied by `column_count - 1`.
  /// - Count of columns (multiplied by 1, because at least one cell should be available to render
  ///   meaningful information).
  ///
  /// This function is similar to `output_format::Records::min_width`, but it contains a `column_count`
  /// parameter, and it aslo uses the `output_format::Table` style parameters.
  pub fn min_width
  (
    &self,
    column_count : usize,
  ) -> usize
  {
    self.row_prefix.chars().count()
    + self.row_postfix.chars().count()
    + column_count * ( self.cell_postfix.chars().count() + self.cell_prefix.chars().count() )
    + if column_count == 0 { 0 } else { ( column_count - 1 ) * self.cell_separator.chars().count() }
    + column_count
  }
}

impl TableOutputFormat for Table
{
  fn extract_write< 'buf, 'data >( &self, x : &InputExtract< 'data >, c : &mut Context< 'buf > ) -> fmt::Result
  {
    let cell_prefix = &self.cell_prefix;
    let cell_postfix = &self.cell_postfix;
    let cell_separator = &self.cell_separator;
    let row_prefix = &self.row_prefix;
    let row_postfix = &self.row_postfix;
    let row_separator = &self.row_separator;
    let h = self.h.to_string();

    let column_count = x.col_descriptors.len();

    let unchangable_width = self.row_prefix.chars().count()
    + self.row_postfix.chars().count()
    + column_count * ( self.cell_postfix.chars().count() + self.cell_prefix.chars().count() )
    + if column_count == 0 { 0 } else { ( column_count - 1 ) * self.cell_separator.chars().count() };

    if self.max_width != 0 && ( unchangable_width + column_count > self.max_width )
    {
      return Err( fmt::Error );
    }

    let orig_column_space = x.col_descriptors.iter().map( |c| c.width ).sum::<usize>();
    
    let wrapped_text = wrap_text( &x, if self.max_width == 0 { 0 } else { self.max_width - unchangable_width }, orig_column_space );

    let new_column_space = wrapped_text.col_widthes.iter().sum::<usize>();

    let new_row_width = new_column_space + unchangable_width;

    let mut actual_rows = 0;

    for row in wrapped_text.data.iter()
    {
      if actual_rows == wrapped_text.first_row_height && x.has_header && self.delimitting_header
      {
        write!( c.buf, "{}", row_separator )?;
        write!( c.buf, "{}", h.repeat( new_row_width ) )?;
      }
      
      if actual_rows > 0
      {
        write!( c.buf, "{}", row_separator )?;
      }

      actual_rows += 1;

      write!( c.buf, "{}", row_prefix )?;

      for ( icol, col ) in row.iter().enumerate()
      {
        let cell_width = col.wrap_width;
        let col_width = wrapped_text.col_widthes[ icol ];
        let slice_width = col.content.chars().count();
        
        if icol > 0
        {
          write!( c.buf, "{}", cell_separator )?;
        }

        write!( c.buf, "{}", cell_prefix )?;
        
        let lspaces = ( col_width - cell_width ) / 2;
        let rspaces = ( ( col_width - cell_width ) as f32 / 2 as f32 ).round() as usize + cell_width - slice_width;

        if lspaces > 0
        {
          write!( c.buf, "{:<width$}", " ", width = lspaces )?;
        }
        
        write!( c.buf, "{}", col.content )?;

        if rspaces > 0
        {
          write!( c.buf, "{:>width$}", " ", width = rspaces )?;
        }

        write!( c.buf, "{}", cell_postfix )?;
      }

      write!( c.buf, "{}", row_postfix )?;
    }

    Ok(())
  }
}

/// Struct that represents a wrapped tabular data. It is similar to `InputExtract`,
/// but we cannot use it as it does not wrap the text and it contains wrong column
/// widthes and height (as they are dependent on wrapping, too).
///
/// This struct is similar to `output_format::Records::WrappedInputExtract` (which is
/// private, too), but it is suited for tabular data with several columns.
#[ derive( Debug ) ]
struct WrappedInputExtract< 'data >
{
  /// Tabular data of rows and columns.
  /// Note: these cells does not represent the actual information cells in the 
  /// original table. These cells are wrapped and used only for displaying. This also
  /// means that one row in original table can be represented here with one or more
  /// rows.
  data: Vec< Vec< WrappedCell< 'data > > >,

  /// New widthes of columns that include wrapping.
  col_widthes : Vec< usize >,

  /// Size of the first row of the table.
  /// This parameter is used in case header of the table should be displayed.
  first_row_height : usize,
}

/// Struct that represents a content of a wrapped cell.
/// It contains the slice of the cell as well as its original width.
///
/// Parameter `wrap_width` is needed as text in `output_format::Table` is centered.
/// However it is centered according to whole cell size and not the size of wrapped
/// text slice.
/// 
/// Example that depicts the importance of `wrap_width` parameter:
/// 
/// 1) | [        |  2) |    [     |
///    |   line1, |     |   line1, |
///    |   line2  |     |   line2  |
///    | ]        |     |    ]     |
///
/// The first case seems to be properly formatted, while the second case took centering
/// too literally. That is why `wrap_width` is introduced, and additional spaces to the 
/// right side will be included in the output formatter.
#[ derive( Debug ) ]
struct WrappedCell< 'data >
{
  wrap_width : usize,
  content : Cow< 'data, str >
}

/// Convert `InputExtract` data to properly wrapped table that is suitable for displaying.
/// `InputExtract` contains logical data of the table but it does not perform wrapping of
/// the cells (as wrapped text will be represented by new rows).
///
/// Wrapping is controlled by `limit_column_space` and `orig_column_space` parameters.
/// `orig_column_space` is the size occupied column widthes of original tabular data.
/// `limit_column_space` is the size space that is allowed to be occupied by columns.
///
/// The function will perform wrapping and shrink the columns so that they occupy not
/// more than `limit_column_space`.
///
/// When you use this function, do not forget that it accepts column space, but not the
/// maximum width of the table. It means that to calculate allowed space you need to subtract
/// lengthes of visual elements (prefixes, postfixes, separators, etc.) from the maximum width.
fn wrap_text< 'data >
(
  x : &'data InputExtract< 'data >,
  limit_column_space : usize,
  orig_column_space : usize,
) 
-> WrappedInputExtract< 'data >
{
  let mut first_row_height = 0;
  let mut new_data = Vec::new();
  let mut col_widthes = Vec::new();

  if limit_column_space == 0 || limit_column_space >= orig_column_space
  {
    col_widthes.extend( x.col_descriptors.iter().map( |d| d.width ) );
  }
  else
  {
    let shrink_factor: f32 = ( limit_column_space as f32 ) / ( orig_column_space as f32 );

    for ( icol, col ) in x.col_descriptors.iter().enumerate()
    {
      let col_width = col.width;

      let col_limit_float = ( col_width as f32 ) * shrink_factor;
      let col_limit = col_limit_float.floor() as usize;

      let col_width_to_put = if icol == x.col_descriptors.len() - 1
      {
        limit_column_space - col_widthes.iter().sum::<usize>()
      }
      else
      {
        col_limit.max(1)
      };

      col_widthes.push( col_width_to_put );
    }
  }

  for ( irow, row ) in x.data.iter().enumerate()
  {
    let row_descriptor = &x.row_descriptors[ irow ];

    if !row_descriptor.vis
    {
      continue;
    }

    let mut wrapped_rows : Vec< Vec< Cow< 'data, str > > > = vec![];

    for ( icol, col ) in row.iter().enumerate()
    {
      let col_limit = col_widthes[ icol ];
      let wrapped_col = string::lines_with_limit( col.0.as_ref(), col_limit ).map( Cow::from ).collect();
      wrapped_rows.push( wrapped_col );
    }

    let max_rows = wrapped_rows.iter().map( Vec::len ).max().unwrap_or(0);

    let mut transposed : Vec< Vec< WrappedCell< 'data > > > = Vec::new();

    if max_rows == 0 
    {
      transposed.push( vec![] );
    }
    
    for i in 0..max_rows
    {
      let mut row_vec : Vec< WrappedCell< 'data > > = Vec::new();

      for col_lines in &wrapped_rows
      {
        if col_lines.len() > i
        {
          let wrap_width = col_lines.iter().map( |c| c.len() ).max().unwrap_or(0);
          row_vec.push( WrappedCell { wrap_width , content : col_lines[ i ].clone() } );
        }
        else
        {
          row_vec.push( WrappedCell { wrap_width : 0, content : Cow::from( "" ) } );
        }
      }

      transposed.push( row_vec );
    }

    if irow == 0
    {
      first_row_height += transposed.len();
    }

    new_data.extend(transposed);
  }

  WrappedInputExtract
  {
    data: new_data,
    first_row_height,
    col_widthes
  }
}