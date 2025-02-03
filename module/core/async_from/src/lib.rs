
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/async_from/latest/async_from/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::async_trait;
}

/// Define a private namespace for all its items.
#[ cfg( feature = "enabled" ) ]
mod private
{

  pub use async_trait::async_trait;
  use std::fmt::Debug;

  /// Trait for asynchronous conversions from a type `T`.
  ///
  /// This trait allows for conversions that occur asynchronously, returning a `Future`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use async_from::{ async_trait, AsyncFrom };
  ///
  /// struct MyNumber( u32 );
  ///
  /// #[ async_trait ]
  /// impl AsyncFrom< String > for MyNumber
  /// {
  ///   async fn async_from( value : String ) -> Self
  ///   {
  ///     let num = value.parse::< u32 >().unwrap_or( 0 );
  ///     MyNumber( num )
  ///   }
  /// }
  ///
  /// #[ tokio::main ]
  /// async fn main()
  /// {
  ///   let num = MyNumber::async_from( "42".to_string() ).await;
  ///   println!( "Converted: {}", num.0 );
  /// }
  /// ```
  #[ cfg( feature = "async_from" ) ]
  #[ async_trait ]
  pub trait AsyncFrom< T > : Sized
  {
    /// Asynchronously converts a value of type `T` into `Self`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be converted.
    ///
    /// # Returns
    ///
    /// * `Self` - The converted value.
    async fn async_from( value : T ) -> Self;
  }

  /// Trait for asynchronous conversions into a type `T`.
  ///
  /// This trait provides a method to convert `Self` into `T` asynchronously.
  ///
  /// # Example
  ///
  /// ```rust
  /// use async_from::{ async_trait, AsyncFrom, AsyncInto };
  ///
  /// struct MyNumber( u32 );
  ///
  /// #[ async_trait ]
  /// impl AsyncFrom< String > for MyNumber
  /// {
  ///   async fn async_from( value : String ) -> Self
  ///   {
  ///     let num = value.parse::< u32 >().unwrap_or( 0 );
  ///     MyNumber( num )
  ///   }
  /// }
  ///
  /// #[ tokio::main ]
  /// async fn main()
  /// {
  ///   let num : MyNumber = "42".to_string().async_into().await;
  ///   println!( "Converted: {}", num.0 );
  /// }
  /// ```
  #[ async_trait ]
  #[ cfg( feature = "async_from" ) ]
  pub trait AsyncInto< T > : Sized
  {
    /// Asynchronously converts `Self` into a value of type `T`.
    ///
    /// # Returns
    ///
    /// * `T` - The converted value.
    async fn async_into( self ) -> T;
  }

  /// Blanket implementation of `AsyncInto` for any type that implements `AsyncFrom`.
  ///
  /// This implementation allows any type `T` that implements `AsyncFrom<U>` to also implement `AsyncInto<U>`.
  #[ async_trait ]
  #[ cfg( feature = "async_from" ) ]
  impl< T, U > AsyncInto< U > for T
  where
    U : AsyncFrom< T > + Send,
    T : Send,
  {
    /// Asynchronously converts `Self` into a value of type `U` using `AsyncFrom`.
    ///
    /// # Returns
    ///
    /// * `U` - The converted value.
    async fn async_into( self ) -> U
    {
      U::async_from( self ).await
    }
  }

