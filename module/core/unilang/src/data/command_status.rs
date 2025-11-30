//! Command status and lifecycle management
//!
//! Provides type-safe command status enumeration for managing command lifecycle
//! stages (active, experimental, deprecated, internal) with structured metadata.


  ///
  /// Command status indicating lifecycle stage and availability.
  ///
  /// # Type Safety Guarantees
  /// - No typos in status strings (compile-time checked)
  /// - Structured deprecation data
  /// - Clear distinction between stable/experimental/internal
  ///
  /// # Examples
  /// ```
  /// use unilang::data::CommandStatus;
  ///
  /// let active = CommandStatus::Active;
  /// assert!(active.is_active());
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


