#[ doc( hidden ) ]
#[ macro_export ]
macro_rules! _implements
{
  ( $V : expr => $( $Traits : tt )+ ) =>
  {{
    use ::core::marker::PhantomData;

    trait False
    {
      fn get( self : &'_ Self ) -> bool { false }
    }

    impl< T > False
    for &'_ PhantomData< T >
    where T : ?Sized,
    {}

    trait True
    {
      fn get( self : &'_ Self ) -> bool { true }
    }

    impl< T > True
    for PhantomData< T >
    where T : $( $Traits )+ + ?Sized,
    {}

    fn does< T : Sized >( _ : &T ) -> PhantomData< T >
    {
      PhantomData
    }
    ( &does( &$V ) ).get()

  }};
}
