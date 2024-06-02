
#[ allow( unused_imports ) ]
use super::*;

//

// qqq2 : organize tests in the same way tests organized for derive_tools
#[ test ]
fn manual()
{

  trait Trait1
  where
    Self : clone_dyn::CloneDyn,
  {
    fn val( &self ) -> i32;
  }

  //

  impl Trait1 for i32
  {
    fn val( &self ) -> i32
    {
      self.clone()
    }
  }

  impl Trait1 for i64
  {
    fn val( &self ) -> i32
    {
      self.clone().try_into().unwrap()
    }
  }

  //

  #[ allow( non_local_definitions ) ]
  impl < 'c > Clone
  for Box< dyn Trait1 + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      let x = &**self;
      // inspect_type::inspect_type_of!( x );
      // clone_dyn::clone( self )
      clone_dyn::clone_into_box( &**self )
    }
  }

  #[ allow( non_local_definitions ) ]
  impl < 'c > Clone
  for Box< dyn Trait1 + Send + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
  }

  #[ allow( non_local_definitions ) ]
  impl < 'c > Clone
  for Box< dyn Trait1 + Sync + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
  }

  #[ allow( non_local_definitions ) ]
  impl < 'c > Clone
  for Box< dyn Trait1 + Send + Sync + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
  }

  //

  let a : String = "abc".to_string();
  let b : Box< String > = clone_dyn::clone_into_box( &a );
  a_id!( a, *b );

  //

  let a : String = "abc".to_string();
  let b = clone_dyn::clone( &a );
  a_id!( a, b );

  //

  let a : &str = "abc";
  let b = clone_dyn::clone( &a );
  // let b = a.clone();
  a_id!( a, b );

  //

  let vec : Vec< Box< dyn Trait1 > > = vec![ Box::new( 13 ), Box::new( 14 ) ];
  let vec2 = clone_dyn::clone( &vec );
  let vec = vec.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  let vec2 = vec2.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  a_id!( vec, vec2 );

  //

  let vec : Vec< Box< dyn Trait1 > > = vec![ Box::new( 13 ), Box::new( 14 ) ];
  let vec2 = vec.clone();
  let vec = vec.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  let vec2 = vec2.iter().map( | e | e.val() ).collect::< Vec< _ > >();
  a_id!( vec, vec2 );

}
