use super::*;
use the_module::exposed::impls3;
use the_module::exposed::{ index };

//

#[ test ]
fn basic()
{

  impls3!
  {
    fn f1()
    {
      println!( "f1" );
      // panic!( "x" );
    }
    pub fn f2()
    {
      // panic!( "x" );
      println!( "f2" );
    }
  };

  // trace_macros!( true );
  f1!();
  f2!();
  // trace_macros!( false );

  f1();
  f2();

}

//

#[ test ]
fn impl_index()
{

  impls3!
  {
    fn f1()
    {
      println!( "f1" );
    }
    pub fn f2()
    {
      println!( "f2" );
    }
  };

  // trace_macros!( true );
  index!
  {
    f1,
    f2,
  }
  // trace_macros!( false );

  f1();
  f2();

}

#[ test ]
fn impl_as()
{

  impls3!
  {
    fn f1()
    {
      println!( "f1" );
      // panic!( "x" );
    }
    pub fn f2()
    {
      println!( "f2" );
    }
  };

  // trace_macros!( true );
  f1!( as f1b );
  f2!( as f2b );
  // trace_macros!( false );

  f1b();
  f2b();
}

#[ test ]
fn impl_index_as()
{

  impls3!
  {
    fn f1()
    {
      println!( "f1" );
      // panic!( "x" );
    }
    pub fn f2()
    {
      println!( "f2" );
    }
  };

  // trace_macros!( true );
  index!
  {
    f1,
    f2 as f2b,
  }
  // trace_macros!( false );

  f1();
  f2b();

}
