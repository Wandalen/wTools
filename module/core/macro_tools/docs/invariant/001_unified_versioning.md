# Invariant: Unified Versioning

### Scope
- **Purpose**: Prevent token type incompatibility caused by version mismatches in syn, quote, and proc-macro2 across consumers.
- **Responsibility**: Document the invariant that macro_tools is the sole direct dependent on proc-macro primitives within the ecosystem.
- **In Scope**: syn, quote, and proc-macro2 version ownership and re-export obligation.
- **Out of Scope**: Internal dependency versioning within macro_tools itself; runtime version detection.

### Invariant Statement
For every crate in the wTools ecosystem that performs proc-macro operations: all access to
syn, quote, and proc-macro2 occurs through macro_tools re-exports, never through independent
Cargo.toml dependency declarations on these libraries.

### Enforcement Mechanism
macro_tools is the single crate declaring syn, quote, and proc-macro2 in its dependencies.
Consumer meta-crates declare only macro_tools as a dependency and import all proc-macro
primitives through macro_tools. The workspace Cargo.toml reinforces this by not listing
syn, quote, or proc-macro2 as shared workspace dependencies — only macro_tools needs them.
Code review checks that no consumer Cargo.toml contains direct syn, quote, or proc-macro2
entries.

### Violation Consequences
If any consumer crate independently declares its own syn, quote, or proc-macro2 dependency
at a version that differs from macro_tools' version, even by a minor release: token types
from the two versions become incompatible at the type level despite appearing structurally
identical; proc-macro2 token stream values cannot cross the version boundary; the resulting
compile errors manifest as obscure type mismatches rather than clear version conflict
messages, making diagnosis difficult and time-consuming.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `Cargo.toml` | Declares the sole syn, quote, and proc-macro2 dependency versions |
| doc | `docs/pattern/001_abstraction_layer.md` | Architectural pattern that makes this invariant maintainable |
