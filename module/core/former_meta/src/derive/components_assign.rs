use super::*;
use macro_tools::{ attr, diag, type_struct, Result };
use iter_tools::{ Itertools, process_results };

///
/// Generate `ComponentsAssign` trait implementation for the type, providing `components_assign` function
///
/// Output example can be found in in the root of the module
///

pub fn components_assign( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  use convert_case::{ Case, Casing };
  let original_input = input.clone();
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let has_debug = attr::has_debug( parsed.item.attrs.iter() )?;

  // name
  let item_name = parsed.item_name;
  let trait_name = format!( "{}ComponentsAssign", item_name );
  let trait_ident = syn::Ident::new( &trait_name, item_name.span() );
  let method_name = format!( "{}_assign", item_name.to_string().to_case( Case::Snake ) );
  let method_ident = syn::Ident::new( &method_name, item_name.span() );

  // fields
  let ( bounds1, bounds2, component_assigns ) : ( Vec< _ >, Vec< _ >, Vec< _ > ) = parsed.fields.iter().map( | field |
  {
    let field_type = &field.ty;
    let bound1 = generate_trait_bounds( field_type );
    let bound2 = generate_impl_bounds( field_type );
    let component_assign = generate_component_assign_call( field );
    ( bound1, bound2, component_assign )
  }).multiunzip();

  let bounds1 : Vec< _ > = process_results( bounds1, | iter | iter.collect() )?;
  let bounds2 : Vec< _ > = process_results( bounds2, | iter | iter.collect() )?;
  let component_assigns : Vec< _ > = process_results( component_assigns, | iter | iter.collect() )?;

  // code
  let doc = format!( "Interface to assign instance from set of components exposed by a single argument." );
  let trait_bounds = qt! { #( #bounds1 )* IntoT : Clone };
  let impl_bounds = qt! { #( #bounds2 )* #( #bounds1 )* IntoT : Clone };
  let component_assigns = qt! { #( #component_assigns )* };
  let result = qt!
  {

    #[ doc = #doc ]
    pub trait #trait_ident< IntoT >
    where
      #trait_bounds,
    {
      fn #method_ident( &mut self, component : IntoT );
    }

    impl< T, IntoT > #trait_ident< IntoT > for T
    where
      #impl_bounds,
    {
      #[ inline( always ) ]
      #[ doc = #doc ]
      fn #method_ident( &mut self, component : IntoT )
      {
        #component_assigns
      }
    }

  };

  if has_debug
  {
    diag::debug_report_print( "derive : ComponentsAssign", original_input, &result );
  }
  Ok( result )
}

///
/// Generate trait bounds needed for `components_assign`
///
/// ### Output example
///
/// ```ignore
/// IntoT : Into< i32 >
/// ```
///
fn generate_trait_bounds( field_type : &syn::Type ) -> Result< proc_macro2::TokenStream >
{
  Ok
  (
    qt!
    {
      IntoT : Into< #field_type >,
    }
  )
}

///
/// Generate impl bounds needed for `components_assign`
///
/// ### Output example
///
/// ```ignore
/// T : former::ComponentAssign< i32, IntoT >,
/// ```
///
fn generate_impl_bounds( field_type : &syn::Type ) -> Result< proc_macro2::TokenStream >
{
  Ok
  (
    qt!
    {
      T : former::ComponentAssign< #field_type, IntoT >,
    }
  )
}

///
/// Generate set calls needed by `components_assign`
/// Returns a "unit" of work of `components_assign` function, performing `set` on each field.
///
/// Output example
///
/// ```ignore
/// former::ComponentAssign::< i32, _ >::assign( self.component.clone() );
/// ```
///
fn generate_component_assign_call( field : &syn::Field ) -> Result< proc_macro2::TokenStream >
{
  // let field_name = field.ident.as_ref().expect( "Expected the field to have a name" );
  let field_type = &field.ty;
  Ok
  (
    qt!
    {
      former::ComponentAssign::< #field_type, _ >::assign( self, component.clone() );
    }
  )
}

// ///
// /// Options2ComponentsAssign.
// ///
//
// pub trait Options2ComponentsAssign< IntoT >
// where
//   IntoT : Into< i32 >,
//   IntoT : Into< String >,
//   IntoT : Clone,
// {
//   fn components_assign( &mut self, component : IntoT );
// }
//
// impl< T, IntoT > Options2ComponentsAssign< IntoT > for T
// where
//   T : former::ComponentAssign< i32, IntoT >,
//   T : former::ComponentAssign< String, IntoT >,
//   IntoT : Into< i32 >,
//   IntoT : Into< String >,
//   IntoT : Clone,
// {
//   #[ inline( always ) ]
//   fn components_assign( &mut self, component : IntoT )
//   {
//     former::ComponentAssign::< i32, _ >::assign( self, component.clone() );
//     former::ComponentAssign::< String, _ >::assign( self, component.clone() );
//   }
// }
