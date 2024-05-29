use type_constructor::prelude::*;


fn main()
{
  types!
  {

    single Bad : std::sync::Arc< std::sync::Mutex< T > >;

  }
}
