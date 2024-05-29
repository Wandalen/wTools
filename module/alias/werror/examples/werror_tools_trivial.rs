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
fn f1() -> werror::Result< () >
{
  let _read = std::fs::read_to_string( "Cargo.toml" )?;
  Err( werror::BasicError::new( "Some error" ).into() )
}
