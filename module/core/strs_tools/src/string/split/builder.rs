//! Builder types and factory functions for string splitting.

#[ cfg( feature = "std" ) ]
use std::{ vec, vec::Vec };
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
use alloc::{ vec, vec::Vec };
#[ cfg( all( feature = "string_parse_request", feature = "std" ) ) ]
use crate::string::parse_request::OpType;
use super::SplitFlags;
use super::types::{ Split, Searcher, SplitOptions };
use super::iterator::{ SplitFastIterator, SplitIterator };

impl<'a> SplitOptions<'a, Vec< &'a str >> {
  /// Consumes the options and returns a `SplitIterator`.
  #[ must_use ]
  pub fn split(self) -> SplitIterator<'a> {
    SplitIterator::new(&self)
  }
}

impl<'a, D: Searcher + Default + Clone> SplitOptions<'a, D> {
  /// Consumes the options and returns a `SplitFastIterator`.
  #[ allow( dead_code ) ]
  pub( super ) fn split_fast(self) -> SplitFastIterator<'a, D> {
    SplitFastIterator::new(&self)
  }
}
impl<'a> core::iter::IntoIterator for SplitOptions<'a, Vec< &'a str >> {
  type Item = Split<'a>;
  type IntoIter = SplitIterator<'a>;

  fn into_iter(self) -> Self::IntoIter {
    SplitIterator::new(&self)
  }
}

/// Basic builder for creating simple `SplitOptions` without `OpType` dependency.
#[ derive( Debug ) ]
pub struct BasicSplitBuilder<'a> {
  src: &'a str,
  delimiters: Vec<&'a str>,
  flags: SplitFlags,
  quoting_prefixes: Vec<&'a str>,
  quoting_postfixes: Vec<&'a str>,
}

impl<'a> Default for BasicSplitBuilder<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> BasicSplitBuilder<'a> {
  /// Creates a new `BasicSplitBuilder`.
  #[ must_use ]
  pub fn new() -> BasicSplitBuilder<'a> {
    Self {
      src: "",
      delimiters: vec![],
      flags: SplitFlags::PRESERVING_DELIMITERS, // Default
      quoting_prefixes: vec![],
      quoting_postfixes: vec![],
    }
  }

  /// Sets the source string to split.
  pub fn src(&mut self, value: &'a str) -> &mut Self {
    self.src = value;
    self
  }

  /// Sets a single delimiter.
  pub fn delimiter(&mut self, value: &'a str) -> &mut Self {
    self.delimiters = vec![value];
    self
  }

  /// Sets multiple delimiters.
  pub fn delimiters(&mut self, value: &[&'a str]) -> &mut Self {
    self.delimiters = value.to_vec();
    self
  }

  /// Sets quoting behavior.
  pub fn quoting(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::QUOTING);
      // Set default quoting characters if not already set
      if self.quoting_prefixes.is_empty() {
        self.quoting_prefixes = vec!["\"", "'"];
      }
      if self.quoting_postfixes.is_empty() {
        self.quoting_postfixes = vec!["\"", "'"];
      }
    } else {
      self.flags.remove(SplitFlags::QUOTING);
    }
    self
  }

  /// Sets stripping behavior.
  pub fn stripping(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::STRIPPING);
    } else {
      self.flags.remove(SplitFlags::STRIPPING);
    }
    self
  }

  /// Sets whether to preserve empty segments.
  pub fn preserving_empty(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::PRESERVING_EMPTY);
    } else {
      self.flags.remove(SplitFlags::PRESERVING_EMPTY);
    }
    self
  }

  /// Sets whether to preserve delimiters in output.
  pub fn preserving_delimiters(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::PRESERVING_DELIMITERS);
    } else {
      self.flags.remove(SplitFlags::PRESERVING_DELIMITERS);
    }
    self
  }

  /// Sets whether to preserve quoting in output.
  pub fn preserving_quoting(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::PRESERVING_QUOTING);
    } else {
      self.flags.remove(SplitFlags::PRESERVING_QUOTING);
    }
    self
  }

  /// Sets quoting prefixes.
  pub fn quoting_prefixes(&mut self, value: &[&'a str]) -> &mut Self {
    self.quoting_prefixes = value.to_vec();
    self
  }

  /// Sets quoting postfixes.
  pub fn quoting_postfixes(&mut self, value: &[&'a str]) -> &mut Self {
    self.quoting_postfixes = value.to_vec();
    self
  }

  /// Performs the split operation and returns a `SplitIterator`.
  pub fn perform(&mut self) -> SplitIterator<'a> {
    let options = SplitOptions {
      src: self.src,
      delimiter: self.delimiters.clone(),
      flags: self.flags,
      quoting_prefixes: self.quoting_prefixes.clone(),
      quoting_postfixes: self.quoting_postfixes.clone(),
    };
    options.split()
  }

  /// Attempts to create a SIMD-optimized iterator when simd feature is enabled.
  #[ cfg( feature = "simd" ) ]
  pub fn perform_simd(&mut self) -> SplitIterator<'a> {
    // For now, just use regular perform - SIMD integration needs more work
    self.perform()
  }
  
  /// Attempts to create a SIMD-optimized iterator - fallback version when simd feature is disabled.
  #[ cfg( not( feature = "simd" ) ) ]
  pub fn perform_simd(&mut self) -> SplitIterator<'a> {
    self.perform()
  }
}

/// Former (builder) for creating `SplitOptions`.
// This lint is addressed by using SplitFlags
#[ cfg( all( feature = "string_parse_request", feature = "std" ) ) ]
#[ derive( Debug ) ]
pub struct SplitOptionsFormer<'a> {
  src: &'a str,
  delimiter: OpType<&'a str>,
  flags: SplitFlags,
  quoting_prefixes: Vec< &'a str >,
  quoting_postfixes: Vec< &'a str >,
}

