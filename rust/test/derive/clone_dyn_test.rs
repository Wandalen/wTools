
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
    use TheModule::clone_dyn;

    #[ clone_dyn ]
    trait Trait1
    {
    }

    //

    let vec = Vec::< Box< dyn Trait1 > >::new();
    let vec2 = vec.clone();

  }

  //

  fn prelude()
  {
    use TheModule::prelude::*;

    #[ clone_dyn ]
    trait Trait1
    {
    }

    //

    let vec = Vec::< Box< dyn Trait1 > >::new();
    let vec2 = vec.clone();

  }

  //

  fn parametrized()
  {
    use TheModule::clone_dyn;

    #[ clone_dyn ]
    trait Trait2< T1 : Copy, T2 : Copy >
    where
      T2 : Clone + fmt::Debug,
    {
    }

    //

    let vec = Vec::< Box< dyn Trait2< i32, f32 > > >::new();
    let vec2 = vec.clone();

  }

  //

  fn sample()
  {
    use TheModule::clone_dyn;

    #[ clone_dyn ]
    trait Trait1
    {
    }

    let vec = Vec::< Box< dyn Trait1 > >::new();
    let vec2 = vec.clone(); /* <- it does not work without `clone_dyn` */

  }

}

//

tests_index!
{
  manual,
  basic,
  prelude,
  parametrized,
  sample,
}
