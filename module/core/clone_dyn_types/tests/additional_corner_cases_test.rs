//! Additional corner case tests for `clone_dyn_types`
//!
//! Tests critical scenarios not covered by existing test suite

#![cfg(feature = "enabled")]

use clone_dyn_types::{CloneDyn, clone_into_box};

/// Helper trait for iterator testing
pub trait IterTrait<'a, T>
where
  T: 'a,
  Self: Iterator<Item = T> + ExactSizeIterator<Item = T> + DoubleEndedIterator,
  Self: CloneDyn,
{}

impl<'a, T, I> IterTrait<'a, T> for I
where
  T: 'a,
  Self: Iterator<Item = T> + ExactSizeIterator<Item = T> + DoubleEndedIterator,
  Self: CloneDyn,
{}

#[allow(non_local_definitions)]
impl<'c, T> Clone for Box<dyn IterTrait<'c, T> + 'c>
{
  fn clone(&self) -> Self
  {
    clone_into_box(&**self)
  }
}

/// Test: Single element iterator cloning
#[test]
fn iterator_clone_single_element()
{
  let data = [42];
  let iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  let cloned = iter.clone();

  let original_values: Vec<&i32> = iter.collect();
  let cloned_values: Vec<&i32> = cloned.collect();

  assert_eq!(original_values.len(), 1);
  assert_eq!(cloned_values.len(), 1);
  assert_eq!(*original_values[0], 42);
  assert_eq!(*cloned_values[0], 42);
}

/// Test: Multiple consecutive clones
#[test]
fn iterator_clone_multiple_consecutive()
{
  let data = [1, 2, 3];
  let iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  let clone1 = iter.clone();
  let clone2 = clone1.clone();
  let clone3 = clone2.clone();
  let clone4 = clone3.clone();

  assert_eq!(iter.count(), 3);
  assert_eq!(clone1.count(), 3);
  assert_eq!(clone2.count(), 3);
  assert_eq!(clone3.count(), 3);
  assert_eq!(clone4.count(), 3);
}

/// Test: Iterator independence after partial consumption - CRITICAL
#[test]
fn iterator_clone_independence_after_consumption()
{
  let data = [10, 20, 30, 40, 50];
  let mut iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  // Consume one element from original
  let first = iter.next();
  assert_eq!(first, Some(&10));

  // Clone after partial consumption
  let cloned = iter.clone();

  // Original should continue from where it left off
  let original_remaining: Vec<&i32> = iter.collect();
  assert_eq!(original_remaining, vec![&20, &30, &40, &50]);

  // Clone should start from same position (after consuming first element)
  let cloned_remaining: Vec<&i32> = cloned.collect();
  assert_eq!(cloned_remaining, vec![&20, &30, &40, &50]);
}

/// Test: `len()` accuracy after clone
#[test]
fn iterator_clone_len_accuracy()
{
  let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let mut iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  // Initial len
  assert_eq!(iter.len(), 10);

  // Consume 3 elements
  iter.next();
  iter.next();
  iter.next();

  // Check len after consumption
  assert_eq!(iter.len(), 7);

  // Clone and check both lengths
  let cloned = iter.clone();
  assert_eq!(iter.len(), 7);
  assert_eq!(cloned.len(), 7);

  // Consume from original, verify cloned unaffected
  iter.next();
  assert_eq!(iter.len(), 6);
  assert_eq!(cloned.len(), 7); // Should remain 7
}

/// Test: Clone and immediate drop (memory safety check)
#[test]
fn iterator_clone_and_immediate_drop()
{
  let data = [1, 2, 3];
  let iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  // Clone and immediately drop
  {
    let _cloned = iter.clone();
    // Cloned drops here
  }

  // Original should still work
  let count = iter.count();
  assert_eq!(count, 3);
}

/// Test: Large iterator (10,000 elements)
#[test]
fn iterator_clone_large()
{
  let data: Vec<i32> = (0..10000).collect();
  let iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  let cloned = iter.clone();

  let original_count = iter.count();
  let cloned_count = cloned.count();

  assert_eq!(original_count, 10000);
  assert_eq!(cloned_count, 10000);
}

/// Test: `DoubleEndedIterator` operations after clone
#[test]
fn iterator_clone_double_ended()
{
  let data = [1, 2, 3, 4, 5];
  let mut iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  // Use next_back before clone
  let last = iter.next_back();
  assert_eq!(last, Some(&5));

  // Clone after next_back
  let mut cloned = iter.clone();

  // Both should work with remaining elements
  assert_eq!(iter.next(), Some(&1));
  assert_eq!(cloned.next_back(), Some(&4));
}

/// Test: Clone after full consumption
#[test]
fn iterator_clone_after_full_consumption()
{
  let data = [1, 2, 3];
  let mut iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  // Fully consume original
  while iter.next().is_some() {}
  assert_eq!(iter.len(), 0);

  // Clone empty iterator
  let cloned = iter.clone();
  assert_eq!(cloned.len(), 0);
  assert_eq!(cloned.count(), 0);
}

/// Test: Mixed forward/backward iteration before clone
#[test]
fn iterator_clone_mixed_iteration()
{
  let data = [1, 2, 3, 4, 5, 6, 7];
  let mut iter: Box<dyn IterTrait<'_, &i32> + '_> = Box::new(data.iter());

  // Mixed iteration
  assert_eq!(iter.next(), Some(&1));
  assert_eq!(iter.next_back(), Some(&7));
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next_back(), Some(&6));

  // Clone after mixed iteration
  let cloned = iter.clone();

  // Both should have same remaining elements
  assert_eq!(iter.len(), 3);
  assert_eq!(cloned.len(), 3);

  let iter_remaining: Vec<&i32> = iter.collect();
  let cloned_remaining: Vec<&i32> = cloned.collect();

  assert_eq!(iter_remaining, vec![&3, &4, &5]);
  assert_eq!(cloned_remaining, vec![&3, &4, &5]);
}
