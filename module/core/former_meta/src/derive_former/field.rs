
use super::*;
use macro_tools::{ container_kind };

///
/// Definition of a field.
///

#[ allow( dead_code ) ]
pub struct FormerField< 'a >
{
  pub attrs : FieldAttributes,
  pub vis : &'a syn::Visibility,
  pub ident : &'a syn::Ident,
  pub colon_token : &'a Option< syn::token::Colon >,
  pub ty : &'a syn::Type,
  pub non_optional_ty : &'a syn::Type,
  pub is_optional : bool,
  pub of_type : container_kind::ContainerKind,
  pub for_storage : bool,
  pub for_formed : bool,
}

impl< 'a > FormerField< 'a >
{

/** methods

from_syn

storage_fields_none
storage_field_optional
storage_field_preform
storage_field_name
former_field_setter
subform_setter
container_setter
scalar_setter

scalar_setter_name
container_setter_name
subform_setter_name
scalar_setter_required

*/

  /// Construct former field from [`syn::Field`]
  pub fn from_syn( field : &'a syn::Field, for_storage : bool, for_formed : bool ) -> Result< Self >
  {
    let attrs = FieldAttributes::from_attrs( field.attrs.iter() )?;
    let vis = &field.vis;
    let ident = field.ident.as_ref()
    .ok_or_else( || syn_err!( field, "Expected that each field has key, but some does not:\n  {}", qt!{ #field } ) )?;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_optional = typ::is_optional( ty );
    let of_type = container_kind::of_optional( ty ).0;
    let non_optional_ty : &syn::Type = if is_optional { typ::parameter_first( ty )? } else { ty };
    let field2 = Self
    {
      attrs,
      vis,
      ident,
      colon_token,
      ty,
      non_optional_ty,
      is_optional,
      of_type,
      for_storage,
      for_formed,
    };
    Ok( field2 )
  }

  ///
  /// Generate fields for initializer of a struct setting each field to `None`.
  ///
  /// Used for initializing a Container, where on initialization all fields are None. User can alter them through builder pattern
  ///
  /// ### Basic use-case. of output
  ///
  /// ```ignore
  /// int_1 : core::option::Option::None,
  /// string_1 : core::option::Option::None,
  /// int_optional_1 : core::option::Option::None,
  /// ```
  ///

  #[ inline( always ) ]
  pub fn storage_fields_none( &self ) -> TokenStream
  {
    let ident = Some( self.ident.clone() );
    let tokens = qt! { ::core::option::Option::None };
    let ty2 : syn::Type = syn::parse2( tokens ).unwrap();

    qt!
    {
      #ident : #ty2
    }
  }

  ///
  /// Generate field of the former for a field of the structure
  ///
  /// Used to generate a Container
  ///
  /// ### Basic use-case. of output
  ///
  /// ```ignore
  /// pub int_1 : core::option::Option< i32 >,
  /// pub string_1 : core::option::Option< String >,
  /// pub int_optional_1 :  core::option::Option< i32 >,
  /// pub string_optional_1 : core::option::Option< String >,
  /// ```
  ///

  #[ inline( always ) ]
  pub fn storage_field_optional( &self ) -> TokenStream
  {
    let ident = Some( self.ident.clone() );
    let ty = self.ty.clone();

    // let ty2 = if is_optional( &ty )
    let ty2 = if self.is_optional
    {
      qt! { #ty }
    }
    else
    {
      qt! { ::core::option::Option< #ty > }
    };

    qt!
    {
      pub #ident : #ty2
    }

  }

  ///
  /// Generate code converting a field of the former to the field of the structure.
  ///
  /// In simple terms, used on `form()` call to unwrap contained values from the former's storage.
  /// Will try to use default values if no values supplied by the former and the type implements `Default` trait.
  ///
  /// ### Generated code will look similar to this :
  ///
  /// ```ignore
  /// let int_1 : i32 = if self.storage.int_1.is_some()
  /// {
  ///   // if int_1 is optional
  ///   Some( self.storage.int_1.take().unwrap() )
  ///
  ///   // if int_1 isn't optional
  ///   self.storage.int_1.take().unwrap()
  /// }
  /// else
  /// {
  ///   // if int_1 is optional and has default
  ///   Some( i32::default().into() )
  ///
  ///   // if int_1 is optional and doesn't have default
  ///   None
  ///
  ///   // if int_1 isn't optional and has default
  ///   i32::default().into()
  ///
  ///   // if int_1 isn't optional and hasn't default
  ///   panic!( "Field 'int_1' isn't initialized" )
  /// };
  /// ```
  ///

  #[ inline( always ) ]
  pub fn storage_field_preform( &self ) -> Result< TokenStream >
  {

    if !self.for_formed
    {
      return Ok( qt!{} )
    }

    let ident = self.ident;
    let ty = self.ty;
    let default : Option< &syn::Expr > = self.attrs.config.as_ref()
    .and_then( | attr | attr.default.as_ref() );

    let tokens = if self.is_optional
    {

      let _else = match default
      {
        None =>
        {
          qt!
          {
            ::core::option::Option::None
          }
        }

        Some( default_val ) =>
        {
          qt!
          {
            ::core::option::Option::Some( ::core::convert::Into::into( #default_val ) )
          }
        }
      };

      qt!
      {
        let #ident = if self.#ident.is_some()
        {
          ::core::option::Option::Some( self.#ident.take().unwrap() )
        }
        else
        {
          #_else
        };
      }

    }
    else
    {

      let _else = match default
      {
        None =>
        {
          let panic_msg = format!( "Field '{}' isn't initialized", ident );
          qt!
          {
            {
              // By hardly utilizing deref coercion, we achieve conditional trait implementation
              trait MaybeDefault< T >
              {
                fn maybe_default( self : &Self ) -> T { panic!( #panic_msg ) }
              }

              // Panic on non-`Default` types
              impl< T > MaybeDefault< T >
              for &::core::marker::PhantomData< T >
              {}

              // Return default value on `Default`` types
              impl< T > MaybeDefault< T >
              for ::core::marker::PhantomData< T >
              where T : ::core::default::Default,
              {
                fn maybe_default( self : &Self ) -> T
                {
                  T::default()
                }
              }

              // default if `impl Default`, otherwise - panic
              ( &::core::marker::PhantomData::< #ty > ).maybe_default()
            }
          }
        }
        Some( default_val ) =>
        {
          qt!
          {
            ::core::convert::Into::into( #default_val )
          }
        }
      };

      qt!
      {
        let #ident = if self.#ident.is_some()
        {
          self.#ident.take().unwrap()
        }
        else
        {
          #_else
        };
      }

    };

    Ok( tokens )
  }

  ///
  /// Extract name of a field out.
  ///

  #[ inline( always ) ]
  pub fn storage_field_name( &self ) -> TokenStream
  {

    if !self.for_formed
    {
      return qt!{}
    }

    let ident = self.ident;
    qt!{ #ident, }

  }

  /// Generates former setters for the specified field within a struct or enum.
  ///
  /// This function is responsible for dynamically creating code that allows for the building
  /// or modifying of fields within a `Former`-enabled struct or enum. It supports different
  /// types of setters based on the field attributes, such as scalar setters, container setters,
  /// and subform setters.
  ///
  /// # Returns
  ///
  /// Returns a pair of `TokenStream` instances:
  /// - The first `TokenStream` contains the generated setter functions for the field.
  /// - The second `TokenStream` includes additional namespace or supporting code that might
  ///   be required for the setters to function correctly, such as definitions for end conditions
  ///   or callbacks used in the formation process.
  ///
  /// The generation of setters is dependent on the attributes of the field:
  /// - **Scalar Setters**: Created for basic data types and simple fields.
  /// - **Container Setters**: Generated when the field is annotated to behave as a container,
  ///   supporting operations like adding or replacing elements.
  /// - **Subform Setters**: Generated for fields annotated as subforms, allowing for nested
  ///   forming processes where a field itself can be formed using a dedicated former.
  ///

  #[ inline ]
  pub fn former_field_setter
  (
    &self,
    stru : &syn::Ident,
    struct_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
    former : &syn::Ident,
    former_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
    former_storage : &syn::Ident,
    original_input : &proc_macro::TokenStream,
  )
  -> Result< ( TokenStream, TokenStream ) >
  {
    let namespace_code = qt! {};
    let setters_code = self.scalar_setter
    (
      former,
      former_storage,
    );

    // container setter
    let ( setters_code, namespace_code ) = if let Some( _ ) = &self.attrs.container
    {
      let ( setters_code2, namespace_code2 ) = self.container_setter
      (
        stru,
        former,
        former_storage,
        former_generics_impl,
        former_generics_ty,
        former_generics_where,
        original_input,
      )?;
      ( qt! { #setters_code #setters_code2 }, qt! { #namespace_code #namespace_code2 } )
    }
    else
    {
      ( setters_code, namespace_code )
    };

    // subform setter
    let ( setters_code, namespace_code ) = if self.attrs.subform.is_some()
    {
      let ( setters_code2, namespace_code2 ) = self.subform_setter
      (
        stru,
        former,
        former_storage,
        former_generics_ty,
        struct_generics_impl,
        struct_generics_ty,
        struct_generics_where,
      )?;
      ( qt! { #setters_code #setters_code2 }, qt! { #namespace_code #namespace_code2 } )
    }
    else
    {
      ( setters_code, namespace_code )
    };

    // tree_print!( setters_code.as_ref().unwrap() );
    Ok( ( setters_code, namespace_code ) )
  }

  /// Generates setter functions for subforms within a container structure in a builder pattern.
  ///
  /// This function is a key component of the `former` crate's capability to dynamically create setters for manipulating
  /// data within a nested container structure like a `HashMap` or a `Vec`. The setters facilitate the addition or
  /// modification of entries within the container, directly from the parent former's context.
  ///
  /// See `examples/subformer_subform_manual.rs` for example of generated code.
  ///

  #[ inline ]
  pub fn subform_setter
  (
    &self,
    stru : &syn::Ident,
    former : &syn::Ident,
    former_storage : &syn::Ident,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  )
  -> Result< ( TokenStream, TokenStream ) >
  {

    // if self.attrs.subform.is_none()
    // {
    //   return Ok( qt!{ } );
    // }

    use convert_case::{ Case, Casing };
    let field_ident = self.ident;
    let field_typ = self.non_optional_ty;
    let attr = self.attrs.subform.as_ref().unwrap();
    // let params = typ::type_parameters( &self.non_optional_ty, .. );

    // example : `child`
    let setter_name = self.subform_setter_name();

    // example : `ParentFormerAddChildrenEnd``
    let former_add_end_name = format!( "{}FormerAdd{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
    let former_add_end = syn::Ident::new( &former_add_end_name, field_ident.span() );

    // example : `_children_former`
    let field_add_name = format!( "_{}_add", field_ident );
    let field_add = syn::Ident::new( &field_add_name, field_ident.span() );

    let doc = format!
    (
      r#"

Initiates the addition of {field_ident} to the `{stru}` entity using a dedicated subformer.

This method configures and returns a subformer specialized for the `{0}` entities' formation process,
which is part of the `{stru}` entity's construction. The subformer is set up with a specific end condition
handled by `{former_add_end}`, ensuring that the {field_ident} are properly integrated into the
parent's structure once formed.

# Returns

Returns an instance of `Former2`, a subformer ready to begin the formation process for `{0}` entities,
allowing for dynamic and flexible construction of the `{stru}` entity's {field_ident}.

      "#,
      format!( "{}", qt!{ #field_typ } ),
    );

    let setters_code = qt!
    {

      #[ doc = #doc ]
      #[ inline( always ) ]
      pub fn #field_add< Former2, Definition2 >( self ) -> Former2
      where
        Definition2 : former::FormerDefinition
        <
          End = #former_add_end< Definition >,
          Storage = < < #field_typ as former::Container >::Val as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Definition2::Types : former::FormerDefinitionTypes
        <
          Storage = < < #field_typ as former::Container >::Val as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Former2 : former::FormerBegin< Definition2 >,
      {
        Former2::former_begin( None, Some( self ), #former_add_end::default() )
      }

    };

    let setters_code = if attr.setter()
    {

      let doc = format!
      (
        r#"
Provides a user-friendly interface to add an instancce of {field_ident} to the {stru}.

# Returns

Returns an instance of `Former2`, a subformer ready to begin the formation process for `{0}` entities,
allowing for dynamic and flexible construction of the `{stru}` entity's {field_ident}.

        "#,
        format!( "{}", qt!{ #field_typ } ),
      );

      qt!
      {
        #setters_code

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) ->
        < < #field_typ as former::Container >::Val as former::EntityToFormer
          <
            <
              < #field_typ as former::Container >::Val as former::EntityToDefinition< Self, Self, #former_add_end < Definition > >
            >::Definition,
          >
        >::Former
        // #as_subformer< Self, impl #as_subformer_end< Self > >
        {
          self.#field_add
          ::< < < #field_typ as former::Container >::Val as former::EntityToFormer< _ > >::Former, _, >()
          // ::< #former< _ >, _, >()
        }
      }

      // #[ inline( always ) ]
      // pub fn child( self ) ->
      // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
      // {
      //   self._children_add
      //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
      // }

    }
    else
    {
      setters_code
    };

    if attr.hint
    {
      let hint = format!
      (
        r#"

/// Initializes and configures a subformer for adding named child entities. This method leverages an internal function
/// to create and return a configured subformer instance. It allows for the dynamic addition of children with specific names,
/// integrating them into the formation process of the parent entity.

impl< Definition > {}< Definition >
where
  Definition : former::FormerDefinition< Storage = {} >,
{{

  #[ inline( always ) ]
  pub fn {}( self ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {{
    self.{}::< ChildFormer< _ >, _, >()
  }}
  // Replace Child with name of type of element value.

}}
        "#,
        former,
        former_storage,
        field_ident,
        field_add_name,
      );
      println!( "{hint}" );
    }

    let doc = format!
    (
      r#"

Implements the `FormingEnd` trait for `{former_add_end}` to handle the final
stage of the forming process for a `{stru}` container that contains `{0}` elements.

This implementation is tailored to manage the transition of {field_ident} elements from a substorage
temporary state into their final state within the `{stru}`'s storage. The function ensures
that the `{stru}`'s {field_ident} storage is initialized if not already set, and then adds the
preformed elements to this storage.

# Type Parameters

- `Types2`: Represents the specific types associated with the `Former` trait being applied,
  which include storage, formed type, and context.
- `Definition`: Defines the `FormerDefinition` that outlines the storage structure and
  the end conditions for the formation process.

# Parameters

- `substorage`: The storage from which {field_ident} elements are preformed and retrieved.
- `super_former`: An optional context which, upon invocation, contains the `{former}`
  instance being formed.

# Returns

Returns the updated `{former}` instance with newly added {field_ident}, completing the
formation process of the `{stru}`.

      "#,
      format!( "{}", qt!{ #field_typ } ),
    );


    let namespace_code = qt!
    {

      #[ doc = #doc ]
      pub struct #former_add_end< Definition >
      {
        _phantom : core::marker::PhantomData< fn( Definition ) >,
      }

      impl< Definition > Default
      for #former_add_end< Definition >
      {
        #[ inline( always ) ]
        fn default() -> Self
        {
          Self
          {
            _phantom : core::marker::PhantomData,
          }
        }
      }

      impl< #struct_generics_impl Types2, Definition > former::FormingEnd< Types2, >
      for #former_add_end< Definition >
      where
        Definition : former::FormerDefinition
        <
          Storage = < #stru < #struct_generics_ty > as former::EntityToStorage >::Storage,
        >,
        Types2 : former::FormerDefinitionTypes
        <
          Storage = < < #field_typ as former::Container >::Val as former::EntityToStorage >::Storage,
          Formed = #former< #former_generics_ty >,
          Context = #former< #former_generics_ty >,
        >,
        #struct_generics_where
      {
        #[ inline( always ) ]
        fn call
        (
          &self,
          substorage : Types2::Storage,
          super_former : core::option::Option< Types2::Context >,
        )
        -> Types2::Formed
        {
          let mut super_former = super_former.unwrap();
          if super_former.storage.#field_ident.is_none()
          {
            super_former.storage.#field_ident = Some( Default::default() );
          }
          if let Some( ref mut field ) = super_former.storage.#field_ident
          {
            former::ContainerAdd::add
            (
              field,
              < < #field_typ as former::Container >::Val as former::ValToEntry< #field_typ > >
              ::val_to_entry( former::StoragePreform::preform( substorage ) ),
            );
          }
          super_former
        }
      }

    };

    // tree_print!( setters_code.as_ref().unwrap() );
    Ok( ( setters_code, namespace_code ) )
  }

  ///
  /// Generate a container setter for the 'field_ident' with the 'setter_name' name.
  ///
  /// See `examples/subformer_container_manual.rs` for example of generated code.

  #[ inline ]
  pub fn container_setter
  (
    &self,
    stru : &syn::Ident,
    former : &syn::Ident,
    former_storage : &syn::Ident,
    former_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
    original_input : &proc_macro::TokenStream,
  )
  -> Result< ( TokenStream, TokenStream ) >
  {
    let attr = self.attrs.container.as_ref().unwrap();
    let field_ident = &self.ident;
    let field_typ = &self.non_optional_ty;
    let params = typ::type_parameters( &field_typ, .. );

    use convert_case::{ Case, Casing };
    let former_assign_end_name = format!( "{}FormerAssign{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
    let former_assign_end = syn::Ident::new( &former_assign_end_name, field_ident.span() );
    let field_assign_name = format!( "_{}_container_former", field_ident );
    let field_assign = syn::Ident::new( &field_assign_name, field_ident.span() );

    // example : `former::VectorDefinition`
    let subformer_definition = &attr.definition;
    let subformer_definition = if subformer_definition.is_some()
    {
      qt!
      {
        #subformer_definition
        <
          #( #params, )*
          Self,
          Self,
          #former_assign_end< Definition >,
        >
      }
      // former::VectorDefinition< String, Self, Self, Struct1FormerAssignVec1End, >
    }
    else
    {
      qt!
      {
        <
          #field_typ as former::EntityToDefinition< Self, Self, #former_assign_end< Definition > >
        >::Definition
      }
      // < Vec< String > as former::EntityToDefinition< Self, Self, Struct1FormerAssignVec1End > >::Definition
    };

    let doc = format!
    (
      "Container setter for the '{}' field. Method {} unlike method {} accept custom container subformer.",
      field_ident,
      field_assign_name,
      field_ident,
    );

    let setter1 =
    qt!
    {

      #[ doc = #doc ]
      #[ inline( always ) ]
      pub fn #field_assign< Former2 >( self ) -> Former2
      where
        Former2 : former::FormerBegin
        <
          #subformer_definition,
        >,
        #subformer_definition : former::FormerDefinition
        <
          // Storage : former::ContainerAdd< Entry = < #field_typ as former::Container >::Entry >,
          Storage = #field_typ,
          Context = #former< #former_generics_ty >,
          End = #former_assign_end< Definition >,
        >,
      {
        Former2::former_begin( None, Some( self ), #former_assign_end::< Definition >::default() )
      }

      // #[ inline( always ) ]
      // pub fn _hashset_1_assign< Former2 >( self ) -> Former2
      // where
      //   Former2 : former::FormerBegin
      //   <
      //     former::HashSetDefinition< String, Self, Self, Struct1FormerAssignHashset1End< Definition > >,
      //   >,
      //   former::HashSetDefinition< String, Self, Self, Struct1FormerAssignHashset1End< Definition > > : former::FormerDefinition
      //   <
      //     Storage : former::ContainerAdd< Entry = < collection_tools::HashSet< String > as former::Container >::Entry >,
      //     Context = Struct1Former< Definition >,
      //     End = Struct1FormerAssignHashset1End< Definition >,
      //   >,
      // {
      //   Former2::former_begin( None, Some( self ), Struct1FormerAssignHashset1End::< Definition >::default() )
      // }

    };

    let setter_name = self.container_setter_name();
    let setter2 = if let Some( setter_name ) = setter_name
    {
      qt!
      {

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) -> former::ContainerFormer::
        <
          // ( #( #params, )* ),
          < #field_typ as former::Container >::Entry,
          #subformer_definition,
        >
        where
          #subformer_definition : former::FormerDefinition
          <
            // Storage : former::ContainerAdd< Entry = < #field_typ as former::Container >::Entry >,
            Storage = #field_typ,
            Context = #former< #former_generics_ty >,
            End = #former_assign_end < Definition >,
          >,
        {
          self.#field_assign::< former::ContainerFormer::
          <
            _,
            _,
            // ( #( #params, )* ),
            //  #subformer_definition,
          > > ()
        }

        // #[ inline( always ) ]
        // pub fn hashset_1( self ) -> former::ContainerFormer::
        // <
        //   String,
        //   former::HashSetDefinition< String, Self, Self, Struct1FormerAssignHashset1End< Definition > >,
        // >
        // where
        //   former::HashSetDefinition< String, Self, Self, Struct1FormerAssignHashset1End< Definition > > : former::FormerDefinition
        //   <
        //     Storage : former::ContainerAdd< Entry = < collection_tools::HashSet< String > as former::Container >::Entry >,
        //     Context = Struct1Former< Definition >,
        //     End = Struct1FormerAssignHashset1End< Definition >,
        //   >,
        // {
        //   self._hashset_1_assign::< former::ContainerFormer::
        //   <
        //     String,
        //     former::HashSetDefinition< String, Self, Self, Struct1FormerAssignHashset1End< Definition > >,
        //   > > ()
        // }

      }
    }
    else
    {
      qt!{}
    };

    if attr.hint
    {
      let hint = format!
      (
        r#"

/// The containr setter provides a container setter that returns a ContainerFormer tailored for managing a collection of child entities. It employs a generic container definition to facilitate operations on the entire collection, such as adding or updating elements.

impl< Definition, > {}< Definition, >
where
  Definition : former::FormerDefinition< Storage = {} >,
{{

  #[ inline( always ) ]
  pub fn {}( self ) -> former::ContainerFormer::
  <
    ( {} ),
    former::HashMapDefinition< {} Self, Self, {}< Definition >, >
    // Replace `HashMapDefinition` with definition for your container
  >
  {{
    self.{}()
  }}

}}

        "#,
        former,
        former_storage,
        field_ident,
        format!( "{}", qt!{ #( #params, )* } ),
        format!( "{}", qt!{ #( #params, )* } ),
        former_assign_end,
        field_assign,
      );
      let about = format!
      (
r#"derive : Former
structure : {stru}
field : {field_ident}"#,
      );
      diag::report_print( about, original_input, hint );
    }

    let setters_code = qt!
    {
      #setter1
      #setter2
    };

    // example : `former::VectorDefinition``
    let subformer_definition = &self.attrs.container.as_ref().unwrap().definition;

    let former_assign_end_doc = format!
    (
      r#"
A callback structure to manage the final stage of forming a `{0}` for the `{stru}` container.

This callback is used to integrate the contents of a temporary `{0}` back into the original `{stru}` former
after the subforming process is completed. It replaces the existing content of the `{field_ident}` field in `{stru}`
with the new content generated during the subforming process.
      "#,
      format!( "{}", qt!{ #field_typ } ),
    );

    let subformer_definition_types = if let Some( ref _subformer_definition ) = subformer_definition
    {
      let subformer_definition_types_string = format!( "{}Types", qt!{ #subformer_definition } );
      let subformer_definition_types : syn::Type = syn::parse_str( &subformer_definition_types_string )?;
      qt!
      {
        #subformer_definition_types
        <
          #( #params, )*
          #former< #former_generics_ty >,
          #former< #former_generics_ty >,
        >
      }
    }
    else
    {
      qt!
      {
        <
          #field_typ as former::EntityToDefinitionTypes
          <
            #former< #former_generics_ty >,
            #former< #former_generics_ty >,
          >
        >::Types
      }
    };

    let r = qt!
    {

      #[ doc = #former_assign_end_doc ]
      pub struct #former_assign_end< Definition >
      {
        _phantom : core::marker::PhantomData< ( Definition, ) >,
      }

      impl< Definition > Default
      for #former_assign_end< Definition >
      {

        #[ inline( always ) ]
        fn default() -> Self
        {
          Self
          {
            _phantom : core::marker::PhantomData,
          }
        }

      }

      #[ automatically_derived ]
      impl< #former_generics_impl > former::FormingEnd
      <
        // VectorDefinitionTypes
        #subformer_definition_types,
      >
      for #former_assign_end< Definition >
      where
        #former_generics_where
      {
        #[ inline( always ) ]
        fn call
        (
          &self,
          storage : #field_typ,
          super_former : Option< #former< #former_generics_ty > >,
        )
        -> #former< #former_generics_ty >
        {
          let mut super_former = super_former.unwrap();
          if let Some( ref mut field ) = super_former.storage.#field_ident
          {
            former::ContainerAssign::assign( field, storage );
          }
          else
          {
            super_former.storage.#field_ident = Some( storage );
          }
          super_former
        }
      }

    };

    // tree_print!( r.as_ref().unwrap() );
    let namespace_code = r;

    Ok( ( setters_code, namespace_code ) )
  }

  ///
  /// Generate a single scalar setter for the 'field_ident' with the 'setter_name' name.
  ///
  /// Used as a helper function for former_field_setter(), which generates alias setters
  ///
  /// # Example of generated code
  ///
  /// ```ignore
  /// #[ doc = "Setter for the 'int_1' field." ]
  /// #[ inline ]
  /// pub fn int_1< Src >( mut self, src : Src ) -> Self
  /// where
  ///   Src : ::core::convert::Into< i32 >,
  /// {
  ///   debug_assert!( self.int_1.is_none() );
  ///   self.storage.int_1 = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
  ///   self
  /// }
  /// ```

  #[ inline ]
  pub fn scalar_setter
  (
    &self,
    former : &syn::Ident,
    former_storage : &syn::Ident,
  )
  -> TokenStream
  {
    let field_ident = self.ident;
    let typ = self.non_optional_ty;
    let setter_name = self.scalar_setter_name();
    let attr = self.attrs.scalar.as_ref();

    if attr.is_some() && attr.unwrap().hint
    {
      let hint = format!
      (
        r#"

impl< Definition > {}< Definition >
where
  Definition : former::FormerDefinition< Storage = {} >,
{{
  #[ inline ]
  pub fn {}< Src >( mut self, src : Src ) -> Self
  where
    Src : ::core::convert::Into< {} >,
  {{
    debug_assert!( self.storage.{}.is_none() );
    self.storage.{} = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
    self
  }}
}}

        "#,
        former,
        former_storage,
        field_ident,
        format!( "{}", qt!{ #typ } ),
        field_ident,
        field_ident,
      );
      println!( "{hint}" );
    }

    if !self.scalar_setter_required()
    {
      return qt! {};
    }

    let doc = format!
    (
      "Scalar setter for the '{}' field.",
      field_ident,
    );

    qt!
    {
      #[ doc = #doc ]
      #[ inline ]
      pub fn #setter_name< Src >( mut self, src : Src ) -> Self
      where
        Src : ::core::convert::Into< #typ >,
      {
        debug_assert!( self.storage.#field_ident.is_none() );
        self.storage.#field_ident = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
        self
      }
    }
  }

  /// Get name of scalar setter.
  pub fn scalar_setter_name( &self ) -> &syn::Ident
  {
    if let Some( ref attr ) = self.attrs.scalar
    {
      if let Some( ref name ) = attr.name
      {
        return name
      }
    }
    return &self.ident;
  }

  /// Get name of setter for container if such setter should be generated.
  pub fn container_setter_name( &self ) -> Option< &syn::Ident >
  {

    if let Some( ref attr ) = self.attrs.container
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name
        {
          return Some( &name )
        }
        else
        {
          return Some( &self.ident )
        }
      }
    }

    return None;
  }

  /// Get name of setter for subform if such setter should be generated.
  pub fn subform_setter_name( &self ) -> Option< &syn::Ident >
  {

    if let Some( ref attr ) = self.attrs.subform
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name
        {
          return Some( &name )
        }
        else
        {
          return Some( &self.ident )
        }
      }
    }

    return None;
  }

  /// Is scalar setter required. Does not if container of subformer setter requested.
  pub fn scalar_setter_required( &self ) -> bool
  {

    let mut explicit = false;
    if let Some( ref attr ) = self.attrs.scalar
    {
      if let Some( setter ) = attr.setter
      {
        if setter == false
        {
          return false
        }
        explicit = true;
      }
      if let Some( ref _name ) = attr.name
      {
        explicit = true;
      }
    }

    if self.attrs.container.is_some() && !explicit
    {
      return false;
    }

    if self.attrs.subform.is_some() && !explicit
    {
      return false;
    }

    return true;
  }

}
