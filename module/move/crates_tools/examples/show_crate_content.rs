#![ allow( missing_docs ) ]
use crates_tools::*;

fn main()
{
  let krate = CrateArchive::download_crates_io( "test_experimental_c", "0.1.0" ).unwrap();

  for path in krate.list()
  {
    let bytes = krate.content_bytes( path ).unwrap();
    let string = std::str::from_utf8( bytes ).unwrap();

    println!( "# {}\n```\n{}```", path.display(), string );
  }
}
