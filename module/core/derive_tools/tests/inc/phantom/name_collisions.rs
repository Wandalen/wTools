#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;
use std::marker::PhantomData;
use core::marker::PhantomData as CorePhantomData;


pub struct NameCollisions< T >
{
  _phantom : CorePhantomData< T >,
}

// Shared test logic
include!( "../phantom_only_test.rs" );