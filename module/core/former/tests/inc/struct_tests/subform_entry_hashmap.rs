#![allow(dead_code)]

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use collection_tools::HashMap;

// Child struct with Former derived for builder pattern support
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, former::Former)]
#[derive(Debug, PartialEq)]
pub struct Child {
  name: String,
  description: String,
}

// Parent struct to hold commands
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, PartialEq, former::Former)]
#[derive(Debug, PartialEq)]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Parent {
  #[subform_entry]
  command: HashMap<String, Child>,
}

impl former::ValToEntry<HashMap<String, Child>> for Child {
  type Entry = (String, Child);
  #[inline(always)]
  fn val_to_entry(self) -> Self::Entry {
    (self.name.clone(), self)
  }
}

// == begin of generated

// == end of generated

#[test]
fn basic() {
  let got = Parent::former()
  .command()
    .name( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command()
    .name( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  a_id!(got.command.len(), 2);
}
