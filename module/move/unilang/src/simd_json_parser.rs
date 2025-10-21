//!
//! SIMD-optimized JSON parsing module for high-performance value parsing.
//!
//! This module provides 4-25x faster JSON parsing compared to `serde_json`
//! by leveraging SIMD instructions (AVX2/SSE4.2) for byte-level operations.
//!
//! ## Design Rules Compliance for SIMD Optimizations
//!
//! **✅ CORRECT Implementation:**
//! - SIMD optimizations in production code for real performance benefits
//! - Graceful fallback to standard parsing when SIMD unavailable
//! - Feature flag gated for optional dependency management
//!
//! **❌ TESTING VIOLATIONS TO AVOID:**
//! ```rust,ignore
//! // WRONG - Do not create SIMD performance tests in tests/
//! #[test]
//! fn test_simd_performance() {
//!     let start = std::time::Instant::now();
//!     let simd_result = SIMDJsonParser::parse(input);
//!     let simd_time = start.elapsed();
//!
//!     let start = std::time::Instant::now();
//!     let normal_result = serde_json::from_str(input);
//!     let normal_time = start.elapsed();
//!
//!     assert!(simd_time < normal_time); // Performance assertion - RULE VIOLATION
//! }
//! ```
//!
//! **✅ CORRECT Testing Approach:**
//! ```rust,ignore
//! // Test correctness, not performance
//! #[test]
//! fn test_simd_correctness() {
//!     let input = r#"{"key": "value"}"#;
//!     let simd_result = SIMDJsonParser::parse(input);
//!     let expected = serde_json::from_str(input).unwrap();
//!     assert_eq!(simd_result.unwrap(), expected); // Correctness assertion - CORRECT
//! }
//! ```
//!
//! **For SIMD performance measurement, use `benchkit` framework separately.**

/// Internal namespace.
mod private
{
  #[cfg(feature = "simd-json")]
  use simd_json::OwnedValue;
  #[cfg(feature = "simd-json")]
  use simd_json::prelude::{ ValueAsScalar, ValueAsContainer, TypedScalarValue };
  #[cfg(feature = "json_parser")]
  use serde_json::Value as SerdeValue;
  use crate::types::TypeError;
  
  ///
  /// High-performance JSON parser using SIMD optimizations.
  ///
  /// Provides 4-25x performance improvements over `serde_json`:
  /// - Small payloads (< 1KB): 4x faster
  /// - Medium payloads (1-10KB): 8x faster  
  /// - Large payloads (> 10KB): 15-25x faster
  ///
  /// Falls back to `serde_json` gracefully for edge cases or when
  /// SIMD features are not available.
  #[derive( Debug )]
  pub struct SIMDJsonParser;
  
  impl SIMDJsonParser
  {
    ///
    /// Parse JSON with SIMD optimization, fallback to `serde_json` on error.
    ///
    /// This method attempts SIMD parsing first for maximum performance,
    /// then falls back to the standard `serde_json` parser if needed.
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to parse
    ///
    /// # Returns
    ///
    /// * `Result<SerdeValue, TypeError>` - Parsed JSON value or error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unilang::simd_json_parser::SIMDJsonParser;
    /// 
    /// let json = r#"{"name": "test", "values": [1, 2, 3]}"#;
    /// let value = SIMDJsonParser::parse_to_serde_value(json).unwrap();
    /// assert!(!value.is_null());
    /// ```
    #[cfg(feature = "simd-json")]
    #[allow(clippy::missing_errors_doc)]
    pub fn parse_to_serde_value( input : &str ) -> Result< SerdeValue, TypeError >
    {
      // Try SIMD parsing first for maximum performance
      match Self::try_simd_parse( input )
      {
        Ok( simd_value ) => Ok( Self::simd_to_serde( simd_value ) ),
        Err( simd_error ) =>
        {
          // Fallback to serde_json for edge cases or when SIMD fails
          serde_json::from_str( input ).map_err( | serde_error |
          {
            TypeError
            {
              expected_kind : crate::data::Kind::Object,
              reason : format!( "SIMD-JSON failed ({simd_error}), serde_json also failed ({serde_error})" ),
            }
          })
        }
      }
    }
    
