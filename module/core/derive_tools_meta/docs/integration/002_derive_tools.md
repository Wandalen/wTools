# Integration: derive_tools

### Scope

- Documents the consumer relationship with `derive_tools`.
- Covers how `derive_tools` re-exports and surfaces this crate's macros.
- Audience: contributors understanding the two-crate split or adding new macros.
- Does not describe the `derive_tools` crate's own features.

### System Description

`derive_tools` is the public facade crate that consumers depend on. It
re-exports the procedural macros defined in this crate and may provide
additional convenience items alongside them.

The separation exists because Rust requires procedural macro implementations
to live in a dedicated proc-macro crate. `derive_tools` is not a proc-macro
crate and therefore can serve as a normal library dependency.

### Integration Points

`derive_tools` lists this crate as a dependency and re-exports its macros
under its own namespace. Consumers only need to add `derive_tools` to their
dependencies — they do not reference this crate directly.

### Error Handling

Since this crate's macros are re-exported through `derive_tools`, diagnostic
messages seen by consumers will reference macro names as they appear in
`derive_tools`'s re-exports. Span information points to the macro call site.

### Compatibility Requirements

The public interface of this crate is defined by what `derive_tools` re-exports.
Changes to macro names or behavior in this crate constitute breaking changes
from the perspective of `derive_tools` consumers and must be coordinated with
a version bump in `derive_tools`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| invariant | docs/invariant/001_proc_macro_separation.md | Why this crate and derive_tools are separate |
| feature | docs/feature/001_derive_macros.md | Macro collection re-exported by derive_tools |
| api | docs/api/001_derive_api.md | Macro interface re-exported by derive_tools |

### Sources

- [../../spec.md](../../spec.md) — Consumers and Related Crates sections
