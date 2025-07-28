#![allow(dead_code)]

use super::*;

/// Child
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[derive(Debug, Default, PartialEq, the_module::Former)]
#[derive(Debug, Default, PartialEq)]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Debug, Default, PartialEq, the_module::Former)]

#[derive(Debug, Default, PartialEq)]
// #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent {
  #[subform_entry(setter = false)]
  children: Vec<Child>,
}

impl<Definition> ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = <Parent as former::EntityToStorage>::Storage>,
{
  #[inline(always)]
  pub fn child(self, name: &str) -> ChildAsSubformer<Self, impl ChildAsSubformerEnd<Self>> {
    self._children_subform_entry::<ChildFormer<_>, _>().name(name)
  }

  #[inline(always)]
  pub fn _child(self) -> ChildAsSubformer<Self, impl ChildAsSubformerEnd<Self>> {
    self._children_subform_entry::<<Child as former::EntityToFormer<_>>::Former, _>()
  }
}

// == begin of generated

// == end of generated

include!("./only_test/subform_entry_child.rs");
