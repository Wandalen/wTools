//! Health checks for unilang projects
//!
//! Detects common mistakes and anti-patterns.

mod build_rs;
mod deps;
mod api;

pub use build_rs::*;
pub use deps::*;
pub use api::*;

/// Result of a health check
#[derive( Debug, Clone )]
pub struct CheckResult
{
  pub passed : bool,
  pub issue_type : String,
  pub location : String,
  pub issue : String,
  pub fix : String,
  pub docs_url : String,
}

impl CheckResult
{
  pub fn passed() -> Self
  {
    Self
    {
      passed : true,
      issue_type : String::new(),
      location : String::new(),
      issue : String::new(),
      fix : String::new(),
      docs_url : String::new(),
    }
  }

  pub fn failed( issue_type : impl Into< String >, location : impl Into< String >, issue : impl Into< String >, fix : impl Into< String >, docs_url : impl Into< String > ) -> Self
  {
    Self
    {
      passed : false,
      issue_type : issue_type.into(),
      location : location.into(),
      issue : issue.into(),
      fix : fix.into(),
      docs_url : docs_url.into(),
    }
  }
}
