
use super::*;

mod func_test;
mod impls_basic_test;
mod impls1_test;
mod impls2_test;
mod impls3_test;

mod index_test;
mod tests_index_test;

only_for_terminal_module!
{

  // stable have different information about error
  // that's why these tests are active only for nightly
  #[ test_tools::nightly ]
  #[ cfg( feature = "enabled" ) ]
  #[ test ]
  fn former_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = test_tools::compiletime::TestCases::new();
    // xxx : enable and use process::run

    // t.compile_fail( "tests/inc/compiletime/former_bad_attr.rs" );
    // t.pass( "tests/inc/compiletime/former_hashmap_without_parameter.rs" );
    // t.pass( "tests/inc/compiletime/former_vector_without_parameter.rs" );

    //t.compile_fail( "tests/inc/compiletime/components_component_from_debug.rs" );

  }

}

//

// use std::process::{ Child, Command, Stdio };
//
// pub fn process_run() -> Result< Child, Box< dyn std::error::Error > >
// {
//
//   let output = Command::new( "cargo" )
//   .arg( "build" )
//   .output()?;
//
//   if !output.status.success()
//   {
//     let error_message = String::from_utf8_lossy( &output.stderr );
//     let error = std::io::Error::other( format!( "Compilation failed: {}", error_message ) );
//     return Err( error.into() );
//   }
//
//   // Run the application
//   let exe_path = "./target/debug/app";
//   let mut child = Command::new( exe_path )
//   .stdout( Stdio::piped() )
//   .stderr( Stdio::piped() )
//   .spawn()?;
//
//   return Ok( child );
//
// //   let output_result = child.wait_with_output().expect( "Failed to read stdout/stderr" );
// //
// //   // Check if the application failed
// //   if output_result.status.success()
// //   {
// //     println!( "Application ran successfully." );
// //   }
// //   else
// //   {
// //     // The application has failed, process the output
// //     let stderr = String::from_utf8_lossy( &output_result.stderr );
// //     if stderr.contains( "thread 'main' panicked at" )
// //     {
// //       println!( "Application panicked. Stacktrace:" );
// //       println!( "{}", stderr );
// //     }
// //     else
// //     {
// //       println!( "Application failed without a panic. Output:" );
// //       println!( "{}", stderr );
// //     }
// //   }
//
// }

// pub fn process_run() -> std::result::Result< std::process::Child, Box< dyn std::error::Error > >
// {
//   // Create a temporary directory
//   let temp_dir = tempdir::TempDir::new( "my_build" )?;
//   let temp_path = temp_dir.path();
//
//   // Compile the application within the temporary directory
//   let output = std::process::Command::new( "cargo" )
//   .arg( "build" )
//   .current_dir( &temp_path ) // Set the current directory to the temp directory
//   .output()?;
//
//   if !output.status.success()
//   {
//     let error_message = String::from_utf8_lossy( &output.stderr );
//     let error = std::io::Error::new( std::io::ErrorKind::Other, format!( "Compilation failed: {}", error_message ) );
//     return Err( Box::new( error ) );
//   }
//
//   // Assuming the build outputs to the default target directory, adjust the executable path accordingly
//   let exe_path = temp_path.join( "target/debug/app" );
//   let child = std::process::Command::new( exe_path )
//   .stdout( std::process::Stdio::piped() )
//   .stderr( std::process::Stdio::piped() )
//   .spawn()?;
//
//   Ok( child )
// }
