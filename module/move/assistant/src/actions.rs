//!
//! CLI actions of the tool.
//!

mod private {}

crate::mod_interface!
{
  layer openai;
  layer openai_assistants_list;
  layer openai_files_list;
  layer openai_runs_list;
}
