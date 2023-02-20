pub( crate ) mod private
{
  use wtools::{ Result, err };

  /// -
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub enum Type
  {
    /// String
    String,
    /// Number
    Number,
    /// Path
    Path,
  }

  /// Can be implemented for something that represents a type of value
  pub trait TryCast< T >
  {
    /// return casted value
    fn try_cast( &self, value : String ) -> Result< T >;
  }

  /// -
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum Value
  {
    /// String value
    String( String ),
    /// Number value(float number but can be casted to another types)
    Number( f64 ),
    /// Path
    Path( std::path::PathBuf ),
  }

  macro_rules! value_into_impl
  {
    ( $( $value_kind : path => $( $kind : ty => $cast : expr ),+ );+ ) =>
    {
      $( $(
        impl From< Value > for $kind
        {
          fn from( value : Value ) -> Self
          {
            match value
            {
              #[ allow( clippy::redundant_closure_call ) ]// ok because of it improve understanding what is `value` at macro call
              $value_kind( value ) => ( $cast )( value ),
              _ => panic!( "Unknown cast variant. Got `{value:?}` and try to cast to `{}`", stringify!( $kind ) )
            }
          }
        }
      )+ )+
    };
  }

  // makes from Value variant an native value
  value_into_impl!
  {
    Value::Number =>
      u32 => | value | value as u32,
      u64 => | value | value as u64,
      i32 => | value | value as i32,
      i64 => | value | value as i64,
      f32 => | value | value as f32,
      f64 => | value | value as f64;
    Value::String =>
      String => String::from,
      &'static str => | value : String | Box::leak( value.into_boxed_str() );
    Value::Path =>
      std::path::PathBuf => | value | value
  }

  impl TryCast< Value > for Type
  {
    fn try_cast( &self, value : String ) -> Result< Value >
    {
      match self
      {
        Self::String => Ok( Value::String( value ) ),
        Self::Number => value.parse().map_err( | _ | err!( "Can not parse number from `{}`", value ) ).map( Value::Number ),
        Self::Path => Ok( Value::Path( value.into() ) )
      }
    }
  }
}

//

crate::mod_interface!
{
  prelude use Type;
  prelude use Value;
  prelude use TryCast;
}
