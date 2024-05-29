use type_constructor::prelude::*;


fn main()
{
  types!
  {

    // struct Bad( Box< Bad > ); compiles without errors
    single Bad : Box< Bad >;

  }
}
