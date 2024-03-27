//! doc
fn main()
{
  #[ cfg( feature = "type_constructor" ) ]
  {
    use data_type::prelude::*;

    types!
    {
      pub single MySingle : f32;
      pub single SingleWithParametrized : std::sync::Arc< T : Copy >;
      pub single SingleWithParameter : < T >;

      pub pair MyPair : f32;
      pub pair PairWithParametrized : std::sync::Arc< T1 : Copy >, std::sync::Arc< T2 : Copy >;
      pub pair PairWithParameter : < T1, T2 >;

      pub pair MyHomoPair : f32;
      pub pair HomoPairWithParametrized : std::sync::Arc< T : Copy >;
      pub pair HomoPairWithParameter : < T >;

      pub many MyMany : f32;
      pub many ManyWithParametrized : std::sync::Arc< T : Copy >;
      pub many ManyWithParameter : < T >;
    }
  }
}
