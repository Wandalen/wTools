
use super::*;

//

#[ test ]
fn basic()
{

  let mut generics_a : syn::Generics = parse_quote!{ < T : Clone, U : Default > };
  generics_a.where_clause = parse_quote!{ where T : Default };
  let mut generics_b : syn::Generics = parse_quote!{ < V : std::fmt::Debug > };
  generics_b.where_clause = parse_quote!{ where V : Sized };
  let got = generics::merge( &generics_a, &generics_b );

  let mut exp : syn::Generics = parse_quote!
  {
    < T : Clone, U : Default, V : std::fmt::Debug >
  };
  exp.where_clause = parse_quote!
  {
    where
      T : Default,
      V : Sized
  };

  // a_id!( tree_print!( got ), tree_print!( exp ) );
  // code_print!( got );
  // code_print!( exp );
  // code_print!( got.where_clause );
  // code_print!( exp.where_clause );

  assert_eq!( got.params, exp.params );
  assert_eq!( got.where_clause, exp.where_clause );
  assert_eq!( got, exp );

}