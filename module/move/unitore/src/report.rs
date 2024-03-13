// qqq : rid off the file. ask

// use gluesql::prelude::{ Payload, Value };
// use cli_table::
// {
//   format::{ Border, Separator}, Cell, Style, Table
// };

// use crate::executor::endpoints::frames::{FramesReport, SelectedEntries};

// const EMPTY_CELL : &'static str = "";
// const INDENT_CELL : &'static str = "  ";

// /// Information about result of execution of command for frames.
// #[ derive( Debug ) ]
// pub struct FramesReport
// {
//   pub feed_title : String,
//   pub updated_frames : usize,
//   pub new_frames : usize,
//   pub selected_frames : SelectedEntries,
//   pub existing_frames : usize,
//   pub is_new_feed : bool,
// }

// impl FramesReport
// {
//   pub fn new( feed_title : String ) -> Self
//   {
//     Self
//     {
//       feed_title,
//       updated_frames : 0,
//       new_frames : 0,
//       selected_frames : SelectedEntries::new(),
//       existing_frames : 0,
//       is_new_feed : false,
//     }
//   }
// }

// /// General report.
// pub trait Report : std::fmt::Display + std::fmt::Debug
// {
//   fn report( &self )
//   {
//     println!( "{self}" );
//   }
// }

// impl std::fmt::Display for FramesReport
// {
//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     let initial = vec![ vec![ format!( "Feed title: {}", self.feed_title).cell().bold( true )  ] ];
//     let table_struct = initial.table()
//     .border( Border::builder().build() )
//     .separator( Separator::builder().build() );

//     let table = table_struct.display().unwrap(); 
//     write!( f, "{}", table )?;

//     let mut rows = vec![
//       vec![ EMPTY_CELL.cell(), format!( "Updated frames: {}", self.updated_frames ).cell() ],
//       vec![ EMPTY_CELL.cell(), format!( "Inserted frames: {}", self.new_frames ).cell() ],
//       vec![ EMPTY_CELL.cell(), format!( "Number of frames in storage: {}", self.existing_frames ).cell() ],
//     ];

//     if !self.selected_frames.selected_columns.is_empty()
//     {
//       rows.push( vec![ EMPTY_CELL.cell(), format!( "Selected frames:" ).cell() ] );
//     }
//     let table_struct = rows.table()
//     .border( Border::builder().build() )
//     .separator( Separator::builder().build() );

//     let table = table_struct.display().unwrap(); 

//     write!( f, "{}", table )?;
      
//     for frame in &self.selected_frames.selected_rows
//     {
//       let mut rows = Vec::new();
//       for i in 0..self.selected_frames.selected_columns.len()
//       {
//         let inner_row = vec!
//         [
//           INDENT_CELL.cell(),
//           self.selected_frames.selected_columns[ i ].clone().cell(),
//           textwrap::fill( &String::from( frame[ i ].clone() ), 120 ).cell(),
//         ];
//         rows.push( inner_row );
//       }
      
//       let table_struct = rows.table()
//       .border( Border::builder().build() )
//       .separator( Separator::builder().build() )
//       ;
      
  
//       let table = table_struct.display().unwrap();
//       writeln!( f, "{}", table )?;
//     }

//     Ok( () )
//   }
// }

// impl Report for FramesReport {}

// /// Information about result of execution of command for fileds.
// #[ derive( Debug ) ]
// pub struct FieldsReport
// {
//   pub fields_list : Vec< [ &'static str; 3 ] >,
// }

// impl std::fmt::Display for FieldsReport
// {

//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     let mut rows = Vec::new();
//     for field in &self.fields_list
//     {
//       rows.push( vec![ EMPTY_CELL.cell(), field[ 0 ].cell(), field[ 1 ].cell(), field[ 2 ].cell() ] );
//     }
//     let table_struct = rows.table()
//     .title( vec!
//     [
//       EMPTY_CELL.cell(),
//       "name".cell().bold( true ),
//       "type".cell().bold( true ),
//       "explanation".cell().bold( true ),
//     ] )
//     .border( Border::builder().build() )
//     .separator( Separator::builder().build() );

//     let table = table_struct.display().unwrap();

//     writeln!( f, "\n\n\nFrames fields:" )?;
//     writeln!( f, "{}", table )?;

//     Ok( () )
//   }
// }

// impl Report for FieldsReport {}

// #[ derive( Debug ) ]
// pub struct SelectedEntries
// {
//   pub selected_columns : Vec< String >,
//   pub selected_rows : Vec< Vec< Value > >,
// }

