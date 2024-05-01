//! In Rust, you often need to wrap a given type into a new one.
//! The role of the orphan rules in particular is basically to prevent you from implementing external traits for external types.
//! To overcome the restriction developer usually wrap the external type into a tuple introducing a new type.
//! Type constructor does exactly that and auto-implement traits From, Into, Deref and few more for the constructed type.
//! 
//! Macro [types](https://docs.rs/type_constructor/latest/type_constructor/types/macro.types.html) is responsible for generating code for Single, Pair, Homopair, Many. Each type constructor has its own keyword for that, but Pair and Homopair use the same keyword difference in a number of constituent types. It is possible to define all types at once.
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
