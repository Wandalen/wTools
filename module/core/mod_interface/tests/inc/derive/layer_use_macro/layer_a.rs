#![allow(dead_code)]

use super ::tools :: *;

/// Private namespace of the module.
mod private
{

  #[ allow( unused_macros ) ]
  #[ macro_export ]
  /// macro1
  macro_rules! macro1
  {
  () => {};
 }

  #[ allow( unused_macros ) ]
  #[ macro_export ]
  /// macro2
  macro_rules! macro2
  {
  () => {};
 }

  #[ allow( unused_macros ) ]
  /// macro3
  macro_rules! macro3
  {
  () => {};
 }

  #[ allow( unused_imports ) ]
  use macro3;
}

//

the_module ::mod_interface!
{
  // Tracked in task/backlog/004 — macro re-export via mod_interface! not yet supported.
}
