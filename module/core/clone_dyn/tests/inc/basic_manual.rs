
#[ allow( unused_imports ) ]
use super::*;

trait Trait1
where
  Self : the_module::CloneDyn,
{
  fn val( &self ) -> i32;
}

//

impl Trait1 for i32
{
  fn val( &self ) -> i32
  {
    *self
  }
}

impl Trait1 for i64
{
  fn val( &self ) -> i32
  {
    self.clone().try_into().unwrap()
  }
}

impl Trait1 for String
{
  fn val( &self ) -> i32
  {
    self.len().try_into().unwrap()
  }
}

impl< T > Trait1 for &[ T ]
where
  T : the_module::CloneDyn,
{
  fn val( &self ) -> i32
  {
    self.len().try_into().unwrap()
  }
}

impl Trait1 for &str
{
  fn val( &self ) -> i32
  {
    self.len().try_into().unwrap()
  }
}

// == begin of generated

#[ allow( non_local_definitions ) ]
impl < 'c > Clone
for Box< dyn Trait1 + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self
  {
    the_module::clone_into_box( &**self )
  }
}

#[ allow( non_local_definitions ) ]
impl < 'c > Clone
for Box< dyn Trait1 + Send + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self { the_module::clone_into_box( &**self ) }
}

#[ allow( non_local_definitions ) ]
impl < 'c > Clone
for Box< dyn Trait1 + Sync + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self { the_module::clone_into_box( &**self ) }
}

#[ allow( non_local_definitions ) ]
impl < 'c > Clone
for Box< dyn Trait1 + Send + Sync + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self { the_module::clone_into_box( &**self ) }
}

// == end of generated

include!( "./only_test/basic.rs" );
