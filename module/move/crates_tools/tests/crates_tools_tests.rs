use std::path::Path;
#[ cfg( feature = "enabled" ) ]
use crates_tools::CrateArchive;

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn download()
{
  let crate_archive = CrateArchive::download_crates_io( "test_experimental_c", "0.1.0" ).unwrap();

  let mut expected_files : Vec< &Path > = vec!
  [
    "test_experimental_c-0.1.0/.cargo_vcs_info.json".as_ref(),
    "test_experimental_c-0.1.0/src/lib.rs".as_ref(),
    "test_experimental_c-0.1.0/Cargo.toml".as_ref(),
    "test_experimental_c-0.1.0/Cargo.toml.orig".as_ref(),
  ];
  expected_files.sort();

  let mut actual_files = crate_archive.list();
  actual_files.sort();

  assert_eq!( expected_files, actual_files );
}
