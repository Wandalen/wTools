// Compilation guard for the no-features (`default = []`) opt-in model.
// This test runs when no features are active, proving the crate compiles
// with zero API surface exposed — no accidental activation through transitive deps.
//
// Fix( no-features-invariant )
// Root cause: switching to `default = []` could silently regress if any code
//   migrates outside a `#[cfg(feature = "enabled")]` gate; this catches it early.
// Pitfall: do not add `use error_tools::...` here — all public items are gated
//   on `enabled` and would cause a compile error in the no-features context.

#[ cfg( not( feature = "enabled" ) ) ]
#[ test ]
fn no_features_compiles()
{
}