#[ cfg( all( feature = "string_parse_request", feature = "std" ) ) ]
impl<'a> SplitOptionsFormer<'a> {
  /// Initializes builder with delimiters to support fluent configuration of split options.
  pub fn new<D: Into<OpType<&'a str>>>(delimiter: D) -> SplitOptionsFormer<'a> {
    Self {
      src: "",
      delimiter: OpType::Vector(vec![]).append(delimiter.into()),
      flags: SplitFlags::PRESERVING_DELIMITERS, // Default
      quoting_prefixes: vec![],
      quoting_postfixes: vec![],
    }
  }
  /// Controls empty segment handling to accommodate different parsing requirements.
  pub fn preserving_empty(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::PRESERVING_EMPTY);
    } else {
      self.flags.remove(SplitFlags::PRESERVING_EMPTY);
    }
    self
  }
  /// Controls delimiter preservation to support scenarios needing delimiter tracking.
  pub fn preserving_delimiters(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::PRESERVING_DELIMITERS);
    } else {
      self.flags.remove(SplitFlags::PRESERVING_DELIMITERS);
    }
    self
  }
  /// Controls quote character preservation for maintaining original format integrity.
  pub fn preserving_quoting(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::PRESERVING_QUOTING);
    } else {
      self.flags.remove(SplitFlags::PRESERVING_QUOTING);
    }
    self
  }
  /// Controls whitespace trimming to support clean data extraction scenarios.
  pub fn stripping(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::STRIPPING);
    } else {
      self.flags.remove(SplitFlags::STRIPPING);
    }
    self
  }
  /// Enables quote-aware splitting to handle complex strings with embedded delimiters.
  pub fn quoting(&mut self, value: bool) -> &mut Self {
    if value {
      self.flags.insert(SplitFlags::QUOTING);
    } else {
      self.flags.remove(SplitFlags::QUOTING);
    }
    self
  }
  /// Configures quote start markers to support custom quotation systems.
  pub fn quoting_prefixes(&mut self, value: Vec< &'a str >) -> &mut Self {
    self.quoting_prefixes = value;
    self
  }
  /// Configures quote end markers to support asymmetric quotation systems.
  pub fn quoting_postfixes(&mut self, value: Vec< &'a str >) -> &mut Self {
    self.quoting_postfixes = value;
    self
  }
  /// Provides input string to enable convenient chained configuration.
  pub fn src(&mut self, value: &'a str) -> &mut Self {
    self.src = value;
    self
  }
  /// Sets the delimiter(s) to use for splitting.
  pub fn delimiter<D: Into<OpType<&'a str>>>(&mut self, value: D) -> &mut Self {
    self.delimiter = OpType::Vector(vec![]).append(value.into());
    self
  }
  /// Consumes the former and returns configured `SplitOptions`.
  ///
  /// # Panics
  /// Panics if `delimiter` field contains an `OpType::Primitive(None)` which results from `<&str>::default()`,
  /// and `vector()` method on `OpType` is not robust enough to handle it (currently it would unwrap a None).
  pub fn form(&mut self) -> SplitOptions<'a, Vec< &'a str >> {
    if self.flags.contains(SplitFlags::QUOTING) {
      if self.quoting_prefixes.is_empty() {
        self.quoting_prefixes = vec!["\"", "`", "'"];
      }
      if self.quoting_postfixes.is_empty() {
        self.quoting_postfixes = vec!["\"", "`", "'"];
      }
    }
    SplitOptions {
      src: self.src,
      delimiter: self.delimiter.clone().vector().unwrap(),
      flags: self.flags,
      quoting_prefixes: self.quoting_prefixes.clone(),
      quoting_postfixes: self.quoting_postfixes.clone(),
    }
  }
  /// Consumes the former, builds `SplitOptions`, and returns a `SplitIterator`.
  pub fn perform(&mut self) -> SplitIterator<'a> {
    self.form().split()
  }
  
  /// Attempts to create a SIMD-optimized iterator when the simd feature is enabled.
  /// Falls back to the regular iterator if SIMD is not available or fails.
  #[ cfg( feature = "simd" ) ]
  pub fn perform_simd(&mut self) -> SplitIterator<'a> {
    // Try SIMD first for multi-delimiter cases
    if let OpType::Vector(ref delims) = self.delimiter {
      if delims.len() > 1 {
        // For multi-delimiter splitting, SIMD provides significant benefits
        if let Ok(_simd_iter) = super::simd_split_cached(self.src, delims) {
          // TODO: Bridge SIMD iterator with standard format for performance optimization
          return self.perform(); // For now, fallback to regular - we'll enhance this
        }
        // SIMD failed, use regular implementation
      }
    }
    
    // Fallback to regular splitting
    self.perform()
  }
  
  /// Attempts to create a SIMD-optimized iterator - fallback version when simd feature is disabled.
  #[ cfg( not( feature = "simd" ) ) ]
  pub fn perform_simd(&mut self) -> SplitIterator<'a> {
    self.perform()
  }
}
/// Creates a basic split iterator builder for string splitting functionality.
/// This is the main entry point for using basic string splitting.
#[ must_use ]
pub fn split<'a>() -> BasicSplitBuilder<'a> {
  BasicSplitBuilder::new()
}

/// Creates a new `SplitOptionsFormer` to build `SplitOptions` for splitting a string.
/// This is the main entry point for using advanced string splitting functionality.
#[ cfg( all( feature = "string_parse_request", feature = "std" ) ) ]
#[ must_use ]
pub fn split_advanced<'a>() -> SplitOptionsFormer<'a> {
  SplitOptionsFormer::new(<&str>::default())
}
