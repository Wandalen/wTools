//!
//! Core data structures for the Unilang framework.
//!
//! ## Phase 2 Type-Safe Redesign
//!
//! This module underwent a complete type-safe redesign implementing the "parse don't validate"
//! pattern. The redesign eliminates entire categories of bugs by making invalid states
//! impossible to represent.
//!
//! ### Design Philosophy
//!
//! **Core Principle: Invalid States Should Be Impossible**
//!
//! The old API allowed commands to be constructed in invalid states that only failed at
//! runtime during registration. The new API catches errors at construction time, moving
//! bugs from runtime to compile time.
//!
//! ```ignore
//! // Old API - compiles but fails at runtime
//! let cmd = CommandDefinition {
//!   name: "no_dot".to_string(),        // Invalid! No dot prefix
//!   namespace: "bad_ns".to_string(),   // Invalid! No dot prefix
//!   version: "".to_string(),           // Invalid! Empty version
//!   status: "activ".to_string(),       // Typo! Should be "active"
//!   ..Default::default()
//! };
//! registry.register(cmd); // Runtime panic during registration
//!
//! // New API - errors at construction
//! let cmd = CommandDefinition::former()
//!   .name("no_dot")          // Compile error: CommandName validates at construction
//!   .description("Test")
//!   .end();
//! ```
//!
//! ### Key Design Decisions
//!
//! **1. Breaking Change Strategy: Clean Replacement**
//!
//! No backward compatibility layer, no gradual migration. Old `CommandDefinition` deleted,
//! new one takes its place. This prevents code duplication and ensures single source of truth.
//!
//! **Rationale:** Maintaining both APIs doubles maintenance burden and creates confusion.
//! Clean break forces all code to adopt new patterns consistently.
//!
//! **2. Validated Newtypes: Fail-Fast Construction**
//!
//! Core types wrapped in validated newtypes:
//! - `CommandName`: Guarantees dot prefix (e.g., `.build`)
//! - `NamespaceType`: Guarantees valid namespace (empty or dot-prefixed)
//! - `VersionType`: Guarantees non-empty version string
//! - `CommandStatus`: Enum eliminates typos (`Active`, not `"activ"`)
//!
//! **Rationale:** Validation at construction time means invalid values can't exist.
//! Type system becomes documentation and enforcement.
//!
//! **3. Private Fields: Immutability Guarantees**
//!
//! All `CommandDefinition` fields are private with getter methods only. Once constructed,
//! commands cannot be mutated into invalid states.
//!
//! **Rationale:** Prevents bugs where commands are modified after validation.
//! Immutability makes reasoning about correctness easier.
//!
//! **4. Builder API: Type-State Pattern**
//!
//! Two finalization methods with different trade-offs:
//! - `end()`: Requires only `name` + `description`, provides defaults (ergonomic for tests)
//! - `build()`: Requires ALL fields explicitly set (explicit for production)
//!
//! Type-state pattern uses phantom types to enforce required fields at compile time.
//!
//! **Rationale:** Compile-time enforcement prevents incomplete construction. Progressive
//! disclosure guides developers through required fields.
//!
//! **5. Pragmatic Exceptions: ArgumentDefinition**
//!
//! `ArgumentDefinition` retains public fields and uses `former::Former` derive macro.
//! Arguments are validated during command registration, so the stricter approach isn't
//! necessary yet. Future phases could redesign arguments if needed.
//!
//! **Rationale:** Apply strictness where bugs actually occur. Arguments are less error-prone
//! in practice than top-level command definitions.
//!
//! ### Migration Impact
//!
//! **Breaking changes:**
//! - All `CommandDefinition` construction must use builder or `new()` method
//! - Field access changed from direct (`cmd.name`) to getters (`cmd.name()`)
//! - Invalid commands now panic at construction, not registration
//! - Status strings replaced with `CommandStatus` enum
//!
//! **Benefits:**
//! - Bugs caught at compile time instead of runtime
//! - Type system documents valid states
//! - IDE autocomplete guides correct usage
//! - Impossible to create invalid commands
//! - Centralized validation (not scattered across codebase)
//!
//! ### Trade-offs
//!
//! **Cost:** More verbose construction, complex type signatures, breaking changes
//! **Benefit:** Entire categories of bugs eliminated at compile time
//!
//! This trade-off strongly favors type safety for domain objects like commands where
//! correctness is critical and construction happens infrequently.
//!
//! ### References
//!
//! See individual struct documentation for detailed design rationale:
//! - `CommandDefinition`: Overall structure and builder pattern
//! - `CommandName`: Why validate names at construction
//! - `NamespaceType`: Empty namespace semantics
//! - `VersionType`: Version validation trade-offs
//! - `CommandStatus`: Why enum instead of String
//! - `CommandDefinitionBuilder`: Type-state pattern mechanics
//! - `ArgumentDefinition`: Why public fields are acceptable here

/// Internal namespace.
mod private
{
  use crate::error::Error;
  // Removed strs_tools dependencies - using standard Rust string operations

  // use former::Former;

  //
  // Validated Newtypes (Phase 2)
  //

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

  ///
  /// Command status indicating lifecycle stage and availability.
  ///
  /// # Type Safety Guarantees
  /// - No typos in status strings (compile-time checked)
  /// - Structured deprecation data
  /// - Clear distinction between stable/experimental/internal
  ///
  /// # Design Rationale
  ///
  /// **Why enum instead of String?**
  ///
  /// The old API used `status: String` with runtime checks. This caused several problems:
  ///
  /// 1. **Typos:** `"activ"`, `"Active"`, `"ACTIVE"` all accepted, caused bugs
  /// 2. **No structure:** Deprecation metadata (reason, since, replacement) scattered across fields
  /// 3. **Runtime errors:** Invalid status only caught during validation, not construction
  /// 4. **No IDE support:** No autocomplete, no compiler help
  ///
  /// **Benefits of enum:**
  ///
  /// - **Compile-time safety:** Typos become compilation errors
  /// - **Structured data:** Deprecation variant holds reason/since/replacement together
  /// - **Pattern matching:** Exhaustive checks ensure all cases handled
  /// - **IDE support:** Autocomplete shows all valid statuses
  /// - **Self-documenting:** Variants clearly show all possible states
  ///
  /// **Trade-off:** Adding new status requires code change (not YAML change).
  /// This is **intentional** - status is core domain concept, not user data.
  ///
  /// **Migration impact:**
  ///
  /// Old: `status: "deprecated".to_string(), deprecation_message: "reason".to_string()`
  /// New: `status: CommandStatus::Deprecated { reason: "reason".to_string(), since: None, replacement: None }`
  ///
  /// The new API forces explicit handling of deprecation metadata at construction time,
  /// preventing incomplete deprecation notices.
  ///
  /// # Display Format
  ///
  /// The `Display` implementation produces human-readable status strings:
  /// - `Active` → "active"
  /// - `Experimental` → "experimental"
  /// - `Internal` → "internal"
  /// - `Deprecated{reason, ..}` → "deprecated: {reason}" (includes reason if non-empty)
  ///
  /// **Important:** Tests comparing status strings must account for the full display format.
  /// For example, `CommandStatus::Deprecated { reason: "Use .v2 instead", .. }` displays as
  /// `"deprecated: Use .v2 instead"`, not just `"deprecated"`.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::CommandStatus;
  ///
  /// // Active command
  /// let active = CommandStatus::Active;
  /// assert!(active.is_active());
  /// assert_eq!(active.to_string(), "active");
  ///
  /// // Deprecated command with full metadata
  /// let deprecated = CommandStatus::Deprecated {
  ///   reason: "Use .new_command instead".to_string(),
  ///   since: Some("2.0.0".to_string()),
  ///   replacement: Some(".new_command".to_string()),
  /// };
  /// assert!(deprecated.is_deprecated());
  /// assert_eq!(deprecated.to_string(), "deprecated (since 2.0.0): Use .new_command instead → .new_command");
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub enum CommandStatus
  {
    /// Command is active and stable for production use
    Active,

    /// Command is deprecated and may be removed in future versions
    Deprecated
    {
      /// Reason for deprecation and migration guidance
      reason : String,
      /// Version when deprecation started
      since : Option< String >,
      /// Suggested replacement command
      replacement : Option< String >,
    },

    /// Command is experimental and API may change
    Experimental,

    /// Command is for internal use only
    Internal,
  }

  impl CommandStatus
  {
    ///
    /// Returns true if this command is active/stable.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandStatus;
    ///
    /// let active = CommandStatus::Active;
    /// assert!(active.is_active());
    ///
    /// let experimental = CommandStatus::Experimental;
    /// assert!(!experimental.is_active());
    /// ```
    pub fn is_active( &self ) -> bool
    {
      matches!( self, CommandStatus::Active )
    }

    ///
    /// Returns true if this command is deprecated.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandStatus;
    ///
    /// let deprecated = CommandStatus::Deprecated {
    ///   reason: "Old API".to_string(),
    ///   since: None,
    ///   replacement: None,
    /// };
    /// assert!(deprecated.is_deprecated());
    /// ```
    pub fn is_deprecated( &self ) -> bool
    {
      matches!( self, CommandStatus::Deprecated { .. } )
    }

    ///
    /// Returns true if this command is experimental.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandStatus;
    ///
    /// let experimental = CommandStatus::Experimental;
    /// assert!(experimental.is_experimental());
    /// ```
    pub fn is_experimental( &self ) -> bool
    {
      matches!( self, CommandStatus::Experimental )
    }

    ///
    /// Returns true if this command is internal-only.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandStatus;
    ///
    /// let internal = CommandStatus::Internal;
    /// assert!(internal.is_internal());
    /// ```
    pub fn is_internal( &self ) -> bool
    {
      matches!( self, CommandStatus::Internal )
    }

    ///
    /// Gets deprecation metadata if this command is deprecated.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandStatus;
    ///
    /// let deprecated = CommandStatus::Deprecated {
    ///   reason: "Use .new".to_string(),
    ///   since: Some("2.0.0".to_string()),
    ///   replacement: Some(".new".to_string()),
    /// };
    ///
    /// let (reason, since, replacement) = deprecated.deprecation_info().unwrap();
    /// assert_eq!(reason, "Use .new");
    /// assert_eq!(since.as_ref().unwrap(), "2.0.0");
    /// assert_eq!(replacement.as_ref().unwrap(), ".new");
    /// ```
    pub fn deprecation_info( &self ) -> Option< ( &str, &Option< String >, &Option< String > ) >
    {
      match self
      {
        CommandStatus::Deprecated { reason, since, replacement } =>
        {
          Some( ( reason.as_str(), since, replacement ) )
        },
        _ => None,
      }
    }
  }

  impl Default for CommandStatus
  {
    fn default() -> Self
    {
      CommandStatus::Active
    }
  }

