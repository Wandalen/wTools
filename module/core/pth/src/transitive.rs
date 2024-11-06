/// Define a private namespace for all its items.
mod private
{
  // xxx : move to derive_tools

  // qqq : write tests, lool into example
  //
  // impl< Initial > TransitiveTryFrom< AbsolutePath, PathError, Initial >
  // for CrateDir
  // where
  //   AbsolutePath : TryFrom< Initial >,
  //   PathError : From< < AbsolutePath as TryFrom< Initial > >::Error >,
  // {
  // }

  // qqq : implement transitive_from
  // qqq : implement transitive_into

  // qqq : move to derive_tools
  // qqq : write tests, look into example
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
  // qqq : implement transitive_into

  /// A trait to perform a transitive `try_from` conversion.
  ///
  /// This trait allows for a two-step conversion process where an initial type `Initial`
  /// is first converted to an intermediate type `Transitive`, and then to the final type `Self`.
  ///
  /// # Type Parameters
  ///
  /// - `Error`: The error type that can be produced during the conversion.
  /// - `Initial`: The initial type from which the conversion starts.
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
  /// use pth::TransitiveTryFrom;
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
  /// let initial = InitialType;
  /// let final_result : Result< FinalType, ConversionError > = FinalType::transitive_try_from::< IntermediateType >( initial );
  /// ```
  pub trait TransitiveTryFrom< Error, Initial >
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
    /// - `Ok(Self)`: If both conversions succeed.
    /// - `Err(Error)`: If either conversion fails.
    ///
    /// # Example
    ///
    /// See the trait-level documentation for an example.
    #[ inline( always ) ]
    fn transitive_try_from< Transitive >( src : Initial ) -> Result< Self, Error >
    where
      Transitive : TryFrom< Initial >,
      Self : TryFrom< Transitive, Error = Error >,
      Error : From< < Transitive as TryFrom< Initial > >::Error >,
    {
      let src2 = TryFrom::< Initial >::try_from( src )?;
      TryFrom::< Transitive >::try_from( src2 )
    }
  }

  impl< Initial, Error, Final > TransitiveTryFrom< Error, Initial > for Final {}

  /// A trait to perform a transitive `try_into` conversion.
  ///
  /// This trait allows for a two-step conversion process where an initial type `Self`
  /// is first converted to an intermediate type `Transitive`, and then to the final type `Final`.
  ///
  /// # Type Parameters
  ///
  /// - `Error`: The error type that can be produced during the conversion.
  /// - `Final`: The final type to which `Transitive` is converted.
  ///
  /// # Requirements
  ///
  /// - `Self` must implement `TryInto<Transitive>`.
  /// - `Transitive` must implement `TryInto<Final>` with the same error type.
  /// - `Error` must implement `From<<Self as TryInto<Transitive>>::Error>`.
  ///
  /// # Example
  ///
  /// ```rust
  /// use pth::TransitiveTryInto;
  /// use std::convert::TryInto;
  ///
  /// struct InitialType;
  /// struct IntermediateType;
  /// struct FinalType;
  /// struct ConversionError;
  ///
  /// impl TryInto< IntermediateType > for InitialType
  /// {
  ///   type Error = ConversionError;
  ///   fn try_into( self ) -> Result< IntermediateType, Self::Error >
  ///   {
  ///     // Conversion logic here
  ///     Ok( IntermediateType )
  ///   }
  /// }
  ///
  /// impl TryInto< FinalType > for IntermediateType
  /// {
  ///   type Error = ConversionError;
  ///   fn try_into( self ) -> Result< FinalType, Self::Error >
  ///   {
  ///     // Conversion logic here
  ///     Ok( FinalType )
  ///   }
  /// }
  ///
  /// let initial = InitialType;
  /// let final_result : Result< FinalType, ConversionError > = initial.transitive_try_into::< IntermediateType >();
  /// ```
  pub trait TransitiveTryInto< Error, Final > : Sized
  {
    /// Performs a transitive `try_into` conversion.
    ///
    /// This method first converts `self` to the intermediate type `Transitive`,
    /// and then converts the intermediate type to the final type `Final`.
    ///
    /// # Returns
    ///
    /// - `Ok(Final)`: If both conversions succeed.
    /// - `Err(Error)`: If either conversion fails.
    ///
    /// # Example
    ///
    /// See the trait-level documentation for an example.
    #[ inline( always ) ]
    fn transitive_try_into< Transitive >( self ) -> Result< Final, Error >
    where
      Self : TryInto< Transitive >,
      Transitive : TryInto< Final, Error = Error >,
      Error : From< < Self as TryInto< Transitive > >::Error >,
    {
      let src2 = TryInto::< Transitive >::try_into( self )?;
      TryInto::< Final >::try_into( src2 )
    }
  }

  impl< Error, Final, Initial > TransitiveTryInto< Error, Final > for Initial {}

}

crate::mod_interface!
{
  exposed use TransitiveTryFrom;
  exposed use TransitiveTryInto;
}
