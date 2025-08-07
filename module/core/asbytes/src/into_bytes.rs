/// Define a private namespace for all its items.
mod private {

  pub use bytemuck::{Pod};

  /// Trait for consuming data into an owned byte vector.
  /// This trait is for types that can be meaningfully converted into a `Vec< u8 >`
  /// by consuming the original value.
  pub trait IntoBytes {
    /// Consumes the value and returns its byte representation as an owned `Vec< u8 >`.
    fn into_bytes(self) -> Vec<u8>;
  }

  // --- Implementations for IntoBytes ---

  /// Implementation for single POD types wrapped in a tuple `(T,)`.
  /// This mirrors the approach used in `AsBytes` for consistency with single items.
  /// Covers primitive types (u8, i32, f64, bool, etc.) and other POD structs when wrapped.
  impl<T: Pod> IntoBytes for (T,) {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // self.0 is the owned T value. Get bytes using bytes_of and clone to Vec.
      bytemuck::bytes_of(&self.0).to_vec()
    }
  }

  /// Implementation for &T.
  impl<T: Pod> IntoBytes for &T {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      bytemuck::bytes_of(self).to_vec()
    }
  }

  /// Implementation for String.
  impl IntoBytes for String {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // String::into_bytes already returns Vec< u8 >
      self.into_bytes()
    }
  }

  /// Implementation for &str.
  /// This handles string slices specifically.
  impl IntoBytes for &str {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // &str has a built-in method to get bytes.
      self.as_bytes().to_vec()
    }
  }

  /// Implementation for owned arrays of POD types.
  impl<T: Pod, const N: usize> IntoBytes for [T; N] {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // Since T: Pod, [T; N] is Copy (or moves if T isn't Copy, but Pod implies Copy usually).
      // Get a byte slice view using cast_slice (requires &self)
      // and then clone it into a Vec.
      bytemuck::cast_slice(&self).to_vec()
    }
  }

  /// Implementation for owned vectors of POD types.
  impl<T: Pod> IntoBytes for Vec<T> {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // Use bytemuck's safe casting for Vec<T> to Vec< u8 >
      bytemuck::cast_slice(self.as_slice()).to_vec()
    }
  }

  /// Implementation for Box<T> where T is POD.
  impl<T: Pod> IntoBytes for Box<T> {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // Dereference the Box to get T, get its bytes, and clone into a Vec.
      // The Box is dropped after self is consumed.
      bytemuck::bytes_of(&*self).to_vec()
    }
  }

  /// Implementation for &[T] where T is Pod.
  /// This handles slices of POD types specifically.
  impl<T: Pod> IntoBytes for &[T] {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // Use cast_slice on the borrowed slice and convert to owned Vec.
      bytemuck::cast_slice(self).to_vec()
    }
  }

  /// Implementation for Box<[T]> where T is POD.
  impl<T: Pod> IntoBytes for Box<[T]> {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // Dereference the Box to get &[T], cast to bytes, and clone into a Vec.
      // The Box is dropped after self is consumed.
      bytemuck::cast_slice(&self).to_vec()
    }
  }

  /// Implementation for `VecDeque`<T> where T is POD.
  impl<T: Pod> IntoBytes for std::collections::VecDeque<T> {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // Iterate through the deque, consuming it, and extend a byte vector
      // with the bytes of each element. This handles the potentially
      // non-contiguous nature of the deque's internal ring buffer safely.
      let mut bytes = Vec::with_capacity(self.len() * core::mem::size_of::<T>());
      for element in self {
        bytes.extend_from_slice(bytemuck::bytes_of(&element));
      }
      bytes
    }
  }

  /// Implementation for `CString`.
  /// Returns the byte slice *without* the trailing NUL byte.
  impl IntoBytes for std::ffi::CString {
    #[inline]
    fn into_bytes(self) -> Vec<u8> {
      // CString::into_bytes() returns the underlying buffer without the NUL.
      self.into_bytes()
    }
  }
}

#[doc(inline)]
#[allow(unused_imports)]
pub use own::*;

/// Own namespace of the module.
#[allow(unused_imports)]
pub mod own {
  use super::*;

  #[doc(inline)]
  pub use orphan::*;
}

#[doc(inline)]
#[allow(unused_imports)]
pub use own::*;

/// Orphan namespace of the module.
#[allow(unused_imports)]
pub mod orphan {
  use super::*;
  #[doc(inline)]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[allow(unused_imports)]
pub mod exposed {
  use super::*;

  #[doc(inline)]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[allow(unused_imports)]
pub mod prelude {
  use super::*;
  pub use private::IntoBytes;
}
