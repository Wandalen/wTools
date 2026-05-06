//! Demonstrates building an execution plan and running it.

fn main()
{
  use program_tools::prelude::*;

  // Build a multi-file program: main.rs calls a helper in lib.rs.
  let plan = Plan::former()
    .program()
      .source()
        .file_path( "src/main.rs" )
        .data( "fn main() { lib::greet(); }" )
        .end()
      .source()
        .file_path( "src/lib.rs" )
        .data( r#"pub fn greet() { println!( "hello from script" ); }"# )
        .end()
      .end()
    .form();

  // Execute the plan and inspect captured output.
  let output = run( plan ).expect( "failed to run script" );
  println!( "exit : {}", output.exit_status );
  println!( "stdout : {}", output.stdout_str() );
}
