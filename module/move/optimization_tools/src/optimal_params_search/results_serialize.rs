use std::
{
  fs::OpenOptions, path::{ PathBuf, Path },
  io::{ BufRead, BufReader, Write },
  process::Command,
};
use rkyv::{ Archive, Deserialize, Serialize} ;

#[ derive( Archive, Deserialize, Serialize, Debug ) ]
#[ archive
(
    compare( PartialEq ),
    check_bytes,
) ]

#[ archive_attr( derive( Debug ) ) ]
struct ObjectiveFunctionValue 
{
    point : Vec< f64 >,
    value : f64,
}

pub fn save_result( point : Vec< f64 >, value : f64 ) -> Result< (), Box< dyn std::error::Error > >
{
  let obj_value = ObjectiveFunctionValue{ point, value };

  let dir_path = format!( "{}/target", workspace_dir().to_string_lossy() );
  _ = std::fs::create_dir( &dir_path );
  let path = format!( "{}/test.txt", dir_path );

  let bytes = rkyv::to_bytes::<_, 256>( &obj_value ).unwrap();
  let mut file = OpenOptions::new()
  .write( true )
  .append( true )
  .create( true )
  .open( &path )
  .unwrap();

  file.write( &bytes)?;

  Ok( () )
}

pub fn read_results() -> Result< (), Box< dyn std::error::Error > >
{

  let dir_path = format!( "{}/target", workspace_dir().to_string_lossy() );
  _ = std::fs::create_dir( &dir_path );
  let path = format!( "{}/test.txt", dir_path );

  let read_file = OpenOptions::new().read( true ).open( &path )?;
  let mut reader = BufReader::new( read_file );
  let mut buffer: Vec< u8 > = Vec::new();
  reader.read_until( 0x0A as u8, &mut buffer )?;

  let archived = rkyv::check_archived_root::< ObjectiveFunctionValue >( &buffer[..] ).unwrap();

  Ok( () )
}

pub fn workspace_dir() -> PathBuf 
{
  let output = Command::new( env!( "CARGO" ) )
  .arg( "locate-project" )
  .arg( "--workspace" )
  .arg( "--message-format=plain" )
  .output()
  ;
  if let Ok( output ) = output
  {
    let path = output.stdout;
    let cargo_path = Path::new( std::str::from_utf8( &path ).unwrap().trim() );
    cargo_path.parent().unwrap().to_path_buf()
  }
  else 
  {
    std::env::current_dir().unwrap()
  }
}