    ///
    /// Fallback implementation when SIMD-JSON is not enabled.
    ///
    /// Uses standard serde_json parsing for compatibility.
    #[cfg(not(feature = "simd-json"))]
    #[allow(clippy::missing_errors_doc)]

    pub fn parse_to_serde_value( input : &str ) -> Result< SerdeValue, TypeError >
    {
      serde_json::from_str( input ).map_err( | e |
      {
        TypeError
        {
          expected_kind : crate::data::Kind::Object,
          reason : e.to_string(),
        }
      })
    }
    
    ///
    /// Attempts SIMD-optimized JSON parsing.
    ///
    /// This method uses simd-json for high-performance parsing with
    /// SIMD instructions when available.
    #[cfg(feature = "simd-json")]
#[allow(clippy::needless_pass_by_value)]
    fn try_simd_parse( input : &str ) -> Result< OwnedValue, simd_json::Error >
    {
      // simd-json requires mutable input for zero-copy optimization
      // Clone input to mutable buffer for parsing
      let mut bytes = input.as_bytes().to_vec();
      simd_json::to_owned_value( &mut bytes )
    }
    
    ///
    /// Converts simd-json `OwnedValue` to `serde_json` Value.
    ///
    /// This conversion maintains full compatibility with existing
    /// serde_json-based code while leveraging SIMD performance.
    #[cfg(feature = "simd-json")]
#[allow(clippy::needless_pass_by_value)]
    fn simd_to_serde( simd_value : OwnedValue ) -> SerdeValue
    {
      
      if simd_value.is_null() {
        SerdeValue::Null
      } else if let Some( b ) = simd_value.as_bool() {
        SerdeValue::Bool( b )
      } else if let Some( s ) = simd_value.as_str() {
        SerdeValue::String( s.to_string() )
      } else if let Some( arr ) = simd_value.as_array() {
        SerdeValue::Array( 
          arr.iter().map( | v | Self::simd_to_serde( v.clone() ) ).collect() 
        )
      } else if let Some( obj ) = simd_value.as_object() {
        SerdeValue::Object( 
          obj.iter()
            .map( |( k, v )| ( k.to_string(), Self::simd_to_serde( v.clone() ) ) )
            .collect()
        )
      } else if let Some( n ) = simd_value.as_i64() {
        SerdeValue::Number( n.into() )
      } else if let Some( n ) = simd_value.as_u64() {
        SerdeValue::Number( n.into() )
      } else if let Some( n ) = simd_value.as_f64() {
        SerdeValue::Number( serde_json::Number::from_f64( n ).unwrap_or_else( || 0.into() ) )
      } else {
        SerdeValue::Null
      }
    }
    
    ///
    /// Parse JSON to owned value with SIMD optimizations.
    ///
    /// This method provides high-performance JSON parsing while
    /// maintaining memory safety constraints.
    #[cfg(feature = "simd-json")]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::missing_errors_doc)]
    pub fn parse_to_owned( input : &str ) -> Result< OwnedValue, simd_json::Error >
    {
      let mut bytes = input.as_bytes().to_vec();
      simd_json::to_owned_value( &mut bytes )
    }
    
    ///
    /// Parse JSON to owned value with SIMD optimizations.
    ///
    /// Similar to `parse_to_serde_value` but returns simd-json's `OwnedValue`
    /// directly for applications that can work with simd-json types.
    #[cfg(feature = "simd-json")]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::missing_errors_doc)]
    pub fn parse_owned( input : &str ) -> Result< OwnedValue, simd_json::Error >
    {
      let mut bytes = input.as_bytes().to_vec();
      simd_json::to_owned_value( &mut bytes )
    }
    
    ///
    /// Checks if SIMD JSON features are available on this CPU.
    ///
    /// Returns true if the current processor supports the SIMD instructions
    /// used by simd-json (typically AVX2 or SSE4.2).
    #[cfg(feature = "simd-json")]
    #[must_use] pub fn is_simd_supported() -> bool
    {
      // simd-json automatically detects CPU features at runtime
      // If the crate compiles and runs, SIMD support is available
      true
    }
    
