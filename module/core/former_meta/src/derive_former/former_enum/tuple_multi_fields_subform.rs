use super::*;
use macro_tools::{ Result, quote::quote, ident::cased_ident_from_ident };
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let fields = &ctx.variant_field_info;

  let ( impl_generics, _, where_clause ) = ctx.generics.split_for_impl();

  // Use proper generics with bounds for type positions
  let ( _, ty_generics, _ ) = ctx.generics.split_for_impl();

  // Generate unique names for the variant former infrastructure
  let variant_name_str = variant_name.to_string();
  let storage_name = format_ident!("{}{}FormerStorage", enum_name, variant_name_str);
  let definition_types_name = format_ident!("{}{}FormerDefinitionTypes", enum_name, variant_name_str);
  let definition_name = format_ident!("{}{}FormerDefinition", enum_name, variant_name_str);
  let former_name = format_ident!("{}{}Former", enum_name, variant_name_str);
  let end_name = format_ident!("{}{}End", enum_name, variant_name_str);

  // Generate field types and names
  let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
  let field_indices: Vec<_> = (0..fields.len()).collect();
  let field_names: Vec<_> = field_indices.iter().map(|i| format_ident!("field{}", i)).collect();
  let setter_names: Vec<_> = field_indices.iter().map(|i| format_ident!("_{}", i)).collect();

  // Create the preformed tuple type
  let preformed_type = quote! { ( #( #field_types ),* ) };

  // Generate the storage struct and its impls
  let storage_impls = quote!
  {
    pub struct #storage_name #ty_generics
    #where_clause
    {
      #( #field_names : Option< #field_types > ),*
    }

    impl #impl_generics Default for #storage_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self { #( #field_names : None ),* }
      }
    }

    impl #impl_generics former::Storage for #storage_name #ty_generics
    #where_clause
    {
      type Preformed = #preformed_type;
    }

    impl #impl_generics former::StoragePreform for #storage_name #ty_generics
    where
      #( #field_types : Default, )*
    {
      fn preform( mut self ) -> Self::Preformed
      {
        #( let #field_names = self.#field_names.take().unwrap_or_default(); )*
        ( #( #field_names ),* )
      }
    }
  };

  // Generate the DefinitionTypes struct and its impls
  let definition_types_impls = quote!
  {
    #[ derive( Debug ) ]
    pub struct #definition_types_name #ty_generics
    #where_clause
    {
      _p : std::marker::PhantomData #ty_generics,
    }

    impl #impl_generics Default for #definition_types_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self { _p : std::marker::PhantomData }
      }
    }

    impl #impl_generics former::FormerDefinitionTypes for #definition_types_name #ty_generics
    #where_clause
    {
      type Storage = #storage_name #ty_generics;
      type Context = ();
      type Formed = #enum_name #ty_generics;
    }

    impl #impl_generics former::FormerMutator for #definition_types_name #ty_generics
    #where_clause
    {}
  };

  // Generate the Definition struct and its impls
  let definition_impls = quote!
  {
    #[ derive( Debug ) ]
    pub struct #definition_name #ty_generics
    #where_clause
    {
      _p : std::marker::PhantomData #ty_generics,
    }

    impl #impl_generics Default for #definition_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self { _p : std::marker::PhantomData }
      }
    }

    impl #impl_generics former::FormerDefinition for #definition_name #ty_generics
    #where_clause
    {
      type Storage = #storage_name #ty_generics;
      type Context = ();
      type Formed = #enum_name #ty_generics;
      type Types = #definition_types_name #ty_generics;
      type End = #end_name #ty_generics;
    }
  };

  // Generate the Former struct and its impls
  let former_impls = quote!
  {
    pub struct #former_name #ty_generics
    #where_clause
    {
      storage : #storage_name #ty_generics,
      context : Option< () >,
      on_end : Option< #end_name #ty_generics >,
    }

    impl #impl_generics #former_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      pub fn form( self ) -> #enum_name #ty_generics
      {
        self.end()
      }

      #[ inline( always ) ]
      pub fn end( mut self ) -> #enum_name #ty_generics
      {
        let on_end = self.on_end.take().unwrap();
        let context = self.context.take();
        < #definition_types_name #ty_generics as former::FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
        former::FormingEnd::call( &on_end, self.storage, context )
      }

      #[ inline( always ) ]
      pub fn begin( storage : Option< #storage_name #ty_generics >, context : Option< () >, on_end : #end_name #ty_generics ) -> Self
      {
        Self { storage : storage.unwrap_or_default(), context, on_end : Some( on_end ) }
      }

      #[ allow( dead_code ) ]
      #[ inline( always ) ]
      pub fn new( on_end : #end_name #ty_generics ) -> Self
      {
        Self::begin( None, None, on_end )
      }

      #(
        #[ inline ]
        pub fn #setter_names( mut self, src : impl Into< #field_types > ) -> Self
        {
          self.storage.#field_names = Some( src.into() );
          self
        }
      )*
    }
  };

  // Generate the End struct and its impl
  let end_impls = quote!
  {
    #[ derive( Debug ) ]
    pub struct #end_name #ty_generics
    #where_clause
    {}

    impl #impl_generics Default for #end_name #ty_generics
    #where_clause
    {
      fn default() -> Self
      {
        Self {}
      }
    }

    impl #impl_generics former::FormingEnd< #definition_types_name #ty_generics >
    for #end_name #ty_generics
    #where_clause
    {
      #[ inline( always ) ]
      fn call(
        &self,
        sub_storage : #storage_name #ty_generics,
        _context : Option< () >,
      ) -> #enum_name #ty_generics
      {
        let ( #( #field_names ),* ) = former::StoragePreform::preform( sub_storage );
        #enum_name #ty_generics :: #variant_name ( #( #field_names ),* )
      }
    }
  };

  // Push all the generated infrastructure to the context
  ctx.end_impls.push( storage_impls );
  ctx.end_impls.push( definition_types_impls );
  ctx.end_impls.push( definition_impls );
  ctx.end_impls.push( former_impls );
  ctx.end_impls.push( end_impls );

  // Generate the method that returns the implicit variant former
  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name() -> #former_name #ty_generics
    #where_clause
    {
      #former_name::begin( None, None, #end_name::default() )
    }
  };

  // Generate standalone constructor if requested
  if ctx.struct_attrs.standalone_constructors.value(false) {
      let standalone_method = quote!
      {
        #[ inline( always ) ]
        #vis fn #method_name() -> #former_name #ty_generics
        #where_clause
        {
          #former_name::begin( None, None, #end_name::default() )
        }
      };
      ctx.standalone_constructors.push( standalone_method );
  }

  Ok( result )
}