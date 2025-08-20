// use test_tools::exposed::*;
use super::*;

//

#[ test ]
fn implements_basic() {
  trait Trait1 {}
  fn impl_trait1(_: &impl Trait1) -> bool {
    true
  }

  impl<T: Sized> Trait1 for &[T] {}
  impl<T: Sized, const N: usize> Trait1 for [T; N] {}
  impl<T: Sized, const N: usize> Trait1 for &[T; N] {}
  let src: &[i32] = &[1, 2, 3];
  assert!(the_module::implements!( src => Trait1 ));
  assert!(impl_trait1(&src));
  assert!(the_module::implements!( &[ 1, 2, 3 ] => Trait1 ));
  assert!(impl_trait1(&[1, 2, 3]));
  assert!(the_module::implements!( [ 1, 2, 3 ] => Trait1 ));

  impl<T: Sized> Trait1 for Vec<T> {}
  assert!(the_module::implements!( std::vec!( 1, 2, 3 ) => Trait1 ));

  impl Trait1 for f32 {}
  assert!(the_module::implements!( 13_f32 => Trait1 ));

  assert!(the_module::implements!( true => Copy ));
  assert!(the_module::implements!( true => Clone ));

  let src = true;
  assert!(the_module::implements!( src => Copy ));
  assert!(the_module::implements!( src => Clone ));

  let src = Box::new(true);
  assert!(!the_module::implements!( src => Copy ));
  assert!(the_module::implements!( src => Clone ));

  assert!(!the_module::implements!( Box::new( true ) => core::marker::Copy ));
  assert!(the_module::implements!( Box::new( true ) => core::clone::Clone ));
}

//

#[ test ]
fn instance_of_basic() {
  let src = Box::new(true);
  assert!(!the_module::instance_of!( src => Copy ));
  assert!(the_module::instance_of!( src => Clone ));
}

//

#[ test ]
fn implements_functions() {
  let test_f_simple = || {
    println!("hello");
  };
  let _ = test_f_simple; // Explicitly ignore to prevent unused warning

  let fn_context = std::vec![1, 2, 3];
  let test_fn = || {
    println!("hello {fn_context:?}");
  };

  let mut fn_mut_context = std::vec![1, 2, 3];
  let test_fn_mut = || {
    fn_mut_context[0] = 3;
    println!("{fn_mut_context:?}");
  };

  let mut fn_once_context = std::vec![1, 2, 3];
  let test_fn_once = || {
    fn_once_context[0] = 3;
    let x = fn_once_context;
    println!("{x:?}");
  };

  /* */

  assert!(the_module::implements!( test_fn => Copy ));
  assert!(the_module::implements!( test_fn => Clone ));
  assert!(!the_module::implements!( test_fn => core::ops::Not ));
  let _ = test_fn;

  /* */

  // assert_eq!( the_module::implements!( function1 => fn() -> () ), true );
  // assert_eq!( the_module::implements!( &function1 => Fn() -> () ), true );
  // assert_eq!( the_module::implements!( &function1 => FnMut() -> () ), true );
  // assert_eq!( the_module::implements!( &function1 => FnOnce() -> () ), true );

  // assert_eq!( the_module::implements!( test_fn => fn() -> () ), true );
  assert!(the_module::implements!( test_fn => Fn() ));
  assert!(the_module::implements!( test_fn => FnMut() ));
  assert!(the_module::implements!( test_fn => FnOnce() ));

  // assert_eq!( the_module::implements!( test_fn_mut => fn() -> () ), false );
  // assert_eq!( the_module::implements!( test_fn_mut => Fn() -> () ), false );
  assert!(the_module::implements!( test_fn_mut => FnMut() ));
  assert!(the_module::implements!( test_fn_mut => FnOnce() ));

  // assert_eq!( the_module::implements!( test_fn_once => fn() -> () ), false );
  // assert_eq!( the_module::implements!( test_fn_once => Fn() -> () ), false );
  // assert_eq!( the_module::implements!( test_fn_once => FnMut() -> () ), false );
  assert!(the_module::implements!( test_fn_once => FnOnce() ));

  // fn is_f < R >                             ( _x : fn() -> R )      -> bool { true }
  // fn is_fn < R, F : Fn() -> R >             ( _x : &F )             -> bool { true }
  // fn is_fn_mut < R, F : FnMut() -> R >      ( _x : &F )             -> bool { true }
  // fn is_fn_once < R, F : FnOnce() -> R >    ( _x : &F )             -> bool { true }
  // fn function1() -> bool { true }
}

//

#[ test ]
fn pointer_experiment() {
  let pointer_size = core::mem::size_of::<&u8>();
  dbg!(&pointer_size);
  assert_eq!(2 * pointer_size, core::mem::size_of::<&[u8]>());
  assert_eq!(2 * pointer_size, core::mem::size_of::<*const [u8]>());
  assert_eq!(2 * pointer_size, core::mem::size_of::<Box< [u8] >>());
  assert_eq!(2 * pointer_size, core::mem::size_of::<std::rc::Rc< [u8] >>());
  assert_eq!(pointer_size, core::mem::size_of::<&[u8; 20]>());
}

//

#[ test ]
fn fn_experiment() {
  fn function1() -> bool {
    true
  }

  let test_closure = || {
    println!("hello");
  };

  let fn_context = std::vec![1, 2, 3];
  let test_fn_capture = || {
    println!("hello {fn_context:?}");
  };

  let mut fn_mut_context = std::vec![1, 2, 3];
  let test_fn_mut2 = || {
    fn_mut_context[0] = 3;
    println!("{fn_mut_context:?}");
  };

  let mut fn_once_context = std::vec![1, 2, 3];
  let test_fn_once2 = || {
    fn_once_context[0] = 3;
    let x = fn_once_context;
    println!("{x:?}");
  };

  assert!(is_f(function1));
  assert!(is_fn(&function1));
  assert!(is_fn_mut(&function1));
  assert!(is_fn_once(&function1));

  assert!(is_f(test_closure));
  assert!(is_fn(&test_closure));
  assert!(is_fn_mut(&test_closure));
  assert!(is_fn_once(&test_closure));

  // assert_eq!( is_f( test_fn_capture ), true );
  assert!(is_fn(&test_fn_capture));
  assert!(is_fn_mut(&test_fn_capture));
  assert!(is_fn_once(&test_fn_capture));

  // assert_eq!( is_f( test_fn_mut2 ), true );
  // assert_eq!( is_fn( &test_fn_mut2 ), true );
  assert!(is_fn_mut(&test_fn_mut2));
  assert!(is_fn_once(&test_fn_mut2));

  // assert_eq!( is_f( test_fn_once2 ), true );
  // assert_eq!( is_fn( &test_fn_once2 ), true );
  // assert_eq!( is_fn_mut( &test_fn_once2 ), true );
  assert!(is_fn_once(&test_fn_once2));

  // type Routine< R > = fn() -> R;
  fn is_f<R>(_x: fn() -> R) -> bool {
    true
  }
  // fn is_f < R >                             ( _x : Routine< R > )   -> bool { true }
  fn is_fn<R, F: Fn() -> R>(_x: &F) -> bool {
    true
  }
  fn is_fn_mut<R, F: FnMut() -> R>(_x: &F) -> bool {
    true
  }
  fn is_fn_once<R, F: FnOnce() -> R>(_x: &F) -> bool {
    true
  }
}

//

// tests_index!
// {
//   implements_basic,
//   instance_of_basic,
//   implements_functions,
//   pointer_experiment,
//   fn_experiment,
// }
