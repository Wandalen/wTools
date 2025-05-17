//! Purpose: Provides a manual implementation of constructors for an enum with unit variants,
//! including static methods and standalone functions, to serve as a reference for verifying
//! the `#[derive(Former)]` macro's behavior.
//!
//! Coverage:
//! - Rule 3a (Unit + Default): Manual implementation of static methods `Status::pending()` and `Status::complete()`.
//! - Rule 1a (Unit + `#[scalar]`): Manual implementation of static methods (as default for unit is scalar).
//! - Rule 4a (#[standalone_constructors]): Manual implementation of standalone functions `pending()` and `complete()`.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `Status` with unit variants `Pending` and `Complete`.
//! - Manually implements static methods (`Status::pending()`, `Status::complete()`) and standalone functions (`pending()`, `complete()`) that mirror the expected generated code.
//! - This file is included by `unit_variant_only_test.rs` to provide the manual implementations that the shared tests compare against.
use super::*;

/// Enum with only unit variants for testing.
#[derive(Debug, PartialEq)]
pub enum Status // Made enum public
{
  Pending, // Variants are public by default if enum is public
  Complete,
}

// Manual implementation of static constructors
impl Status
{
  #[inline(always)]
  pub fn pending() -> Self
  {
    Self::Pending
  }

  #[inline(always)]
  pub fn complete() -> Self
  {
    Self::Complete
  }
}

// Manual implementation of standalone constructors (moved before include!)
#[inline(always)]
pub fn pending() -> Status
{
  Status::Pending
}

#[inline(always)]
pub fn complete() -> Status
{
  Status::Complete
}

// Include the test logic (now defined after standalone constructors)
include!("unit_variant_only_test.rs");