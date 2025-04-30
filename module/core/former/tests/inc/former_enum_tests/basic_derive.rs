
use super::*;

// Define the inner structs
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition : bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command : String }

// Derive Former on the simplified enum - This should generate static methods
#[ derive( Debug, Clone, PartialEq ) ]
// #[ derive( Debug, Clone, PartialEq, former::Former ) ]
// #[ debug ]
enum FunctionStep
{
  Break( Break ),
  Run( Run ),
}

// xxx : generated code for debugging

//
//   #[automatically_derived] impl < > FunctionStep < > where
//   {
//       #[inline(always)] fn r#break() -> BreakFormer < BreakFormerDefinition < ()
//       FunctionStep < > , FunctionStepBreakEnd < > > >
//       {
//           BreakFormer ::
//           begin(None, None, FunctionStepBreakEnd :: < > :: default())
//       } #[inline(always)] fn run() -> RunFormer < RunFormerDefinition < ()
//       FunctionStep < > , FunctionStepRunEnd < > > >
//       { RunFormer :: begin(None, None, FunctionStepRunEnd :: < > :: default()) }
//   } #[derive(Default, Debug)] struct FunctionStepBreakEnd < > where
//   { _phantom : :: core :: marker :: PhantomData < () > , }
//   #[automatically_derived] impl < > former :: FormingEnd <
//   BreakFormerDefinitionTypes < () FunctionStep < > > > for FunctionStepBreakEnd
//   < > where
//   {
//       #[inline(always)] fn
//       call(& self, sub_storage : BreakFormerStorage < > , _context : Option < ()
//       >) -> FunctionStep < >
//       {
//           let data = former :: StoragePreform :: preform(sub_storage);
//           FunctionStep :: Break(data)
//       }
//   } #[derive(Default, Debug)] struct FunctionStepRunEnd < > where
//   { _phantom : :: core :: marker :: PhantomData < () > , }
//   #[automatically_derived] impl < > former :: FormingEnd <
//   RunFormerDefinitionTypes < () FunctionStep < > > > for FunctionStepRunEnd < >
//   where
//   {
//       #[inline(always)] fn
//       call(& self, sub_storage : RunFormerStorage < > , _context : Option < ()
//       >) -> FunctionStep < >
//       {
//           let data = former :: StoragePreform :: preform(sub_storage);
//           FunctionStep :: Run(data)
//       }
//   }


// xxx : generated code for debugging

// Include the test logic
include!( "basic_only_test.rs" );
// qqq : xxx : uncomment and make it working
