#![ warn( missing_docs ) ]

//!
//! Module::instance_of with macro `instance_of` to answer the question: does it implement a trait?
//!

// #[ macro_use ]
mod implements_impl;

///
/// Macro `implements` to answer the question: does it implement a trait?
///
/// # Example
/// ```
/// use instance_of::implements;
///
/// dbg!( implements!( 13_i32 => Copy ) );
/// // < implements!( 13_i32 => Copy ) : true
/// dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
/// // < implements!( 13_i32 => Copy ) : false
/// ```

#[ macro_export ]
macro_rules! implements
{
  ( $( $arg : tt )+ ) =>
  {
    $crate::_implements!( $( $arg )+ );
  }
}

///
/// Macro `instance_of` to answer the question: does it implement a trait? Alias of the macro `implements`.
///
/// # Example
/// ```
/// use instance_of::instance_of;
///
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
    $crate::_implements!( $( $arg )+ );
  }
}
