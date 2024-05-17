
#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
#[ path = "." ]
mod if_enabled
{

  use super::*;
  use the_module::exposed::*;

  mod attr;
  mod basic;
  mod derive;
  mod drop;
  mod generic_args;
  mod generic_params;
  mod item;
  mod item_struct;
  mod phantom;
  mod quantifier;
  mod struct_like;
  mod syntax;
  mod tokens;
  mod typ;

}
