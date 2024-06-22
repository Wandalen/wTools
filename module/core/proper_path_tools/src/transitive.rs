/// Internal namespace.
pub( crate ) mod private
{

  pub trait TransitiveTryFrom< T > : Sized
  {
    type Error;

    // Required method
    fn transitive_try_from( value : T ) -> Result< Self, Self::Error >;
  }

//   impl< IntoAbsolutePathType > TransitiveTryFrom< IntoAbsolutePathType > for CrateDir
//   where
//     IntoAbsolutePathType : TryInto< AbsolutePath >,
//     PathError : From< < IntoAbsolutePathType as TryInto< AbsolutePath > >::Error >,
//   {
//     type Error = PathError;
//
//     #[ inline( always ) ]
//     fn transitive_try_from( crate_dir_path : IntoAbsolutePathType ) -> Result< Self, Self::Error >
//     {
//       // let crate_dir_path = IntoAbsolutePathType::into_absolute_path( crate_dir_path )?;
//       let crate_dir_path = TryInto::< AbsolutePath >::try_into( crate_dir_path )?;
//       if !crate_dir_path.as_ref().join( "Cargo.toml" ).is_file()
//       {
//         let err =  io::Error::new( io::ErrorKind::InvalidData, format!( "Cannot find crate dir at {crate_dir_path:?}" ) );
//         return Err( PathError::Io( err ) );
//       }
//       Ok( Self( crate_dir_path ) )
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
