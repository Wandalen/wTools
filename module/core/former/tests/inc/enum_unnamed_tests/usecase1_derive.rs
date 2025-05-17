use super::*;
use former::Former;

// Define the inner structs that the enum variants will hold.
// These need to derive Former themselves if you want to build them easily.
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Prompt { pub content: String }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition: bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct InstructionsApplyToFiles { pub instruction: String }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command: String }

// Derive Former on the enum.
// By default, this should generate subformer starter methods for each variant.
// #[ debug ]
#[derive(Debug, Clone, PartialEq, Former)]
pub enum FunctionStep
{
  Prompt(Prompt),
  Break(Break),
  InstructionsApplyToFiles(InstructionsApplyToFiles),
  Run(Run),
}

include!("usecase1_only_test.rs");