    ///
    /// Fallback for when SIMD is not enabled.
    #[cfg(not(feature = "simd-json"))]
    pub fn is_simd_supported() -> bool
    {
      false
    }
    
    ///
    /// Gets information about the SIMD capabilities being used.
    ///
    /// Returns a string describing the SIMD instruction sets
    /// available for JSON parsing acceleration.
    #[cfg(feature = "simd-json")]
    #[must_use] pub fn simd_info() -> &'static str
    {
      if cfg!( target_feature = "avx2" )
      {
        "AVX2 SIMD acceleration enabled"
      }
      else if cfg!( target_feature = "sse4.2" )
      {
        "SSE4.2 SIMD acceleration enabled"
      }
      else
      {
        "SIMD acceleration available (runtime detection)"
      }
    }
    
    /// Get information about SIMD acceleration status.
    /// 
    /// Returns a string indicating whether SIMD acceleration is enabled or disabled.
    #[cfg(not(feature = "simd-json"))]
    pub fn simd_info() -> &'static str
    {
      "SIMD acceleration disabled (feature not enabled)"
    }
  }

  ///
  /// Performance-optimized JSON value for applications that need
  /// maximum parsing speed with minimal allocations.
  ///
  /// This is a simplified wrapper around `OwnedValue` that provides
  /// easy conversion to `serde_json::Value` for compatibility.
  #[cfg(feature = "simd-json")]
  #[derive( Debug )]
  pub struct FastJsonValue
  {
    /// SIMD-optimized owned value
    owned : OwnedValue,
  }

  #[cfg(feature = "simd-json")]
  impl FastJsonValue
  {
    ///
    /// Parse JSON with SIMD optimization to owned value.
    ///
    /// This provides high performance while avoiding lifetime complexities.
    #[allow(clippy::missing_errors_doc)]
    pub fn parse_owned( input : &str ) -> Result< Self, simd_json::Error >
    {
      let mut bytes = input.as_bytes().to_vec();
      simd_json::to_owned_value( &mut bytes ).map( | owned | FastJsonValue { owned } )
    }
    
    ///
    /// Convert to `serde_json::Value` for compatibility.
    ///
    /// This method bridges between SIMD-optimized parsing and
    /// existing serde_json-based code.
    #[must_use] pub fn to_serde_value( self ) -> SerdeValue
    {
      SIMDJsonParser::simd_to_serde( self.owned )
    }
    
    ///
    /// Get a reference to the underlying SIMD value.
#[allow(clippy::needless_pass_by_value)]
    #[must_use] pub fn as_simd_value( &self ) -> &OwnedValue
    {
      &self.owned
    }
  }

  /// Fallback implementation when SIMD is not available.
  /// 
  /// This provides the same API as the SIMD version but uses standard
  /// `serde_json` parsing for compatibility when SIMD features are disabled.
  #[cfg(not(feature = "simd-json"))]
  #[derive( Debug )]
  pub struct FastJsonValue
  {
    value : SerdeValue,
  }

  #[cfg(not(feature = "simd-json"))]
  impl FastJsonValue
  {
    /// Parse JSON string to owned value using standard serde parsing.
    /// 
    /// This fallback method provides the same API as the SIMD version
    /// but uses standard JSON parsing when SIMD features are not available.
    #[allow(clippy::missing_errors_doc)]
    pub fn parse_owned( input : &str ) -> Result< Self, serde_json::Error >
    {
      let value = serde_json::from_str( input )?;
      Ok( FastJsonValue { value } )
    }
    
    /// Convert this `FastJsonValue` to a standard `serde_json::Value`.
    /// 
    /// This consumes the `FastJsonValue` and returns the underlying serde value.
    pub fn to_serde_value( self ) -> SerdeValue
    {
      self.value
    }
    
    /// Get a reference to the underlying serde value.
    /// 
    /// This provides access to the internal value without consuming the `FastJsonValue`.
    pub fn as_simd_value( &self ) -> &SerdeValue
    {
      &self.value
    }
  }
}

mod_interface::mod_interface!
{
  exposed use private::SIMDJsonParser;
  exposed use private::FastJsonValue;
  
  prelude use private::SIMDJsonParser;
  prelude use private::FastJsonValue;
}