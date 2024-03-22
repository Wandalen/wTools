// use test_tools::exposed::*;
use super::*;

//

tests_impls!
{

  #[ test ]
  fn implements_basic()
  {

    trait Trait1 {}
    fn impl_trait1( _ : &impl Trait1 ) -> bool { true }

    impl< T : Sized > Trait1 for &[ T ] {}
    impl< T : Sized, const N : usize > Trait1 for [ T; N ] {}
    impl< T : Sized, const N : usize > Trait1 for &[ T; N ] {}
    let src : &[ i32 ] = &[ 1, 2, 3 ];
    a_id!( the_module::implements!( src => Trait1 ), true );
    a_id!( impl_trait1( &src ), true );
    a_id!( the_module::implements!( &[ 1, 2, 3 ] => Trait1 ), true );
    a_id!( impl_trait1( &[ 1, 2, 3 ] ), true );
    a_id!( the_module::implements!( [ 1, 2, 3 ] => Trait1 ), true );

    impl< T : Sized > Trait1 for Vec< T > {}
    a_id!( the_module::implements!( vec!( 1, 2, 3 ) => Trait1 ), true );

    impl Trait1 for f32 {}
    a_id!( the_module::implements!( 13_f32 => Trait1 ), true );

    a_id!( the_module::implements!( true => Copy ), true );
    a_id!( the_module::implements!( true => Clone ), true );

    let src = true;
    a_id!( the_module::implements!( src => Copy ), true );
    a_id!( the_module::implements!( src => Clone ), true );

    let src = Box::new( true );
    a_id!( the_module::implements!( src => Copy ), false );
    a_id!( the_module::implements!( src => Clone ), true );

    a_id!( the_module::implements!( Box::new( true ) => std::marker::Copy ), false );
    a_id!( the_module::implements!( Box::new( true ) => std::clone::Clone ), true );

  }

  //

  #[ test ]
  fn instance_of_basic()
  {

    let src = Box::new( true );
    a_id!( the_module::instance_of!( src => Copy ), false );
    a_id!( the_module::instance_of!( src => Clone ), true );

  }

  //

  #[ test ]
  fn implements_functions()
  {

    let _f = ||
    {
      println!( "hello" );
    };

    let fn_context = vec!( 1, 2, 3 );
    let _fn = ||
    {
      println!( "hello {:?}", fn_context );
    };

    let mut fn_mut_context = vec!( 1, 2, 3 );
    let _fn_mut = ||
    {
      fn_mut_context[ 0 ] = 3;
      println!( "{:?}", fn_mut_context );
    };

    let mut fn_once_context = vec!( 1, 2, 3 );
    let _fn_once = ||
    {
      fn_once_context[ 0 ] = 3;
      let x = fn_once_context;
      println!( "{:?}", x );
    };

    /* */

    a_id!( the_module::implements!( _fn => Copy ), true );
    a_id!( the_module::implements!( _fn => Clone ), true );
    a_id!( the_module::implements!( _fn => core::ops::Not ), false );
    let _ = _fn.clone();

    /* */

    // a_id!( the_module::implements!( function1 => fn() -> () ), true );
    // a_id!( the_module::implements!( &function1 => Fn() -> () ), true );
    // a_id!( the_module::implements!( &function1 => FnMut() -> () ), true );
    // a_id!( the_module::implements!( &function1 => FnOnce() -> () ), true );

    // a_id!( the_module::implements!( _fn => fn() -> () ), true );
    a_id!( the_module::implements!( _fn => Fn() -> () ), true );
    a_id!( the_module::implements!( _fn => FnMut() -> () ), true );
    a_id!( the_module::implements!( _fn => FnOnce() -> () ), true );

    // a_id!( the_module::implements!( _fn_mut => fn() -> () ), false );
    // a_id!( the_module::implements!( _fn_mut => Fn() -> () ), false );
    a_id!( the_module::implements!( _fn_mut => FnMut() -> () ), true );
    a_id!( the_module::implements!( _fn_mut => FnOnce() -> () ), true );

    // a_id!( the_module::implements!( _fn_once => fn() -> () ), false );
    // a_id!( the_module::implements!( _fn_once => Fn() -> () ), false );
    // a_id!( the_module::implements!( _fn_once => FnMut() -> () ), false );
    a_id!( the_module::implements!( _fn_once => FnOnce() -> () ), true );

    // fn is_f < R >                             ( _x : fn() -> R )      -> bool { true }
    // fn is_fn < R, F : Fn() -> R >             ( _x : &F )             -> bool { true }
    // fn is_fn_mut < R, F : FnMut() -> R >      ( _x : &F )             -> bool { true }
    // fn is_fn_once < R, F : FnOnce() -> R >    ( _x : &F )             -> bool { true }
    // fn function1() -> bool { true }

  }

  //

  #[ test ]
  fn pointer_experiment()
  {

    let pointer_size = std::mem::size_of::< &u8 >();
    dbg!( &pointer_size );
    a_id!( 2 * pointer_size, std::mem::size_of::< &[ u8 ] >() );
    a_id!( 2 * pointer_size, std::mem::size_of::< *const [ u8 ] >() );
    a_id!( 2 * pointer_size, std::mem::size_of::< Box< [ u8 ] > >() );
    a_id!( 2 * pointer_size, std::mem::size_of::< std::rc::Rc< [ u8 ] > >() );
    a_id!( 1 * pointer_size, std::mem::size_of::< &[ u8 ; 20 ] >() );

  }

  //

  #[ test ]
  fn fn_experiment()
  {

    fn function1() -> bool { true }

    let _f = ||
    {
      println!( "hello" );
    };

    let fn_context = vec!( 1, 2, 3 );
    let _fn = ||
    {
      println!( "hello {:?}", fn_context );
    };

    let mut fn_mut_context = vec!( 1, 2, 3 );
    let _fn_mut = ||
    {
      fn_mut_context[ 0 ] = 3;
      println!( "{:?}", fn_mut_context );
    };

    let mut fn_once_context = vec!( 1, 2, 3 );
    let _fn_once = ||
    {
      fn_once_context[ 0 ] = 3;
      let x = fn_once_context;
      println!( "{:?}", x );
    };

    a_id!( is_f( function1 ), true );
    a_id!( is_fn( &function1 ), true );
    a_id!( is_fn_mut( &function1 ), true );
    a_id!( is_fn_once( &function1 ), true );

    a_id!( is_f( _f ), true );
    a_id!( is_fn( &_f ), true );
    a_id!( is_fn_mut( &_f ), true );
    a_id!( is_fn_once( &_f ), true );

    // a_id!( is_f( _fn ), true );
    a_id!( is_fn( &_fn ), true );
    a_id!( is_fn_mut( &_fn ), true );
    a_id!( is_fn_once( &_fn ), true );

    // a_id!( is_f( _fn_mut ), true );
    // a_id!( is_fn( &_fn_mut ), true );
    a_id!( is_fn_mut( &_fn_mut ), true );
    a_id!( is_fn_once( &_fn_mut ), true );

    // a_id!( is_f( _fn_once ), true );
    // a_id!( is_fn( &_fn_once ), true );
    // a_id!( is_fn_mut( &_fn_once ), true );
    a_id!( is_fn_once( &_fn_once ), true );

    // type Routine< R > = fn() -> R;
    fn is_f < R >                             ( _x : fn() -> R )      -> bool { true }
    // fn is_f < R >                             ( _x : Routine< R > )   -> bool { true }
    fn is_fn < R, F : Fn() -> R >             ( _x : &F )             -> bool { true }
    fn is_fn_mut < R, F : FnMut() -> R >      ( _x : &F )             -> bool { true }
    fn is_fn_once < R, F : FnOnce() -> R >    ( _x : &F )             -> bool { true }
  }

}

//

tests_index!
{
  implements_basic,
  instance_of_basic,
  implements_functions,
  pointer_experiment,
  fn_experiment,
}
