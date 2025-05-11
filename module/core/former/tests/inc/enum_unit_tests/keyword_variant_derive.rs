use former::Former; // Ensure derive is in scope

#[derive(Debug, PartialEq, Former)]
pub enum KeywordTestMin {
  r#fn,
}
// No include, no other attributes for now