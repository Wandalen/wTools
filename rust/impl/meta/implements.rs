#![ warn( missing_docs ) ]

//!
//! Module::implements with macro `implements` to answer the question: does it implement a trait?
//!

///
/// Macro `implements` to answer the question: does it implement a trait?
///
/// # Example
/// ```
/// dbg!( implements!( 13_i32 => Copy ) );
/// // < implements!( 13_i32 => Copy ) : true
/// dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
/// // < implements!( 13_i32 => Copy ) : false
/// ```

#[ macro_export ]
macro_rules! implements
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

  }}
}

///
/// Macro `instance_of` to answer the question: does it implement a trait? Alias of the macro `implements`.
///
/// # Example
/// ```
/// dbg!( instance_of!( 13_i32 => Copy ) );
/// // < instance_of!( 13_i32 => Copy ) : true
/// dbg!( instance_of!( Box::new( 13_i32 ) => Copy ) );
/// // < instance_of!( 13_i32 => Copy ) : false
/// ```

#[ macro_export ]
macro_rules! instance_of
{
  ( $( $arg : tt )+ ) =>
  {
    $crate::implements!( $( $arg )+ );
  }
}
