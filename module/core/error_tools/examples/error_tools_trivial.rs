//! qqq : write proper description
fn main()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  {
    let err = f1();
    println!( "{err:#?}" );
    // < Err(
    // <    BasicError {
    // <        msg: "Some error",
    // <    },
    // < )
  }
}

#[ cfg( not( feature = "no_std" ) ) ]
fn f1() -> error_tools::Result< () >
{
  let _read = std::fs::read_to_string( "Cargo.toml" )?;
  Err( error_tools::BasicError::new( "Some error" ).into() )
}
