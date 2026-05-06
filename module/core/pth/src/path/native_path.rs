/// Type alias for `NormalizedPath`.
///
/// `NativePath` is a maintained alias to `NormalizedPath` for semantic clarity.
/// Use this type when you want to emphasize native path handling semantics.
///
/// The `NativePath` and `CanonicalPath` types were consolidated into `NormalizedPath`
/// as of v0.30.0 to eliminate code duplication. Both aliases remain available as
/// permanent, semantically meaningful type names.
///
/// # Usage
///
/// ```rust
/// use pth::NativePath;
/// let path : NativePath = NativePath ::try_from( "/some/path" ).unwrap();
/// let _ = path;
/// ```
///
/// # Note
///
/// `NativePath`, `CanonicalPath`, and `NormalizedPath` are all the same type.
/// Choose whichever name best expresses your intent in the given context.
mod private
{
  use super::super::normalized_path::NormalizedPath;

  /// Type alias for `NormalizedPath`.
  ///
  /// `NativePath` and `CanonicalPath` were functionally identical types that both represented
  /// paths normalized via `path::canonicalize()`. They have been consolidated into a single
  /// `NormalizedPath` type to eliminate code duplication, with both original names maintained
  /// as permanent aliases for semantic clarity.
  ///
  /// # Rationale
  ///
  /// The anticipated platform-specific differences between `NativePath` and `CanonicalPath`
  /// never materialized. Both types had identical implementations (338 lines of duplicated code),
  /// causing maintenance burden with no functional benefit. Rather than deprecate the names,
  /// they are maintained as aliases to preserve semantic meaning in different contexts.
  ///
  /// See spec ADR-009 for full architectural decision record.
  pub type NativePath = NormalizedPath;
}

crate ::mod_interface!
{
  exposed use NativePath;
}
