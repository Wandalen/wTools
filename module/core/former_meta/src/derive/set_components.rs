use super::*;
use macro_tools::{ attr, diag, type_struct, Result };
use iter_tools::{ Itertools, process_results };

pub fn set_components( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let has_debug = attr::has_debug( parsed.item.attrs.iter() )?;

  // name
  let item_name = parsed.item_name;
  let trait_name = format!( "{}SetComponents", item_name );
  let trait_ident = syn::Ident::new( &trait_name, item_name.span() );

  // fields
  let ( bounds1, bounds2, component_sets ) : ( Vec< _ >, Vec< _ >, Vec< _ > ) = parsed.fields.iter().map( | field |
  {
    let field_type = &field.ty;
    let bound1 = bound1( field_type );
    let bound2 = bound2( field_type );
    let component_set = generate_component_set_call( field );
    ( bound1, bound2, component_set )
  }).multiunzip();

  let bounds1 : Vec< _ > = process_results( bounds1, | iter | iter.collect() )?;
  let bounds2 : Vec< _ > = process_results( bounds2, | iter | iter.collect() )?;
  let component_sets : Vec< _ > = process_results( component_sets, | iter | iter.collect() )?;

  // code
  let doc = format!( "Interface to assign instance from set of components exposed by a single argument." );
  let trait_bounds = qt! { #( #bounds1 )* IntoT : Clone };
  let impl_bounds = qt! { #( #bounds2 )* #( #bounds1 )* IntoT : Clone };
  let component_sets = qt! { #( #component_sets )* };
  let result = qt!
  {

    #[ doc = #doc ]
    pub trait #trait_ident< IntoT >
    where
      #trait_bounds,
    {
      fn components_set( &mut self, component : IntoT );
    }

    impl< T, IntoT > #trait_ident< IntoT > for T
    where
      #impl_bounds,
    {
      #[ inline( always ) ]
      #[ doc = #doc ]
      fn components_set( &mut self, component : IntoT )
      {
        #component_sets
      }
    }

  };

  if has_debug
  {
    diag::debug_report_print( "derive : SetComponents", original_input, &result );
  }
  Ok( result )
}

fn bound1( field_type : &syn::Type ) -> Result< proc_macro2::TokenStream >
{
  Ok
  (
    qt!
    {
      IntoT : Into< #field_type >,
    }
  )
}

fn bound2( field_type : &syn::Type ) -> Result< proc_macro2::TokenStream >
{
  Ok
  (
    qt!
    {
      T : former::SetComponent< #field_type, IntoT >,
    }
  )
}

fn generate_component_set_call( field : &syn::Field ) -> Result< proc_macro2::TokenStream >
{
  // let field_name = field.ident.as_ref().expect( "Expected the field to have a name" );
  let field_type = &field.ty;
  Ok
  (
    qt!
    {
      former::SetComponent::< #field_type, _ >::set( self, component.clone() );
    }
  )
}

// ///
// /// Options2SetComponents.
// ///
//
// pub trait Options2SetComponents< IntoT >
// where
//   IntoT : Into< i32 >,
//   IntoT : Into< String >,
//   IntoT : Clone,
// {
//   fn components_set( &mut self, component : IntoT );
// }
//
// impl< T, IntoT > Options2SetComponents< IntoT > for T
// where
//   T : former::SetComponent< i32, IntoT >,
//   T : former::SetComponent< String, IntoT >,
//   IntoT : Into< i32 >,
//   IntoT : Into< String >,
//   IntoT : Clone,
// {
//   #[ inline( always ) ]
//   fn components_set( &mut self, component : IntoT )
//   {
//     former::SetComponent::< i32, _ >::set( self, component.clone() );
//     former::SetComponent::< String, _ >::set( self, component.clone() );
//   }
// }
