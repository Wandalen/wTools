
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
scalar_setter
subform_entry_setter
subform_collection_setter

scalar_setter_name
subform_scalar_setter_name,
subform_collection_setter_name
subform_entry_setter_name
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
  /// Used for initializing a Collection, where on initialization all fields are None. User can alter them through builder pattern
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
  /// Used to generate a Collection
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
    .and_then( | attr | attr.default.ref_internal() );

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
  /// types of setters based on the field attributes, such as scalar setters, collection setters,
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
  /// - **Collection Setters**: Generated when the field is annotated to behave as a collection,
  ///   supporting operations like adding or replacing elements.
  /// - **Subform Setters**: Generated for fields annotated as subforms, allowing for nested
  ///   forming processes where a field itself can be formed using a dedicated former.
  ///

  #[ inline ]
  pub fn former_field_setter
  (
    &self,
    item : &syn::Ident,
    original_input : &proc_macro::TokenStream,
    struct_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
    former : &syn::Ident,
    former_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
    former_storage : &syn::Ident,
  )
  -> Result< ( TokenStream, TokenStream ) >
  {

    // scalar setter
    let namespace_code = qt! {};
    let setters_code = self.scalar_setter
    (
      item,
      former,
      former_storage,
      original_input,
    );

    // subform scalar setter
    let ( setters_code, namespace_code ) = if self.attrs.subform_scalar.is_some()
    {
      let ( setters_code2, namespace_code2 ) = self.subform_scalar_setter
      (
        item,
        former,
        former_storage,
        former_generics_ty,
        struct_generics_impl,
        struct_generics_ty,
        struct_generics_where,
        original_input,
      )?;
      ( qt! { #setters_code #setters_code2 }, qt! { #namespace_code #namespace_code2 } )
    }
    else
    {
      ( setters_code, namespace_code )
    };

    // subform collection setter
    let ( setters_code, namespace_code ) = if let Some( _ ) = &self.attrs.subform_collection
    {
      let ( setters_code2, namespace_code2 ) = self.subform_collection_setter
      (
        item,
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

    // subform entry setter
    let ( setters_code, namespace_code ) = if self.attrs.subform_entry.is_some()
    {
      let ( setters_code2, namespace_code2 ) = self.subform_entry_setter
      (
        item,
        former,
        former_storage,
        former_generics_ty,
        struct_generics_impl,
        struct_generics_ty,
        struct_generics_where,
        original_input,
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
    item : &syn::Ident,
    former : &syn::Ident,
    former_storage : &syn::Ident,
    original_input : &proc_macro::TokenStream,
  )
  -> TokenStream
  {
    let field_ident = self.ident;
    let typ = self.non_optional_ty;
    let setter_name = self.scalar_setter_name();
    let attr = self.attrs.scalar.as_ref();

    if attr.is_some() && attr.unwrap().debug.value( false )
    {
      let debug = format!
      (
        r#"
impl< Definition > {former}< Definition >
where
  Definition : former::FormerDefinition< Storage = {former_storage} >,
{{
  #[ inline ]
  pub fn {field_ident}< Src >( mut self, src : Src ) -> Self
  where
    Src : ::core::convert::Into< {0} >,
  {{
    debug_assert!( self.storage.{field_ident}.is_none() );
    self.storage.{field_ident} = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
    self
  }}
}}
        "#,
        format!( "{}", qt!{ #typ } ),
      );
      let about = format!
      (
r#"derive : Former
item : {item}
field : {field_ident}"#,
      );
      diag::report_print( about, original_input, debug );
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

  ///
  /// Generate a collection setter for the 'field_ident' with the 'setter_name' name.
  ///
  /// See `tests/inc/former_tests/subform_collection_manual.rs` for example of generated code.
  ///

  #[ inline ]
  pub fn subform_collection_setter
  (
    &self,
    item : &syn::Ident,
    former : &syn::Ident,
    former_storage : &syn::Ident,
    former_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
    original_input : &proc_macro::TokenStream,
  )
  -> Result< ( TokenStream, TokenStream ) >
  {
    let attr = self.attrs.subform_collection.as_ref().unwrap();
    let field_ident = &self.ident;
    let field_typ = &self.non_optional_ty;
    let params = typ::type_parameters( &field_typ, .. );

    use convert_case::{ Case, Casing };

    // example : `ParentSubformCollectionChildrenEnd`
    let subform_collection_end = format_ident!
    {
      "{}SubformCollection{}End",
      item,
      field_ident.to_string().to_case( Case::Pascal )
    };

    // example : `_children_subform_collection`
    let subform_collection = format_ident!
    {
      "_{}_subform_collection",
      field_ident
    };
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
          #subform_collection_end< Definition >,
        >
      }
      // former::VectorDefinition< String, Self, Self, Struct1SubformCollectionVec1End, >
    }
    else
    {
      qt!
      {
        <
          #field_typ as former::EntityToDefinition< Self, Self, #subform_collection_end< Definition > >
        >::Definition
      }
      // < Vec< String > as former::EntityToDefinition< Self, Self, Struct1SubformCollectionVec1End > >::Definition
    };

    let doc = format!
    (
      "Collection setter for the '{}' field. Method {} unlike method {} accept custom collection subformer.",
      field_ident,
      subform_collection,
      field_ident,
    );

    let setter1 =
    qt!
    {

      #[ doc = #doc ]
      #[ inline( always ) ]
      pub fn #subform_collection< Former2 >( self ) -> Former2
      where
        Former2 : former::FormerBegin
        <
          #subformer_definition,
        >,
        #subformer_definition : former::FormerDefinition
        <
          // Storage : former::CollectionAdd< Entry = < #field_typ as former::Collection >::Entry >,
          Storage = #field_typ,
          Context = #former< #former_generics_ty >,
          End = #subform_collection_end< Definition >,
        >,
      {
        Former2::former_begin( None, Some( self ), #subform_collection_end::< Definition >::default() )
      }

      // #[ inline( always ) ]
      // pub fn _hashset_1_assign< Former2 >( self ) -> Former2
      // where
      //   Former2 : former::FormerBegin
      //   <
      //     former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > >,
      //   >,
      //   former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > > : former::FormerDefinition
      //   <
      //     Storage : former::CollectionAdd< Entry = < collection_tools::HashSet< String > as former::Collection >::Entry >,
      //     Context = Struct1Former< Definition >,
      //     End = Struct1SubformCollectionHashset1End< Definition >,
      //   >,
      // {
      //   Former2::former_begin( None, Some( self ), Struct1SubformCollectionHashset1End::< Definition >::default() )
      // }

    };

    let setter_name = self.subform_collection_setter_name();
    let setter2 = if let Some( setter_name ) = setter_name
    {
      qt!
      {

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) -> former::CollectionFormer::
        <
          // ( #( #params, )* ),
          < #field_typ as former::Collection >::Entry,
          #subformer_definition,
        >
        where
          #subformer_definition : former::FormerDefinition
          <
            // Storage : former::CollectionAdd< Entry = < #field_typ as former::Collection >::Entry >,
            Storage = #field_typ,
            Context = #former< #former_generics_ty >,
            End = #subform_collection_end < Definition >,
          >,
        {
          self.#subform_collection::< former::CollectionFormer::
          <
            _,
            _,
            // ( #( #params, )* ),
            //  #subformer_definition,
          > > ()
        }

        // #[ inline( always ) ]
        // pub fn hashset_1( self ) -> former::CollectionFormer::
        // <
        //   String,
        //   former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > >,
        // >
        // where
        //   former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > > : former::FormerDefinition
        //   <
        //     Storage : former::CollectionAdd< Entry = < collection_tools::HashSet< String > as former::Collection >::Entry >,
        //     Context = Struct1Former< Definition >,
        //     End = Struct1SubformCollectionHashset1End< Definition >,
        //   >,
        // {
        //   self._hashset_1_assign::< former::CollectionFormer::
        //   <
        //     String,
        //     former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > >,
        //   > > ()
        // }

      }
    }
    else
    {
      qt!{}
    };

    if attr.debug.value( false )
    {
      let debug = format!
      (
        r#"
/// The collection setter provides a collection setter that returns a CollectionFormer tailored for managing a collection of child entities. It employs a generic collection definition to facilitate operations on the entire collection, such as adding or updating elements.

impl< Definition, > {former}< Definition, >
where
  Definition : former::FormerDefinition< Storage = {former_storage} >,
{{

  #[ inline( always ) ]
  pub fn {field_ident}( self ) -> former::CollectionFormer::
  <
    ( {0} ),
    former::HashMapDefinition< {0} Self, Self, {subform_collection_end}< Definition >, >
    // Replace `HashMapDefinition` with definition for your collection
  >
  {{
    self.{subform_collection}()
  }}

}}
        "#,
        format!( "{}", qt!{ #( #params, )* } ),
      );
      let about = format!
      (
r#"derive : Former
item : {item}
field : {field_ident}"#,
      );
      diag::report_print( about, original_input, debug );
    }

    let setters_code = qt!
    {
      #setter1
      #setter2
    };

    // example : `former::VectorDefinition``
    let subformer_definition = self.attrs.subform_collection.as_ref().unwrap().definition.ref_internal();

    let subform_collection_end_doc = format!
    (
      r#"
A callback structure to manage the final stage of forming a `{0}` for the `{item}` collection.

This callback is used to integrate the contents of a temporary `{0}` back into the original `{item}` former
after the subforming process is completed. It replaces the existing content of the `{field_ident}` field in `{item}`
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

      #[ doc = #subform_collection_end_doc ]
      pub struct #subform_collection_end< Definition >
      {
        _phantom : core::marker::PhantomData< ( Definition, ) >,
      }

      impl< Definition > Default
      for #subform_collection_end< Definition >
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
      for #subform_collection_end< Definition >
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
            former::CollectionAssign::assign( field, storage );
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

  /// Generates setter functions to subform entries of a collection.
  ///
  /// This function is a key component of the `former` crate's capability to dynamically create setters for manipulating
  /// data within a nested collection structure like a `HashMap` or a `Vec`. The setters facilitate the addition or
  /// modification of entries within the collection, directly from the parent former's context.
  ///
  /// See `tests/inc/former_tests/subform_entry_manual.rs` for example of generated code.
  ///

  #[ inline ]
  pub fn subform_entry_setter
  (
    &self,
    item : &syn::Ident,
    former : &syn::Ident,
    former_storage : &syn::Ident,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
    original_input : &proc_macro::TokenStream,
  )
  -> Result< ( TokenStream, TokenStream ) >
  {

    use convert_case::{ Case, Casing };
    let field_ident = self.ident;
    let field_typ = self.non_optional_ty;
    let entry_typ : &syn::Type = typ::parameter_first( field_typ )?;

    let attr = self.attrs.subform_entry.as_ref().unwrap();
    // let params = typ::type_parameters( &self.non_optional_ty, .. );

    // example : `children`
    let setter_name = self.subform_entry_setter_name();

    // example : `ParentSubformEntryChildrenEnd`
    let subform_entry_end = format_ident!
    {
      "{}SubformEntry{}End",
      item,
      field_ident.to_string().to_case( Case::Pascal )
    };

    // example : `_children_subform_entry`
    let subform_entry = format_ident!
    {
      "_{}_subform_entry",
      field_ident
    };

    let doc = format!
    (
      r#"

Initiates the addition of {field_ident} to the `{item}` entity using a dedicated subformer.

This method configures and returns a subformer specialized for the `{0}` entities' formation process,
which is part of the `{item}` entity's construction. The subformer is set up with a specific end condition
handled by `{subform_entry_end}`, ensuring that the {field_ident} are properly integrated into the
parent's structure once formed.

# Returns

Returns an instance of `Former2`, a subformer ready to begin the formation process for `{0}` entities,
allowing for dynamic and flexible construction of the `{item}` entity's {field_ident}.

      "#,
      format!( "{}", qt!{ #field_typ } ),
    );

    let setters_code = qt!
    {

      #[ doc = #doc ]
      #[ inline( always ) ]
      pub fn #subform_entry< Former2, Definition2 >( self ) -> Former2
      where
        Definition2 : former::FormerDefinition
        <
          End = #subform_entry_end< Definition >,
          Storage = < < #field_typ as former::Collection >::Val as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Definition2::Types : former::FormerDefinitionTypes
        <
          Storage = < < #field_typ as former::Collection >::Val as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Former2 : former::FormerBegin< Definition2 >,
      {
        Former2::former_begin( None, Some( self ), #subform_entry_end::default() )
      }

    };

    let setters_code = if attr.setter()
    {

      let doc = format!
      (
        r#"
Provides a user-friendly interface to add an instancce of {field_ident} to the {item}.

# Returns

Returns an instance of `Former2`, a subformer ready to begin the formation process for `{0}` entities,
allowing for dynamic and flexible construction of the `{item}` entity's {field_ident}.

        "#,
        format!( "{}", qt!{ #field_typ } ),
      );

      qt!
      {
        #setters_code

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) ->
        < < #field_typ as former::Collection >::Val as former::EntityToFormer
          <
            <
              < #field_typ as former::Collection >::Val as former::EntityToDefinition< Self, Self, #subform_entry_end < Definition > >
            >::Definition,
          >
        >::Former
        // #as_subformer< Self, impl #as_subformer_end< Self > >
        {
          self.#subform_entry
          ::< < < #field_typ as former::Collection >::Val as former::EntityToFormer< _ > >::Former, _, >()
          // ::< #former< _ >, _, >()
        }
      }

      // #[ inline( always ) ]
      // pub fn child( self ) ->
      // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
      // {
      //   self._children_subform_entry
      //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
      // }

    }
    else
    {
      setters_code
    };

    if attr.debug.value( false )
    {
      let debug = format!
      (
        r#"
/// Initializes and configures a subformer for adding named child entities. This method leverages an internal function
/// to create and return a configured subformer instance. It allows for the dynamic addition of children with specific names,
/// integrating them into the formation process of the parent entity.

impl< Definition > {former}< Definition >
where
  Definition : former::FormerDefinition< Storage = {former_storage} >,
{{

  #[ inline( always ) ]
  pub fn {field_ident}( self ) -> {0}AsSubformer< Self, impl {0}AsSubformerEnd< Self > >
  {{
    self.{subform_entry}::< {0}Former< _ >, _, >()
  }}
  // Replace {0} with name of type of entry value.

}}
        "#,
        format!( "{}", qt!{ #entry_typ } ),
      );
      let about = format!
      (
r#"derive : Former
item : {item}
field : {field_ident}"#,
      );
      diag::report_print( about, original_input, debug );
    }

    let doc = format!
    (
      r#"

Implements the `FormingEnd` trait for `{subform_entry_end}` to handle the final
stage of the forming process for a `{item}` collection that contains `{0}` elements.

This implementation is tailored to manage the transition of {field_ident} elements from a substorage
temporary state into their final state within the `{item}`'s storage. The function ensures
that the `{item}`'s {field_ident} storage is initialized if not already set, and then adds the
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
formation process of the `{item}`.

      "#,
      format!( "{}", qt!{ #field_typ } ),
    );


    let namespace_code = qt!
    {

      #[ doc = #doc ]
      pub struct #subform_entry_end< Definition >
      {
        _phantom : core::marker::PhantomData< fn( Definition ) >,
      }

      impl< Definition > Default
      for #subform_entry_end< Definition >
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
      for #subform_entry_end< Definition >
      where
        Definition : former::FormerDefinition
        <
          Storage = < #item < #struct_generics_ty > as former::EntityToStorage >::Storage,
        >,
        Types2 : former::FormerDefinitionTypes
        <
          Storage = < < #field_typ as former::Collection >::Val as former::EntityToStorage >::Storage,
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
            former::CollectionAdd::add
            (
              field,
              < < #field_typ as former::Collection >::Val as former::ValToEntry< #field_typ > >
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

  /// Generates setter functions to subform scalar and all corresponding helpers.
  ///
  /// See `tests/inc/former_tests/subform_scalar_manual.rs` for example of generated code.

  #[ inline ]
  pub fn subform_scalar_setter
  (
    &self,
    item : &syn::Ident,
    former : &syn::Ident,
    _former_storage : &syn::Ident,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
    original_input : &proc_macro::TokenStream,
  )
  -> Result< ( TokenStream, TokenStream ) >
  {

    use convert_case::{ Case, Casing };
    let field_ident = self.ident;
    let field_typ = self.non_optional_ty;
    let attr = self.attrs.subform_scalar.as_ref().unwrap();
    // let params = typ::type_parameters( &self.non_optional_ty, .. );

    // example : `children`
    let setter_name = self.subform_scalar_setter_name();

    // example : `ParentSubformScalarChildrenEnd`
    let subform_scalar_end = format_ident!
    {
      "{}SubformScalar{}End",
      item,
      field_ident.to_string().to_case( Case::Pascal )
    };

    // example : `_children_subform_scalar`
    let subform_scalar = format_ident!
    {
      "_{}_subform_scalar",
      field_ident
    };

    let doc = format!
    (
      r#"

Initiates the scalar subformer for a `{0}` entity within a `{item}`.

This function creates a subformer specifically for handling scalar values associated with a `{0}` entity,
leveraging a dedicated end structure to integrate the formed value seamlessly back into the `{item}`.

## Type Parameters

- `Former2`: Represents the specific former to be returned.
- `Definition2`: Defines the former's setup including its end action and storage specifics.

## Returns

- `Former2`: An instance of the former configured to handle the scalar formation of a `{0}`.

This method prepares the forming context, ensuring that the subforming process for a scalar field in `{item}`
is properly initialized with all necessary configurations, including the default end action for integration.

## Usage

This function is typically called internally by a more user-friendly method that abstracts away the complex
generics, providing a cleaner interface for initiating subform operations on scalar fields.

      "#,
      format!( "{}", qt!{ #field_typ } ),
    );

    let setters_code = qt!
    {

      #[ doc = #doc ]
      #[ inline( always ) ]
      pub fn #subform_scalar< Former2, Definition2 >( self ) ->
      Former2
      where
        Definition2 : former::FormerDefinition
        <
          End = #subform_scalar_end< Definition >,
          Storage = < #field_typ as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Definition2::Types : former::FormerDefinitionTypes
        <
          Storage = < #field_typ as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Former2 : former::FormerBegin< Definition2 >,
      {
        Former2::former_begin( None, Some( self ), #subform_scalar_end::default() )
      }

      // #[ inline( always ) ]
      // pub fn _child_scalar_subformer< Former2, Definition2 >( self ) ->
      // Former2
      // where
      //   Definition2 : former::FormerDefinition
      //   <
      //     End = ParentFormerSubformScalarChildEnd< Definition >,
      //     Storage = < Child as former::EntityToStorage >::Storage,
      //     Formed = Self,
      //     Context = Self,
      //   >,
      //   Definition2::Types : former::FormerDefinitionTypes
      //   <
      //     Storage = < Child as former::EntityToStorage >::Storage,
      //     Formed = Self,
      //     Context = Self,
      //   >,
      //   Former2 : former::FormerBegin< Definition2 >,
      // {
      //   Former2::former_begin( None, Some( self ), ParentFormerSubformScalarChildEnd::default() )
      // }

    };

    let setters_code = if attr.setter()
    {

      let doc = format!
      (
        r#"
Provides a user-friendly interface to begin subforming a scalar `{0}` field within a `{item}`.

This method abstracts the underlying complex generics involved in setting up the former, simplifying the
user interaction needed to initiate the subform process for a scalar field associated with a `{0}`.

This method utilizes the more generic `{subform_scalar}` method to set up and return the subformer,
providing a straightforward and type-safe interface for client code. It encapsulates details about the specific
former and end action types, ensuring a seamless developer experience when forming parts of a `{item}`.

        "#,
        format!( "{}", qt!{ #field_typ } ),
      );

      qt!
      {
        #setters_code

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) ->
        < #field_typ as former::EntityToFormer
          <
            <
              #field_typ as former::EntityToDefinition< Self, Self, #subform_scalar_end < Definition > >
            >::Definition,
          >
        >::Former
        {
          self.#subform_scalar
          ::< < #field_typ as former::EntityToFormer< _ > >::Former, _, >()
        }

        // #[ inline( always ) ]
        // pub fn child( self ) ->
        // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
        // {
        //   self._child_scalar_subformer
        //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
        // }

      }

    }
    else
    {
      setters_code
    };

    if attr.debug.value( false )
    {
      let debug = format!
      (
        r#"
/// Extends `{former}` to include a method that initializes and configures a subformer for the '{field_ident}' field.
/// This function demonstrates the dynamic addition of a named {field_ident}, leveraging a subformer to specify detailed properties.

impl< Definition > {former}< Definition >
where
  Definition : former::FormerDefinition< Storage = < {item} as former::EntityToStorage >::Storage >,
{{
  #[ inline( always ) ]
  pub fn {field_ident}( self, name : &str ) -> {0}AsSubformer< Self, impl {0}AsSubformerEnd< Self > >
  {{
    self._{field_ident}_subform_scalar::< {0}Former< _ >, _, >().name( name )
  }}
}}
        "#,
        format!( "{}", qt!{ #field_typ } ),
      );
      let about = format!
      (
r#"derive : Former
item : {item}
field : {field_ident}"#,
      );
      diag::report_print( about, original_input, debug );
    }

    let doc = format!
    (
      r#"

Represents the endpoint for the forming process of a scalar field managed by a subformer within a `{item}` entity.

This structure is a critical component of the forming process when using a subform scalar setter. It handles
the finalization of the scalar field's value that has been configured through its dedicated subformer.
Essentially, this end action integrates the individually formed scalar value back into the parent structure.

## Type Parameters

- `Definition`: The type that defines the former setup for the `{item}` entity, influencing storage and behavior during forming.

## Parameters of `call`

- `substorage`: Storage type specific to the `{0}`, containing the newly formed scalar value.
- `super_former`: An optional context of the `{former}`, which will receive the value. The function ensures
  that this context is not `None` and inserts the formed value into the designated field within `{item}`'s storage.

      "#,
      format!( "{}", qt!{ #field_typ } ),
    );

    let namespace_code = qt!
    {

      #[ doc = #doc ]
      pub struct #subform_scalar_end< Definition >
      {
        _phantom : core::marker::PhantomData< fn( Definition ) >,
      }

      impl< Definition > Default
      for #subform_scalar_end< Definition >
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
      for #subform_scalar_end< Definition >
      where
        Definition : former::FormerDefinition
        <
          Storage = < #item < #struct_generics_ty > as former::EntityToStorage >::Storage,
        >,
        Types2 : former::FormerDefinitionTypes
        <
          Storage = < #field_typ as former::EntityToStorage >::Storage,
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
          debug_assert!( super_former.storage.#field_ident.is_none() );
          super_former.storage.#field_ident = Some( ::core::convert::Into::into( former::StoragePreform::preform( substorage ) ) );
          super_former
        }
      }

//       pub struct ParentFormerSubformScalarChildEnd< Definition >
//       {
//         _phantom : core::marker::PhantomData< fn( Definition ) >,
//       }
//
//       impl< Definition > Default
//       for ParentFormerSubformScalarChildEnd< Definition >
//       {
//         #[ inline( always ) ]
//         fn default() -> Self
//         {
//           Self
//           {
//             _phantom : core::marker::PhantomData,
//           }
//         }
//       }
//
//       impl< Types2, Definition > former::FormingEnd< Types2, >
//       for ParentFormerSubformScalarChildEnd< Definition >
//       where
//         Definition : former::FormerDefinition
//         <
//           Storage = < Parent as former::EntityToStorage >::Storage,
//         >,
//         Types2 : former::FormerDefinitionTypes
//         <
//           Storage = < Child as former::EntityToStorage >::Storage,
//           Formed = ParentFormer< Definition >,
//           Context = ParentFormer< Definition >,
//         >,
//       {
//         #[ inline( always ) ]
//         fn call
//         (
//           &self,
//           substorage : Types2::Storage,
//           super_former : core::option::Option< Types2::Context >,
//         )
//         -> Types2::Formed
//         {
//           let mut super_former = super_former.unwrap();
//           debug_assert!( super_former.storage.child.is_none() );
//           super_former.storage.child = Some( ::core::convert::Into::into( former::StoragePreform::preform( substorage ) ) );
//           super_former
//         }
//       }

    };

    // tree_print!( setters_code.as_ref().unwrap() );
    Ok( ( setters_code, namespace_code ) )
  }

  /// Get name of scalar setter.
  pub fn scalar_setter_name( &self ) -> &syn::Ident
  {
    if let Some( ref attr ) = self.attrs.scalar
    {
      if let Some( ref name ) = attr.name.ref_internal()
      {
        return name
      }
    }
    return &self.ident;
  }

  /// Get name of setter for subform scalar if such setter should be generated.
  pub fn subform_scalar_setter_name( &self ) -> Option< &syn::Ident >
  {
    if let Some( ref attr ) = self.attrs.subform_scalar
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name.ref_internal()
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

  /// Get name of setter for collection if such setter should be generated.
  pub fn subform_collection_setter_name( &self ) -> Option< &syn::Ident >
  {
    if let Some( ref attr ) = self.attrs.subform_collection
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name.ref_internal()
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
  pub fn subform_entry_setter_name( &self ) -> Option< &syn::Ident >
  {
    if let Some( ref attr ) = self.attrs.subform_entry
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name.as_ref()
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

  /// Is scalar setter required. Does not if collection of subformer setter requested.
  pub fn scalar_setter_required( &self ) -> bool
  {

    let mut explicit = false;
    if let Some( ref attr ) = self.attrs.scalar
    {
      if let Some( setter ) = attr.setter.internal()
      {
        if setter == false
        {
          return false
        }
        explicit = true;
      }
      if let Some( ref _name ) = attr.name.ref_internal()
      {
        explicit = true;
      }
    }

    if self.attrs.subform_scalar.is_some() && !explicit
    {
      return false;
    }

    if self.attrs.subform_collection.is_some() && !explicit
    {
      return false;
    }

    if self.attrs.subform_entry.is_some() && !explicit
    {
      return false;
    }

    return true;
  }

}
