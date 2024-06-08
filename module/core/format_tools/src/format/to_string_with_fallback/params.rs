//!
//! Marker type for trait `_ToStringWithFallback` with type parameters.
//!

/// Marker type for trait `_ToStringWithFallback` with type parameters.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct ToStringWithFallbackParams< How, Fallback >( ::core::marker::PhantomData< fn() -> ( How, Fallback ) > );
