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
    dbg!( instruction );

    let current_path = env::current_dir().unwrap();

    // ???
    match instruction.subject.as_str()
    {
      ".crate.info" => packages_iterate( current_path, OrderStrategy::Random )
      .for_each( | p |
      {
        let info = p.info();
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
        let failed = packages_iterate( current_path, OrderStrategy::Random )
        .filter_map( | p |
        {
          let info = p.info();
          let mut success = true;
          println!
          (
            "=== Verification ===\nlocation: {}\nLicense: {}\nReadme: {}\nDocumentation: {}\nTests: {}",
            p.path().display(),
            if info.license.is_some() { "Yes" } else { success = false; "No" },
            if info.readme.is_some() { "Yes" } else { success = false; "No" },
            if info.documentation.is_some() { "Yes" } else { success = false; "No" },
            if p.is_tests_passed() { "Passed" } else { success = false; "Failed" }
          );
          if !success { Some( info ) }
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
