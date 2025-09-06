//! ## Test Matrix for `ErrWith` Trait Coverage
//!
//! | ID   | Scenario                               | Expected Behavior                               |
//! |------|----------------------------------------|-------------------------------------------------|
//! | T8.1 | `err_with` on `Ok` result              | Returns `Ok` with original value                |
//! | T8.2 | `err_with` on `Err` result             | Returns `Err` with custom report and original error |
//! | T8.3 | `err_with_report` on `Ok` result       | Returns `Ok` with original value                |
//! | T8.4 | `err_with_report` on `Err` result      | Returns `Err` with cloned report and original error |
//! | T8.5 | `ResultWithReport` type alias usage    | Correctly defines a Result with tuple error     |
//!
use super::*;
use test_tools::ErrWith;
use test_tools::error_tools::ResultWithReport;
use std::io;

/// Tests `err_with` on an `Ok` result.
/// Test Combination: T8.1
#[ test ]
fn test_err_with_on_ok() {
  let result: core::result::Result<u32, io::Error> = core::result::Result::Ok(10);
  let processed: core::result::Result<u32, (String, io::Error)> = result.err_with(|| "context".to_string());
  assert!(processed.is_ok());
  assert_eq!(processed.unwrap(), 10);
}

/// Tests `err_with` on an `Err` result.
/// Test Combination: T8.2
#[ test ]
fn test_err_with_on_err() {
  let error = io::Error::new(io::ErrorKind::NotFound, "file not found");
  let result: core::result::Result<u32, io::Error> = core::result::Result::Err(error);
  let processed: core::result::Result<u32, (String, io::Error)> = result.err_with(|| "custom report".to_string());
  assert_eq!(
    processed.map_err(|(r, e): (String, io::Error)| (r, e.kind(), e.to_string())),
    core::result::Result::Err((
      "custom report".to_string(),
      io::ErrorKind::NotFound,
      "file not found".to_string()
    ))
  );
}

/// Tests `err_with_report` on an `Ok` result.
/// Test Combination: T8.3
#[ test ]
fn test_err_with_report_on_ok() {
  let result: core::result::Result<u32, io::Error> = core::result::Result::Ok(20);
  let report = "fixed report".to_string();
  let processed: core::result::Result<u32, (String, io::Error)> = result.err_with_report(&report);
  assert!(processed.is_ok());
  assert_eq!(processed.unwrap(), 20);
}

/// Tests `err_with_report` on an `Err` result.
/// Test Combination: T8.4
#[ test ]
fn test_err_with_report_on_err() {
  let error = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
  let result: core::result::Result<u32, io::Error> = core::result::Result::Err(error);
  let report = "security issue".to_string();
  let processed: core::result::Result<u32, (String, io::Error)> = result.err_with_report(&report);
  assert_eq!(
    processed.map_err(|(r, e): (String, io::Error)| (r, e.kind(), e.to_string())),
    core::result::Result::Err((
      "security issue".to_string(),
      io::ErrorKind::PermissionDenied,
      "access denied".to_string()
    ))
  );
}

/// Tests `ResultWithReport` type alias usage.
/// Test Combination: T8.5
#[ test ]
fn test_result_with_report_alias() {
  type MyResult = ResultWithReport<String, io::Error>;
  let ok_val: MyResult = core::result::Result::Ok("30".to_string());
  assert!(ok_val.is_ok());
  if let Ok(val) = ok_val {
    assert_eq!(val, "30".to_string());
  }

  let err_val: MyResult =
    core::result::Result::Err(("report".to_string(), io::Error::new(io::ErrorKind::BrokenPipe, "pipe broken")));
  assert_eq!(
    err_val.map_err(|(r, e): (String, io::Error)| (r, e.kind(), e.to_string())),
    core::result::Result::Err(("report".to_string(), io::ErrorKind::BrokenPipe, "pipe broken".to_string()))
  );
}
