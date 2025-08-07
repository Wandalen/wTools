#![allow(unused_imports)]
#![allow(dead_code)]

use test_tools::prelude::*;
use core::marker::PhantomData;
use core::marker::PhantomData as CorePhantomData;

pub struct BoundsMixed<T: ToString, U> {
  _phantom: CorePhantomData<(T, U)>,
}

// Shared test logic
include!("../phantom_only_test.rs");
