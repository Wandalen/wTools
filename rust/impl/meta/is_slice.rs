#![ warn( missing_docs ) ]

//!
//! Macro to answer the question: is it a slice?
//!

/// Macro to answer the question: is it a slice?
///
/// # Sample
/// ```
/// use is_slice::*;
///
/// fn main()
/// {
///   dbg!( is_slice!( &[ 1, 2, 3 ] ) );
///   dbg!( is_slice!( Box::new( true ) ) );
/// }
/// ```

#[ macro_export ]
macro_rules! is_slice
{
  ( $V : expr ) =>
  {{
    use ::core::marker::PhantomData;

    trait NotSlice
    {
      fn is_slice( self : &'_ Self ) -> bool { false }
    }

    impl< T > NotSlice
    for &'_ PhantomData< T >
    where T : ?Sized,
    {}

    trait Slice
    {
      fn is_slice( self : &'_ Self ) -> bool { true }
    }

    impl< 'a, T > Slice for PhantomData< &'a &[ T ] >
    {}

    fn does< T : Sized >( _ : &T ) -> PhantomData< &T >
    {
      PhantomData
    }

    ( &does( &$V ) ).is_slice()

  }}
}
