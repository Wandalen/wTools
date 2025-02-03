/// Provides a generic interface for setting a component of a certain type on an object.
///
/// This trait abstracts the action of setting or replacing a component, where a component
/// can be any part or attribute of an object, such as a field value. It is designed to be
/// generic over the type of the component being set (`T`) and the type that can be converted
/// into the component (`IntoT`). This design allows for flexible implementations that can
/// accept various types that can then be converted into the required component type.
///
/// # Type Parameters
///
/// - `T`: The type of the component to be set on the implementing object. This type represents
///   the final form of the component as it should be stored or represented in the object.
/// - `IntoT`: The type that can be converted into `T`. This allows the `assign` method to accept
///   different types that are capable of being transformed into the required component type `T`,
///   providing greater flexibility in setting the component.
///
/// # Examples
///
/// Implementing `Assign` to set a name string on a struct:
///
/// ```rust
/// use former_types::Assign; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
///
/// struct MyStruct {
///   name: String,
/// }
///
/// impl< IntoT : Into< String > > Assign< String, IntoT > for MyStruct
/// {
///   fn assign( &mut self, component : IntoT )
///   {
///     self.name = component.into();
///   }
/// }
///
/// let mut obj = MyStruct { name : String::new() };
/// obj.assign( "New Name" );
/// assert_eq!( obj.name, "New Name" );
/// ```
#[ cfg( feature = "types_component_assign" ) ]
pub trait Assign< T, IntoT >
where
  IntoT : Into< T >,
{
  /// Sets or replaces the component on the object with the given value.
  ///
  /// This method takes ownership of the given value (`component`), which is of type `IntoT`.
  /// `component` is then converted into type `T` and set as the component of the object.
  fn assign( &mut self, component : IntoT );

  /// Sets or replaces the component on the object with the given value.
  /// Unlike function (`assing`) function (`impute`) also consumes self and return it what is useful for builder pattern.
  #[ inline( always ) ]
  #[ must_use ]
  fn impute( mut self, component : IntoT ) -> Self
  where
    Self : Sized,
  {
    self.assign( component );
    self
  }

}

/// Extension trait to provide a method for setting a component on an `Option<Self>`
/// if the `Option` is currently `None`. If the `Option` is `Some`, the method will
/// delegate to the `Assign` trait's `assign` method.
///
/// # Type Parameters
///
/// - `T`: The type of the component to be set on the implementing object. This type represents
///   the final form of the component as it should be stored or represented in the object.
///
/// # Examples
///
/// Using `option_assign` to set a component on an `Option`:
///
/// ```rust
/// use former_types::{ Assign, OptionExt }; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
///
/// struct MyStruct
/// {
///   name : String,
/// }
///
/// impl< IntoT : Into< MyStruct > > Assign< MyStruct, IntoT > for MyStruct
/// {
///   fn assign( &mut self, component : IntoT )
///   {
///     self.name = component.into().name;
///   }
/// }
///
/// let mut opt_struct: Option< MyStruct > = None;
/// opt_struct.option_assign( MyStruct { name: "New Name".to_string() } );
/// assert_eq!( opt_struct.unwrap().name, "New Name" );
/// ```
#[ cfg( feature = "types_component_assign" ) ]
pub trait OptionExt< T > : sealed::Sealed
where
  T : Sized + Assign< T, T >,
{
  /// Sets the component on the `Option` if it is `None`.
  ///
  /// If the `Option` is `Some`, the `assign` method is called to update the existing value.
  ///
  /// # Parameters
  ///
  /// - `src`: The value to assign to the `Option`.
  fn option_assign( & mut self, src : T );
}

#[ cfg( feature = "types_component_assign" ) ]
impl< T > OptionExt< T > for Option< T >
where
  T : Sized + Assign< T, T >,
{
  #[ inline( always ) ]
  fn option_assign( & mut self, src : T )
  {
    match self
    {
      Some( self_ref ) => Assign::assign( self_ref, Into::< T >::into( src ) ),
      None => * self = Some( src ),
    }
  }
}

#[ cfg( feature = "types_component_assign" ) ]
mod sealed
{
  pub trait Sealed {}
  impl< T > Sealed for Option< T >
  where
    T : Sized + super::Assign< T, T >,
  {}
}

/// The `AssignWithType` trait provides a mechanism to set a component on an object,
/// utilizing the type information explicitly. This trait extends the functionality of `Assign`
/// by allowing implementers to specify the component's type at the method call site,
/// enhancing expressiveness in code that manipulates object states.
///
/// # Type Parameters
///
/// - `T`: The type of the component to be set on the implementing object. This specifies
///   the exact type expected by the object as its component.
/// - `IntoT`: A type that can be converted into `T`, providing flexibility in the types of values
///   that can be used to set the component.
///
/// # Examples
///
/// Implementing `AssignWithType` to set a username on a struct:
///
/// ```rust
/// use former_types::{ Assign, AssignWithType }; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
///
/// struct UserProfile
/// {
///   username : String,
/// }
///
/// impl< IntoT : Into< String > > Assign< String, IntoT > for UserProfile
/// {
///   fn assign( &mut self, component : IntoT )
///   {
///     self.username = component.into();
///   }
/// }
///
/// let mut user_profile = UserProfile { username : String::new() };
/// user_profile.assign_with_type::< String, _ >("john_doe");
///
/// assert_eq!( user_profile.username, "john_doe" );
/// ```
#[ cfg( feature = "types_component_assign" ) ]
pub trait AssignWithType
{
  /// Sets the value of a component by its type.
  ///
  /// This method allows an implementer of `AssignWithType` to set a component on `self`
  /// where the component's type is `T`, and the input value is of type `IntoT`, which can be
  /// converted into `T`. This method bridges the gap between dynamic type usage and static type
  /// enforcement, providing a flexible yet type-safe interface for modifying object states.
  ///
  /// # Parameters
  ///
  /// - `component`: The value to assign to the component.
  ///
  /// # Type Parameters
  ///
  /// - `T`: The type of the component to be set on the implementing object.
  /// - `IntoT`: A type that can be converted into `T`.
  fn assign_with_type< T, IntoT >( & mut self, component : IntoT )
  where
    IntoT : Into< T >,
    Self : Assign< T, IntoT >;
}

#[ cfg( feature = "types_component_assign" ) ]
impl< S > AssignWithType for S
{
  #[ inline( always ) ]
  fn assign_with_type< T, IntoT >( & mut self, component : IntoT )
  where
    IntoT : Into< T >,
    Self : Assign< T, IntoT >,
  {
    Assign::< T, IntoT >::assign( self, component );
  }
}
