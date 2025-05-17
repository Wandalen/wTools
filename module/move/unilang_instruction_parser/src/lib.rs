//! `unilang_instruction_parser` is a crate for parsing unilang CLI syntax.
//!
//! It takes string input (either a single `&str` or a slice `&[&str]`) and
//! produces a vector of `GenericInstruction`s, representing the parsed commands
//! and their arguments. The parser is designed to provide precise, location-aware
//! error reporting.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
// #![deny(unsafe_code)] // Not strictly needed for this crate yet, but good practice.

pub mod config;
pub mod error;
pub mod instruction;
pub mod parser_engine;

pub use config::UnilangParserOptions;
pub use error::{ParseError, ErrorKind, SourceLocation};
pub use instruction::{Argument, GenericInstruction};
pub use parser_engine::Parser;
