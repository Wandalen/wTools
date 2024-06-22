/// Internal namespace.
pub( crate ) mod private
{

//   pub trait TransitiveTryFrom< Transitive, Initial >
//   where
//
//     Transitive : TryFrom< Initial >,
//     < Self as TransitiveTryFrom< Transitive, Initial > >::Error : From< < Transitive as TryFrom< Initial > >::Error >,
//
//     Self : TryFrom< Transitive, Error = < Self as TransitiveTryFrom< Transitive, Initial > >::Error >,
//     // < Self as TryFrom< Transitive > >::Error : From< < Transitive as TryFrom< Initial > >::Error >,
//
//   {
//     type Error;
//
//     fn transitive_try_from( src : Initial ) -> Result< Self, < Self as TransitiveTryFrom< Transitive, Initial > >::Error >
//     {
//       let src2 = TryFrom::< Initial >::try_from( src )?;
//       TryFrom::< Transitive >::try_from( src2 )
//     }
//
//   }

  pub trait TransitiveTryFrom< Transitive, Initial >
  where
    Transitive : TryFrom< Initial >,
    < Self as TransitiveTryFrom< Transitive, Initial > >::Error : From< < Transitive as TryFrom< Initial > >::Error >,
    Self : TryFrom< Transitive, Error = < Self as TransitiveTryFrom< Transitive, Initial > >::Error >,
  {
    type Error;

    fn transitive_try_from( src : Initial ) -> Result< Self, < Self as TransitiveTryFrom< Transitive, Initial > >::Error >
    {
      let src2 = TryFrom::< Initial >::try_from( src )?;
      TryFrom::< Transitive >::try_from( src2 )
    }

  }

}

crate::mod_interface!
{
  exposed use TransitiveTryFrom;
  // exposed use TransitiveTryInto;
}
