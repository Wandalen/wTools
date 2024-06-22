/// Internal namespace.
pub( crate ) mod private
{
  // qqq : move to derive_tools
  // qqq : write tests
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
    Error : From< <Transitive as TryFrom< Initial >>::Error >,
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
    fn transitive_try_from( src : Initial ) -> Result< Self, Error >
    {
      let src2 = TryFrom::< Initial >::try_from( src )?;
      TryFrom::< Transitive >::try_from( src2 )
    }
  }

  // impl< Transitive, Initial, Error, Final > TransitiveTryFrom< Transitive, Initial, Error >
  // for Final
  // where
  //   Transitive : TryFrom< Initial >,
  //   Self : TryFrom< Transitive, Error = Error >,
  //   Error : From< < Transitive as TryFrom< Initial > >::Error >,
  // {
  // }

}

crate::mod_interface!
{
  exposed use TransitiveTryFrom;
  // exposed use TransitiveTryInto;
}
