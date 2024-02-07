use crate::TheModule::endpoint::list::Io2FmtWrite;

#[ test ]
fn io2fmt_write()
{

  // Arrange
  fn accepts_io_write< W : std::io::Write >( mut w : W ) -> std::io::Result< () >
  {
    w.write( b"Hello, world!" )?;

    Ok( () )
  }

  let mut string = String::new();

  // Act
  accepts_io_write( Io2FmtWrite { f : &mut string } ).unwrap();

  // Assert
  assert_eq!( "Hello, world!", &string );
}
