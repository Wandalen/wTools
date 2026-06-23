//! OpType wrapper for polymorphic value handling.

#[ cfg( feature = "std" ) ]
use std::{ vec, vec::Vec, string::String, collections::HashMap };
#[ cfg( all( feature = "use_alloc", not( feature = "std" ) ) ) ]
use alloc::{ vec, vec::Vec, string::String, collections::BTreeMap as HashMap };

///
/// Wrapper types to make transformation.
///
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum OpType<T> {
  /// Wrapper over single element of type `<T>`.
  Primitive(T),
  /// Wrapper over vector of elements of type `<T>`.
  Vector(Vec< T >),
  /// Wrapper over hash map of elements of type `<T>`.
  Map(HashMap<String, T>),
}

impl<T: Default> Default for OpType<T> {
  fn default() -> Self {
    OpType::Primitive(T::default())
  }
}

impl<T> From<T> for OpType<T> {
  fn from(value: T) -> Self {
    OpType::Primitive(value)
  }
}

impl<T> From<Vec< T >> for OpType<T> {
  fn from(value: Vec< T >) -> Self {
    OpType::Vector(value)
  }
}

#[ allow( clippy::from_over_into ) ]
impl<T> Into<Vec< T >> for OpType<T> {
  fn into(self) -> Vec< T > {
    match self {
      OpType::Primitive(val) => vec![val],
      OpType::Vector(vec) => vec,
      OpType::Map(_) => panic!("Cannot convert OpType::Map into Vec"),
    }
  }
}

impl<T: Clone> OpType<T> {
  /// Append item of `OpType` to current value. If current type is `Primitive`, then it will be converted to
  /// `Vector`.
  /// # Panics
  /// Panics if `self` or `item` is `OpType::Map` — use `insert` to add items to map variants.
  #[ must_use ]
  pub fn append(mut self, item: OpType<T>) -> OpType<T> {
    let mut mut_item = item;
    match self {
      OpType::Primitive(value) => match mut_item {
        OpType::Primitive(ins) => {
          let vector = vec![value, ins];
          OpType::Vector(vector)
        }
        OpType::Vector(ref mut vector) => {
          vector.insert(0, value);
          mut_item
        }
        OpType::Map(_) => panic!("Unexpected operation. Please, use method `insert` to insert item in hash map."),
      },
      OpType::Vector(ref mut vector) => match mut_item {
        OpType::Primitive(ins) => {
          vector.push(ins);
          self
        }
        OpType::Vector(ref mut ins_vec) => {
          vector.append(ins_vec);
          self
        }
        OpType::Map(_) => panic!("Unexpected operation. Please, use method `insert` to insert item in hash map."),
      },
      OpType::Map(_) => panic!("Unexpected operation. Please, use method `insert` to insert item in hash map."),
    }
  }

  /// Unwrap primitive value. Consumes self.
  pub fn primitive(self) -> Option< T > {
    match self {
      OpType::Primitive(v) => Some(v),
      OpType::Vector(_) | OpType::Map(_) => None,
    }
  }

  /// Unwrap vector value. Consumes self.
  pub fn vector(self) -> Option<Vec< T >> {
    match self {
      OpType::Vector(vec) => Some(vec),
      OpType::Primitive(_) | OpType::Map(_) => None,
    }
  }
}
