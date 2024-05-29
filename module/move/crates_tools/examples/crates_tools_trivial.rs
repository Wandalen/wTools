#![ allow( missing_docs ) ]
use crates_tools::*;

fn main()
{
  #[ cfg( feature = "enabled" ) ]
  {
    // download a package with specific version from `crates.io`
    let crate_archive = CrateArchive::download_crates_io( "test_experimental_c", "0.1.0" ).unwrap();

    for path in crate_archive.list()
    {
      // take content from a specific file from the archive
      let bytes = crate_archive.content_bytes( path ).unwrap();
      let string = std::str::from_utf8( bytes ).unwrap();

      println!("# {}\n```\n{}```", path.display(), string);
    }
  }
}