// impl SelectedEntries
// {
//   pub fn new() -> Self
//   {
//     SelectedEntries { selected_columns : Vec::new(), selected_rows : Vec::new() }
//   }
// }

// impl std::fmt::Display for SelectedEntries
// {
//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     if !self.selected_columns.is_empty()
//     {
//       for row in &self.selected_rows
//       {
//         for i in 0..self.selected_columns.len()
//         {
//           write!( f, "{} : {}, ", self.selected_columns[ i ], RowValue( &row[ i ] ) )?;
//         }
//         writeln!( f, "" )?;
//       }
//     }

//     Ok( () )
//   }
// }

// /// Information about result of execution of command for feed.
// #[ derive( Debug ) ]
// pub struct FeedsReport
// {
//   pub selected_entries : SelectedEntries,
// }

// impl FeedsReport
// {
//   pub fn new() -> Self
//   {
//     Self { selected_entries : SelectedEntries::new() }
//   }
// }

// impl std::fmt::Display for FeedsReport
// {
//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     writeln!( f, "Selected feeds:" )?;
//     if !self.selected_entries.selected_rows.is_empty()
//     {
//       let mut rows = Vec::new();
//       for row in &self.selected_entries.selected_rows
//       {
//         let mut new_row = vec![ EMPTY_CELL.cell() ];
//         new_row.extend( row.iter().map( | cell | String::from( cell ).cell() ) );
//         rows.push( new_row );
//       }
//       let mut headers = vec![ EMPTY_CELL.cell() ];
//       headers.extend( self.selected_entries.selected_columns.iter().map( | header | header.cell().bold( true ) ) );
//       let table_struct = rows.table()
//       .title( headers )
//       .border( Border::builder().build() )
//       .separator( Separator::builder().build() );

//       let table = table_struct.display().unwrap();
//       writeln!( f, "{}", table )?;
//     }
//     else
//     {
//       writeln!( f, "No items currently in storage!" )?;
//     }

//     Ok( () )
//   }
// }

// impl Report for FeedsReport {}

// /// Information about result of execution of custom query.
// #[ derive( Debug ) ]
// pub struct QueryReport
// {
//   pub result : Vec< gluesql::prelude::Payload >,
// }

// impl std::fmt::Display for QueryReport
// {
//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     for payload in &self.result
//     {
//       match payload
//       {
//         Payload::ShowColumns( columns ) =>
//         {
//           writeln!( f, "Show columns:" )?;
//           for column in columns
//           {
//             writeln!( f, "{} : {}", column.0, column.1 )?;
//           }
//         },
//         Payload::Create => writeln!( f, "Table created" )?,
//         Payload::Insert( number ) => writeln!( f, "Inserted {} rows", number )?,
//         Payload::Delete( number ) => writeln!( f, "Deleted {} rows", number )?,
//         Payload::Update( number ) => writeln!( f, "Updated {} rows", number )?,
//         Payload::DropTable => writeln!( f, "Table dropped" )?,
//         Payload::Select { labels: label_vec, rows: rows_vec } =>
//         {
//           writeln!( f, "Selected entries:" )?;
//           for row in rows_vec
//           {
//             let mut rows = Vec::new();
//             for i in 0..label_vec.len()
//             {
//               let new_row = vec!
//               [
//                 EMPTY_CELL.cell(),
//                 label_vec[ i ].clone().cell(),
//                 textwrap::fill( &String::from( row[ i ].clone() ), 120 ).cell(),
//               ];
//               rows.push( new_row );
//             }
//             let table_struct = rows.table()
//             .border( Border::builder().build() )
//             .separator( Separator::builder().build() );

//             let table = table_struct.display().unwrap();

//             writeln!( f, "{}", table )?;
//           }
//         },
//         Payload::AlterTable => writeln!( f, "Table altered" )?,
//         Payload::StartTransaction => writeln!( f, "Transaction started" )?,
//         Payload::Commit => writeln!( f, "Transaction commited" )?,
//         Payload::Rollback => writeln!( f, "Transaction rolled back" )?,
//         _ => {},
//       };
//     }

//     Ok( () )
//   }
// }

// impl Report for QueryReport {}

// pub struct RowValue< 'a >( pub &'a Value );

