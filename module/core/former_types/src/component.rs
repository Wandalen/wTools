
/// Provides a generic interface for setting a component of a certain type on an object.
///
/// This trait abstracts the action of setting or replacing a component, where a component
/// can be any part or attribute of an object, such as a field value. It is designed to be
/// generic over the type of the component being set ( `T` ) and the type that can be converted
/// into the component ( `IntoT` ). This design allows for flexible implementations that can
/// accept various types that can then be converted into the required component type.
///
/// # Type Parameters
///
/// - `T` : The type of the component to be set on the implementing object. This type represents
///   the final form of the component as it should be stored or represented in the object.
/// - `IntoT` : The type that can be converted into `T`. This allows the `set` method to accept
///   different types that are capable of being transformed into the required component type `T`,
///   providing greater flexibility in setting the component.
///
/// # Examples
///
/// Implementing `ComponentAssign` to set a name string on a struct :
///
/// ```rust
/// use former_types::ComponentAssign; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
///
/// struct MyStruct
/// {
///   name : String,
/// }
///
/// impl< IntoT : Into< String > > ComponentAssign< String, IntoT > for MyStruct
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
#[ cfg( any( feature = "types_component_assign" ) ) ]
pub trait ComponentAssign< T, IntoT >
where
  IntoT : Into< T >,
{
  /// Sets or replaces the component on the object with the given value.
  ///
  /// This method takes ownership of the given value ( `component` ), which is of type `IntoT`.
  /// `component` is then converted into type `T` and set as the component of the object.
  fn assign( &mut self, component : IntoT );
}

/// The `AssignWithType` trait provides a mechanism to set a component on an object, utilizing the type information explicitly. This trait extends the functionality of `SetComponen`t by allowing implementers to specify the component's type at the method call site, enhancing expressiveness in code that manipulates object states.
///
/// ### Method Detail
///
/// - `assign_with_type::< T, IntoT >( &mut self, component : IntoT )`
///
/// This method allows an implementer of `SetWithTyp`e to set a component on self where the component's type is T, and the input value is of type `IntoT`, which can be converted into `T`. This method bridges the gap between dynamic type usage and static type enforcement, providing a flexible yet type-safe interface for modifying object states.
///
/// ### Type Parameters
///
/// - `T` : The type of the component to be set on the implementing object. This specifies the exact type expected by the object as its component.
/// - `IntoT` : A type that can be converted into T, providing flexibility in the types of values that can be used to set the component.
///
/// ### Example
///
/// ```rust
/// use former_types::{ ComponentAssign, AssignWithType }; // use crate `former` instead of crate `former_types` unless you need to use crate `former_types` directly
///
/// struct UserProfile
/// {
///   username : String,
/// }
///
/// impl< IntoT : Into< String > > ComponentAssign< String, IntoT > for UserProfile
//  where String: From< String >,
/// {
///   fn assign( &mut self, component : IntoT )
///   {
///     self.username = component.into();
///   }
/// }
///
/// let mut user_profile = UserProfile { username : String::new() };
/// user_profile.assign_with_type::< String, _ >( "john_doe" );
///
/// assert_eq!( user_profile.username, "john_doe" );
/// ```
///

#[ cfg( any( feature = "types_component_assign" ) ) ]
pub trait AssignWithType
{
  /// Function to set value of a component by its type.
  fn assign_with_type< T, IntoT >( &mut self, component : IntoT )
  where
    IntoT : Into< T >,
    Self : ComponentAssign< T, IntoT >;
}

#[ cfg( any( feature = "types_component_assign" ) ) ]
impl< S > AssignWithType for S
{

  #[ inline( always ) ]
  fn assign_with_type< T, IntoT >( &mut self, component : IntoT )
  where
    IntoT : Into< T >,
    Self : ComponentAssign< T, IntoT >,
  {
    ComponentAssign::< T, IntoT >::assign( self, component );
  }

}
