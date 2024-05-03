
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
}

impl< 'a > FormerField< 'a >
{

/** methods

scalar_setter_name
container_setter_name
subform_setter_name
scalar_setter_required

none_map
optional_map
form_map
name_map
setter_map
subform_add_setter_map
container_setter
scalar_setter
former_assign_end_map
former_add_end_map

*/

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
  pub fn none_map( &self ) -> TokenStream
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
  pub fn optional_map( &self ) -> TokenStream
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
  pub fn form_map( &self ) -> Result< TokenStream >
  {
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
  pub fn name_map( &self ) -> syn::Ident
  {
    self.ident.clone()
  }

  // zzz : outdated, please update documentation
  ///
  /// Generate a former setter for the field.
  ///
  /// If aliases provided, also generate aliases
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
  ///
  /// /// #[ doc = "Setter for the 'int_1' field." ]
  /// #[ inline ]
  /// pub fn int_1_alias< Src >( mut self, src : Src ) -> Self
  /// where
  ///   Src : ::core::convert::Into< i32 >,
  /// {
  ///   debug_assert!( self.int_1.is_none() );
  ///   self.storage.int_1 = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
  ///   self
  /// }
  /// ```

  #[ inline ]
  pub fn setter_map
  (
    &self,
    stru : &syn::Ident,
  )
  -> Result< TokenStream >
  {
    let r = qt!{};

    // scalar setter
    let r = if self.scalar_setter_required()
    {
      let r2 = self.scalar_setter();
      qt!
      {
        #r
        #r2
      }
    }
    else
    {
      r
    };

    // container setter
    let r = if let Some( _ ) = &self.attrs.container
    {
      let r2 = self.container_setter( stru );
      qt!
      {
        #r
        #r2
      }
    }
    else
    {
      r
    };

    // subform setter
    let r = if self.attrs.subform.is_some()
    {
      let r2 = self.subform_add_setter_map( stru )?;
      qt!
      {
        #r
        #r2
      }
    }
    else
    {
      r
    };

    // tree_print!( r.as_ref().unwrap() );
    Ok( r )
  }

  /// zzz : write documentation
  #[ inline ]
  pub fn subform_add_setter_map
  (
    &self,
    stru : &syn::Ident,
  )
  -> Result< TokenStream >
  {

    if self.attrs.subform.is_none()
    {
      return Ok( qt!{ } );
    }

    use convert_case::{ Case, Casing };
    let field_ident = self.ident;
    let field_ty = self.non_optional_ty;
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

    let r = qt!
    {

      // zzz : improve documentation
      /// Setter returning former of element of container of the field as subformer.
      #[ inline( always ) ]
      pub fn #field_add< Former2, Definition2 >( self ) -> Former2
      where
        Definition2 : former::FormerDefinition
        <
          End = #former_add_end< Definition >,
          Storage = < < #field_ty as former::Container >::Val as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Definition2::Types : former::FormerDefinitionTypes
        <
          Storage = < < #field_ty as former::Container >::Val as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Former2 : former::FormerBegin< Definition2 >,
      {
        Former2::former_begin( None, Some( self ), #former_add_end::default() )
      }

    };

    // xxx : it should be printed by hint also
    let r = if attr.setter()
    {
      qt!
      {
        #r

        #[ inline( always ) ]
        pub fn #setter_name( self ) ->
        < < #field_ty as former::Container >::Val as former::EntityToFormer
          <
            <
              < #field_ty as former::Container >::Val as former::EntityToDefinition< Self, Self, #former_add_end < Definition > >
            >::Definition,
          >
        >::Former
        // #as_subformer< Self, impl #as_subformer_end< Self > >
        {
          self.#field_add
          ::< < < #field_ty as former::Container >::Val as former::EntityToFormer< _ > >::Former, _, >()
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
      r
    };

    // tree_print!( r.as_ref().unwrap() );
    Ok( r )
  }

  ///
  /// Generate a container setter for the 'field_ident' with the 'setter_name' name.
  ///
  /// # Example of generated code
  ///
  /// ```ignore
  /// pub fn hashmap_strings_1( mut self ) -> former::HashMapSubformer
  /// <
  ///   String,
  ///   String,
  ///   std::collections::HashMap< String, String >,
  ///   Struct1Former,
  ///   impl Fn( std::collections::HashMap< String, String >, core::option::Option< Self > ) -> Self
  /// >
  /// {
  ///   let formed = self.hashmap_strings_1.take();
  ///   let on_end = | formed : std::collections::HashMap< String, String >, mut former : core::option::Option< Self > | -> Self
  ///   {
  ///     former.hashmap_strings_1 = Some( formed );
  ///     former
  ///   };
  ///   former::HashMapSubformer::begin_coercing( formed, self, on_end )
  /// }
  /// ```
  /// zzz : update example

  #[ inline ]
  pub fn container_setter
  (
    &self,
    stru : &syn::Ident,
  )
  -> TokenStream
  {
    let field_ident = &self.ident;
    let non_optional_ty = &self.non_optional_ty;
    let params = typ::type_parameters( &non_optional_ty, .. );

    use convert_case::{ Case, Casing };
    let former_assign_end_name = format!( "{}FormerAssign{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
    let former_assign_end = syn::Ident::new( &former_assign_end_name, field_ident.span() );
    let field_assign_name = format!( "_{}_assign", field_ident );
    let field_assign = syn::Ident::new( &field_assign_name, field_ident.span() );

    // example : `former::VectorDefinition`
    let subformer_definition = &self.attrs.container.as_ref().unwrap().definition;
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
        < #non_optional_ty as former::EntityToDefinition< Self, Self, #former_assign_end< Definition > > >::Definition
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
          #subformer_definition
        >,
      {
        Former2::former_begin( None, Some( self ), #former_assign_end::< Definition >::default() )
      }
    };

    let setter_name = self.container_setter_name();
    let setter2 = if let Some( setter_name ) = setter_name
    {
      if params.len() > 1
      {
        qt!
        {

          #[ doc = #doc ]
          #[ inline( always ) ]
          pub fn #setter_name( self ) ->
          former::ContainerSubformer::
          <
            ( #( #params, )* ), #subformer_definition
          >
          {
            self.#field_assign::< former::ContainerSubformer::
            <
              ( #( #params, )* ), #subformer_definition
            > >()
          }

        }
      }
      else
      {
        qt!
        {

          #[ doc = #doc ]
          #[ inline( always ) ]
          pub fn #setter_name( self ) ->
          former::ContainerSubformer::
          <
            #( #params, )* #subformer_definition
          >
          {
            self.#field_assign::< former::ContainerSubformer::
            <
              #( #params, )* #subformer_definition
            > >()
          }

        }
      }
    }
    else
    {
      qt!{}
    };

    qt!
    {
      #setter1
      #setter2
    }

  //   #[ inline( always ) ]
  //   pub fn vec_1_assign< Former2 >( self ) -> Former2
  //   where
  //     Former2 : former::FormerBegin
  //     <
  //       former::VectorDefinition
  //       <
  //         String,
  //         Self,
  //         Self,
  //         Struct1FormerVec_1End,
  //       >
  //     >,
  //   {
  //     Former2::_begin( None, Some( self ), Struct1FormerVec_1End )
  //   }
  //
  //   pub fn vec_1( self ) ->
  //   former::ContainerSubformer::
  //   <
  //     String, former::VectorDefinition< String, Self, Self, Struct1FormerVec_1End >
  //   >
  //   {
  //     self.vec_1_assign::< former::ContainerSubformer::
  //     <
  //       String, former::VectorDefinition< String, Self, Self, Struct1FormerVec_1End >
  //     >>()
  //   }

  }

  ///
  /// Generate a single scalar setter for the 'field_ident' with the 'setter_name' name.
  ///
  /// Used as a helper function for setter_map(), which generates alias setters
  ///
  /// # Example of generated code
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
  )
  -> TokenStream
  {
    let field_ident = &self.ident;
    let typ = &self.non_optional_ty;
    let setter_name = self.scalar_setter_name();

    let doc = format!
    (
      "Setter for the '{}' field.",
      field_ident,
    );

    qt!
    {
      #[ doc = #doc ]
      #[ inline ]
      pub fn #setter_name< Src >( mut self, src : Src ) -> Self
      where Src : ::core::convert::Into< #typ >,
      {
        debug_assert!( self.storage.#field_ident.is_none() );
        self.storage.#field_ident = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
        self
      }
    }
  }

  // zzz : description and exmaple
  /// Generate unit struct which is descriptor of callback which should be called after subforming process of a specific field. Childs are used insted of closures to inline code and let optimizer play with optimization.
  ///
  /// # Example of generated code
  ///
  /// ```rust, ignore
  /// pub struct Struct1FormerVec_1End;
  /// #[ automatically_derived ]
  /// impl< Definition > former::FormingEnd
  /// <
  ///   former::VectorDefinition< String, Struct1Former< Definition >, Struct1Former< Definition >, former::NoEnd >,
  /// >
  /// for Struct1FormerVec_1End
  /// where
  ///   Definition : former::FormerDefinition,
  ///   Definition::Types : former::FormerDefinitionTypes
  ///   <
  ///     Storage = Struct1FormerStorage
  ///   >,
  /// {
  ///   #[ inline( always ) ]
  ///   pub fn call
  ///   (
  ///     &self, storage : Vec< String >,
  ///     super_former : Option< Struct1Former< Definition > >,
  ///   )
  ///   -> Struct1Former< Definition >
  ///   {
  ///     let mut super_former = super_former.unwrap();
  ///     if let Some( ref mut field ) = super_former.storage.vec_1
  ///     {
  ///       former::ContainerAssign::assign( field, storage );
  ///     }
  ///     else
  ///     {
  ///       super_former.storage.vec_1 = Some( storage );
  ///     }
  ///     super_former
  ///   }
  /// }
  /// ```

  #[ inline ]
  pub fn former_assign_end_map
  (
    &self,
    stru : &syn::Ident,
    former : &syn::Ident,
    former_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    former_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  )
  ->
  Result< TokenStream >
  {

    if self.attrs.container.is_none()
    {
      return Ok( qt!{ } );
    }

    use convert_case::{ Case, Casing };
    let field_ident = self.ident;
    let field_ty = self.non_optional_ty;
    let params = typ::type_parameters( field_ty, .. );

    // example : `ParentFormerAssignChildsEnd``
    let former_assign_end_name = format!( "{}FormerAssign{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
    let former_assign_end = syn::Ident::new( &former_assign_end_name, field_ident.span() );

    // example : `former::VectorDefinition``
    let subformer_definition = &self.attrs.container.as_ref().unwrap().definition;

    // zzz : improve description
    let former_assign_end_doc = format!
    (
  r#"Callback to return original former after forming of container for `${stru}` is done.#

  Callback replace content of container assigning new content from subformer's storage."#
    );

    let subformer_definition = if subformer_definition.is_some()
    {
      qt!
      {
        #subformer_definition < #( #params, )* #former< #former_generics_ty >, #former< #former_generics_ty >, former::NoEnd >
      }
      // former::VectorDefinition< String, Struct1Former< Definition, >, Struct1Former< Definition, >, former::NoEnd >
    }
    else
    {
      qt!
      {
        < #field_ty as former::EntityToDefinition< #former< #former_generics_ty >, #former< #former_generics_ty >, former::NoEnd > >::Definition
      }
      // < Vec< String > as former::EntityToDefinition< Struct1Former< Definition, >, Struct1Former< Definition, >, former::NoEnd > >::Definition
    };

    let r = qt!
    {

      // zzz : improve description
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
        #subformer_definition,
      >
      for #former_assign_end< Definition >
      where
        #former_generics_where
      {
        #[ inline( always ) ]
        fn call
        (
          &self,
          storage : #field_ty,
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
    Ok( r )
  }

  /// zzz : write documentation

  #[ inline ]
  pub fn former_add_end_map
  (
    &self,
    stru : &syn::Ident,
    former : &syn::Ident,
    former_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
    struct_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  )
  ->
  Result< TokenStream >
  {

    if self.attrs.subform.is_none()
    {
      return Ok( qt!{ } );
    }

    use convert_case::{ Case, Casing };
    let field_ident = self.ident;
    let field_ty = self.non_optional_ty;
    // let params = typ::type_parameters( &self.non_optional_ty, .. );

    // example : `ParentFormerAddChildrenEnd``
    let former_add_end_name = format!( "{}FormerAdd{}End", stru, field_ident.to_string().to_case( Case::Pascal ) );
    let former_add_end = syn::Ident::new( &former_add_end_name, field_ident.span() );

    let r = qt!
    {

      // zzz : improve description
      /// Handles the completion of an element of subformer's container.
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
          Storage = < < #field_ty as former::Container >::Val as former::EntityToStorage >::Storage,
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
              < < #field_ty as former::Container >::Val as former::ValToElement< #field_ty > >
              ::val_to_element( former::StoragePreform::preform( substorage ) ),
            );
          }
          super_former
        }
      }

    };

    // tree_print!( r.as_ref().unwrap() );
    Ok( r )
  }

}