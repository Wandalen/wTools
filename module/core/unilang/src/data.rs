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

// Extracted modules for code organization (per codebase_hygiene.rulebook.md)
mod error_types;
mod validated_types;
mod command_status;
mod argument_types;
mod namespace;
mod command_definition;

/// Internal namespace (placeholder for potential future private types).
mod private
{
  // All data types have been extracted to dedicated modules.
  // This module remains as a placeholder for mod_interface compatibility.
} // mod private
mod_interface::mod_interface!
{
  exposed use validated_types::{ CommandName, NamespaceType, VersionType };
  exposed use command_status::CommandStatus;
  exposed use argument_types::{ ArgumentAttributes, ArgumentDefinition, Kind, ValidationRule };
  exposed use command_definition::{ CommandDefinition, CommandDefinitionBuilder, Set, NotSet };
  exposed use namespace::{ Namespace, OutputData };
  exposed use error_types::{ ErrorData, ErrorCode };

  prelude use validated_types::{ CommandName, NamespaceType, VersionType };
  prelude use command_status::CommandStatus;
  prelude use command_definition::CommandDefinition;
  prelude use argument_types::{ ArgumentDefinition, ArgumentAttributes, Kind };
  prelude use namespace::OutputData;
  prelude use error_types::{ ErrorData, ErrorCode };
}
