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
