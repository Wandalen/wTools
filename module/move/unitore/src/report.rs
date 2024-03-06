use gluesql::prelude::{ Payload, Value };

/// Information about result of execution of command for frames.
pub struct FramesReport
{
  pub updated_frames : usize,
  pub new_frames : usize,
  pub selected_frames : SelectedEntries,
}

impl FramesReport
{
  pub fn new() -> Self
  {
    Self
    {
      updated_frames : 0,
      new_frames : 0,
      selected_frames : SelectedEntries::new(),
    }
  }
}

/// General report.
pub trait Report : std::fmt::Display
{
  fn report( &self )
  {
    println!( "{self}" );
  }
}

impl std::fmt::Display for FramesReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    writeln!( f, "Updated frames: {}", self.updated_frames )?;
    writeln!( f, "Inserted frames: {}", self.new_frames )?;
    if !self.selected_frames.selected_columns.is_empty()
    {
      writeln!( f, "Selected frames:" )?;
      for row in &self.selected_frames.selected_rows
      {
        for i in 0..self.selected_frames.selected_columns.len()
        {
            writeln!( f, "{} : {}, ", self.selected_frames.selected_columns[ i ], DisplayValue( &row[ i ] ) )?;
        }
        writeln!( f, "" )?;
      }
    }
    Ok( () )
  }
}

impl Report for FramesReport {}

/// Information about result of execution of command for fileds.
pub struct FieldsReport
{
  pub fields_list : Vec< [ &'static str; 3 ] >,
}

impl std::fmt::Display for FieldsReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    writeln!( f, "Frames fields:" )?;
    for field in &self.fields_list
    {
      writeln!( f, "{}, type {} : {}", field[ 0 ], field[ 1 ], field[ 2 ] )?;
    }
    Ok( () )
  }
}

impl Report for FieldsReport {}

pub struct SelectedEntries
{
  pub selected_columns : Vec< String >,
  pub selected_rows : Vec< Vec< Value > >,
}

impl SelectedEntries
{
  pub fn new() -> Self
  {
    SelectedEntries { selected_columns : Vec::new(), selected_rows : Vec::new() }
  }
}

impl std::fmt::Display for SelectedEntries
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    if !self.selected_columns.is_empty()
    {
      for row in &self.selected_rows
      {
        for i in 0..self.selected_columns.len()
        {
          write!( f, "{} : {}, ", self.selected_columns[ i ], DisplayValue( &row[ i ] ) )?;
        }
        writeln!( f, "" )?;
      }
    }

    Ok( () )
  }
}

/// Information about result of execution of command for feed.
pub struct FeedsReport
{
  pub selected_entries : SelectedEntries,
}

impl FeedsReport
{
  pub fn new() -> Self
  {
    Self { selected_entries : SelectedEntries::new() }
  }
}

impl std::fmt::Display for FeedsReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    if !self.selected_entries.selected_columns.is_empty()
    {
      writeln!( f, "Selected feeds:" )?;
      println!( "{}", self.selected_entries );
    }

    Ok( () )
  }
}

impl Report for FeedsReport {}

/// Information about result of execution of custom query.
pub struct QueryReport
{
  pub result : Vec< gluesql::prelude::Payload >,
}

impl std::fmt::Display for QueryReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    for payload in &self.result
    {
      match payload
      {
        Payload::ShowColumns( columns ) =>
        {
          writeln!( f, "Show columns:" )?;
          for column in columns
          {
            writeln!( f, "{} : {}", column.0, column.1 )?;
          }
        },
        Payload::Create => writeln!( f, "Table created" )?,
        Payload::Insert( number ) => writeln!( f, "Inserted {} rows", number )?,
        Payload::Delete( number ) => writeln!( f, "Deleted {} rows", number )?,
        Payload::Update( number ) => writeln!( f, "Updated {} rows", number )?,
        Payload::DropTable => writeln!( f, "Table dropped" )?,
        Payload::Select { labels: label_vec, rows: rows_vec } =>
        {
          writeln!( f, "Selected rows:" )?;
          for row in rows_vec
          {
            for i in 0..label_vec.len()
            {
              writeln!( f, "{} : {} ", label_vec[ i ], DisplayValue( &row[ i ] ) )?;
            }
            writeln!( f, "" )?;
          }
        },
        Payload::AlterTable => writeln!( f, "Table altered" )?,
        Payload::StartTransaction => writeln!( f, "Transaction started" )?,
        Payload::Commit => writeln!( f, "Transaction commited" )?,
        Payload::Rollback => writeln!( f, "Transaction rolled back" )?,
        _ => {},
      };
    }

    Ok( () )
  }
}

impl Report for QueryReport {}

struct DisplayValue< 'a >( pub &'a Value );

impl std::fmt::Display for DisplayValue< '_ >
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    use Value::*;
    match &self.0
    {
      Bool( val ) => write!( f, "{}", val )?,
      I8( val ) => write!( f, "{}", val )?,
      I16( val ) => write!( f, "{}", val )?,
      I32( val ) => write!( f, "{}", val )?,
      I64( val ) => write!( f, "{}", val )?,
      I128( val ) => write!( f, "{}", val )?,
      U8( val ) => write!( f, "{}", val )?,
      U16( val ) => write!( f, "{}", val )?,
      U32( val ) => write!( f, "{}", val )?,
      U64( val ) => write!( f, "{}", val )?,
      U128( val ) => write!( f, "{}", val )?,
      F32( val ) => write!( f, "{}", val )?,
      F64( val ) => write!( f, "{}", val )?,
      Str( val ) => write!( f, "{}", val )?,
      Null => write!( f, "Null" )?,
      Timestamp( val ) => write!( f, "{}", val )?,
      _ => write!( f, "" )?,
    }

    Ok( () )
  }
}

/// Information about result of command for subscription config.
pub struct ConfigReport
{
  pub result : Payload,
}

impl std::fmt::Display for ConfigReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    match &self.result
    {
      Payload::Insert( number ) => writeln!( f, "Create {} config", number )?,
      Payload::Delete( number ) => writeln!( f, "Deleted {} config", number )?,
      Payload::Select { labels: label_vec, rows: rows_vec } =>
      {
        writeln!( f, "Selected configs:" )?;
        for row in rows_vec
        {
          for i in 0..label_vec.len()
          {
            writeln!( f, "{} : {} ", label_vec[ i ], DisplayValue( &row[ i ] ) )?;
          }
          writeln!( f, "" )?;
        }
      },
      _ => {},
    };

    Ok( () )
  }
}

impl Report for ConfigReport {}