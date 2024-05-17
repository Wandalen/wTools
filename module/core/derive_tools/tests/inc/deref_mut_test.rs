use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::Deref, the_module::DerefMut ) ]
pub struct IsTransparentSimple( bool );

#[ derive( Debug, Clone, Copy, PartialEq, the_module::Deref, the_module::DerefMut ) ]
pub struct IsTransparentComplex< 'a, 'b : 'a, T, U : ToString + ?Sized, const N : usize >( &'a T, core::marker::PhantomData< &'b U > )
where
    'a : 'b,
    T : AsRef< U >;

include!( "./only_test/deref_mut.rs" );
