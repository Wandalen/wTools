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
/// Implementing `ComponentSet` to set a name string on a struct :
///
/// ```rust
/// use former::ComponentSet;
///
/// struct MyStruct
/// {
///   name : String,
/// }
///
/// impl< IntoT : Into< String > > ComponentSet< String, IntoT > for MyStruct
/// {
///   fn set( &mut self, component : IntoT )
///   {
///     self.name = component.into();
///   }
/// }
///
/// let mut obj = MyStruct { name : String::new() };
/// obj.set( "New Name" );
/// assert_eq!( obj.name, "New Name" );
/// ```
pub trait ComponentSet< T, IntoT >
where
  IntoT : Into< T >,
{
  /// Sets or replaces the component on the object with the given value.
  ///
  /// This method takes ownership of the given value ( `component` ), which is of type `IntoT`.
  /// `component` is then converted into type `T` and set as the component of the object.
  fn set( &mut self, component : IntoT );
}

/// The `SetWithType` trait provides a mechanism to set a component on an object, utilizing the type information explicitly. This trait extends the functionality of `SetComponen`t by allowing implementers to specify the component's type at the method call site, enhancing expressiveness in code that manipulates object states.
///
/// ### Method Detail
///
/// - `set_with_type::< T, IntoT >( &mut self, component : IntoT )`
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
/// use former::{ ComponentSet, SetWithType };
///
/// struct UserProfile
/// {
///   username : String,
/// }
///
/// impl< IntoT : Into< String > > ComponentSet< String, IntoT > for UserProfile
//  where String: From< String >,
/// {
///   fn set( &mut self, component : IntoT )
///   {
///     self.username = component.into();
///   }
/// }
///
/// let mut user_profile = UserProfile { username : String::new() };
/// user_profile.set_with_type::< String, _ >( "john_doe" );
///
/// assert_eq!( user_profile.username, "john_doe" );
/// ```
///

pub trait SetWithType
{
  /// Function to set value of a component by its type.
  fn set_with_type< T, IntoT >( &mut self, component : IntoT )
  where
    IntoT : Into< T >,
    Self : ComponentSet< T, IntoT >;
}

impl< S > SetWithType for S
{

  #[ inline( always ) ]
  fn set_with_type< T, IntoT >( &mut self, component : IntoT )
  where
    IntoT : Into< T >,
    Self : ComponentSet< T, IntoT >,
  {
    ComponentSet::< T, IntoT >::set( self, component );
  }

}
