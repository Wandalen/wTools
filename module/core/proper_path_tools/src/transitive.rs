/// Internal namespace.
pub( crate ) mod private
{
  // qqq : move to derive_tools

  // qqq : write tests, lool into example
  //
  // impl< Initial > TransitiveTryFrom< AbsolutePath, PathError, Initial >
  // for CrateDir
  // where
  //   AbsolutePath : TryFrom< Initial >,
  //   PathError : From< < AbsolutePath as TryFrom< Initial > >::Error >,
  // {
  // }

  // qqq : implement transitive_try_into
  // qqq : implement transitive_from
  // qqq : implement transitive_nto

  /// A trait to perform a transitive `try_from` conversion.
  ///
  /// This trait allows for a two-step conversion process where an initial type `Initial`
  /// is first converted to an intermediate type `Transitive`, and then to the final type `Self`.
  ///
  /// # Type Parameters
  ///
  /// - `Transitive`: The intermediate type to which `Initial` is converted first.
  /// - `Initial`: The initial type from which the conversion starts.
  /// - `Error`: The error type that can be produced during the conversion.
  ///
  /// # Requirements
  ///
  /// - `Transitive` must implement `TryFrom<Initial>`.
  /// - `Self` must implement `TryFrom<Transitive>` with the same error type.
  /// - `Error` must implement `From<<Transitive as TryFrom<Initial>>::Error>`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use proper_path_tools::TransitiveTryFrom;
  /// use std::convert::TryFrom;
  ///
  /// struct InitialType;
  /// struct IntermediateType;
  /// struct FinalType;
  /// struct ConversionError;
  ///
  /// impl TryFrom< InitialType > for IntermediateType
  /// {
  ///   type Error = ConversionError;
  ///   fn try_from( value : InitialType ) -> Result< Self, Self::Error >
  ///   {
  ///     // Conversion logic here
  ///     Ok( IntermediateType )
  ///   }
  /// }
  ///
  /// impl TryFrom< IntermediateType > for FinalType
  /// {
  ///   type Error = ConversionError;
  ///   fn try_from( value : IntermediateType ) -> Result< Self, Self::Error >
  ///   {
  ///     // Conversion logic here
  ///     Ok( FinalType )
  ///   }
  /// }
  ///
  /// impl TransitiveTryFrom< IntermediateType, ConversionError, InitialType > for FinalType {}
  ///
  /// let initial = InitialType;
  /// let final_result : Result< FinalType, ConversionError > = FinalType::transitive_try_from( initial );
  /// ```
  pub trait TransitiveTryFrom< Transitive, Error, Initial >
  where
    Transitive : TryFrom< Initial >,
    Self : TryFrom< Transitive, Error = Error >,
    Error : From< < Transitive as TryFrom< Initial > >::Error >,
  {
    /// Performs a transitive `try_from` conversion.
    ///
    /// This method first converts the `src` of type `Initial` to the intermediate type `Transitive`,
    /// and then converts the intermediate type to the final type `Self`.
    ///
    /// # Arguments
    ///
    /// - `src`: The initial value to be converted.
    ///
    /// # Returns
    ///
    /// - `Ok( Self )`: If both conversions succeed.
    /// - `Err( Error )`: If either conversion fails.
    ///
    /// # Example
    ///
    /// See the trait-level documentation for an example.
    #[ inline( always ) ]
    fn transitive_try_from( src : Initial ) -> Result< Self, Error >
    {
      let src2 = TryFrom::< Initial >::try_from( src )?;
      TryFrom::< Transitive >::try_from( src2 )
    }
  }

  // impl< Transitive, Initial, Error, Final >
  // TransitiveTryFrom< Transitive, Initial, Error >
  // for Final
  // where
  //   Transitive : TryFrom< Initial >,
  //   Self : TryFrom< Transitive, Error = Error >,
  //   Error : From< < Transitive as TryFrom< Initial > >::Error >,
  // {
  // }

  pub trait TransitiveTryInto< Error, Final > : Sized
  // where
    // Self : Unwrap< Self::Inner >,
  {
    // type Inner;
    #[ inline( always ) ]
    fn transitive_try_into< Transitive >( self ) -> Result< Final, Error >
    where
      Self : TryInto< Transitive >,
      Transitive : TryInto< Final, Error = Error >,
      Error : From< < Self as TryInto< Transitive > >::Error >,
    {
      let src2 = TryInto::< Transitive >::try_into( self.unwrap() )?;
      TryInto::< Final >::try_into( src2 )
    }
  }

  impl< Error, Final, Initial > TransitiveTryInto< Error, Final >
  for Initial
  // where
    // Self : Unwrap< Initial >,
  {
    // type Inner = Initial;
  }

  // pub trait TransitiveTryInto< Transitive, Error, Final > : Sized
  // where
  //   Self : Unwrap< Self::Inner >,
  //   Self::Inner : TryInto< Transitive >,
  //   Transitive : TryInto< Final, Error = Error >,
  //   Error : From< < Self::Inner as TryInto< Transitive > >::Error >,
  // {
  //   type Inner;
  //   #[ inline( always ) ]
  //   fn transitive_try_into( self ) -> Result< Final, Error >
  //   {
  //     let src2 = TryInto::< Transitive >::try_into( self.unwrap() )?;
  //     TryInto::< Final >::try_into( src2 )
  //   }
  // }

  // pub trait TransitiveTryInto< Transitive, Error, Final > : Sized
  // where
  //   Self : TryInto< Transitive >,
  //   Transitive : TryInto< Final, Error = Error >,
  //   Error : From< < Self as TryInto< Transitive > >::Error >,
  // {
  //   fn transitive_try_into( self ) -> Result< Final, Error >
  //   {
  //     let src2 = TryInto::< Transitive >::try_into( self )?;
  //     TryInto::< Final >::try_into( src2 )
  //   }
  // }

  // impl< Transitive, Error, Final, Initial > TransitiveTryInto< Transitive, Error, Final >
  // for Initial
  // where
  //   Self : TryInto< Transitive >,
  //   Transitive : TryInto< Final, Error = Error >,
  //   Error : From< < Self as TryInto< Transitive > >::Error >,
  // {
  //   type Inner = Initial;
  // }

  // xxx : move out

  pub trait Unwrap< T >
  {
    fn unwrap( self ) -> T;
  }

  impl< T > Unwrap< T > for ( T, )
  {
    #[ inline( always ) ]
    fn unwrap( self ) -> T
    {
      self.0
    }
  }

  impl< T > Unwrap< T > for T
  {
    #[ inline( always ) ]
    fn unwrap( self ) -> T
    {
      self
    }
  }

}

crate::mod_interface!
{
  exposed use TransitiveTryFrom;
  exposed use TransitiveTryInto;
  exposed use Unwrap;
}
