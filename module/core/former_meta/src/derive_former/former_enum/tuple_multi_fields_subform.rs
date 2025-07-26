use super::*;
use macro_tools::{ Result, quote::quote, ident::cased_ident_from_ident, generic_params::GenericsRef };
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let fields = &ctx.variant_field_info;
  
  let generics_ref = GenericsRef::new( ctx.generics );
  let impl_generics = generics_ref.impl_generics_tokens_if_any();
  let ty_generics = generics_ref.ty_generics_tokens_if_any();
  let where_clause = generics_ref.where_clause_tokens_if_any();
  
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
  
  // Generate the storage struct
  let storage_struct = quote!
  {
    pub struct #storage_name
    {
      #( #field_names : Option< #field_types > ),*
    }
  };
  
  // Generate Default impl for storage
  let storage_default = quote!
  {
    impl Default for #storage_name
    {
      fn default() -> Self
      {
        Self { #( #field_names : None ),* }
      }
    }
  };
  
  // Generate Storage impl
  let storage_impl = quote!
  {
    impl former::Storage for #storage_name
    {
      type Preformed = #preformed_type;
    }
  };
  
  // Generate StoragePreform impl
  let storage_preform = quote!
  {
    impl former::StoragePreform for #storage_name
    {
      fn preform( mut self ) -> Self::Preformed
      {
        #( let #field_names = self.#field_names.take().unwrap_or_default(); )*
        ( #( #field_names ),* )
      }
    }
  };
  
  // Generate Definition Types
  let definition_types = quote!
  {
    #[ derive( Default, Debug ) ]
    pub struct #definition_types_name< C = (), F = #enum_name #ty_generics >
    {
      _p : std::marker::PhantomData< ( C, F ) >,
    }
  };
  
  let definition_types_impl = quote!
  {
    impl #impl_generics former::FormerDefinitionTypes for #definition_types_name< (), #enum_name #ty_generics >
    #where_clause
    {
      type Storage = #storage_name;
      type Context = ();
      type Formed = #enum_name #ty_generics;
    }
  };
  
  let definition_types_mutator = quote!
  {
    impl #impl_generics former::FormerMutator for #definition_types_name< (), #enum_name #ty_generics >
    #where_clause
    {}
  };
  
  // Generate Definition
  let definition = quote!
  {
    #[ derive( Default, Debug ) ]
    pub struct #definition_name< C = (), F = #enum_name #ty_generics, E = #end_name >
    {
      _p : std::marker::PhantomData< ( C, F, E ) >,
    }
  };
  
  let definition_impl = quote!
  {
    impl #impl_generics former::FormerDefinition for #definition_name< (), #enum_name #ty_generics, #end_name >
    where
      #end_name : former::FormingEnd< #definition_types_name< (), #enum_name #ty_generics > >,
      #where_clause
    {
      type Storage = #storage_name;
      type Context = ();
      type Formed = #enum_name #ty_generics;
      type Types = #definition_types_name< (), #enum_name #ty_generics >;
      type End = #end_name;
    }
  };
  
  // Generate Former struct
  let former_struct = quote!
  {
    pub struct #former_name< Definition = #definition_name >
    where
      Definition : former::FormerDefinition< Storage = #storage_name >,
    {
      storage : Definition::Storage,
      context : Option< Definition::Context >,
      on_end : Option< Definition::End >,
    }
  };
  
  // Generate Former impl
  let former_impl = quote!
  {
    impl< Definition > #former_name< Definition >
    where
      Definition : former::FormerDefinition< Storage = #storage_name >,
    {
      #[ inline( always ) ] 
      pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed 
      { 
        self.end() 
      }
      
      #[ inline( always ) ] 
      pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        let on_end = self.on_end.take().unwrap();
        let context = self.context.take();
        < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
        former::FormingEnd::call( &on_end, self.storage, context )
      }
      
      #[ inline( always ) ] 
      pub fn begin( storage : Option< Definition::Storage >, context : Option< Definition::Context >, on_end : Definition::End ) -> Self
      { 
        Self { storage : storage.unwrap_or_default(), context, on_end : Some( on_end ) } 
      }
      
      #[ allow( dead_code ) ]
      #[ inline( always ) ] 
      pub fn new( on_end : Definition::End ) -> Self 
      { 
        Self::begin( None, None, on_end ) 
      }

      // Setters for fields
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
  
  // Generate End struct
  let end_struct = quote!
  {
    #[ derive( Default, Debug ) ]
    pub struct #end_name
    {
    }
  };
  
  // Generate End impl
  let end_impl = quote!
  {
    impl #impl_generics former::FormingEnd< #definition_types_name< (), #enum_name #ty_generics > >
    for #end_name
    #where_clause
    {
      #[ inline( always ) ]
      fn call(
        &self,
        sub_storage : #storage_name,
        _context : Option< () >,
      ) -> #enum_name #ty_generics
      {
        let ( #( #field_names ),* ) = former::StoragePreform::preform( sub_storage );
        #enum_name #ty_generics :: #variant_name ( #( #field_names ),* )
      }
    }
  };
  
  // Push all the generated infrastructure to the context
  ctx.end_impls.push( storage_struct );
  ctx.end_impls.push( storage_default );
  ctx.end_impls.push( storage_impl );
  ctx.end_impls.push( storage_preform );
  ctx.end_impls.push( definition_types );
  ctx.end_impls.push( definition_types_impl );
  ctx.end_impls.push( definition_types_mutator );
  ctx.end_impls.push( definition );
  ctx.end_impls.push( definition_impl );
  ctx.end_impls.push( former_struct );
  ctx.end_impls.push( former_impl );
  ctx.end_impls.push( end_struct );
  ctx.end_impls.push( end_impl );
  
  // Generate the method that returns the implicit variant former
  let result = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name() -> #former_name
    {
      #former_name::begin( None, None, #end_name::default() )
    }
  };

  // Generate standalone constructor if requested
  if ctx.struct_attrs.standalone_constructors.value(false) {
    // Check if all fields have arg_for_constructor
    let all_fields_constructor_args = fields.iter().all(|f| f.is_constructor_arg);
    
    if all_fields_constructor_args {
      // Scalar standalone constructor - takes arguments for all fields
      let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
      let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
      
      let standalone_method = quote!
      {
        #[ inline( always ) ]
        #vis fn #method_name( #( #field_names : impl Into< #field_types > ),* ) -> #enum_name #ty_generics
        {
          #enum_name #ty_generics :: #variant_name ( #( #field_names.into() ),* )
        }
      };
      ctx.standalone_constructors.push( standalone_method );
    } else {
      // Former builder-style standalone constructor - returns the implicit variant former
      let standalone_method = quote!
      {
        #[ inline( always ) ]
        #vis fn #method_name() -> #former_name
        {
          #former_name::begin( None, None, #end_name::default() )
        }
      };
      ctx.standalone_constructors.push( standalone_method );
    }
  }

  Ok( result )
}