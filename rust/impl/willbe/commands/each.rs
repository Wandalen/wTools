/// Internal namespace.
pub( crate ) mod private
{
  use std::env;
  use wtools::error::BasicError;
  use wca::
  {
    Args,
    NoProperties, Context,
  };

  use crate::protected::*;
  use wca::InstructionParser;

  ///
  /// Iterate over subject
  ///

  pub fn each( args : Args< String, NoProperties >, ctx : Context ) -> Result< (), BasicError >
  {
    let ctx : &State = ctx.get_ref().unwrap();

    let current_path = env::current_dir().unwrap();

    println!( "[LOG] Called each command" );

    println!( "context: {:#?}\nargs: {:?}", &ctx, &args );

    let parser = wca::instruction::DefaultInstructionParser::former().form();

    let routine = ctx[&args.subject].routine.clone();
    packages_iterate(current_path)
    .into_iter()
    .for_each( | package |
    {
      env::set_current_dir( package.path() ).unwrap();
      routine.perform
      (
        &parser.parse( &args.subject ).unwrap(),
        Some( wca::Context::new( package.path().to_owned() ) )
      )
      .ok();
    });

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use each;
}