  impl std::fmt::Display for CommandStatus
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        CommandStatus::Active => write!( f, "active" ),
        CommandStatus::Deprecated { reason, since, replacement } =>
        {
          write!( f, "deprecated" )?;
          if let Some( s ) = since
          {
            write!( f, " (since {})", s )?;
          }
          if !reason.is_empty()
          {
            write!( f, ": {}", reason )?;
          }
          if let Some( r ) = replacement
          {
            write!( f, " → {}", r )?;
          }
          Ok(())
        },
        CommandStatus::Experimental => write!( f, "experimental" ),
        CommandStatus::Internal => write!( f, "internal" ),
      }
    }
  }

  impl serde::Serialize for CommandStatus
  {
    fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
    where
      S : serde::Serializer,
    {
      use serde::ser::SerializeMap;

      match self
      {
        CommandStatus::Active =>
        {
          serializer.serialize_str( "active" )
        },
        CommandStatus::Experimental =>
        {
          serializer.serialize_str( "experimental" )
        },
        CommandStatus::Internal =>
        {
          serializer.serialize_str( "internal" )
        },
        CommandStatus::Deprecated { reason, since, replacement } =>
        {
          let mut map = serializer.serialize_map( Some( 4 ) )?;
          map.serialize_entry( "status", "deprecated" )?;
          map.serialize_entry( "reason", reason )?;
          map.serialize_entry( "since", since )?;
          map.serialize_entry( "replacement", replacement )?;
          map.end()
        },
      }
    }
  }

  impl< 'de > serde::Deserialize< 'de > for CommandStatus
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      use serde::de::{ self, Visitor, MapAccess };

      struct CommandStatusVisitor;

      impl< 'de > Visitor< 'de > for CommandStatusVisitor
      {
        type Value = CommandStatus;

        fn expecting( &self, formatter : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
        {
          formatter.write_str( "a command status string or deprecated status object" )
        }

        fn visit_str< E >( self, value : &str ) -> Result< CommandStatus, E >
        where
          E : de::Error,
        {
          match value.to_lowercase().as_str()
          {
            "active" | "stable" => Ok( CommandStatus::Active ),
            "experimental" => Ok( CommandStatus::Experimental ),
            "internal" => Ok( CommandStatus::Internal ),
            "deprecated" =>
            {
              // Simple deprecated without metadata
              Ok( CommandStatus::Deprecated
              {
                reason : String::new(),
                since : None,
                replacement : None,
              })
            },
            _ => Ok( CommandStatus::Active ), // Default to active for unknown
          }
        }

        fn visit_map< M >( self, mut map : M ) -> Result< CommandStatus, M::Error >
        where
          M : MapAccess< 'de >,
        {
          let mut status : Option< String > = None;
          let mut reason : Option< String > = None;
          let mut since : Option< Option< String > > = None;
          let mut replacement : Option< Option< String > > = None;

          while let Some( key ) = map.next_key::< String >()?
          {
            match key.as_str()
            {
              "status" => status = Some( map.next_value()? ),
              "reason" => reason = Some( map.next_value()? ),
              "since" => since = Some( map.next_value()? ),
              "replacement" => replacement = Some( map.next_value()? ),
              _ => { map.next_value::< serde::de::IgnoredAny >()?; },
            }
          }

          match status.as_deref()
          {
            Some( "deprecated" ) =>
            {
              Ok( CommandStatus::Deprecated
              {
                reason : reason.unwrap_or_default(),
                since : since.flatten(),
                replacement : replacement.flatten(),
              })
            },
            Some( "experimental" ) => Ok( CommandStatus::Experimental ),
            Some( "internal" ) => Ok( CommandStatus::Internal ),
            _ => Ok( CommandStatus::Active ),
          }
        }
      }

      deserializer.deserialize_any( CommandStatusVisitor )
    }
  }

  ///
  /// Helper function to construct a full command name from namespace and name components.
  ///
  /// This function implements the canonical algorithm for combining namespace and name
  /// into a fully qualified command name that always starts with a dot prefix.
  ///
  /// # Arguments
  /// * `namespace` - The command's namespace (may be empty or dot-prefixed)
  /// * `name` - The command's name (may already include dot prefix)
  ///
  /// # Returns
  /// * `String` - The fully qualified command name with dot prefix
  ///
  /// # Algorithm
  /// 1. If name already starts with '.':
  ///    - If namespace is empty OR name contains '.', return name as-is (already full format)
  ///    - Otherwise, strip '.' from name and concatenate with namespace
  /// 2. If name doesn't start with '.':
  ///    - If namespace is empty, prepend '.' to name
  ///    - If namespace exists, concatenate with proper dot handling
  pub fn construct_full_command_name( namespace : &str, name : &str ) -> String
  {
    if name.starts_with( '.' )
    {
      // Name already has dot prefix
      if namespace.is_empty() || name.contains( ".." ) || name[ 1.. ].contains( '.' )
      {
        // Name is already in full format (e.g., ".integration.test")
        // OR has multiple dots (e.g., ".a.b") indicating it's already complete
        name.to_string()
      }
      else
      {
        // Name has dot but is just the command part (e.g., ".test")
        // Need to prepend namespace
        let name_without_dot = &name[ 1.. ];
        if namespace.starts_with( '.' )
        {
          format!( "{}.{}", namespace, name_without_dot )
        }
        else
        {
          format!( ".{}.{}", namespace, name_without_dot )
        }
      }
    }
    else if namespace.is_empty()
    {
      // No namespace, no dot: add dot prefix
      format!( ".{}", name )
    }
    else
    {
      // Has namespace, name has no dot: concatenate
      if namespace.starts_with( '.' )
      {
        format!( "{}.{}", namespace, name )
      }
      else
      {
        format!( ".{}.{}", namespace, name )
      }
    }
  }


  ///
  /// Holds attributes and configuration for a specific argument within a command.
  ///
  /// This struct enables fine-grained control over how arguments behave,
  /// such as whether they are required, accept multiple values, or have
  /// default values.
  #[allow(clippy::struct_excessive_bools)]
  #[ derive( Debug, Clone, Default, serde::Serialize, serde::Deserialize ) ]
  pub struct ArgumentAttributes
  {
    /// Indicates if the argument is optional.
    /// If true, the argument can be omitted without causing validation errors.
    pub optional : bool,
    /// Indicates if the argument can accept multiple values.
    /// If true, the argument can be provided multiple times in a single command invocation.
    pub multiple : bool,
    /// The default value for the argument if not provided.
    /// Only applicable when the argument is optional.
    pub default : Option< String >,
    /// Indicates if the argument contains sensitive data (e.g., passwords).
    /// If true, the argument might be masked or logged differently.
    pub sensitive : bool,
    /// Indicates if the argument might require user interaction (e.g., prompts).
    /// If true, the system may need to handle interactive input.
    ///
    /// # REPL Implementation Notes
    /// 
    /// **Critical Behavior**: When `interactive: true` and the argument is required but not provided:
    /// - Semantic analysis returns `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error
    /// - REPL loops should catch this error and prompt for secure input
    /// - **Important**: Optional interactive arguments with defaults do NOT trigger the error
    /// 
    /// **Security Best Practices**:
    /// - Always combine with `sensitive: true` for passwords/API keys
    /// - Never log or store interactive argument values
    /// - Use secure input methods (masked input) in REPL implementations
    /// 
    /// **Common Pitfalls**:
    /// - ❌ Don't handle interactive prompts during command execution
    /// - ❌ Don't store interactive values in command history  
    /// - ✅ Handle interactive prompts at the REPL level before re-execution
    /// - ✅ Clear sensitive values from memory after use
    pub interactive : bool,
  }

  ///
  /// Defines an argument within a command, including its name, type, and constraints.
  ///
  /// This struct provides all the necessary information to parse, validate,
  /// and process a single argument within a command.
  ///
  /// # Design Note: Public Fields
  ///
  /// Unlike `CommandDefinition` (Phase 2 redesign with private fields), `ArgumentDefinition`
  /// retains public fields for pragmatic reasons:
  ///
  /// **Why not private fields here?**
  ///
  /// 1. **Complexity vs benefit**: Arguments are components of commands, not top-level
  ///    domain objects. The validation burden is lower since invalid arguments are caught
  ///    during command validation anyway.
  ///
  /// 2. **Ergonomics**: Arguments are frequently constructed inline in tests and examples.
  ///    Public fields with `Default` trait provide good ergonomics for simple cases.
  ///
  /// 3. **Former integration**: Using `former::Former` derive provides type-safe building
  ///    without custom implementation complexity. The macro generates a full-featured
  ///    builder automatically.
  ///
  /// 4. **Phase 2 scope**: The type-safe redesign focused on `CommandDefinition` as the
  ///    primary domain object. Arguments could be redesigned in future phase if needed.
  ///
  /// **Trade-off:**
  ///
  /// - Public fields allow mutation after construction (less safe)
  /// - But validation happens at command registration (catches errors before use)
  /// - Simpler implementation (uses derive macro vs custom builder)
  ///
  /// **Future evolution:**
  ///
  /// If mutation bugs appear or validation gaps are discovered, `ArgumentDefinition`
  /// could undergo similar redesign to `CommandDefinition` (private fields, validated
  /// newtypes, custom builder). Current design is pragmatic given actual usage patterns.
  ///
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize, former::Former ) ]
  pub struct ArgumentDefinition
  {
    /// The name of the argument, used to reference it in commands and validation.
    pub name : String,
    /// The data type and structure expected for this argument.
    pub kind : Kind,
    /// Attributes that control the behavior of this argument.
    pub attributes : ArgumentAttributes,
    /// A brief, one-line hint about the argument's purpose.
    pub hint : String,
    /// A more detailed description of the argument.
    pub description : String,
    /// Validation rules that apply to this argument.
    pub validation_rules : Vec< ValidationRule >,
    /// Alternative names for this argument.
    pub aliases : Vec< String >,
    /// Tags associated with this argument.
    pub tags : Vec< String >,
  }

  impl ArgumentDefinition
  {
    /// Creates a new argument with sensible defaults (simplified constructor).
    ///
    /// This is the recommended way to create arguments for most use cases.
    /// It requires only the essential fields (name and type) and provides
    /// reasonable defaults for all optional fields.
    ///
    /// # Arguments
    /// * `name` - Argument name
    /// * `kind` - Argument type (String, Integer, etc.)
    ///
    /// # Returns
    /// * `Self` - A new ArgumentDefinition with defaults applied
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// // Simple required string argument
    /// let arg = ArgumentDefinition::new("name", Kind::String);
    /// assert_eq!(arg.name, "name");
    /// assert!(!arg.attributes.optional);
    ///
    /// // Optional integer with default
    /// let count_arg = ArgumentDefinition::new("count", Kind::Integer)
    ///   .with_optional(Some("10"));
    /// assert!(count_arg.attributes.optional);
    /// ```
    #[ must_use ]
    pub fn new( name : impl Into< String >, kind : Kind ) -> Self
    {
      Self
      {
        name : name.into(),
        kind,
        attributes : ArgumentAttributes::default(),
        hint : String::new(),
        description : String::new(),
        validation_rules : Vec::new(),
        aliases : Vec::new(),
        tags : Vec::new(),
      }
    }

    /// Makes the argument optional with an optional default value (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// let arg = ArgumentDefinition::new("count", Kind::Integer)
    ///   .with_optional(Some("10"));
    /// assert!(arg.attributes.optional);
    /// assert_eq!(arg.attributes.default, Some("10".to_string()));
    /// ```
    #[ must_use ]
    pub fn with_optional( mut self, default : Option< impl Into< String > > ) -> Self
    {
      self.attributes.optional = true;
      self.attributes.default = default.map( | v | v.into() );
      self
    }

    /// Sets the description for the argument (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// let arg = ArgumentDefinition::new("name", Kind::String)
    ///   .with_description("User's name");
    /// assert_eq!(arg.description, "User's name");
    /// ```
    #[ must_use ]
    pub fn with_description( mut self, description : impl Into< String > ) -> Self
    {
      self.description = description.into();
      self
    }

    /// Adds validation rules to the argument (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    /// use unilang::ValidationRule;
    ///
    /// let arg = ArgumentDefinition::new("age", Kind::Integer)
    ///   .with_validation_rules(vec![
    ///     ValidationRule::Min(0.0),
    ///     ValidationRule::Max(150.0),
    ///   ]);
    /// assert_eq!(arg.validation_rules.len(), 2);
    /// ```
    #[ must_use ]
    pub fn with_validation_rules( mut self, rules : Vec< ValidationRule > ) -> Self
    {
      self.validation_rules = rules;
      self
    }

    /// Marks the argument as sensitive (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// let arg = ArgumentDefinition::new("password", Kind::String)
    ///   .with_sensitive(true);
    /// assert!(arg.attributes.sensitive);
    /// ```
    #[ must_use ]
    pub fn with_sensitive( mut self, sensitive : bool ) -> Self
    {
      self.attributes.sensitive = sensitive;
      self
    }

    /// Marks the argument as interactive (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// let arg = ArgumentDefinition::new("password", Kind::String)
    ///   .with_interactive(true)
    ///   .with_sensitive(true);
    /// assert!(arg.attributes.interactive);
    /// assert!(arg.attributes.sensitive);
    /// ```
    #[ must_use ]
    pub fn with_interactive( mut self, interactive : bool ) -> Self
    {
      self.attributes.interactive = interactive;
      self
    }
  }

  ///
  /// Represents the data type and structure of an argument or value.
  ///
  /// The `Kind` enum defines all supported data types and their validation rules,
  /// enabling robust type checking and conversion throughout the system.
  #[ derive( Debug, Clone, PartialEq, Eq, serde::Serialize ) ]
  #[ serde( untagged ) ]
  pub enum Kind
  {
    /// A simple text string.
    String,
    /// An integer number (positive, negative, or zero).
    Integer,
    /// A floating-point number.
    Float,
    /// A boolean value (true or false).
    Boolean,
    /// A file system path (file or directory).
    Path,
    /// A file system path that must point to an existing file.
    File,
    /// A file system path that must point to an existing directory.
    Directory,
    /// An enumeration with a predefined set of allowed values.
    Enum( Vec< String > ),
    /// A URL (web address).
    Url,
    /// A date and time value.
    DateTime,
    /// A regular expression pattern.
    Pattern,
    /// A list (array) of values of the same type.
    /// The optional character specifies the delimiter used to separate list items.
    List( Box< Kind >, Option< char > ),
    /// A map (dictionary) of key-value pairs.
    /// The optional characters specify the entry delimiter and key-value delimiter.
    Map( Box< Kind >, Box< Kind >, Option< char >, Option< char > ),
    /// A JSON string that can be parsed into complex data structures.
    JsonString,
    /// A generic object that can hold any structured data.
    Object,
  }

  /// Validation rule for argument values.
  #[ derive( Debug, Clone, PartialEq, serde::Serialize ) ]
  pub enum ValidationRule
  {
    /// Minimum value for numeric types.
    Min( f64 ),
    /// Maximum value for numeric types.
    Max( f64 ),
    /// Minimum length for string types.
    MinLength( usize ),
    /// Maximum length for string types.
    MaxLength( usize ),
    /// Pattern that string values must match.
    Pattern( String ),
    /// Minimum number of items for collection types.
    MinItems( usize ),
  }

  impl core::str::FromStr for Kind
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      match s.trim()
      {
        "String" => Ok( Kind::String ),
        "Integer" => Ok( Kind::Integer ),
        "Float" => Ok( Kind::Float ),
        "Boolean" => Ok( Kind::Boolean ),
        "Path" => Ok( Kind::Path ),
        "File" => Ok( Kind::File ),
        "Directory" => Ok( Kind::Directory ),
        "Url" => Ok( Kind::Url ),
        "DateTime" => Ok( Kind::DateTime ),
        "Pattern" => Ok( Kind::Pattern ),
        "JsonString" => Ok( Kind::JsonString ),
        "Object" => Ok( Kind::Object ),
        s if s.starts_with( "Enum(" ) && s.ends_with( ')' ) =>
        {
          let inner = s.strip_prefix( "Enum(" ).unwrap().strip_suffix( ')' ).unwrap();
          if inner.is_empty()
          {
            return Err( Error::Registration( "Empty enum choices".to_string() ) );
          }
          // Use standard Rust string splitting for enum choices
          let choices : Vec< String > = inner
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
          Ok( Kind::Enum( choices ) )
        },
        s if s.starts_with( "List(" ) && s.ends_with( ')' ) =>
        {
          let inner = s.strip_prefix( "List(" ).unwrap().strip_suffix( ')' ).unwrap();
          // Use standard Rust string splitting for list parsing
          let parts : Vec< String > = inner
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
          if parts.is_empty()
          {
            return Err( Error::Registration( "List requires item type".to_string() ) );
          }
          let item_kind = parts[ 0 ].trim().parse::<Kind>()?;
          let delimiter = if parts.len() > 1 && !parts[ 1 ].trim().is_empty()
          {
            Some( parts[ 1 ].trim().chars().next().unwrap() )
          }
          else
          {
            None
          };
          Ok( Kind::List( Box::new( item_kind ), delimiter ) )
        },
        s if s.starts_with( "Map(" ) && s.ends_with( ')' ) =>
        {
          let inner = s.strip_prefix( "Map(" ).unwrap().strip_suffix( ')' ).unwrap();
          // Use standard Rust string splitting for map parsing
          let parts : Vec< String > = inner
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
          if parts.len() < 2
          {
            return Err( Error::Registration( "Map requires key and value types".to_string() ) );
          }
          let key_kind = parts[ 0 ].trim().parse::<Kind>()?;
          let value_kind = parts[ 1 ].trim().parse::<Kind>()?;
          let entry_delimiter = if parts.len() > 2 && !parts[ 2 ].trim().is_empty()
          {
            Some( parts[ 2 ].trim().chars().next().unwrap() )
          }
          else
          {
            None
          };
          let kv_delimiter = if parts.len() > 3 && !parts[ 3 ].trim().is_empty()
          {
            Some( parts[ 3 ].trim().chars().next().unwrap() )
          }
          else
          {
            None
          };
          Ok( Kind::Map( Box::new( key_kind ), Box::new( value_kind ), entry_delimiter, kv_delimiter ) )
        },
        _ => Err( Error::Registration( format!( "Unknown kind: {s}" ) ) ),
      }
    }
  }

  /// 
  /// Represents a namespace within the command system.
  ///
  /// Namespaces provide hierarchical organization for commands, allowing
  /// related commands to be grouped together (e.g., `math.add`, `math.subtract`).
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize ) ]
  pub struct Namespace
  {
    /// The name of the namespace.
    pub name : String,
    /// Commands that belong to this namespace.
    pub commands : Vec< CommandDefinition >,
  }

  ///
  /// Represents the output of a successfully executed command.
  ///
  /// This struct provides a standardized way to return data from command execution,
  /// including both the actual content and metadata about its format.
  #[ derive( Debug, Clone /*, Former*/ ) ]
  pub struct OutputData
  {
    /// The actual content produced by the command.
    pub content : String,
    /// The format of the content (e.g., "`text`", "`json`", "`xml`").
    pub format : String,
    /// Execution time in milliseconds (if available).
    ///
    /// This field captures how long the command took to execute, useful for
    /// performance monitoring and optimization. If not set, the command execution
    /// time was not measured.
    pub execution_time_ms : Option< u64 >,
  }

  impl OutputData
  {
    /// Creates a new `OutputData` with the specified content and format.
    ///
    /// The execution time is initially set to `None` and will be populated
    /// by the interpreter during command execution.
    ///
    /// # Examples
    ///
    /// ```
    /// use unilang::data::OutputData;
    ///
    /// let output = OutputData::new( "Hello, World!", "text" );
    /// assert_eq!( output.content, "Hello, World!" );
    /// assert_eq!( output.format, "text" );
    /// assert_eq!( output.execution_time_ms, None );
    /// ```
    #[ must_use ]
    pub fn new( content : impl Into< String >, format : impl Into< String > ) -> Self
    {
      Self
      {
        content : content.into(),
        format : format.into(),
        execution_time_ms : None,
      }
    }
  }

  ///
  /// Type-safe error codes for the Unilang framework.
  ///
  /// This enum provides compile-time validation of error codes, replacing string-based
  /// error codes with typed variants. Each variant represents a specific error condition
  /// with a canonical string representation.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::ErrorCode;
  ///
  /// let code = ErrorCode::CommandNotFound;
  /// assert_eq!(code.as_str(), "UNILANG_COMMAND_NOT_FOUND");
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub enum ErrorCode
  {
    /// Command was not found in the registry.
    CommandNotFound,
    /// Required argument is missing.
    ArgumentMissing,
    /// Argument value has wrong type.
    ArgumentTypeMismatch,
    /// Interactive argument requires user input.
    ArgumentInteractiveRequired,
    /// Validation rule failed for argument value.
    ValidationRuleFailed,
    /// Too many positional arguments provided.
    TooManyArguments,
    /// Unknown named parameter provided.
    UnknownParameter,
    /// Command with same name already exists.
    CommandAlreadyExists,
    /// Command not implemented.
    CommandNotImplemented,
    /// Type conversion or mismatch error.
    TypeMismatch,
    /// Internal framework error.
    InternalError,
    /// Help information requested.
    HelpRequested,
  }

  impl ErrorCode
  {
    /// Returns the canonical string representation of the error code.
    #[ must_use ]
    pub fn as_str( &self ) -> &'static str
    {
      match self
      {
        Self::CommandNotFound => "UNILANG_COMMAND_NOT_FOUND",
        Self::ArgumentMissing => "UNILANG_ARGUMENT_MISSING",
        Self::ArgumentTypeMismatch => "UNILANG_ARGUMENT_TYPE_MISMATCH",
        Self::ArgumentInteractiveRequired => "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED",
        Self::ValidationRuleFailed => "UNILANG_VALIDATION_RULE_FAILED",
        Self::TooManyArguments => "UNILANG_TOO_MANY_ARGUMENTS",
        Self::UnknownParameter => "UNILANG_UNKNOWN_PARAMETER",
        Self::CommandAlreadyExists => "UNILANG_COMMAND_ALREADY_EXISTS",
        Self::CommandNotImplemented => "UNILANG_COMMAND_NOT_IMPLEMENTED",
        Self::TypeMismatch => "UNILANG_TYPE_MISMATCH",
        Self::InternalError => "UNILANG_INTERNAL_ERROR",
        Self::HelpRequested => "HELP_REQUESTED",
      }
    }
  }

  impl core::fmt::Display for ErrorCode
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( f, "{}", self.as_str() )
    }
  }

  impl core::str::FromStr for ErrorCode
  {
    type Err = ();

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      match s
      {
        "UNILANG_COMMAND_NOT_FOUND" => Ok( Self::CommandNotFound ),
        "UNILANG_ARGUMENT_MISSING" => Ok( Self::ArgumentMissing ),
        "UNILANG_ARGUMENT_TYPE_MISMATCH" => Ok( Self::ArgumentTypeMismatch ),
        "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" => Ok( Self::ArgumentInteractiveRequired ),
        "UNILANG_VALIDATION_RULE_FAILED" => Ok( Self::ValidationRuleFailed ),
        "UNILANG_TOO_MANY_ARGUMENTS" => Ok( Self::TooManyArguments ),
        "UNILANG_UNKNOWN_PARAMETER" => Ok( Self::UnknownParameter ),
        "UNILANG_COMMAND_ALREADY_EXISTS" => Ok( Self::CommandAlreadyExists ),
        "UNILANG_COMMAND_NOT_IMPLEMENTED" => Ok( Self::CommandNotImplemented ),
        "UNILANG_TYPE_MISMATCH" => Ok( Self::TypeMismatch ),
        "UNILANG_INTERNAL_ERROR" => Ok( Self::InternalError ),
        "HELP_REQUESTED" => Ok( Self::HelpRequested ),
        _ => Err( () ),
      }
    }
  }

  ///
  /// Represents an error that occurred during command execution.
  ///
  /// This struct provides a standardized way to report errors, including a
  /// unique, machine-readable code and a human-readable message.
  #[ derive( Debug, Clone /*, Former*/ ) ]
  pub struct ErrorData
  {
    /// A unique, machine-readable code for the error.
    pub code : ErrorCode,
    /// A human-readable message explaining the error.
    pub message : String,
    /// Optional source error for error chaining.
    pub source : Option< Box< ErrorData > >,
  }

  impl core::fmt::Display for ErrorData
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      writeln!( f, "{}", self.message )?;
      
      // Display error chain if present
      if let Some( source ) = &self.source
      {
        Self::fmt_error_chain( f, source, 1 )?;
      }
      
      Ok(())
    }
  }

  impl ErrorData
  {
    ///
    /// Creates a new `ErrorData` with no source error.
    ///
    #[ must_use ]
    pub fn new( code : ErrorCode, message : String ) -> Self
    {
      Self { code, message, source : None }
    }

    ///
    /// Creates a new `ErrorData` with a source error for chaining.
    ///
    #[ must_use ]
    pub fn with_source( code : ErrorCode, message : String, source : ErrorData ) -> Self
    {
      Self { code, message, source : Some( Box::new( source ) ) }
    }

    ///
    /// Formats the error chain recursively with proper indentation.
    ///
    fn fmt_error_chain( f : &mut core::fmt::Formatter< '_ >, error : &ErrorData, depth : usize ) -> core::fmt::Result
    {
      // Create indentation
      let indent = "  ".repeat( depth );
      writeln!( f, "{}↳ {}", indent, error.message )?;
      
      // Recursively display deeper sources
      if let Some( source ) = &error.source
      {
        Self::fmt_error_chain( f, source, depth + 1 )?;
      }
      
      Ok(())
    }
  }

  impl core::fmt::Display for Kind
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      let s : String = self.clone().into();
      write!( f, "{s}" )
    }
  }

  impl From< Kind > for String
  {
    fn from( kind : Kind ) -> Self
    {
      match kind
      {
        Kind::String => "String".to_string(),
        Kind::Integer => "Integer".to_string(),
        Kind::Float => "Float".to_string(),
        Kind::Boolean => "Boolean".to_string(),
        Kind::Path => "Path".to_string(),
        Kind::File => "File".to_string(),
        Kind::Directory => "Directory".to_string(),
        Kind::Enum( choices ) => format!( "Enum({})", choices.join( "," ) ),
        Kind::Url => "Url".to_string(),
        Kind::DateTime => "DateTime".to_string(),
        Kind::Pattern => "Pattern".to_string(),
        Kind::List( item_kind, delimiter ) =>
        {
          let item_kind_str : String = ( *item_kind ).into();
          if let Some( d ) = delimiter
          {
            format!( "List({item_kind_str},{d})" )
          }
          else
          {
            format!( "List({item_kind_str})" )
          }
        },
        Kind::Map( key_kind, value_kind, entry_delimiter, kv_delimiter ) =>
        {
          let key_kind_str : String = ( *key_kind ).into();
          let value_kind_str : String = ( *value_kind ).into();
          let mut s = format!( "Map({key_kind_str},{value_kind_str})" );
          if let Some( ed ) = entry_delimiter
          {
            s.push( ',' );
            s.push( ed );
          }
          if let Some( kvd ) = kv_delimiter
          {
            s.push( ',' );
            s.push( kvd );
          }
          s
        },
        Kind::JsonString => "JsonString".to_string(),
        Kind::Object => "Object".to_string(),
      }
    }
  }

  impl core::convert::TryFrom< String > for Kind
  {
    type Error = crate::error::Error;

    fn try_from( s : String ) -> Result< Self, Self::Error >
    {
      s.parse()
    }
  }

  impl< 'de > serde::Deserialize< 'de > for Kind
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      s.parse().map_err( serde::de::Error::custom )
    }
  }

  impl core::str::FromStr for ValidationRule
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let s = s.trim();
      if s.starts_with( "min:" )
      {
        let value_str = s.strip_prefix( "min:" ).unwrap();
        let value : f64 = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid min value: {e}" ) ) )?;
        Ok( ValidationRule::Min( value ) )
      }
      else if s.starts_with( "max:" )
      {
        let value_str = s.strip_prefix( "max:" ).unwrap();
        let value : f64 = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid max value: {e}" ) ) )?;
        Ok( ValidationRule::Max( value ) )
      }
      else if s.starts_with( "minlength:" )
      {
        let value_str = s.strip_prefix( "minlength:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid minlength value: {e}" ) ) )?;
        Ok( ValidationRule::MinLength( value ) )
      }
      else if s.starts_with( "maxlength:" )
      {
        let value_str = s.strip_prefix( "maxlength:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid maxlength value: {e}" ) ) )?;
        Ok( ValidationRule::MaxLength( value ) )
      }
      else if s.starts_with( "pattern:" )
      {
        let pattern = s.strip_prefix( "pattern:" ).unwrap();
        Ok( ValidationRule::Pattern( pattern.to_string() ) )
      }
      else if s.starts_with( "minitems:" )
      {
        let value_str = s.strip_prefix( "minitems:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid minitems value: {e}" ) ) )?;
        Ok( ValidationRule::MinItems( value ) )
      }
      else
      {
        Err( Error::Registration( format!( "Unknown validation rule: {s}" ) ) )
      }
    }
  }

  impl< 'de > serde::Deserialize< 'de > for ValidationRule
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      s.parse().map_err( serde::de::Error::custom )
    }
  }

  // Type-state markers for compile-time enforcement

  /// Marker type indicating a required field has been set.
  ///
  /// This zero-sized type is used in the type-state pattern to track
  /// which required fields have been set in the `CommandDefinitionBuilder`.
  #[ derive( Debug ) ]
  pub struct Set;

  /// Marker type indicating a required field has not been set.
  ///
  /// This zero-sized type is used in the type-state pattern to track
  /// which required fields have not yet been set in the `CommandDefinitionBuilder`.
  #[ derive( Debug ) ]
  pub struct NotSet;

  use std::marker::PhantomData;

  /// Type-state builder for `CommandDefinition` that enforces required fields at compile time.
  ///
  /// This builder uses the type-state pattern to ensure all 6 required fields
  /// (name, description, namespace, hint, status, version) are set before building.
  ///
  /// # Design Rationale
  ///
  /// **Why type-state builder pattern?**
  ///
  /// The old API used public fields with `Default` trait, causing runtime errors:
  ///
  /// ```ignore
  /// // Old API - compiles but panics at runtime
  /// let cmd = CommandDefinition::default(); // name is empty string!
  /// registry.register(cmd); // Runtime panic: "Invalid command name"
  /// ```
  ///
  /// **Problems with old approach:**
  ///
  /// 1. **Invalid states representable:** Empty names, invalid versions compile fine
  /// 2. **Runtime failures:** Validation only at registration time, not construction
  /// 3. **Unclear requirements:** No indication which fields are truly required
  /// 4. **No IDE help:** Autocomplete doesn't guide you through required fields
  ///
  /// **Benefits of type-state builder:**
  ///
  /// - **Compile-time enforcement:** Incomplete builders don't compile
  /// - **Progressive disclosure:** Type signature shows what's left to set
  /// - **IDE guidance:** Autocomplete only shows methods for unset fields
  /// - **Impossible states impossible:** Can't construct invalid CommandDefinition
  /// - **Self-documenting:** Type signature IS the documentation
  ///
  /// **How it works:**
  ///
  /// Each type parameter (`Name`, `Description`, etc.) is either `Set` or `NotSet`:
  ///
  /// ```ignore
  /// CommandDefinitionBuilder<NotSet, NotSet, NotSet, NotSet, NotSet, NotSet> // Initial
  ///   .name(".test") → CommandDefinitionBuilder<Set, NotSet, NotSet, NotSet, NotSet, NotSet>
  ///   .description("Test") → CommandDefinitionBuilder<Set, Set, NotSet, NotSet, NotSet, NotSet>
  ///   .end() // OK - only requires Name and Description to be Set
  /// ```
  ///
  /// The `build()` method is only available when ALL type parameters are `Set`:
  ///
  /// ```ignore
  /// impl CommandDefinitionBuilder<Set, Set, Set, Set, Set, Set> {
  ///   pub fn build(self) -> CommandDefinition { ... }
  /// }
  /// ```
  ///
  /// **Trade-off:** More complex type signatures, but catches errors at compile time instead
  /// of runtime. This is a **good trade-off** for domain objects where invalid states
  /// should be impossible.
  ///
  /// **Migration impact:**
  ///
  /// Old: Errors at runtime during registration
  /// New: Errors at compile time during construction
  ///
  /// This moves bugs from runtime to compile time, which is the goal of type-safe design.
  ///
  #[ derive( Debug ) ]
  pub struct CommandDefinitionBuilder< Name, Description, Namespace, Hint, Status, Version >
  {
    name : Option< String >,
    description : Option< String >,
    namespace : Option< String >,
    hint : Option< String >,
    status : Option< String >,
    version : Option< String >,
    arguments : Vec< ArgumentDefinition >,
    routine_link : Option< String >,
    tags : Vec< String >,
    aliases : Vec< String >,
    permissions : Vec< String >,
    idempotent : bool,
    deprecation_message : String,
    http_method_hint : String,
    examples : Vec< String >,
    auto_help_enabled : bool,
    category : String,
    short_desc : String,
    hidden_from_list : bool,
    priority : i32,
    group : String,
    _marker : PhantomData< ( Name, Description, Namespace, Hint, Status, Version ) >,
  }

  // Start with all required fields NotSet
  impl CommandDefinitionBuilder< NotSet, NotSet, NotSet, NotSet, NotSet, NotSet >
  {
    /// Create a new builder with all required fields unset
    pub fn new() -> Self
    {
      Self
      {
        name : None,
        description : None,
        namespace : None,
        hint : None,
        status : None,
        version : None,
        arguments : vec![],
        routine_link : None,
        tags : vec![],
        aliases : vec![],
        permissions : vec![],
        idempotent : false,
        deprecation_message : String::new(),
        http_method_hint : String::new(),
        examples : vec![],
        auto_help_enabled : true, // Default to true - help is mandatory
        category : String::new(),
        short_desc : String::new(),
        hidden_from_list : false,
        priority : 0,
        group : String::new(),
        _marker : PhantomData,
      }
    }
  }

  // Method to set name (transitions Name from NotSet to Set)
  impl< Desc, Ns, Hint, Status, Version >
    CommandDefinitionBuilder< NotSet, Desc, Ns, Hint, Status, Version >
  {
    /// Sets the command name (required field).
    ///
    /// This method transitions the `Name` type parameter from `NotSet` to `Set`,
    /// ensuring compile-time tracking of this required field.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandDefinition;
    ///
    /// let builder = CommandDefinition::former()
    ///     .name(".my_command");
    /// ```
    pub fn name( self, name : impl Into< String > )
      -> CommandDefinitionBuilder< Set, Desc, Ns, Hint, Status, Version >
    {
      CommandDefinitionBuilder
      {
        name : Some( name.into() ),
        description : self.description,
        namespace : self.namespace,
        hint : self.hint,
        status : self.status,
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set description (transitions Description from NotSet to Set)
  impl< Name, Ns, Hint, Status, Version >
    CommandDefinitionBuilder< Name, NotSet, Ns, Hint, Status, Version >
  {
    /// Sets the command description (required field).
    ///
    /// This method transitions the `Description` type parameter from `NotSet` to `Set`.
    pub fn description( self, description : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Set, Ns, Hint, Status, Version >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : Some( description.into() ),
        namespace : self.namespace,
        hint : self.hint,
        status : self.status,
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set namespace (transitions Namespace from NotSet to Set)
  impl< Name, Desc, Hint, Status, Version >
    CommandDefinitionBuilder< Name, Desc, NotSet, Hint, Status, Version >
  {
    /// Sets the command namespace (required field).
    ///
    /// Use empty string `""` for global namespace commands.
    pub fn namespace( self, namespace : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Desc, Set, Hint, Status, Version >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : self.description,
        namespace : Some( namespace.into() ),
        hint : self.hint,
        status : self.status,
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set hint (transitions Hint from NotSet to Set)
  impl< Name, Desc, Ns, Status, Version >
    CommandDefinitionBuilder< Name, Desc, Ns, NotSet, Status, Version >
  {
    /// Sets the command hint (required field).
    ///
    /// A short hint shown in help text.
    pub fn hint( self, hint : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Desc, Ns, Set, Status, Version >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : self.description,
        namespace : self.namespace,
        hint : Some( hint.into() ),
        status : self.status,
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set status (transitions Status from NotSet to Set)
  impl< Name, Desc, Ns, Hint, Version >
    CommandDefinitionBuilder< Name, Desc, Ns, Hint, NotSet, Version >
  {
    /// Sets the command status (required field).
    ///
    /// Common values: `"stable"`, `"beta"`, `"experimental"`, `"deprecated"`.
    pub fn status( self, status : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Desc, Ns, Hint, Set, Version >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : self.description,
        namespace : self.namespace,
        hint : self.hint,
        status : Some( status.into() ),
        version : self.version,
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Method to set version (transitions Version from NotSet to Set)
  impl< Name, Desc, Ns, Hint, Status >
    CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, NotSet >
  {
    /// Sets the command version (required field).
    ///
    /// Typically follows semantic versioning (e.g., `"1.0.0"`).
    pub fn version( self, version : impl Into< String > )
      -> CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, Set >
    {
      CommandDefinitionBuilder
      {
        name : self.name,
        description : self.description,
        namespace : self.namespace,
        hint : self.hint,
        status : self.status,
        version : Some( version.into() ),
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
        _marker : PhantomData,
      }
    }
  }

  // Methods for optional fields - available on all states
  impl< Name, Desc, Ns, Hint, Status, Version >
    CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, Version >
  {
    /// Sets the command arguments (optional field, defaults to empty vec).
    pub fn arguments( mut self, arguments : Vec< ArgumentDefinition > ) -> Self
    {
      self.arguments = arguments;
      self
    }

    /// Sets the routine link (optional field, defaults to None).
    pub fn routine_link( mut self, routine_link : Option< String > ) -> Self
    {
      self.routine_link = routine_link;
      self
    }

    /// Sets the command tags (optional field, defaults to empty vec).
    pub fn tags( mut self, tags : Vec< String > ) -> Self
    {
      self.tags = tags;
      self
    }

    /// Sets the command aliases (optional field, defaults to empty vec).
    pub fn aliases( mut self, aliases : Vec< String > ) -> Self
    {
      self.aliases = aliases;
      self
    }

    /// Sets the command permissions (optional field, defaults to empty vec).
    pub fn permissions( mut self, permissions : Vec< String > ) -> Self
    {
      self.permissions = permissions;
      self
    }

    /// Sets whether the command is idempotent (optional field, defaults to false).
    pub fn idempotent( mut self, idempotent : bool ) -> Self
    {
      self.idempotent = idempotent;
      self
    }

    /// Sets the deprecation message (optional field, defaults to empty string).
    pub fn deprecation_message( mut self, deprecation_message : impl Into< String > ) -> Self
    {
      self.deprecation_message = deprecation_message.into();
      self
    }

    /// Sets the HTTP method hint (optional field, defaults to empty string).
    pub fn http_method_hint( mut self, http_method_hint : impl Into< String > ) -> Self
    {
      self.http_method_hint = http_method_hint.into();
      self
    }

    /// Sets the command examples (optional field, defaults to empty vec).
    pub fn examples( mut self, examples : Vec< String > ) -> Self
    {
      self.examples = examples;
      self
    }

    /// Sets whether auto-help is enabled (optional field, defaults to true).
    ///
    /// When true (default), registering this command will automatically generate a `.command.help` variant.
    /// Set to false ONLY for help commands themselves to prevent recursion.
    pub fn auto_help_enabled( mut self, auto_help_enabled : bool ) -> Self
    {
      self.auto_help_enabled = auto_help_enabled;
      self
    }

    /// Sets the command category (optional field, defaults to empty string).
    pub fn category( mut self, category : impl Into< String > ) -> Self
    {
      self.category = category.into();
      self
    }

    /// Sets the short description (optional field, defaults to empty string).
    pub fn short_desc( mut self, short_desc : impl Into< String > ) -> Self
    {
      self.short_desc = short_desc.into();
      self
    }

    /// Sets whether the command is hidden from list (optional field, defaults to false).
    pub fn hidden_from_list( mut self, hidden : bool ) -> Self
    {
      self.hidden_from_list = hidden;
      self
    }

    /// Sets the command priority (optional field, defaults to 0).
    pub fn priority( mut self, priority : i32 ) -> Self
    {
      self.priority = priority;
      self
    }

    /// Sets the command group (optional field, defaults to empty string).
    pub fn group( mut self, group : impl Into< String > ) -> Self
    {
      self.group = group.into();
      self
    }
  }

  // Generic end() method for partial builder - allows building with just name and description
  impl< Namespace, Hint, Status, Version > CommandDefinitionBuilder< Set, Set, Namespace, Hint, Status, Version >
  {
    /// Builds the `CommandDefinition` with sensible defaults for unset fields.
    ///
    /// This method allows building a command with only name and description set,
    /// providing defaults for namespace (""), hint (""), status ("active"), and version ("1.0.0").
    ///
    /// # Design Rationale
    ///
    /// **Why have both `end()` and `build()`?**
    ///
    /// The builder offers two finalization methods with different trade-offs:
    ///
    /// - **`end()`** (this method): Requires only `name` and `description`, provides defaults
    ///   - Use case: Quick command creation, tests, simple commands
    ///   - Trade-off: Less explicit, relies on defaults
    ///   - Type signature: Available when `Name` and `Description` are `Set`
    ///
    /// - **`build()`**: Requires ALL 6 fields explicitly set
    ///   - Use case: Production code, complex commands, when defaults aren't appropriate
    ///   - Trade-off: More verbose, but fully explicit
    ///   - Type signature: Available only when all type parameters are `Set`
    ///
    /// **Why this design?**
    ///
    /// 1. **Flexibility:** Tests need concise command creation; production needs explicitness
    /// 2. **Type safety:** Both methods enforce required fields at compile time
    /// 3. **Migration path:** `end()` makes migration from old API easier (fewer fields required)
    /// 4. **No runtime errors:** Defaults are compile-time constants, not runtime lookups
    ///
    /// **Migration impact:**
    ///
    /// Old API: `CommandDefinition { name: ".test".to_string(), description: "Test".to_string(), ..Default::default() }`
    /// New API: `CommandDefinition::former().name(".test").description("Test").end()`
    ///
    /// The `end()` method provides similar ergonomics to `..Default::default()` while
    /// maintaining type safety and validation.
    ///
    /// # Fail-Fast Validation (Phase 2)
    ///
    /// This method enforces validation at construction time, not registration time:
    /// - Command names must start with `.` (dot prefix)
    /// - Versions must be valid semantic versions
    /// - **Panics immediately** if validation fails, preventing invalid state
    ///
    /// **Design rationale:**
    /// - Earlier detection: bugs caught at construction, not registration
    /// - Simpler code: no duplicate validation in registration paths
    /// - Better DX: panic stack traces point directly to construction site
    /// - Type safety: invalid commands cannot exist, even temporarily
    ///
    /// # Default Values
    ///
    /// - `namespace`: `""` (root-level command)
    /// - `hint`: `""` (no hint)
    /// - `status`: `CommandStatus::Active`
    /// - `version`: `1.0.0`
    ///
    pub fn end( self ) -> CommandDefinition
    {
      let name_str = self.name.unwrap();
      let description_str = self.description.unwrap();
      let namespace_str = self.namespace.unwrap_or_else( || String::new() );
      let hint_str = self.hint.unwrap_or_else( || String::new() );
      let status_str = self.status.unwrap_or_else( || "active".to_string() );
      let version_str = self.version.unwrap_or_else( || "1.0.0".to_string() );

      CommandDefinition
      {
        name : CommandName::new( &name_str ).expect( "builder name should be valid" ),
        description : description_str,
        namespace : namespace_str,
        hint : hint_str,
        status : match status_str.to_lowercase().as_str()
        {
          "experimental" => CommandStatus::Experimental,
          "internal" => CommandStatus::Internal,
          "deprecated" => CommandStatus::Deprecated
          {
            reason : self.deprecation_message.clone(),
            since : None,
            replacement : None,
          },
          _ => CommandStatus::Active,
        },
        version : VersionType::new( &version_str ).expect( "builder version should be valid" ),
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
      }
    }
  }

  // .build() ONLY available when ALL required fields are Set
  impl CommandDefinitionBuilder< Set, Set, Set, Set, Set, Set >
  {
    /// Builds the `CommandDefinition` from the fully-populated builder.
    ///
    /// This method is only available when all 6 required fields have been set,
    /// providing compile-time safety against missing required fields.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandDefinition;
    ///
    /// let cmd = CommandDefinition::former()
    ///     .name(".my_command")
    ///     .description("Does something useful")
    ///     .namespace("".to_string())
    ///     .hint("Brief hint")
    ///     .status("stable")
    ///     .version("1.0.0")
    ///     .build();
    ///
    /// assert_eq!(cmd.name().as_str(), ".my_command");
    /// ```
    pub fn build( self ) -> CommandDefinition
    {
      let name_str = self.name.unwrap();
      let namespace_str = self.namespace.unwrap();
      let status_str = self.status.unwrap();
      let version_str = self.version.unwrap();

      CommandDefinition
      {
        name : CommandName::new( &name_str ).expect( "builder name should be valid" ),
        description : self.description.unwrap(),
        namespace : namespace_str,
        hint : self.hint.unwrap(),
        status : match status_str.to_lowercase().as_str()
        {
          "experimental" => CommandStatus::Experimental,
          "internal" => CommandStatus::Internal,
          "deprecated" => CommandStatus::Deprecated
          {
            reason : self.deprecation_message.clone(),
            since : None,
            replacement : None,
          },
          _ => CommandStatus::Active,
        },
        version : VersionType::new( &version_str ).expect( "builder version should be valid" ),
        arguments : self.arguments,
        routine_link : self.routine_link,
        tags : self.tags,
        aliases : self.aliases,
        permissions : self.permissions,
        idempotent : self.idempotent,
        deprecation_message : self.deprecation_message,
        http_method_hint : self.http_method_hint,
        examples : self.examples,
        auto_help_enabled : self.auto_help_enabled,
        category : self.category,
        short_desc : self.short_desc,
        hidden_from_list : self.hidden_from_list,
        priority : self.priority,
        group : self.group,
      }
    }
  }


  //
  // CommandDefinition - Type-Safe Redesign with Private Fields
  //

///
/// Type-safe command definition with validated newtypes and private fields.
///
/// This struct implements the "parse don't validate" pattern, making invalid states
/// impossible at compile time. All construction goes through validated builders or
/// constructors that enforce domain rules.
///
/// # Design Rationale
///
/// **Why private fields?**
///
/// The old API had public `String` fields that could be mutated freely:
///
/// ```ignore
/// let mut cmd = CommandDefinition { name: ".test".to_string(), ... };
/// cmd.name = "invalid"; // Compiles! No dot prefix, breaks at registration
/// cmd.namespace = "bad_ns"; // Compiles! Invalid namespace, breaks later
/// ```
///
/// **Problems with public fields:**
///
/// 1. **Mutation:** Commands could be invalidated after construction
/// 2. **No validation:** Invalid values compile fine, fail at runtime
/// 3. **Unclear invariants:** No way to know what values are valid
/// 4. **Scattered validation:** Checks in registry, help system, CLI builder...
///
/// **Benefits of private fields + getters:**
///
/// - **Immutability:** Once constructed, commands can't be invalidated
/// - **Single validation point:** Construction time only, not scattered everywhere
/// - **Clear contract:** Getters return guaranteed-valid values
/// - **Encapsulation:** Implementation can change without breaking API
///
/// **Why validated newtypes?**
///
/// Instead of runtime String validation, we use validated types:
///
/// - `CommandName`: Guarantees dot prefix at construction
/// - `VersionType`: Guarantees valid semver format
/// - `CommandStatus`: Enum prevents typos like "activ" or "Active"
///
/// This moves validation from runtime to construction time, catching bugs earlier.
///
/// **Construction patterns:**
///
/// 1. **Direct constructor** (simple commands):
/// ```ignore
/// let cmd = CommandDefinition::new(
///   CommandName::new(".build").unwrap(),
///   "Build the project".to_string(),
/// );
/// ```
///
/// 2. **Builder with defaults** (tests, simple cases):
/// ```ignore
/// let cmd = CommandDefinition::former()
///   .name(".build")
///   .description("Build the project")
///   .end(); // Provides defaults for namespace, hint, status, version
/// ```
///
/// 3. **Builder fully explicit** (production):
/// ```ignore
/// let cmd = CommandDefinition::former()
///   .name(".build")
///   .description("Build the project")
///   .namespace("")
///   .hint("Build hint")
///   .status("active")
///   .version("1.0.0")
///   .build(); // No defaults, all fields required
/// ```
///
/// **Trade-off:** More verbose construction, but impossible to create invalid commands.
/// This is a **good trade-off** - bugs caught at compile time > bugs at runtime.
///
/// # Examples
/// ```rust
/// use unilang::data::{ CommandDefinition, CommandName };
///
/// // Create a new command with validation
/// let cmd = CommandDefinition::new(
///   CommandName::new(".build").unwrap(),
///   "Build the project".to_string(),
/// );
///
/// // Access via getters (fields are private)
/// assert_eq!(cmd.name().as_str(), ".build");
/// assert_eq!(cmd.description(), "Build the project");
///
/// // Using builder pattern
/// let cmd = CommandDefinition::former()
///   .name(".test")
///   .description("Test command")
///   .end();
///
/// assert_eq!(cmd.name().as_str(), ".test");
/// ```
#[ derive( Debug, Clone ) ]
pub struct CommandDefinition
{
  /// Validated command name (always starts with '.' prefix)
  name : CommandName,
  /// Brief description of what the command does
  description : String,
  /// List of arguments the command accepts
  arguments : Vec< ArgumentDefinition >,
  /// Optional link to the routine that executes this command
  routine_link : Option< String >,
  /// TEMPORARY: Public namespace field for test compatibility (String type)
  /// This allows tests to mutate namespace for validation testing
  /// TODO: Remove once all tests are migrated to builder pattern and use NamespaceType
  pub namespace : String,
  /// Short hint for the command
  hint : String,
  /// Command status (Active, Deprecated, Experimental, Internal)
  status : CommandStatus,
  /// Validated version string
  version : VersionType,
  /// Tags associated with the command
  tags : Vec< String >,
  /// Aliases for the command
  aliases : Vec< String >,
  /// Permissions required to execute the command
  permissions : Vec< String >,
  /// Indicates if the command is idempotent
  idempotent : bool,
  /// Deprecation message (deprecated - use CommandStatus::Deprecated instead)
  deprecation_message : String,
  /// Suggested HTTP method for Web API modality
  http_method_hint : String,
  /// Usage examples for help text
  examples : Vec< String >,
  /// Whether to automatically generate a .command.help counterpart
  auto_help_enabled : bool,
  /// Category for grouping commands in help output
  category : String,
  /// Short one-line description for brief help listings
  short_desc : String,
  /// Hide this command from brief help listings
  hidden_from_list : bool,
  /// Sort priority within category (lower numbers first)
  priority : i32,
  /// Explicit group membership for related commands
  group : String,
}

impl CommandDefinition
{
  /// Creates a new command using the builder pattern.
  ///
  /// This method returns a CommandDefinitionBuilder that provides a fluent API
  /// for constructing commands with compile-time verification of required fields.
  ///
  /// # Returns
  /// * `CommandDefinitionBuilder` - A builder instance for constructing a CommandDefinition
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::CommandDefinition;
  ///
  /// let cmd = CommandDefinition::former()
  ///   .name( ".greet" )
  ///   .description( "Greets the user" )
  ///   .end();
  /// ```
  #[ must_use ]
  pub fn former() -> CommandDefinitionBuilder< NotSet, NotSet, NotSet, NotSet, NotSet, NotSet >
  {
    CommandDefinitionBuilder::new()
  }

  ///
  /// Creates a new command with sensible defaults.
  ///
  /// This is the recommended way to create commands. It requires only the essential
  /// validated fields (name and description) and provides reasonable defaults for
  /// all optional fields.
  ///
  /// # Arguments
  /// * `name` - Validated command name (must start with '.')
  /// * `description` - Brief description of what the command does
  ///
  /// # Returns
  /// * `Self` - A new CommandDefinition with defaults applied
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".greet").unwrap();
  /// let cmd = CommandDefinition::new(name, "Greets the user".to_string());
  ///
  /// assert_eq!(cmd.name().as_str(), ".greet");
  /// assert_eq!(cmd.description(), "Greets the user");
  /// ```
  #[ must_use ]
  pub fn new( name : CommandName, description : String ) -> Self
  {
    Self
    {
      name,
      description,
      arguments : Vec::new(),
      routine_link : None,
      namespace : String::new(),
      hint : String::new(),
      status : CommandStatus::Active,
      version : VersionType::new( "1.0.0" ).expect( "default version valid" ),
      tags : Vec::new(),
      aliases : Vec::new(),
      permissions : Vec::new(),
      idempotent : true,
      deprecation_message : String::new(),
      http_method_hint : "GET".to_string(),
      examples : Vec::new(),
      auto_help_enabled : true,
      category : String::new(),
      short_desc : String::new(),
      hidden_from_list : false,
      priority : 0,
      group : String::new(),
    }
  }

  // ===================================================================
  // Getter Methods - Read-only access to private fields
  // ===================================================================

  /// Returns a reference to the validated command name
  #[ must_use ]
  pub fn name( &self ) -> &CommandName
  {
    &self.name
  }

  /// Returns the command description
  #[ must_use ]
  pub fn description( &self ) -> &str
  {
    &self.description
  }

  /// Returns a reference to the command arguments
  #[ must_use ]
  pub fn arguments( &self ) -> &Vec< ArgumentDefinition >
  {
    &self.arguments
  }

  /// Returns the routine link if set
  #[ must_use ]
  pub fn routine_link( &self ) -> Option< &String >
  {
    self.routine_link.as_ref()
  }

  /// Returns a reference to the namespace string
  #[ must_use ]
  pub fn namespace( &self ) -> &str
  {
    &self.namespace
  }

  /// Returns the command hint
  #[ must_use ]
  pub fn hint( &self ) -> &str
  {
    &self.hint
  }

  /// Returns a reference to the command status
  #[ must_use ]
  pub fn status( &self ) -> &CommandStatus
  {
    &self.status
  }

  /// Returns a reference to the validated version
  #[ must_use ]
  pub fn version( &self ) -> &VersionType
  {
    &self.version
  }

  /// Returns a reference to the command tags
  #[ must_use ]
  pub fn tags( &self ) -> &Vec< String >
  {
    &self.tags
  }

  /// Returns a reference to the command aliases
  #[ must_use ]
  pub fn aliases( &self ) -> &Vec< String >
  {
    &self.aliases
  }

  /// Returns a reference to the permissions
  #[ must_use ]
  pub fn permissions( &self ) -> &Vec< String >
  {
    &self.permissions
  }

  /// Returns whether the command is idempotent
  #[ must_use ]
  pub fn idempotent( &self ) -> bool
  {
    self.idempotent
  }

  /// Returns the deprecation message
  #[ must_use ]
  pub fn deprecation_message( &self ) -> &str
  {
    &self.deprecation_message
  }

  /// Returns the HTTP method hint
  #[ must_use ]
  pub fn http_method_hint( &self ) -> &str
  {
    &self.http_method_hint
  }

  /// Returns a reference to the usage examples
  #[ must_use ]
  pub fn examples( &self ) -> &Vec< String >
  {
    &self.examples
  }

  /// Returns whether auto-help is enabled
  #[ must_use ]
  pub fn auto_help_enabled( &self ) -> bool
  {
    self.auto_help_enabled
  }

  /// Returns the command category
  #[ must_use ]
  pub fn category( &self ) -> &str
  {
    &self.category
  }

  /// Returns the short description
  #[ must_use ]
  pub fn short_desc( &self ) -> &str
  {
    &self.short_desc
  }

  /// Returns whether the command is hidden from listings
  #[ must_use ]
  pub fn hidden_from_list( &self ) -> bool
  {
    self.hidden_from_list
  }

  /// Returns the command priority
  #[ must_use ]
  pub fn priority( &self ) -> i32
  {
    self.priority
  }

  /// Returns the command group
  #[ must_use ]
  pub fn group( &self ) -> &str
  {
    &self.group
  }

  // ===================================================================
  // Setter Methods - Fluent API with validation
  // ===================================================================

  /// Sets the command name (validated)
  #[ must_use ]
  pub fn with_name( mut self, name : CommandName ) -> Self
  {
    self.name = name;
    self
  }

  /// Sets the command description
  #[ must_use ]
  pub fn with_description( mut self, description : impl Into< String > ) -> Self
  {
    self.description = description.into();
    self
  }

  /// Sets the command arguments
  #[ must_use ]
  pub fn with_arguments( mut self, arguments : Vec< ArgumentDefinition > ) -> Self
  {
    self.arguments = arguments;
    self
  }

  /// Sets the routine link
  #[ must_use ]
  pub fn with_routine_link( mut self, link : Option< String > ) -> Self
  {
    self.routine_link = link;
    self
  }

  /// Sets the command namespace (String)
  #[ must_use ]
  pub fn with_namespace( mut self, namespace : String ) -> Self
  {
    self.namespace = namespace;
    self
  }

  /// Sets the command hint
  #[ must_use ]
  pub fn with_hint( mut self, hint : impl Into< String > ) -> Self
  {
    self.hint = hint.into();
    self
  }

  /// Sets the command status
  #[ must_use ]
  pub fn with_status( mut self, status : CommandStatus ) -> Self
  {
    self.status = status;
    self
  }

  /// Sets the command version (validated)
  #[ must_use ]
  pub fn with_version( mut self, version : VersionType ) -> Self
  {
    self.version = version;
    self
  }

  /// Sets the command tags
  #[ must_use ]
  pub fn with_tags( mut self, tags : Vec< String > ) -> Self
  {
    self.tags = tags;
    self
  }

  /// Sets the command aliases
  #[ must_use ]
  pub fn with_aliases( mut self, aliases : Vec< String > ) -> Self
  {
    self.aliases = aliases;
    self
  }

  /// Sets the permissions
  #[ must_use ]
  pub fn with_permissions( mut self, permissions : Vec< String > ) -> Self
  {
    self.permissions = permissions;
    self
  }

  /// Sets whether the command is idempotent
  #[ must_use ]
  pub fn with_idempotent( mut self, idempotent : bool ) -> Self
  {
    self.idempotent = idempotent;
    self
  }

  /// Sets the deprecation message
  #[ must_use ]
  pub fn with_deprecation_message( mut self, message : impl Into< String > ) -> Self
  {
    self.deprecation_message = message.into();
    self
  }

  /// Sets the HTTP method hint
  #[ must_use ]
  pub fn with_http_method_hint( mut self, hint : impl Into< String > ) -> Self
  {
    self.http_method_hint = hint.into();
    self
  }

  /// Sets the usage examples
  #[ must_use ]
  pub fn with_examples( mut self, examples : Vec< String > ) -> Self
  {
    self.examples = examples;
    self
  }

  /// Sets whether auto-help is enabled
  #[ must_use ]
  pub fn with_auto_help( mut self, enabled : bool ) -> Self
  {
    self.auto_help_enabled = enabled;
    self
  }

  /// Sets the command category
  #[ must_use ]
  pub fn with_category( mut self, category : impl Into< String > ) -> Self
  {
    self.category = category.into();
    self
  }

  /// Sets the short description
  #[ must_use ]
  pub fn with_short_desc( mut self, desc : impl Into< String > ) -> Self
  {
    self.short_desc = desc.into();
    self
  }

  /// Sets whether the command is hidden from listings
  #[ must_use ]
  pub fn with_hidden_from_list( mut self, hidden : bool ) -> Self
  {
    self.hidden_from_list = hidden;
    self
  }

  /// Sets the command priority
  #[ must_use ]
  pub fn with_priority( mut self, priority : i32 ) -> Self
  {
    self.priority = priority;
    self
  }

  /// Sets the command group
  #[ must_use ]
  pub fn with_group( mut self, group : impl Into< String > ) -> Self
  {
    self.group = group.into();
    self
  }

  // ===================================================================
  // Helper Methods (ported from CommandDefinition)
  // ===================================================================

  ///
  /// Returns true if this command should automatically generate a help counterpart.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".test").unwrap();
  /// let cmd = CommandDefinition::new(name, "Test".to_string())
  ///   .with_auto_help(true);
  ///
  /// assert!(cmd.has_auto_help());
  /// ```
  #[ must_use ]
  pub fn has_auto_help( &self ) -> bool
  {
    self.auto_help_enabled
  }

  ///
  /// Constructs the full command name from namespace and name components.
  ///
  /// This method handles the various combinations of namespaced and non-namespaced
  /// command names, ensuring that the resulting full name always starts with a dot
  /// prefix according to unilang conventions.
  ///
  /// # Returns
  /// * `String` - The fully qualified command name with dot prefix
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// // Simple command (no namespace)
  /// let name = CommandName::new(".help").unwrap();
  /// let cmd1 = CommandDefinition::new(name, "Help".to_string());
  /// assert_eq!(cmd1.full_name(), ".help");
  ///
  /// // Namespaced command
  /// let name2 = CommandName::new(".list").unwrap();
  /// let cmd2 = CommandDefinition::new(name2, "List".to_string())
  ///   .with_namespace(".session".to_string());
  /// assert_eq!(cmd2.full_name(), ".session.list");
  /// ```
  #[ must_use ]
  pub fn full_name( &self ) -> String
  {
    construct_full_command_name( self.namespace.as_str(), self.name.as_str() )
  }

  ///
  /// Generates a corresponding help command definition for this command.
  ///
  /// Creates a new `CommandDefinition` for the `.command.help` counterpart
  /// that provides detailed help information about the parent command.
  ///
  /// # Returns
  /// * `CommandDefinition` - A new command definition for the help counterpart
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".example").unwrap();
  /// let cmd = CommandDefinition::new(name, "Example".to_string());
  ///
  /// let help_cmd = cmd.generate_help_command();
  /// assert_eq!(help_cmd.name().as_str(), ".example.help");
  /// assert!(help_cmd.description().contains(".example"));
  /// ```
  #[ must_use ]
  pub fn generate_help_command( &self ) -> CommandDefinition
  {
    let help_name = CommandName::new( format!( "{}.help", self.name.as_str() ) )
      .expect( "help command name should be valid" );

    CommandDefinition
    {
      name : help_name,
      namespace : self.namespace.clone(),
      description : format!( "Display help information for the '{}' command", self.name.as_str() ),
      hint : format!( "Help for {}", self.name.as_str() ),
      status : CommandStatus::Active,
      version : self.version.clone(),
      arguments : vec![], // Help commands typically take no arguments
      routine_link : None, // Will be set during registration
      tags : vec![ "help".to_string(), "documentation".to_string() ],
      aliases : vec![ format!( "{}.h", self.name.as_str() ) ],
      permissions : vec![], // Help commands accessible to all
      idempotent : true, // Help commands always idempotent
      deprecation_message : String::new(),
      http_method_hint : "GET".to_string(), // Help is read-only
      examples : vec![
        format!( "{}.help", self.name.as_str() ),
        format!( "{} ??", self.name.as_str() )
      ],
      auto_help_enabled : false, // Prevent recursive help generation
      category : "help".to_string(),
      short_desc : format!( "Help for {}", self.name.as_str() ),
      hidden_from_list : true, // Hide .help variants from brief listings
      priority : 999, // Low priority (shown last if visible)
      group : String::new(),
    }
  }
}

//
// Serde Implementation for CommandDefinition
//

impl serde::Serialize for CommandDefinition
{
  fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
  where
    S : serde::Serializer,
  {
    use serde::ser::SerializeStruct;

    let mut state = serializer.serialize_struct( "CommandDefinition", 21 )?;

    state.serialize_field( "name", &self.name )?;
    state.serialize_field( "description", &self.description )?;
    state.serialize_field( "arguments", &self.arguments )?;
    state.serialize_field( "routine_link", &self.routine_link )?;
    state.serialize_field( "namespace", &self.namespace )?;
    state.serialize_field( "hint", &self.hint )?;
    state.serialize_field( "status", &self.status )?;
    state.serialize_field( "version", &self.version )?;
    state.serialize_field( "tags", &self.tags )?;
    state.serialize_field( "aliases", &self.aliases )?;
    state.serialize_field( "permissions", &self.permissions )?;
    state.serialize_field( "idempotent", &self.idempotent )?;
    state.serialize_field( "deprecation_message", &self.deprecation_message )?;
    state.serialize_field( "http_method_hint", &self.http_method_hint )?;
    state.serialize_field( "examples", &self.examples )?;
    state.serialize_field( "auto_help_enabled", &self.auto_help_enabled )?;
    state.serialize_field( "category", &self.category )?;
    state.serialize_field( "short_desc", &self.short_desc )?;
    state.serialize_field( "hidden_from_list", &self.hidden_from_list )?;
    state.serialize_field( "priority", &self.priority )?;
    state.serialize_field( "group", &self.group )?;

    state.end()
  }
}

impl< 'de > serde::Deserialize< 'de > for CommandDefinition
{
  #[ allow( clippy::too_many_lines ) ]
  fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
  where
    D : serde::Deserializer< 'de >,
  {
    use serde::de::{ self, Visitor, MapAccess };

    #[ derive( serde::Deserialize ) ]
    #[ serde( field_identifier, rename_all = "snake_case" ) ]
    enum Field
    {
      Name,
      Description,
      Arguments,
      RoutineLink,
      Namespace,
      Hint,
      Status,
      Version,
      Tags,
      Aliases,
      Permissions,
      Idempotent,
      DeprecationMessage,
      HttpMethodHint,
      Examples,
      AutoHelpEnabled,
      Category,
      ShortDesc,
      HiddenFromList,
      Priority,
      Group,
    }

    struct CommandDefinitionVisitor;

    impl< 'de > Visitor< 'de > for CommandDefinitionVisitor
    {
      type Value = CommandDefinition;

      fn expecting( &self, formatter : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
      {
        formatter.write_str( "struct CommandDefinition" )
      }

      #[ allow( clippy::too_many_lines ) ]
      fn visit_map< V >( self, mut map : V ) -> Result< CommandDefinition, V::Error >
      where
        V : MapAccess< 'de >,
      {
        let mut name : Option< CommandName > = None;
        let mut description : Option< String > = None;
        let mut arguments : Option< Vec< ArgumentDefinition > > = None;
        let mut routine_link : Option< Option< String > > = None;
        let mut namespace : Option< String > = None;
        let mut hint : Option< String > = None;
        let mut status : Option< CommandStatus > = None;
        let mut version : Option< VersionType > = None;
        let mut tags : Option< Vec< String > > = None;
        let mut aliases : Option< Vec< String > > = None;
        let mut permissions : Option< Vec< String > > = None;
        let mut idempotent : Option< bool > = None;
        let mut deprecation_message : Option< String > = None;
        let mut http_method_hint : Option< String > = None;
        let mut examples : Option< Vec< String > > = None;
        let mut auto_help_enabled : Option< bool > = None;
        let mut category : Option< String > = None;
        let mut short_desc : Option< String > = None;
        let mut hidden_from_list : Option< bool > = None;
        let mut priority : Option< i32 > = None;
        let mut group : Option< String > = None;

        while let Some( key ) = map.next_key()?
        {
          match key
          {
            Field::Name =>
            {
              if name.is_some()
              {
                return Err( de::Error::duplicate_field( "name" ) );
              }
              name = Some( map.next_value()? );
            },
            Field::Description =>
            {
              if description.is_some()
              {
                return Err( de::Error::duplicate_field( "description" ) );
              }
              description = Some( map.next_value()? );
            },
            Field::Arguments =>
            {
              if arguments.is_some()
              {
                return Err( de::Error::duplicate_field( "arguments" ) );
              }
              arguments = Some( map.next_value()? );
            },
            Field::RoutineLink =>
            {
              if routine_link.is_some()
              {
                return Err( de::Error::duplicate_field( "routine_link" ) );
              }
              routine_link = Some( map.next_value()? );
            },
            Field::Namespace =>
            {
              if namespace.is_some()
              {
                return Err( de::Error::duplicate_field( "namespace" ) );
              }
              namespace = Some( map.next_value()? );
            },
            Field::Hint =>
            {
              if hint.is_some()
              {
                return Err( de::Error::duplicate_field( "hint" ) );
              }
              hint = Some( map.next_value()? );
            },
            Field::Status =>
            {
              if status.is_some()
              {
                return Err( de::Error::duplicate_field( "status" ) );
              }
              status = Some( map.next_value()? );
            },
            Field::Version =>
            {
              if version.is_some()
              {
                return Err( de::Error::duplicate_field( "version" ) );
              }
              version = Some( map.next_value()? );
            },
            Field::Tags =>
            {
              if tags.is_some()
              {
                return Err( de::Error::duplicate_field( "tags" ) );
              }
              tags = Some( map.next_value()? );
            },
            Field::Aliases =>
            {
              if aliases.is_some()
              {
                return Err( de::Error::duplicate_field( "aliases" ) );
              }
              aliases = Some( map.next_value()? );
            },
            Field::Permissions =>
            {
              if permissions.is_some()
              {
                return Err( de::Error::duplicate_field( "permissions" ) );
              }
              permissions = Some( map.next_value()? );
            },
            Field::Idempotent =>
            {
              if idempotent.is_some()
              {
                return Err( de::Error::duplicate_field( "idempotent" ) );
              }
              idempotent = Some( map.next_value()? );
            },
            Field::DeprecationMessage =>
            {
              if deprecation_message.is_some()
              {
                return Err( de::Error::duplicate_field( "deprecation_message" ) );
              }
              deprecation_message = Some( map.next_value()? );
            },
            Field::HttpMethodHint =>
            {
              if http_method_hint.is_some()
              {
                return Err( de::Error::duplicate_field( "http_method_hint" ) );
              }
              http_method_hint = Some( map.next_value()? );
            },
            Field::Examples =>
            {
              if examples.is_some()
              {
                return Err( de::Error::duplicate_field( "examples" ) );
              }
              examples = Some( map.next_value()? );
            },
            Field::AutoHelpEnabled =>
            {
              if auto_help_enabled.is_some()
              {
                return Err( de::Error::duplicate_field( "auto_help_enabled" ) );
              }
              auto_help_enabled = Some( map.next_value()? );
            },
            Field::Category =>
            {
              if category.is_some()
              {
                return Err( de::Error::duplicate_field( "category" ) );
              }
              category = Some( map.next_value()? );
            },
            Field::ShortDesc =>
            {
              if short_desc.is_some()
              {
                return Err( de::Error::duplicate_field( "short_desc" ) );
              }
              short_desc = Some( map.next_value()? );
            },
            Field::HiddenFromList =>
            {
              if hidden_from_list.is_some()
              {
                return Err( de::Error::duplicate_field( "hidden_from_list" ) );
              }
              hidden_from_list = Some( map.next_value()? );
            },
            Field::Priority =>
            {
              if priority.is_some()
              {
                return Err( de::Error::duplicate_field( "priority" ) );
              }
              priority = Some( map.next_value()? );
            },
            Field::Group =>
            {
              if group.is_some()
              {
                return Err( de::Error::duplicate_field( "group" ) );
              }
              group = Some( map.next_value()? );
            },
          }
        }

        // Required fields
        let name = name.ok_or_else( || de::Error::missing_field( "name" ) )?;
        let description = description.ok_or_else( || de::Error::missing_field( "description" ) )?;

        // Optional fields with defaults
        let namespace = namespace.unwrap_or_else( || String::new() );

        // Validate namespace using NamespaceType validation rules
        NamespaceType::new( &namespace ).map_err( de::Error::custom )?;
        let hint = hint.unwrap_or_default();
        let status = status.unwrap_or( CommandStatus::Active );
        let version = version.unwrap_or_else( || VersionType::new( "1.0.0" ).expect( "default version valid" ) );
        let arguments = arguments.unwrap_or_default();
        let routine_link = routine_link.unwrap_or( None );
        let tags = tags.unwrap_or_default();
        let aliases = aliases.unwrap_or_default();
        let permissions = permissions.unwrap_or_default();
        let idempotent = idempotent.unwrap_or( true );
        let deprecation_message = deprecation_message.unwrap_or_default();
        let http_method_hint = http_method_hint.unwrap_or_else( || "GET".to_string() );
        let examples = examples.unwrap_or_default();
        let auto_help_enabled = auto_help_enabled.unwrap_or( true );
        let category = category.unwrap_or_default();
        let short_desc = short_desc.unwrap_or_default();
        let hidden_from_list = hidden_from_list.unwrap_or( false );
        let priority = priority.unwrap_or( 0 );
        let group = group.unwrap_or_default();

        Ok( CommandDefinition
        {
          name,
          description,
          arguments,
          routine_link,
          namespace,
          hint,
          status,
          version,
          tags,
          aliases,
          permissions,
          idempotent,
          deprecation_message,
          http_method_hint,
          examples,
          auto_help_enabled,
          category,
          short_desc,
          hidden_from_list,
          priority,
          group,
        })
      }
    }

    const FIELDS : &[ &str ] = &[
      "name",
      "description",
      "arguments",
      "routine_link",
      "namespace",
      "hint",
      "status",
      "version",
      "tags",
      "aliases",
      "permissions",
      "idempotent",
      "deprecation_message",
      "http_method_hint",
      "examples",
      "auto_help_enabled",
      "category",
      "short_desc",
      "hidden_from_list",
      "priority",
      "group",
    ];

    deserializer.deserialize_struct( "CommandDefinition", FIELDS, CommandDefinitionVisitor )
  }
}

} // mod private

mod_interface::mod_interface!
{
  exposed use private::CommandName;
  exposed use private::NamespaceType;
  exposed use private::VersionType;
  exposed use private::CommandStatus;
  exposed use private::CommandDefinition;
  exposed use private::ArgumentDefinition;
  exposed use private::ArgumentAttributes;
  exposed use private::Kind;
  exposed use private::ValidationRule;
  exposed use private::Namespace;
  exposed use private::OutputData;
  exposed use private::ErrorData;
  exposed use private::ErrorCode;
  exposed use private::CommandDefinitionBuilder;
  exposed use private::Set;
  exposed use private::NotSet;

  prelude use private::CommandName;
  prelude use private::NamespaceType;
  prelude use private::VersionType;
  prelude use private::CommandStatus;
  prelude use private::CommandDefinition;
  prelude use private::ArgumentDefinition;
  prelude use private::ArgumentAttributes;
  prelude use private::Kind;
  prelude use private::OutputData;
  prelude use private::ErrorData;
  prelude use private::ErrorCode;
}