use type_constructor as TheModule;
use TheModule::prelude::*;

types!
{
  wrong_kind Single : std::sync::Arc< T : Copy >;
}

fn main()
{
}