// impl std::fmt::Display for RowValue< '_ >
// {
//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     use Value::*;
//     match &self.0
//     {
//       Bool( val ) => write!( f, "{}", val )?,
//       I8( val ) => write!( f, "{}", val )?,
//       I16( val ) => write!( f, "{}", val )?,
//       I32( val ) => write!( f, "{}", val )?,
//       I64( val ) => write!( f, "{}", val )?,
//       I128( val ) => write!( f, "{}", val )?,
//       U8( val ) => write!( f, "{}", val )?,
//       U16( val ) => write!( f, "{}", val )?,
//       U32( val ) => write!( f, "{}", val )?,
//       U64( val ) => write!( f, "{}", val )?,
//       U128( val ) => write!( f, "{}", val )?,
//       F32( val ) => write!( f, "{}", val )?,
//       F64( val ) => write!( f, "{}", val )?,
//       Str( val ) => write!( f, "{}", val )?,
//       Null => write!( f, "Null" )?,
//       Timestamp( val ) => write!( f, "{}", val )?,
//       _ => write!( f, "" )?,
//     }

//     Ok( () )
//   }
// }

// impl From< RowValue< '_ > > for String
// {
//   fn from( value : RowValue< '_ > ) -> Self
//   {
//     use Value::*;
//     match &value.0
//     {
//       Str( val ) => val.clone(),
//       _ => String::new(),
//     }
//   }
// }

// #[ derive( Debug ) ]
// pub struct UpdateReport( pub Vec< FramesReport > );

// impl std::fmt::Display for UpdateReport
// {
//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     for report in &self.0
//     {
//       writeln!( f, "{}", report )?;
//     }
//     writeln!( f, "Total new feeds dowloaded : {}", self.0.iter().filter( | fr_report | fr_report.is_new_feed ).count() )?;
//     writeln!
//     (
//       f,
//       "Total feeds with updated or new frames : {}",
//       self.0.iter().filter( | fr_report | fr_report.updated_frames + fr_report.new_frames > 0 ).count()
//     )?;
//     writeln!( f, "Total new frames : {}", self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.new_frames ) )?;
//     writeln!( f, "Total updated frames : {}", self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.updated_frames ) )?;

//     Ok( () )
//   }
// }

// impl Report for UpdateReport {}

// #[ derive( Debug ) ]
// pub struct ListReport( pub Vec< FramesReport > );

// impl std::fmt::Display for ListReport
// {
//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     for report in &self.0
//     {
//       write!( f, "{}", report )?;
//     }
//     writeln!
//     (
//       f,
//       "Total feeds in storage: {}",
//       self.0.len()
//     )?;
//     writeln!
//     (
//       f,
//       "Total frames in storage: {}",
//       self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.selected_frames.selected_rows.len() )
//     )?;
//     writeln!( f, "" )?;

//     Ok( () )
//   }
// }

// impl Report for ListReport {}

// #[ derive( Debug ) ]
// pub struct TablesReport
// {
//   tables : std::collections::HashMap< String, Vec< String > >
// }

// impl TablesReport
// {
//   pub fn new( payload : Vec< Payload > ) -> Self
//   {
//     let mut result = std::collections::HashMap::new();
//     match &payload[ 0 ]
//     {
//       Payload::Select { labels: _label_vec, rows: rows_vec } =>
//       {
//         for row in rows_vec
//         {
//           let table = String::from( row[ 0 ].clone() );
//           result.entry( table )
//           .and_modify( | vec : &mut Vec< String > | vec.push( String::from( row[ 1 ].clone() ) ) )
//           .or_insert( vec![ String::from( row[ 1 ].clone() ) ] )
//           ;
//         }
//       },
//       _ => {},
//     }
//     TablesReport{ tables : result }
//   }
// }

// impl std::fmt::Display for TablesReport
// {
//   fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
//   {
//     writeln!( f, "Storage tables:" )?;
//     let mut rows = Vec::new();
//     for ( table_name, columns ) in &self.tables
//     {
//       let columns_str = if !columns.is_empty()
//       {
//         let first = columns[ 0 ].clone();
//         columns.iter().skip( 1 ).fold( first, | acc, val | format!( "{}, {}", acc, val ) )
//       }
//       else
//       {
//         String::from( "No columns" )
//       };

//       rows.push
//       (
//         vec!
//         [
//           EMPTY_CELL.cell(),
//           table_name.cell(),
//           columns_str.cell(),
//         ]
//       );
//     }

//     let table_struct = rows.table()
//     .border( Border::builder().build() )
//     .separator( Separator::builder().build() )
//     .title( vec!
//     [
//       EMPTY_CELL.cell(),
//       "name".cell().bold( true ),
//       "columns".cell().bold( true ),
//     ] );

//     let table = table_struct.display().unwrap(); 

//     writeln!( f, "{}", table )?;

//     Ok( () )
//   }
// }

// impl Report for TablesReport {}