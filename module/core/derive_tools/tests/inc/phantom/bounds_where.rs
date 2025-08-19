#![allow(unused_imports)]
#![allow(dead_code)]

use test_tools::*;
use core::marker::PhantomData;
use core::marker::PhantomData as CorePhantomData;

pub struct BoundsWhere<T, U>
where
  T: ToString,
{
  _phantom: CorePhantomData<(T, U)>,
}

// Shared test logic
include!("../phantom_only_test.rs");
