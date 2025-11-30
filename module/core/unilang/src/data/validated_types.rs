//! Validated newtype wrappers for command metadata
//!
//! This module provides type-safe wrappers for command names, namespaces, and versions
//! that guarantee validity at construction time, following the "parse don't validate" pattern.
//!
//! ## Design Philosophy
//!
//! Invalid states are impossible to represent. All validation happens at construction time,
//! so if you have a `CommandName`, `NamespaceType`, or `VersionType` instance, you know
//! it's valid - no runtime checks needed.
//!
//! ## Key Types
//!
//! - `CommandName`: Guarantees dot prefix (e.g., `.build`)
//! - `NamespaceType`: Guarantees empty or dot-prefixed namespace
//! - `VersionType`: Guarantees non-empty version string

use crate::error::Error;

  ///
  /// A validated command name that guarantees the dot prefix convention.
  ///
  /// # Type Safety Guarantees
  /// - Cannot be empty
  /// - Always starts with '.' prefix
  /// - Cannot be constructed with invalid values
  /// - Validation happens at construction time
  ///
  /// # Design Rationale
  ///
  /// **Why validate at construction?**
  /// - Fail-fast principle: invalid names panic immediately
  /// - Type system guarantees all CommandName instances are valid
  /// - Eliminates need for runtime validation checks
  /// - Better error messages (MissingDotPrefix vs generic "invalid name")
  ///
  /// **Migration impact:**
  /// Tests expecting invalid names now panic at construction, not registration.
  /// This is **intended behavior** - invalid states are impossible to represent.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::CommandName;
  ///
  /// // Valid construction
  /// let name = CommandName::new(".build").expect("valid name");
  /// assert_eq!(name.as_str(), ".build");
  ///
  /// // Invalid - empty name
  /// assert!(CommandName::new("").is_err());
  ///
  /// // Invalid - missing dot prefix
  /// assert!(CommandName::new("build").is_err());
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct CommandName( String );

  impl CommandName
  {
    ///
    /// Creates a new CommandName with validation.
    ///
    /// # Validation Rules
    /// 1. Name cannot be empty
    /// 2. Name must start with '.' prefix
    ///
    /// # Arguments
    /// * `name` - The command name to validate
    ///
    /// # Returns
    /// * `Ok(CommandName)` - If validation passes
    /// * `Err(Error)` - If validation fails
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandName;
    ///
    /// let valid = CommandName::new(".test");
    /// assert!(valid.is_ok());
    ///
    /// let empty = CommandName::new("");
    /// assert!(empty.is_err());
    ///
    /// let no_prefix = CommandName::new("test");
    /// assert!(no_prefix.is_err());
    /// ```
    pub fn new( name : impl Into< String > ) -> Result< Self, Error >
    {
      let name = name.into();

      // Validation Rule 1: Name cannot be empty
      if name.is_empty()
      {
        return Err( Error::EmptyCommandName );
      }

      // Validation Rule 2: Name must start with '.'
      if !name.starts_with( '.' )
      {
        return Err( Error::MissingDotPrefix( name ) );
      }

      Ok( Self( name ) )
    }

    ///
    /// Returns the command name as a string slice.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandName;
    ///
    /// let name = CommandName::new(".build").unwrap();
    /// assert_eq!(name.as_str(), ".build");
    /// ```
    pub fn as_str( &self ) -> &str
    {
      &self.0
    }

    ///
    /// Consumes the CommandName and returns the inner String.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandName;
    ///
    /// let name = CommandName::new(".build").unwrap();
    /// let inner : String = name.into_inner();
    /// assert_eq!(inner, ".build");
    /// ```
    pub fn into_inner( self ) -> String
    {
      self.0
    }
  }

  impl std::fmt::Display for CommandName
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.0 )
    }
  }

  impl serde::Serialize for CommandName
  {
    fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
    where
      S : serde::Serializer,
    {
      serializer.serialize_str( &self.0 )
    }
  }

  impl< 'de > serde::Deserialize< 'de > for CommandName
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      CommandName::new( s ).map_err( serde::de::Error::custom )
    }
  }

  ///
  /// A validated namespace that guarantees correct naming conventions.
  ///
  /// # Type Safety Guarantees
  /// - Empty namespace allowed (for root-level commands)
  /// - Non-empty namespaces must start with '.' prefix
  /// - Cannot be constructed with invalid values
  /// - Validation happens at construction time
  ///
  /// # Design Rationale
  ///
  /// **Why validate namespaces?**
  ///
  /// Namespaces organize commands into logical groups (e.g., `.video`, `.session`).
  /// The old API allowed invalid namespaces like `"video"` or `"session"` without dots,
  /// breaking the command naming convention and causing runtime errors.
  ///
  /// **Special case: Empty namespace**
  ///
  /// Unlike `CommandName`, empty namespace is valid and represents root-level commands.
  /// A command with `namespace=""` and `name=".help"` has `full_name=".help"`.
  /// This design allows both:
  /// - Root commands: `.help`, `.version`
  /// - Namespaced commands: `.video.search`, `.session.list`
  ///
  /// **Why not just use String?**
  ///
  /// ```ignore
  /// // Old API - compiles but breaks at runtime
  /// let mut cmd = CommandDefinition { namespace: "video".to_string(), ... };
  /// registry.register(cmd); // Runtime error: "Invalid namespace"
  /// ```
  ///
  /// With `NamespaceType`, invalid namespaces are caught at construction:
  ///
  /// ```ignore
  /// let ns = NamespaceType::new("video"); // Compile error or immediate panic
  /// ```
  ///
  /// **Migration impact:**
  ///
  /// Old: `namespace: "".to_string()` or `namespace: ".video".to_string()`
  /// New: `namespace: NamespaceType::new("").unwrap()` or builder with String conversion
  ///
  /// The builder API accepts `String` and validates internally, making migration smooth
  /// for most code while maintaining type safety at the boundary.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::NamespaceType;
  ///
  /// // Valid - empty namespace (root level)
  /// let root = NamespaceType::new("").expect("valid");
  /// assert_eq!(root.as_str(), "");
  ///
  /// // Valid - namespace with dot prefix
  /// let ns = NamespaceType::new(".video").expect("valid");
  /// assert_eq!(ns.as_str(), ".video");
  ///
  /// // Invalid - non-empty without dot prefix
  /// assert!(NamespaceType::new("video").is_err());
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct NamespaceType( String );

  impl NamespaceType
  {
    ///
    /// Creates a new NamespaceType with validation.
    ///
    /// # Validation Rules
    /// 1. Empty namespace is allowed (root-level commands)
    /// 2. Non-empty namespace must start with '.' prefix
    ///
    /// # Arguments
    /// * `namespace` - The namespace to validate
    ///
    /// # Returns
    /// * `Ok(NamespaceType)` - If validation passes
    /// * `Err(Error)` - If validation fails
    ///
    /// # Examples
    /// ```
    /// use unilang::data::NamespaceType;
    ///
    /// let empty = NamespaceType::new("");
    /// assert!(empty.is_ok());
    ///
    /// let valid = NamespaceType::new(".video");
    /// assert!(valid.is_ok());
    ///
    /// let invalid = NamespaceType::new("video");
    /// assert!(invalid.is_err());
    /// ```
    pub fn new( namespace : impl Into< String > ) -> Result< Self, Error >
    {
      let namespace = namespace.into();

      // Validation Rule 1: Empty namespace is allowed
      if namespace.is_empty()
      {
        return Ok( Self( namespace ) );
      }

      // Validation Rule 2: Non-empty namespace must start with '.'
      if !namespace.starts_with( '.' )
      {
        return Err( Error::Registration( format!(
          "Invalid namespace '{}'. Non-empty namespaces must start with dot prefix (e.g., '.video'). \
          Empty namespace is allowed for root-level commands.",
          namespace
        )));
      }

      Ok( Self( namespace ) )
    }

    ///
    /// Returns the namespace as a string slice.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::NamespaceType;
    ///
    /// let ns = NamespaceType::new(".video").unwrap();
    /// assert_eq!(ns.as_str(), ".video");
    /// ```
    pub fn as_str( &self ) -> &str
    {
      &self.0
    }

    ///
    /// Consumes the NamespaceType and returns the inner String.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::NamespaceType;
    ///
    /// let ns = NamespaceType::new(".video").unwrap();
    /// let inner : String = ns.into_inner();
    /// assert_eq!(inner, ".video");
    /// ```
    pub fn into_inner( self ) -> String
    {
      self.0
    }

    ///
    /// Returns true if this is the root namespace (empty).
    ///
    /// # Examples
    /// ```
    /// use unilang::data::NamespaceType;
    ///
    /// let root = NamespaceType::new("").unwrap();
    /// assert!(root.is_root());
    ///
    /// let ns = NamespaceType::new(".video").unwrap();
    /// assert!(!ns.is_root());
    /// ```
    pub fn is_root( &self ) -> bool
    {
      self.0.is_empty()
    }
  }

  impl std::fmt::Display for NamespaceType
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.0 )
    }
  }

  impl serde::Serialize for NamespaceType
  {
    fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
    where
      S : serde::Serializer,
    {
      serializer.serialize_str( &self.0 )
    }
  }

  impl< 'de > serde::Deserialize< 'de > for NamespaceType
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      NamespaceType::new( s ).map_err( serde::de::Error::custom )
    }
  }

  ///
  /// A validated version string.
  ///
  /// # Type Safety Guarantees
  /// - Cannot be empty
  /// - Basic format validation (semver-like)
  /// - Cannot be constructed with invalid values
  /// - Validation happens at construction time
  ///
  /// # Design Rationale
  ///
  /// **Why validate version strings?**
  ///
  /// Command versions track API changes and help users understand stability.
  /// The old API accepted any String including empty strings, leading to problems:
  ///
  /// ```ignore
  /// // Old API - all compile fine but semantically wrong
  /// let cmd1 = CommandDefinition { version: "".to_string(), ... }; // Empty!
  /// let cmd2 = CommandDefinition { version: "version 1".to_string(), ... }; // Invalid format
  /// let cmd3 = CommandDefinition { version: "latest".to_string(), ... }; // Not a version
  /// ```
  ///
  /// **Validation rules:**
  ///
  /// Current validation is minimal (non-empty only) to remain flexible while preventing
  /// the most obvious errors. Future enhancements could enforce strict semver.
  ///
  /// **Why not full semver parsing?**
  ///
  /// Trade-off decision: Strict semver (`1.2.3` only) would break valid use cases like:
  /// - `"2.1"` (two-part versions)
  /// - `"1.0.0-alpha"` (pre-release versions)
  /// - `"0.1"` (development versions)
  ///
  /// Current approach: Validate non-empty, allow flexible formats. This catches the
  /// most common error (empty string) without being overly restrictive.
  ///
  /// **Design evolution:**
  ///
  /// Phase 1: No validation (any String)
  /// Phase 2: Non-empty validation (current)
  /// Phase 3: Could add optional strict semver mode if needed
  ///
  /// **Migration impact:**
  ///
  /// Old: `version: "1.0.0".to_string()` or default `"1.0.0"`
  /// New: `version: VersionType::new("1.0.0").unwrap()` or builder handles conversion
  ///
  /// The builder provides `"1.0.0"` default, making most migrations transparent.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::VersionType;
  ///
  /// // Valid versions
  /// let v = VersionType::new("1.0.0").expect("valid");
  /// assert_eq!(v.as_str(), "1.0.0");
  ///
  /// let v2 = VersionType::new("2.1").expect("valid");
  /// assert_eq!(v2.as_str(), "2.1");
  ///
  /// // Invalid - empty version
  /// assert!(VersionType::new("").is_err());
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct VersionType( String );

  impl VersionType
  {
    ///
    /// Creates a new VersionType with validation.
    ///
    /// # Validation Rules
    /// 1. Version cannot be empty
    ///
    /// # Arguments
    /// * `version` - The version string to validate
    ///
    /// # Returns
    /// * `Ok(VersionType)` - If validation passes
    /// * `Err(Error)` - If validation fails
    ///
    /// # Examples
    /// ```
    /// use unilang::data::VersionType;
    ///
    /// let valid = VersionType::new("1.0.0");
    /// assert!(valid.is_ok());
    ///
    /// let empty = VersionType::new("");
    /// assert!(empty.is_err());
    /// ```
    pub fn new( version : impl Into< String > ) -> Result< Self, Error >
    {
      let version = version.into();

      // Validation Rule 1: Version cannot be empty
      if version.is_empty()
      {
        return Err( Error::Registration(
          "Invalid version: version string cannot be empty".to_string()
        ));
      }

      Ok( Self( version ) )
    }

    ///
    /// Returns the version as a string slice.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::VersionType;
    ///
    /// let v = VersionType::new("1.0.0").unwrap();
    /// assert_eq!(v.as_str(), "1.0.0");
    /// ```
    pub fn as_str( &self ) -> &str
    {
      &self.0
    }

    ///
    /// Consumes the VersionType and returns the inner String.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::VersionType;
    ///
    /// let v = VersionType::new("1.0.0").unwrap();
    /// let inner : String = v.into_inner();
    /// assert_eq!(inner, "1.0.0");
    /// ```
    pub fn into_inner( self ) -> String
    {
      self.0
    }
  }

  impl std::fmt::Display for VersionType
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.0 )
    }
  }

  impl serde::Serialize for VersionType
  {
    fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
    where
      S : serde::Serializer,
    {
      serializer.serialize_str( &self.0 )
    }
  }

  impl< 'de > serde::Deserialize< 'de > for VersionType
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      VersionType::new( s ).map_err( serde::de::Error::custom )
    }
  }
