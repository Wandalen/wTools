/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::error::BasicError;

  ///
  /// Iterate over subject
  /// 

  pub fn each( instruction : &crate::instruction::Instruction ) -> Result< (), BasicError >
  {
    let current_path = env::current_dir().unwrap();

    // ???
    match instruction.subject.as_str()
    {
      ".crate.info" => packages_iterate( current_path, OrderStrategy::Alphabetical )
      .for_each( | p |
      {
        let info = PackageMetadata::try_from( p ).unwrap();
        let info = info.all();
        println!
        (
          "===\nName: {}\nVersion: {}\nDependencies: {:?}\nLocation: {}",
          info.name,
          info.version,
          info.dependencies.iter().map( | d | &d.name ).collect::< Vec< _ > >(),
          info.manifest_path.parent().unwrap()
        )
      }),
      ".crate.publish" =>
      {
        let failed = packages_iterate( current_path, OrderStrategy::Topological )
        .filter_map( | p |
        {
          let info = PackageMetadata::try_from( p ).unwrap();

          let mut success = true;
          println!
          (
            "=== Verification ===\nlocation: {}\nLicense: {}\nReadme: {}\nDocumentation: {}\nTests: {}",
            info.as_package().path().display(),
            if info.has_license() { "Yes" } else { success = false; "No" },
            if info.has_readme() { "Yes" } else { success = false; "No" },
            if info.has_documentation() { "Yes" } else { success = false; "No" },
            if info.is_tests_passed() { "Passed" } else { success = false; "Failed" }
          );
          if !success { Some( info.all().to_owned() ) }
          else { None }
        })
        // collect all failed and after all show result
        .collect::< Vec< _ > >();

        println!( "\t Fails:" );
        for ( n, p ) in failed.iter().enumerate()
        {
          println!( "- {n} -" );
          println!( "Name: {}", p.name );
          println!( "Manifest: {}\n", p.manifest_path );
        }
      },
      _ => {}
    }
    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use each;
}
