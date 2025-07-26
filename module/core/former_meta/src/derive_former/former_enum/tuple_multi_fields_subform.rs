use super::*;
use macro_tools::{ Result, quote::quote, ident::cased_ident_from_ident, generic_params };
use convert_case::Case;

pub fn handle( ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  let variant_name = &ctx.variant.ident;
  let method_name = cased_ident_from_ident(variant_name, Case::Snake);
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let fields = &ctx.variant_field_info;
  
  let (enum_impl_generics, enum_ty_generics, enum_where_clause) = ctx.generics.split_for_impl();
  
  // Extract just the type parameters without angle brackets for concrete instantiation
  let enum_ty_params = if ctx.generics.params.is_empty() {
    quote!{}
  } else {
    let params = ctx.generics.params.iter().map(|param| {
      match param {
        syn::GenericParam::Type(type_param) => {
          let ident = &type_param.ident;
          quote! { #ident }
        },
        syn::GenericParam::Lifetime(lifetime_param) => {
          let lifetime = &lifetime_param.lifetime;
          quote! { #lifetime }
        },
        syn::GenericParam::Const(const_param) => {
          let ident = &const_param.ident;
          quote! { #ident }
        },
      }
    });
    quote! { #( #params ),* }
  };
  
  // Generate unique names for the variant former infrastructure
  let variant_name_str = variant_name.to_string();
  let storage_name = format_ident!("{}{}FormerStorage", enum_name, variant_name_str);
  let definition_types_name = format_ident!("{}{}FormerDefinitionTypes", enum_name, variant_name_str);
  let definition_name = format_ident!("{}{}FormerDefinition", enum_name, variant_name_str);
  let former_name = format_ident!("{}{}Former", enum_name, variant_name_str);
  let end_name = format_ident!("{}{}End", enum_name, variant_name_str);
  
  // Create merged generics for definition types (Context, Formed) 
  let definition_types_extra: macro_tools::generic_params::GenericsWithWhere = syn::parse_quote! {
    < __Context = (), __Formed = #enum_name < #enum_ty_params > >
  };
  let definition_types_generics = generic_params::merge(ctx.generics, &definition_types_extra.into());
  let (
    definition_types_generics_with_defaults,
    definition_types_generics_impl,
    definition_types_generics_ty,
    definition_types_generics_where,
  ) = generic_params::decompose(&definition_types_generics);
  
  // Create merged generics for definition (Context, Formed, End)
  let definition_extra: macro_tools::generic_params::GenericsWithWhere = syn::parse_quote! {
    < __Context = (), __Formed = #enum_name < #enum_ty_params >, __End = #end_name >
  };
  let definition_generics = generic_params::merge(ctx.generics, &definition_extra.into());
  let (
    definition_generics_with_defaults,
    definition_generics_impl,
    definition_generics_ty,
    definition_generics_where,
  ) = generic_params::decompose(&definition_generics);
  
  // Create merged generics for former (Definition)
  let former_extra: macro_tools::generic_params::GenericsWithWhere = syn::parse_quote! {
    < __Definition = #definition_name < #enum_ty_params > >
  };
  let former_generics = generic_params::merge(ctx.generics, &former_extra.into());
  let (
    former_generics_with_defaults,
    former_generics_impl,
    former_generics_ty,
    former_generics_where,
  ) = generic_params::decompose(&former_generics);
  
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
    pub struct #storage_name #enum_impl_generics
    #enum_where_clause
    {
      #( #field_names : Option< #field_types > ),*
    }
  };
  
  // Generate Default impl for storage
  let storage_default = quote!
  {
    impl #enum_impl_generics Default for #storage_name #enum_ty_generics
    #enum_where_clause
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
    impl #enum_impl_generics former::Storage for #storage_name #enum_ty_generics
    #enum_where_clause
    {
      type Preformed = #preformed_type;
    }
  };
  
  // Generate StoragePreform impl
  let storage_preform = quote!
  {
    impl #enum_impl_generics former::StoragePreform for #storage_name #enum_ty_generics
    #enum_where_clause
    {
      fn preform( mut self ) -> Self::Preformed
      {
        #( let #field_names = self.#field_names.take().unwrap_or_default(); )*
        ( #( #field_names ),* )
      }
    }
  };
  
  // Generate Definition Types (manually construct to avoid trailing comma issue)
  let definition_types = if ctx.generics.params.is_empty() {
    quote!
    {
      #[ derive( Default, Debug ) ]
      pub struct #definition_types_name< __Context = (), __Formed = #enum_name >
      {
        _p : std::marker::PhantomData< ( __Context, __Formed ) >,
      }
    }
  } else {
    quote!
    {
      #[ derive( Default, Debug ) ]
      pub struct #definition_types_name< #enum_impl_generics, __Context = (), __Formed = #enum_name < #enum_ty_params > >
      #enum_where_clause
      {
        _p : std::marker::PhantomData< ( #enum_ty_params, __Context, __Formed ) >,
      }
    }
  };
  
  let definition_types_impl = if ctx.generics.params.is_empty() {
    quote!
    {
      impl former::FormerDefinitionTypes for #definition_types_name< (), #enum_name >
      {
        type Storage = #storage_name;
        type Context = ();
        type Formed = #enum_name;
      }
    }
  } else {
    quote!
    {
      impl< #enum_impl_generics, __Context, __Formed > former::FormerDefinitionTypes for #definition_types_name< #enum_ty_params, __Context, __Formed >
      #enum_where_clause
      {
        type Storage = #storage_name #enum_ty_generics;
        type Context = __Context;
        type Formed = __Formed;
      }
    }
  };
  
  let definition_types_mutator = if ctx.generics.params.is_empty() {
    quote!
    {
      impl former::FormerMutator for #definition_types_name< (), #enum_name >
      {}
    }
  } else {
    quote!
    {
      impl< #enum_ty_params, __Context, __Formed > former::FormerMutator for #definition_types_name< #enum_ty_params, __Context, __Formed >
      #enum_where_clause
      {}
    }
  };
  
  // Generate Definition (manually construct to avoid trailing comma issue)
  let definition = if ctx.generics.params.is_empty() {
    quote!
    {
      #[ derive( Default, Debug ) ]
      pub struct #definition_name< __Context = (), __Formed = #enum_name, __End = #end_name >
      {
        _p : std::marker::PhantomData< ( __Context, __Formed, __End ) >,
      }
    }
  } else {
    quote!
    {
      #[ derive( Default, Debug ) ]
      pub struct #definition_name< #enum_ty_params, __Context = (), __Formed = #enum_name < #enum_ty_params >, __End = #end_name >
      #enum_where_clause
      {
        _p : std::marker::PhantomData< ( #enum_ty_params, __Context, __Formed, __End ) >,
      }
    }
  };
  
  let definition_impl = if ctx.generics.params.is_empty() {
    quote!
    {
      impl former::FormerDefinition for #definition_name< (), #enum_name, #end_name >
      where
        #end_name : former::FormingEnd< #definition_types_name< (), #enum_name > >,
      {
        type Storage = #storage_name;
        type Context = ();
        type Formed = #enum_name;
        type Types = #definition_types_name< (), #enum_name >;
        type End = #end_name;
      }
    }
  } else {
    quote!
    {
      impl< #enum_ty_params, __Context, __Formed, __End > former::FormerDefinition for #definition_name< #enum_ty_params, __Context, __Formed, __End >
      where
        __End : former::FormingEnd< #definition_types_name< #enum_ty_params, __Context, __Formed > >,
        #enum_where_clause
      {
        type Storage = #storage_name #enum_ty_generics;
        type Context = __Context;
        type Formed = __Formed;
        type Types = #definition_types_name< #enum_ty_params, __Context, __Formed >;
        type End = __End;
      }
    }
  };
  
  // Generate Former struct (manually construct to avoid trailing comma issue)
  let former_struct = if ctx.generics.params.is_empty() {
    quote!
    {
      pub struct #former_name< __Definition = #definition_name >
      where
        __Definition : former::FormerDefinition< Storage = #storage_name >,
      {
        storage : __Definition::Storage,
        context : Option< __Definition::Context >,
        on_end : Option< __Definition::End >,
      }
    }
  } else {
    quote!
    {
      pub struct #former_name< #enum_ty_params, __Definition = #definition_name < #enum_ty_params > >
      where
        __Definition : former::FormerDefinition< Storage = #storage_name #enum_ty_generics >,
        #enum_where_clause
      {
        storage : __Definition::Storage,
        context : Option< __Definition::Context >,
        on_end : Option< __Definition::End >,
      }
    }
  };
  
  // Generate Former impl (manually construct to avoid trailing comma issue)
  let former_impl = if ctx.generics.params.is_empty() {
    quote!
    {
      impl< __Definition > #former_name< __Definition >
      where
        __Definition : former::FormerDefinition< Storage = #storage_name >,
      {
        #[ inline( always ) ] 
        pub fn form( self ) -> < __Definition::Types as former::FormerDefinitionTypes >::Formed 
        { 
          self.end() 
        }
        
        #[ inline( always ) ] 
        pub fn end( mut self ) -> < __Definition::Types as former::FormerDefinitionTypes >::Formed
        {
          let on_end = self.on_end.take().unwrap();
          let context = self.context.take();
          < __Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
          former::FormingEnd::call( &on_end, self.storage, context )
        }
        
        #[ inline( always ) ] 
        pub fn begin( storage : Option< __Definition::Storage >, context : Option< __Definition::Context >, on_end : __Definition::End ) -> Self
        { 
          Self { storage : storage.unwrap_or_default(), context, on_end : Some( on_end ) } 
        }
        
        #[ allow( dead_code ) ]
        #[ inline( always ) ] 
        pub fn new( on_end : __Definition::End ) -> Self 
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
    }
  } else {
    quote!
    {
      impl< #enum_ty_params, __Definition > #former_name< #enum_ty_params, __Definition >
      where
        __Definition : former::FormerDefinition< Storage = #storage_name #enum_ty_generics >,
        #enum_where_clause
      {
        #[ inline( always ) ] 
        pub fn form( self ) -> < __Definition::Types as former::FormerDefinitionTypes >::Formed 
        { 
          self.end() 
        }
        
        #[ inline( always ) ] 
        pub fn end( mut self ) -> < __Definition::Types as former::FormerDefinitionTypes >::Formed
        {
          let on_end = self.on_end.take().unwrap();
          let context = self.context.take();
          < __Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut self.context );
          former::FormingEnd::call( &on_end, self.storage, context )
        }
        
        #[ inline( always ) ] 
        pub fn begin( storage : Option< __Definition::Storage >, context : Option< __Definition::Context >, on_end : __Definition::End ) -> Self
        { 
          Self { storage : storage.unwrap_or_default(), context, on_end : Some( on_end ) } 
        }
        
        #[ allow( dead_code ) ]
        #[ inline( always ) ] 
        pub fn new( on_end : __Definition::End ) -> Self 
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
    }
  };
  
  // Generate End struct
  let end_struct = quote!
  {
    #[ derive( Default, Debug ) ]
    pub struct #end_name #enum_impl_generics
    #enum_where_clause
    {
    }
  };
  
  // Generate End impl
  let end_impl = quote!
  {
    impl #enum_impl_generics former::FormingEnd< #definition_types_name < #enum_ty_params, (), #enum_name < #enum_ty_params > > >
    for #end_name #enum_ty_generics
    #enum_where_clause
    {
      #[ inline( always ) ]
      fn call(
        &self,
        sub_storage : #storage_name #enum_ty_generics,
        _context : Option< () >,
      ) -> #enum_name #enum_ty_generics
      {
        let ( #( #field_names ),* ) = former::StoragePreform::preform( sub_storage );
        #enum_name #enum_ty_generics :: #variant_name ( #( #field_names ),* )
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
    #vis fn #method_name() -> #former_name #enum_ty_generics
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
        #vis fn #method_name( #( #field_names : impl Into< #field_types > ),* ) -> #enum_name #enum_ty_generics
        {
          #enum_name #enum_ty_generics :: #variant_name ( #( #field_names.into() ),* )
        }
      };
      ctx.standalone_constructors.push( standalone_method );
    } else {
      // Former builder-style standalone constructor - returns the implicit variant former
      let standalone_method = quote!
      {
        #[ inline( always ) ]
        #vis fn #method_name() -> #former_name #enum_ty_generics
        {
          #former_name::begin( None, None, #end_name::default() )
        }
      };
      ctx.standalone_constructors.push( standalone_method );
    }
  }

  Ok( result )
}