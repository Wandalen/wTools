
#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  //

  fn manual()
  {

    trait Trait1
    {
    }

    //

    #[ inline ]
    pub fn _clone_boxed< T >( t : &T ) -> Box< T >
    where
      T : ?Sized,
    {
      unsafe
      {
        let mut ptr = t as *const T;
        let data_ptr = &mut ptr as *mut *const T as *mut *mut ();
        *data_ptr = Box::into_raw( Box::new( t.clone() ) ) as *mut ();
        Box::from_raw( ptr as *mut T )
      }
    }

    //

    impl < 'c > Clone
    for Box< dyn Trait1 + 'c >
    {
      #[ inline ]
      fn clone( &self ) -> Self { _clone_boxed( &**self ) }
    }

    impl < 'c > Clone
    for Box< dyn Trait1 + Send + 'c >
    {
      #[ inline ]
      fn clone( &self ) -> Self { _clone_boxed( &**self ) }
    }

    impl < 'c > Clone
    for Box< dyn Trait1 + Sync + 'c >
    {
      #[ inline ]
      fn clone( &self ) -> Self { _clone_boxed( &**self ) }
    }

    impl < 'c > Clone
    for Box< dyn Trait1 + Send + Sync + 'c >
    {
      #[ inline ]
      fn clone( &self ) -> Self { _clone_boxed( &**self ) }
    }

    //

    let vec = Vec::< Box< dyn Trait1 > >::new();
    let vec2 = vec.clone();

  }

  //

  fn basic()
  {
    // use TheModule::prelude::*;
    use TheModule::clone_dyn;

    #[ clone_dyn ]
    trait Trait1
    {
    }

    //

    let vec = Vec::< Box< dyn Trait1 > >::new();
    let vec2 = vec.clone();

    // xxx
  }

}

//

tests_index!
{
  manual,
  basic,
}