  /// Trait for asynchronous fallible conversions from a type `T`.
  ///
  /// This trait allows for conversions that may fail, returning a `Result` wrapped in a `Future`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use async_from::{ async_trait, AsyncTryFrom };
  /// use std::num::ParseIntError;
  ///
  /// struct MyNumber( u32 );
  ///
  /// #[ async_trait ]
  /// impl AsyncTryFrom< String > for MyNumber
  /// {
  ///   type Error = ParseIntError;
  ///
  ///   async fn async_try_from( value : String ) -> Result< Self, Self::Error >
  ///   {
  ///     let num = value.parse::< u32 >()?;
  ///     Ok( MyNumber( num ) )
  ///   }
  /// }
  ///
  /// #[ tokio::main ]
  /// async fn main()
  /// {
  ///   match MyNumber::async_try_from( "42".to_string() ).await
  ///   {
  ///     Ok( my_num ) => println!( "Converted successfully: {}", my_num.0 ),
  ///     Err( e ) => println!( "Conversion failed: {:?}", e ),
  ///   }
  /// }
  /// ```
  #[ async_trait ]
  #[ cfg( feature = "async_try_from" ) ]
  pub trait AsyncTryFrom< T > : Sized
  {
    /// The error type returned if the conversion fails.
    type Error : Debug;

    /// Asynchronously attempts to convert a value of type `T` into `Self`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be converted.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Self::Error>` - On success, returns the converted value. On failure, returns an error.
    async fn async_try_from( value : T ) -> Result< Self, Self::Error >;
  }

  /// Trait for asynchronous fallible conversions into a type `T`.
  ///
  /// This trait provides a method to convert `Self` into `T`, potentially returning an error.
  ///
  /// # Example
  ///
  /// ```rust
  /// use async_from::{ async_trait, AsyncTryFrom, AsyncTryInto };
  /// use std::num::ParseIntError;
  ///
  /// struct MyNumber( u32 );
  ///
  /// #[ async_trait ]
  /// impl AsyncTryFrom< String > for MyNumber
  /// {
  ///   type Error = ParseIntError;
  ///
  ///   async fn async_try_from( value : String ) -> Result< Self, Self::Error >
  ///   {
  ///     let num = value.parse::< u32 >()?;
  ///     Ok( MyNumber( num ) )
  ///   }
  /// }
  ///
  /// #[ tokio::main ]
  /// async fn main()
  /// {
  ///   let result : Result< MyNumber, _ > = "42".to_string().async_try_into().await;
  ///   match result
  ///   {
  ///     Ok( my_num ) => println!( "Converted successfully using AsyncTryInto: {}", my_num.0 ),
  ///     Err( e ) => println!( "Conversion failed using AsyncTryInto: {:?}", e ),
  ///   }
  /// }
  /// ```
  #[ async_trait ]
  #[ cfg( feature = "async_try_from" ) ]
  pub trait AsyncTryInto< T > : Sized
  {
    /// The error type returned if the conversion fails.
    type Error : Debug;

    /// Asynchronously attempts to convert `Self` into a value of type `T`.
    ///
    /// # Returns
    ///
    /// * `Result<T, Self::Error>` - On success, returns the converted value. On failure, returns an error.
    async fn async_try_into( self ) -> Result< T, Self::Error >;
  }

  /// Blanket implementation of `AsyncTryInto` for any type that implements `AsyncTryFrom`.
  ///
  /// This implementation allows any type `T` that implements `AsyncTryFrom<U>` to also implement `AsyncTryInto<U>`.
  #[ async_trait ]
  #[ cfg( feature = "async_try_from" ) ]
  impl< T, U > AsyncTryInto< U > for T
  where
    U : AsyncTryFrom< T > + Send,
    T : Send,
  {
    type Error = U::Error;

    /// Asynchronously converts `Self` into a value of type `U` using `AsyncTryFrom`.
    ///
    /// # Returns
    ///
    /// * `Result<U, Self::Error>` - On success, returns the converted value. On failure, returns an error.
    async fn async_try_into( self ) -> Result< U, Self::Error >
    {
      U::async_try_from( self ).await
    }
  }

}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use ::async_trait::async_trait;

  #[ cfg( feature = "async_from" ) ]
  pub use private::
  {
    AsyncFrom,
    AsyncInto,
  };

  #[ cfg( feature = "async_try_from" ) ]
  pub use private::
  {
    AsyncTryFrom,
    AsyncTryInto,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
