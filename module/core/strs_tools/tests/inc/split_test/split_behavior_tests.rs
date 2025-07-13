//! ## Test Matrix for `SplitFlags`
//!
//! This matrix outlines the test cases for the custom `SplitFlags` implementation,
//! ensuring it behaves correctly as a bitflag-like type.
//!
//! **Test Factors:**
//! - Flag combination: Individual flags, combined flags, no flags.
//! - Operations: `contains`, `insert`, `remove`, `bitor`, `bitand`, `not`, `from_bits`, `bits`.
//! - Edge cases: Empty flags, all flags.
//!
//! **Test Combinations:**
//!
//! | ID    | Aspect Tested                               | Initial Flags | Operation           | Other Flags / Value | Expected Result / State |
//! |-------|---------------------------------------------|---------------|---------------------|---------------------|-------------------------|
//! | T2.1  | `contains` - single flag                    | `PRESERVING_EMPTY` | `contains`          | `PRESERVING_EMPTY`  | `true`                  |
//! | T2.2  | `contains` - single flag, not contained     | `PRESERVING_EMPTY` | `contains`          | `STRIPPING`         | `false`                 |
//! | T2.3  | `contains` - combined flags                 | `PRESERVING_EMPTY \| STRIPPING` | `contains`          | `PRESERVING_EMPTY`  | `true`                  |
//! | T2.4  | `contains` - combined flags, not fully contained | `PRESERVING_EMPTY` | `contains`          | `PRESERVING_EMPTY \| STRIPPING` | `false`                 |
//! | T2.5  | `insert` - add new flag                     | `PRESERVING_EMPTY` | `insert`            | `STRIPPING`         | `PRESERVING_EMPTY \| STRIPPING` |
//! | T2.6  | `insert` - add existing flag                | `PRESERVING_EMPTY` | `insert`            | `PRESERVING_EMPTY`  | `PRESERVING_EMPTY`      |
//! | T2.7  | `remove` - remove existing flag             | `PRESERVING_EMPTY \| STRIPPING` | `remove`            | `STRIPPING`         | `PRESERVING_EMPTY`      |
//! | T2.8  | `remove` - remove non-existing flag         | `PRESERVING_EMPTY` | `remove`            | `STRIPPING`         | `PRESERVING_EMPTY`      |
//! | T2.9  | `bitor` - combine flags                     | `PRESERVING_EMPTY` | `bitor`             | `STRIPPING`         | `PRESERVING_EMPTY \| STRIPPING` |
//! | T2.10 | `bitand` - intersect flags                  | `PRESERVING_EMPTY \| STRIPPING` | `bitand`            | `PRESERVING_EMPTY`  | `PRESERVING_EMPTY`      |
//! | T2.11 | `not` - invert flags                        | `PRESERVING_EMPTY` | `not`               | N/A                 | All flags except `PRESERVING_EMPTY` |
//! | T2.12 | `from_bits` and `bits`                      | N/A           | `from_bits(value).bits()` | `0b00010101`        | `0b00010101`            |
//! | T2.13 | Default value                               | N/A           | Default             | N/A                 | `SplitFlags(0)`         |
//! | T2.14 | `from` `u8`                                 | N/A           | `from(u8)`          | `0b11111`           | `SplitFlags(0b11111)`   |
//! | T2.15 | `into` `u8`                                 | `PRESERVING_EMPTY` | `into<u8>()`        | N/A                 | `1`                     |

use strs_tools::string::split::SplitFlags;
use std::ops::{ BitOr, BitAnd, Not };

/// Tests `contains` method with a single flag.
/// Test Combination: T2.1
#[test]
fn test_contains_single_flag()
{
    let flags = SplitFlags::PRESERVING_EMPTY;
    assert!(flags.contains(SplitFlags::PRESERVING_EMPTY));
}

/// Tests `contains` method with a single flag not contained.
/// Test Combination: T2.2
#[test]
fn test_contains_single_flag_not_contained()
{
    let flags = SplitFlags::PRESERVING_EMPTY;
    assert!(!flags.contains(SplitFlags::STRIPPING));
}

/// Tests `contains` method with combined flags.
/// Test Combination: T2.3
#[test]
fn test_contains_combined_flags()
{
    let flags = SplitFlags::PRESERVING_EMPTY | SplitFlags::STRIPPING;
    assert!(flags.contains(SplitFlags::PRESERVING_EMPTY));
}

