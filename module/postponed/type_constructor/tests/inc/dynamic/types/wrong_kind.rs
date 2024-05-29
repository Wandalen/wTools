use type_constructor as the_module;
use the_module::prelude::*;

types!
{
  wrong_kind Single : std::sync::Arc< T : Copy >;
}

fn main()
{
}
