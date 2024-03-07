/// Internal namespace.
pub( crate ) mod private
{
  use sha1::{ Sha1, Digest };

  // zzz : not used

  ///
  /// Make sha-1 hash for data.
  ///

  pub fn hash( data : &[ u8 ] ) -> Vec< u8 >
  {
    let mut hasher = Sha1::new();
    hasher.update( data );
    let result = hasher.finalize();
    result.to_vec()
  }
}

//

crate::mod_interface!
{
  orphan use hash;
}
