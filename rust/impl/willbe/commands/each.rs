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
          info.name, info.version, info.dependencies, info.location.display()
        )
      }),
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
