# Integration: former

### Scope

- **Purpose**: Document the consumer relationship where `former` re-exports this crate's macros.
- **Responsibility**: Describe how `former` surfaces this crate's macros and its compatibility implications.
- **In Scope**: How `former` re-exports macros from this crate and version coordination requirements.
- **Out of Scope**: The `former` crate's own features — see that crate's own documentation.

### System Description

`former` is the public facade crate that consumers depend on. It re-exports the
procedural macros defined in this crate and provides additional convenience items
alongside them. Consumers should only add `former` to their dependencies — they do
not reference this crate directly.

The separation exists because Rust requires procedural macro implementations to live in
a dedicated proc-macro crate. `former` is not a proc-macro crate and therefore can serve
as a normal library dependency while also re-exporting the macros from `former_meta`.

### Integration Points

`former` lists this crate as a dependency and re-exports its macros under its own
namespace. The `Former` derive macro and all field attributes are accessible to consumers
through `former` without any direct reference to `former_meta`.

### Error Handling

Since this crate's macros are re-exported through `former`, diagnostic messages seen by
consumers reference macro names as they appear in `former`'s re-exports. Span information
points to the macro call site in consumer code.

### Compatibility Requirements

The public interface of this crate is defined by what `former` re-exports. Changes to
macro names or behavior in this crate constitute breaking changes from the perspective
of `former` consumers and must be coordinated with a version bump in `former`.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../invariant/001_proc_macro_separation.md` | Why this crate and former are separate |
| doc | `../feature/001_former_derive.md` | Macro capability re-exported by former |
| doc | `../api/001_derive_api.md` | Macro interface re-exported by former |
