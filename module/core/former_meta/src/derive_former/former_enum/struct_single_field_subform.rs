use super::*;

use macro_tools::{ Result, quote::{ quote, format_ident }, ident::cased_ident_from_ident, generic_params::GenericsRef };
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let field = &ctx.variant_field_info[0];
  let field_name = &field.ident;
  let field_type = &field.ty;

  let generics_ref = GenericsRef::new(ctx.generics);
  let ( impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl();
  let enum_type_path = if ctx.generics.type_params().next().is_some() {
    let ty_generics_tokens = generics_ref.ty_generics_tokens_if_any();
    quote! { #enum_name :: #ty_generics_tokens }
  } else {
    quote! { #enum_name }
  };

  // Generate the End struct name for this variant
  let end_struct_name = format_ident!("{}{}End", enum_name, variant_name);

  // Generate the End struct for this variant
  let end_struct = quote!
  {
    #[derive(Default, Debug)]
    pub struct #end_struct_name #impl_generics
    #where_clause
    {}
  };

  // Generate the implicit former for the variant
  let variant_former_name = format_ident!("{}{}Former", enum_name, variant_name);
  let variant_former_storage_name = format_ident!("{}{}FormerStorage", enum_name, variant_name);
  let variant_former_definition_name = format_ident!("{}{}FormerDefinition", enum_name, variant_name);
  let variant_former_definition_types_name = format_ident!("{}{}FormerDefinitionTypes", enum_name, variant_name);

  // Generate the storage struct for the variant's fields
  let storage_field_optional = quote! { pub #field_name : ::core::option::Option< #field_type > };
  let storage_field_none = quote! { #field_name : ::core::option::Option::None };
  let storage_field_preform = quote! { let #field_name = self.#field_name.unwrap_or_default(); };
  let storage_field_name = quote! { #field_name };

  let variant_former_code = quote!
  {
    // = definition types: Define the FormerDefinitionTypes struct for the variant.
    #[ derive( Debug ) ]
    pub struct #variant_former_definition_types_name #impl_generics
    #where_clause
    {
      _phantom : ::core::marker::PhantomData< ( #impl_generics ) >,
    }

    impl #impl_generics ::core::default::Default
    for #variant_former_definition_types_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : ::core::marker::PhantomData,
        }
      }
    }

    impl #impl_generics former_types::FormerDefinitionTypes
    for #variant_former_definition_types_name #ty_generics
    #where_clause
    {
      type Storage = #variant_former_storage_name #ty_generics;
      type Formed = #enum_name #ty_generics;
      type Context = ();
    }

    // Add FormerMutator implementation here
    impl #impl_generics former_types::FormerMutator
    for #variant_former_definition_types_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      fn form_mutation
      (
        _storage : &mut Self::Storage,
        _context : &mut Option< Self::Context >,
      )
      {
      }
    }

    // = definition: Define the FormerDefinition struct for the variant.
    #[ derive( Debug ) ]
    pub struct #variant_former_definition_name #impl_generics
    #where_clause
    {
      _phantom : ::core::marker::PhantomData< ( #impl_generics ) >,
    }

    impl #impl_generics ::core::default::Default
    for #variant_former_definition_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : ::core::marker::PhantomData,
        }
      }
    }

    impl #impl_generics former_types::FormerDefinition
    for #variant_former_definition_name #ty_generics
    #where_clause
    {
      type Types = #variant_former_definition_types_name #ty_generics;
      type End = former_types::forming::ReturnPreformed;
      type Storage = #variant_former_storage_name #ty_generics;
      type Formed = #enum_name #ty_generics;
      type Context = ();
    }

    // = storage: Define the FormerStorage struct for the variant.
    #[ doc = "Stores potential values for fields during the formation process." ]
    #[ allow( explicit_outlives_requirements ) ]
    pub struct #variant_former_storage_name #impl_generics
    #where_clause
    {
      /// A field
      #storage_field_optional,
    }

    impl #impl_generics ::core::default::Default
    for #variant_former_storage_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      fn default() -> Self
      {
        Self
        {
          #storage_field_none,
        }
      }
    }

    impl #impl_generics former_types::Storage
    for #variant_former_storage_name #ty_generics
    #where_clause
    {
      type Preformed = #enum_name #ty_generics;
    }

    impl #impl_generics former_types::StoragePreform
    for #variant_former_storage_name #ty_generics
    #where_clause
    {
      fn preform( mut self ) -> Self::Preformed
      {
        #storage_field_preform
        let result = #enum_name::#variant_name { #field_name };
        return result;
      }
    }

    // = former: Define the Former struct itself for the variant.
    pub struct #variant_former_name #impl_generics
    #where_clause
    {
      pub storage : #variant_former_storage_name #ty_generics,
      pub context : ::core::option::Option< () >,
      pub on_end : ::core::option::Option< former_types::forming::ReturnPreformed >,
    }

    impl #impl_generics #variant_former_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      pub fn new
      (
        on_end : former_types::forming::ReturnPreformed
      ) -> Self
      {
        Self::begin_coercing( ::core::option::Option::None, ::core::option::Option::None, on_end )
      }

      #[ inline( always ) ]
      pub fn new_coercing< IntoEnd >
      (
        end : IntoEnd
      ) -> Self
      where
        IntoEnd : ::core::convert::Into< former_types::forming::ReturnPreformed >,
      {
        Self::begin_coercing
        (
          ::core::option::Option::None,
          ::core::option::Option::None,
          end,
        )
      }

      #[ inline( always ) ]
      pub fn begin
      (
        mut storage : ::core::option::Option< #variant_former_storage_name #ty_generics >,
        context : ::core::option::Option< () >,
        on_end : former_types::forming::ReturnPreformed,
      )
      -> Self
      {
        if storage.is_none()
        {
          storage = ::core::option::Option::Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( on_end ),
        }
      }

      #[ inline( always ) ]
      pub fn begin_coercing< IntoEnd >
      (
        mut storage : ::core::option::Option< #variant_former_storage_name #ty_generics >,
        context : ::core::option::Option< () >,
        on_end : IntoEnd,
      ) -> Self
      where
        IntoEnd : ::core::convert::Into< former_types::forming::ReturnPreformed >,
      {
        if storage.is_none()
        {
          storage = ::core::option::Option::Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( ::core::convert::Into::into( on_end ) ),
        }
      }

      #[ inline( always ) ]
      pub fn form( self ) -> #enum_name #ty_generics
      {
        self.end()
      }

      #[ inline( always ) ]
      pub fn end( mut self ) -> #enum_name #ty_generics
      {
        let on_end = self.on_end.take().unwrap();
        let mut context = self.context.take();
        < #variant_former_definition_types_name #ty_generics as former_types::FormerMutator >::form_mutation( &mut self.storage, &mut context );
        former_types::forming::FormingEnd::< #variant_former_definition_types_name #ty_generics >::call( &on_end, self.storage, context )
      }

      // Setter for the single field
      #[ inline( always ) ]
      pub fn #field_name( mut self, value : impl ::core::convert::Into< #field_type > ) -> Self
      {
        self.storage.#field_name = ::core::option::Option::Some( value.into() );
        self
      }
    }

    // = entity to former: Implement former traits linking the variant to its generated components.
    impl #impl_generics former_types::EntityToFormer< #variant_former_definition_name #ty_generics >
    for #enum_name #ty_generics
    #where_clause
    {
      type Former = #variant_former_name #ty_generics;
    }

    impl #impl_generics former_types::EntityToStorage
    for #enum_name #ty_generics
    #where_clause
    {
      type Storage = #variant_former_storage_name #ty_generics;
    }

    impl #impl_generics former_types::EntityToDefinition< (), #enum_name #ty_generics, former_types::forming::ReturnPreformed >
    for #enum_name #ty_generics
    #where_clause
    {
      type Definition = #variant_former_definition_name #ty_generics;
      type Types = #variant_former_definition_types_name #ty_generics;
    }

    impl #impl_generics former_types::EntityToDefinitionTypes< (), #enum_name #ty_generics >
    for #enum_name #ty_generics
    #where_clause
    {
      type Types = #variant_former_definition_types_name #ty_generics;
    }
  };

  // Generate the method for the enum
  let method = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name() -> #variant_former_name #ty_generics
    {
      #variant_former_name::new( former_types::forming::ReturnPreformed::default() )
    }
  };

  // Generate standalone constructor if requested
  if ctx.struct_attrs.standalone_constructors.value(false) {
    let constructor_name_str = method_name.to_string();
    let base_name = constructor_name_str.strip_prefix("r#").unwrap_or(&constructor_name_str);
    let standalone_name = format_ident!("{}_variant", base_name);

    let standalone_method = quote!
    {
      #[ inline( always ) ]
      #vis fn #standalone_name() -> #variant_former_name #ty_generics
      {
        #variant_former_name::new( former_types::forming::ReturnPreformed::default() )
      }
    };
    ctx.standalone_constructors.push(standalone_method);
  }

  ctx.end_impls.push(variant_former_code);

  Ok(method)
}
