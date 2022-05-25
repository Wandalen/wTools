use type_constructor::prelude::*;

types!
{

  single MySingle : f32;
  single SingleWithParametrized : std::sync::Arc< T : Copy >;
  single SingleWithParameter : < T >;

  pair MyPair : f32;
  pair PairWithParametrized : std::sync::Arc< T1 : Copy >, std::sync::Arc< T2 : Copy >;
  pair PairWithParameter : < T1, T2 >;

  pair MyHomoPair : f32;
  pair HomoPairWithParametrized : std::sync::Arc< T : Copy >;
  pair HomoPairWithParameter : < T >;

  many MyMany : f32;
  many ManyWithParametrized : std::sync::Arc< T : Copy >;
  many ManyWithParameter : < T >;

}

fn main()
{
}
