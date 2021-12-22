pub use werror::*;

pub struct Instruction
{
  pub err : Option< Error >
}

impl Instruction
{
  fn new() -> Self
  {
    Self
    {
      err : None,
    }
  }
}

//

pub trait InstructionParseParamsAdapter
{
}

pub struct InstructionParseParams
{
}

impl InstructionParseParams
{
  pub fn new() -> Self
  {
    Self
    {
    }
  }
}

impl InstructionParseParamsAdapter for InstructionParseParams
{
}

//

pub fn instruction_parse_from_args< Params : InstructionParseParamsAdapter, I : core::iter::Iterator >( _params : &Params, args : I ) -> Instruction
where
  < I as Iterator >::Item: std::fmt::Debug
{
  let mut result = Instruction::new();

  args.for_each( | arg | println!( "{:?}", arg ) );

  result.err = Some( err!( "Does not start as command {}", "xxx" ) );

  result
}
