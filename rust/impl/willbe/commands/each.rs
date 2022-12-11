/// Internal namespace.
pub( crate ) mod private
{
  use std::env;
  use wtools::error::BasicError;
  use wca::
  {
    Args,
    NoProperties,
  };

  use crate::protected::*;
  use wca::InstructionParser;

  ///
  /// Iterate over subject
  ///

  pub fn each( args : Args< String, NoProperties >, ctx : &mut wca::Context ) -> Result< (), BasicError >
  {
    let current_path = env::current_dir().unwrap();

    println!( "[LOG] Called each command" );

    println!( "context: {:#?}\nargs: {:?}", &ctx, &args );
    let parser = wca::instruction::DefaultInstructionParser::former().form();

    let routine = ctx.s[&args.subject].routine.clone();
    packages_iterate(current_path)
    .into_iter()
    .for_each( | package |
    {
      env::set_current_dir( package.path() ).unwrap();
      routine.perform
      (
        &parser.parse( &args.subject ).unwrap(),
        Some( ctx )
      )
      .ok();
    });
    // TODO: Remove on release
    // match args.subject.as_str()
    // {
    //   ".crate.info" => packages_iterate( current_path ).ordered( OrderStrategy::Alphabetical ).into_iter()
    //   .for_each( | p |
    //   {
    //     let info = PackageMetadata::try_from( p ).unwrap();
    //     let info = info.all();
    //     println!
    //     (
    //       "===\nName: {}\nVersion: {}\nDependencies: {:?}\nLocation: {}",
    //       info.name,
    //       info.version,
    //       info.dependencies.iter().map( | d | &d.name ).collect::< Vec< _ > >(),
    //       info.manifest_path.parent().unwrap()
    //     )
    //   }),
    //   ".crate.publish" =>
    //   {
    //     let failed = packages_iterate( current_path ).ordered( OrderStrategy::Topological ).into_iter()
    //     .filter_map( | p |
    //     {
    //       let info = PackageMetadata::try_from( p ).unwrap();

    //       let mut success = true;
    //       println!
    //       (
    //         "=== Verification ===\nlocation: {}\nLicense: {}\nReadme: {}\nDocumentation: {}\nTests: {}",
    //         info.as_package().path().display(),
    //         if info.has_license() { "Yes" } else { success = false; "No" },
    //         if info.has_readme() { "Yes" } else { success = false; "No" },
    //         if info.has_documentation() { "Yes" } else { success = false; "No" },
    //         if info.is_tests_passed() { "Passed" } else { success = false; "Failed" }
    //       );
    //       if !success { Some( info.all().to_owned() ) }
    //       else { None }
    //     })
    //     // collect all failed and after all show result
    //     .collect::< Vec< _ > >();

    //     println!( "\t Fails:" );
    //     for ( n, p ) in failed.iter().enumerate()
    //     {
    //       println!( "- {n} -" );
    //       println!( "Name: {}", p.name );
    //       println!( "Manifest: {}\n", p.manifest_path );
    //     }
    //   },
    //   _ => {}
    // }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use each;
}