/// Tests `contains` method with combined flags not fully contained.
/// Test Combination: T2.4
#[test]
fn test_contains_combined_flags_not_fully_contained()
{
    let flags = SplitFlags::PRESERVING_EMPTY;
    assert!(!flags.contains(SplitFlags::PRESERVING_EMPTY | SplitFlags::STRIPPING));
}

/// Tests `insert` method to add a new flag.
/// Test Combination: T2.5
#[test]
fn test_insert_new_flag()
{
    let mut flags = SplitFlags::PRESERVING_EMPTY;
    flags.insert(SplitFlags::STRIPPING);
    assert_eq!(flags, SplitFlags::PRESERVING_EMPTY | SplitFlags::STRIPPING);
}

/// Tests `insert` method to add an existing flag.
/// Test Combination: T2.6
#[test]
fn test_insert_existing_flag()
{
    let mut flags = SplitFlags::PRESERVING_EMPTY;
    flags.insert(SplitFlags::PRESERVING_EMPTY);
    assert_eq!(flags, SplitFlags::PRESERVING_EMPTY);
}

/// Tests `remove` method to remove an existing flag.
/// Test Combination: T2.7
#[test]
fn test_remove_existing_flag()
{
    let mut flags = SplitFlags::PRESERVING_EMPTY | SplitFlags::STRIPPING;
    flags.remove(SplitFlags::STRIPPING);
    assert_eq!(flags, SplitFlags::PRESERVING_EMPTY);
}

/// Tests `remove` method to remove a non-existing flag.
/// Test Combination: T2.8
#[test]
fn test_remove_non_existing_flag()
{
    let mut flags = SplitFlags::PRESERVING_EMPTY;
    flags.remove(SplitFlags::STRIPPING);
    assert_eq!(flags, SplitFlags::PRESERVING_EMPTY);
}

/// Tests `bitor` operator to combine flags.
/// Test Combination: T2.9
#[test]
fn test_bitor_operator()
{
    let flags = SplitFlags::PRESERVING_EMPTY | SplitFlags::STRIPPING;
    assert_eq!(flags, SplitFlags(0b00001001));
}

/// Tests `bitand` operator to intersect flags.
/// Test Combination: T2.10
#[test]
fn test_bitand_operator()
{
    let flags = (SplitFlags::PRESERVING_EMPTY | SplitFlags::STRIPPING) & SplitFlags::PRESERVING_EMPTY;
    assert_eq!(flags, SplitFlags::PRESERVING_EMPTY);
}

/// Tests `not` operator to invert flags.
/// Test Combination: T2.11
#[test]
fn test_not_operator()
{
    let flags = !SplitFlags::PRESERVING_EMPTY;
    // Assuming all 5 flags are the only relevant bits, the inverted value should be
    // 0b11111 (all flags) XOR 0b00001 (PRESERVING_EMPTY) = 0b11110
    let expected_flags = SplitFlags::PRESERVING_DELIMITERS | SplitFlags::PRESERVING_QUOTING | SplitFlags::STRIPPING | SplitFlags::QUOTING;
    assert_eq!(flags.0 & 0b11111, expected_flags.0); // Mask to only relevant bits
}

/// Tests `from_bits` and `bits` methods.
/// Test Combination: T2.12
#[test]
fn test_from_bits_and_bits()
{
    let value = 0b00010101;
    let flags = SplitFlags::from_bits(value).unwrap();
    assert_eq!(flags.bits(), value);
}

/// Tests the default value of `SplitFlags`.
/// Test Combination: T2.13
#[test]
fn test_default_value()
{
    let flags = SplitFlags::default();
    assert_eq!(flags.0, 0);
}

/// Tests `From<u8>` implementation.
/// Test Combination: T2.14
#[test]
fn test_from_u8()
{
    let flags: SplitFlags = 0b11111.into();
    assert_eq!(flags.0, 0b11111);
}

/// Tests `Into<u8>` implementation.
/// Test Combination: T2.15
#[test]
fn test_into_u8()
{
    let flags = SplitFlags::PRESERVING_EMPTY;
    let value: u8 = flags.into();
    assert_eq!(value, 1);
}