//! Caching of results for optimal parameters search.

use std::
{
  collections::HashMap,
  fs::{ File, OpenOptions },
  io::{ BufRead, BufReader, Write },
  sync::{ Arc, Mutex },
};
use rkyv::{ Archive, Deserialize, Serialize } ;
// use crate::optimal_params_search::nelder_mead::Point;

#[ derive( Archive, Deserialize, Serialize, Debug ) ]
#[ archive
(
  compare( PartialEq ),
  check_bytes,
) ]

#[ archive_attr( derive( Debug ) ) ]
struct ObjectiveFunctionValue 
{
  point : ( f64, u32, f64, f64, u32, u32, u32 ),
  value : f64,
}

/// Save results of optimal parameters search.
pub fn save_result( point : super::Point, value : f64, file : Arc< Mutex< File > > ) -> Result< (), Box< dyn std::error::Error > >
{
  let obj_value = ObjectiveFunctionValue{ point : point.into(), value };
  let bytes = rkyv::to_bytes::< _, 256 >( &obj_value ).unwrap();
  
  let mut file = file.lock().unwrap();
  file.write( &bytes )?;
  file.write( &vec![ 0x0A as u8 ] )?;
  
  Ok( () )
}

/// Read results from previous execution.
pub fn read_results( file_path : &str ) -> Result< HashMap< super::Point, f64 >, Box< dyn std::error::Error > >
{
  let read_file = OpenOptions::new().read( true ).open( file_path )?;
  let mut reader = BufReader::new( read_file );
  let mut buffer: Vec< u8 > = Vec::new();
  let mut data = HashMap::new();
  loop 
  {
    let n = reader.read_until( 0x0A as u8, &mut buffer )?;
    if n == 0
    {
      break;
    }

    let archived = rkyv::check_archived_root::< ObjectiveFunctionValue >( &buffer[ ..buffer.len() - 1 ] );
    if let Ok( archived ) = archived
    {
      let deserialized: Result< ObjectiveFunctionValue, _ > = archived.deserialize( &mut rkyv::Infallible );
      if let Ok( deserialized ) = deserialized
      {
        data.insert( super::Point::from( deserialized.point ), deserialized.value );
      }
    }
    
    buffer = Vec::new();
  }

  Ok( data )
}