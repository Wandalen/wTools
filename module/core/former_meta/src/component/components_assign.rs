#[ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::{ attr, diag, Result, format_ident };
use iter_tools::{ Itertools };

///
/// Generate `ComponentsAssign` trait implementation for the type, providing `components_assign` function
///
/// Output example can be found in in the root of the module
///

pub fn components_assign( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  use convert_case::{ Case, Casing };
  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;

  // name
  let item_name = &parsed.ident;
  let trait_ident = format_ident!
  {
    "{}ComponentsAssign",
    item_name
  };
  let method_ident = format_ident!
  {
    "{}_assign",
    item_name.to_string().to_case( Case::Snake )
  };

  // fields
// fields
  let ( bounds1, bounds2, component_assigns ) : ( Vec< _ >, Vec< _ >, Vec< _ > ) = parsed.fields.iter().map( | field |
  {
    let field_type = &field.ty;
    let bound1 = generate_trait_bounds( field_type );
    let bound2 = generate_impl_bounds( field_type );
    let component_assign = generate_component_assign_call( field );
    ( bound1, bound2, component_assign )
  }).multiunzip();

  let bounds1 : Vec< _ > = bounds1.into_iter().collect::< Result< _ > >()?;
  let bounds2 : Vec< _ > = bounds2.into_iter().collect::< Result< _ > >()?;
  let component_assigns : Vec< _ > = component_assigns.into_iter().collect::< Result< _ > >()?;

  // code
  let doc = "Interface to assign instance from set of components exposed by a single argument.".to_string();
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
    let about = format!( "derive : ComponentsAssign\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  // if has_debug
  // {
  //   diag::report_print( "derive : ComponentsAssign", original_input, &result );
  // }

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
#[ allow( clippy::unnecessary_wraps ) ]
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
/// T : former::Assign< i32, IntoT >,
/// ```
///
#[ allow( clippy::unnecessary_wraps ) ]
fn generate_impl_bounds( field_type : &syn::Type ) -> Result< proc_macro2::TokenStream >
{
  Ok
  (
    qt!
    {
      T : former::Assign< #field_type, IntoT >,
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
/// former::Assign::< i32, _ >::assign( self.component.clone() );
/// ```
///
#[ allow( clippy::unnecessary_wraps ) ]
fn generate_component_assign_call( field : &syn::Field ) -> Result< proc_macro2::TokenStream >
{
  // let field_name = field.ident.as_ref().expect( "Expected the field to have a name" );
  let field_type = &field.ty;
  Ok
  (
    qt!
    {
      former::Assign::< #field_type, _ >::assign( self, component.clone() );
    }
  )
}
