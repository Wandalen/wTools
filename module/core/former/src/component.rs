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
/// Implementing `SetComponent` to set a name string on a struct :
///
/// ```rust
/// use former::SetComponent;
///
/// struct MyStruct
/// {
///   name : String,
/// }
///
/// impl SetComponent< String, &str > for MyStruct
/// {
///   fn set( &mut self, component : &str )
///   {
///     self.name = component.into();
///   }
/// }
///
/// let mut obj = MyStruct { name : String::new() };
/// obj.set( "New Name" );
/// assert_eq!( obj.name, "New Name" );
/// ```
pub trait SetComponent< T, IntoT >
where
  IntoT : Into< T >,
{
  /// Sets or replaces the component on the object with the given value.
  ///
  /// This method takes ownership of the given value ( `component` ), which is of type `IntoT`.
  /// `component` is then converted into type `T` and set as the component of the object.
  fn set( &mut self, component : IntoT );
}
