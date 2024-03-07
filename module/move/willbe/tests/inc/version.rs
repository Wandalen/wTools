use crate ::TheModule ::version ::Version;
use std ::str ::FromStr;

#[ test ]
fn patch()
{
  // Arrange
  let version = Version ::from_str( "0.0.0" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "0.0.1", &new_version.to_string() );
}

#[ test ]
fn minor_without_patches()
{
  // Arrange
  let version = Version ::from_str( "0.1.0" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "0.2.0", &new_version.to_string() );
}

#[ test ]
fn minor_with_patch()
{
  // Arrange
  let version = Version ::from_str( "0.1.1" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "0.2.0", &new_version.to_string() );
}

#[ test ]
fn major_without_patches()
{
  // Arrange
  let version = Version ::from_str( "1.0.0" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "2.0.0", &new_version.to_string() );
}

#[ test ]
fn major_with_minor()
{
  // Arrange
  let version = Version ::from_str( "1.1.0" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "2.0.0", &new_version.to_string() );
}

#[ test ]
fn major_with_patches()
{
  // Arrange
  let version = Version ::from_str( "1.1.1" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "2.0.0", &new_version.to_string() );
}
