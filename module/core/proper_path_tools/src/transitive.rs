/// Internal namespace.
pub( crate ) mod private
{

  pub trait TransitiveTryFrom< IntoMiddle, Middle, Error > : Sized
  {
    // type Error;

    // Required method
    fn transitive_try_from( value : IntoMiddle ) -> Result< Self, Error >;
  }

  impl< IntoMiddle, Middle, Final, Error > TransitiveTryFrom< IntoMiddle, Middle, Error > for Final
  where
    IntoMiddle : TryInto< Middle >,
    Error : From< < IntoMiddle as TryInto< Middle > >::Error >,
    Final : TryFrom< Middle, Error = Error >,
    < Final as TryFrom< Middle > >::Error : From< < IntoMiddle as TryInto< Middle > >::Error >
  {
    // type Error = Error;

    #[ inline( always ) ]
    fn transitive_try_from( src : IntoMiddle ) -> Result< Self, Error >
    {
      let src2 = TryInto::< Middle >::try_into( src )?;
      TryFrom::< Middle >::try_from( src2 )
    }
  }

//   impl< IntoMiddle, T > TransitiveTryFrom< IntoMiddle > for T
//   where
//     IntoMiddle : TryInto< T >,
//     // Error : From< < IntoMiddle as TryInto< T > >::Error >,
//   {
//     type Error = < IntoMiddle as TryInto< T > >::Error;
//
//     #[ inline( always ) ]
//     fn transitive_try_from( src : IntoMiddle ) -> Result< Self, Self::Error >
//     {
//       TryInto::< T >::try_into( src )
//     }
//   }

//   impl< T, U > TransitiveTryInto< U > for T
//   where
//     U : TransitiveTryFrom< T >,
//   {
//     type Error = U::Error;
//
//     #[ inline ]
//     fn transitive_try_into( self ) -> Result< U, U::Error >
//     {
//       U::transitive_try_from( self )
//     }
//   }
//
//   pub trait TransitiveTryInto< T > : Sized
//   {
//     type Error;
//
//     // Required method
//     fn transitive_try_into( self ) -> Result< T, Self::Error >;
//   }

}

crate::mod_interface!
{
  exposed use TransitiveTryFrom;
  // exposed use TransitiveTryInto;
}
