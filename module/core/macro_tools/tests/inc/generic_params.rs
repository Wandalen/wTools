
use super::*;

//

#[ test ]
fn generics_with_where()
{

  let got : the_module::GenericsWithWhere = parse_quote!
  {
    < 'a, T : Clone, U : Default, V : std::fmt::Debug >
    where
      Definition : former::FormerDefinition,
  };
  let got = got.unwrap();

  let mut exp : syn::Generics = parse_quote!
  {
    < 'a, T : Clone, U : Default, V : std::fmt::Debug >
  };
  exp.where_clause = parse_quote!
  {
    where
      Definition : former::FormerDefinition,
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

//

#[ test ]
fn merge_assumptions()
{
  use the_module::generic_params;

  let mut generics_a : syn::Generics = parse_quote!{ < T : Clone, U : Default > };
  generics_a.where_clause = parse_quote!{ where T : Default };
  let mut generics_b : syn::Generics = parse_quote!{ < V : std::fmt::Debug > };
  generics_b.where_clause = parse_quote!{ where V : Sized };
  let got = generic_params::merge( &generics_a, &generics_b );

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

//

#[ test ]
fn merge_defaults()
{
  use the_module::generic_params;

  let mut generics_a : syn::Generics = parse_quote!{ < T : Clone, U : Default = Default1 > };
  generics_a.where_clause = parse_quote!{ where T : Default };
  let mut generics_b : syn::Generics = parse_quote!{ < V : std::fmt::Debug = Debug1 > };
  generics_b.where_clause = parse_quote!{ where V : Sized };
  let got = generic_params::merge( &generics_a, &generics_b );

  let mut exp : syn::Generics = parse_quote!
  {
    < T : Clone, U : Default = Default1, V : std::fmt::Debug = Debug1 >
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

//

#[ test ]
fn names()
{

  use macro_tools::syn::parse_quote;

  let mut generics : syn::Generics = parse_quote!{ < T : Clone + Default, U, 'a, const N : usize > };
  generics.where_clause = parse_quote!{ where T: std::fmt::Debug };
  // let generics : Generics = parse_quote!{ < T : Clone + Default, U, 'a, const N : usize > where T: std::fmt::Debug };
  let simplified_generics = macro_tools::generic_params::names( &generics );

  assert_eq!( simplified_generics.params.len(), 4 ); // Contains T, U, 'a, and N
  assert!( simplified_generics.where_clause.is_none() ); // Where clause is removed

}
