#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::{ SetComponent, SetWithType };

///
/// Options1
///

#[
  derive
  (
    Debug,
    Default,
    PartialEq,
    TheModule::ComponentFrom,
    TheModule::SetComponent,
    // TheModule::SetComponents,
    // TheModule::FromComponents,
  )
]
// #[ debug ]
// qqq : make these traits working for generic struct, use `split_for_impl`
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

///
/// Options2
///

#[
  derive
  (
    Debug,
    Default,
    PartialEq,
    TheModule::ComponentFrom,
    TheModule::SetComponent,
    TheModule::SetComponents,
    // TheModule::FromComponents,
  )
]
// #[ debug ]
pub struct Options2
{
  field1 : i32,
  field2 : String,
}


//

impl< T > From< T > for Options2
where
  T : Into< i32 >,
  T : Into< String >,
  T : Clone,
{
  #[ inline( always ) ]
  fn from( src : T ) -> Self
  {
    let field1 = Into::< i32 >::into( src.clone() );
    let field2 = Into::< String >::into( src.clone() );
    Options2
    {
      field1,
      field2,
    }
  }
}

//

include!( "only_test/components_composite.rs" );
