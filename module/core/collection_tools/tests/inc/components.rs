#[ allow( unused_imports ) ]
use super::*;

//

// qqq : implement similar test for all containers
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn vec_iters()
{

  struct MyContainer
  {
    entries : Vec< i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = i32;
    type IntoIter = std::vec::IntoIter< i32 >;
    // type IntoIter = the_module::vec::IntoIter< i32 >;
    // qqq : should work

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter() // Create an iterator from the internal HashSet.
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter< 'a, i32 >;
    // type IntoIter = the_module::vec::Iter< 'a, i32 >;
    // qqq : should work

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter() // Borrow the elements via an iterator.
    }
  }

  let instance = MyContainer { entries : vec![ 1, 2, 3 ] };
  let got : Vec< _ > = ( &instance ).into_iter().cloned().collect();
  let exp = vec![ 1, 2, 3 ];
  a_id!( got, exp );

  let instance = MyContainer { entries : vec![ 1, 2, 3 ] };
  let got : Vec< _ > = instance.into_iter().collect();
  let exp = vec![ 1, 2, 3 ];
  a_id!( got, exp );

}

// qqq : implement VectorInterface
