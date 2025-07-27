// test_kind: bug_reproducer(E0726)
use super::*;
use former_types::
{
  Storage,
  StoragePreform,
  FormerDefinitionTypes,
  FormerMutator,
  ReturnPreformed,
  FormerDefinition,
  FormingEnd,
  FormerBegin,
};

// A simple struct with a lifetime.
#[derive(Debug, PartialEq)]
pub struct Sample<'a> { field: &'a str }

// Manually define the Storage, Definition, and Former for the struct.
pub struct SampleFormerStorage<'a> { pub field: Option<&'a str> }
impl<'a> Default for SampleFormerStorage<'a> { fn default() -> Self { Self { field: None } } }
impl<'a> Storage for SampleFormerStorage<'a> { type Preformed = Sample<'a>; }
impl<'a> StoragePreform for SampleFormerStorage<'a> {
    fn preform(mut self) -> Self::Preformed { Sample { field: self.field.take().unwrap_or("") } }
}

pub struct SampleFormerDefinitionTypes< 'a, C = (), F = Sample< 'a > >
{ _p: core::marker::PhantomData<(&'a(), C, F)> }
impl< 'a, C, F > FormerDefinitionTypes for SampleFormerDefinitionTypes< 'a, C, F >
{
    type Storage = SampleFormerStorage<'a>;
    type Context = C;
    type Formed = F;
}
impl< 'a, C, F > FormerMutator for SampleFormerDefinitionTypes< 'a, C, F > {}

pub struct SampleFormerDefinition< 'a, C = (), F = Sample< 'a >, E = ReturnPreformed >
{ _p: core::marker::PhantomData<(&'a(), C, F, E)> }
impl< 'a, C, F, E > FormerDefinition for SampleFormerDefinition< 'a, C, F, E >
where E: FormingEnd<SampleFormerDefinitionTypes<'a, C, F>>
{
    type Storage = SampleFormerStorage<'a>;
    type Context = C;
    type Formed = F;
    type Types = SampleFormerDefinitionTypes<'a, C, F>;
    type End = E;
}

pub struct SampleFormer< 'a, D = SampleFormerDefinition< 'a > >
where D: FormerDefinition<Storage = SampleFormerStorage<'a>>
{
    storage: D::Storage,
    context: Option<D::Context>,
    on_end: Option<D::End>,
}

// This impl block is what will fail to compile.
// The `FormerBegin` trait needs a lifetime parameter to handle `Definition`
// which now carries the lifetime `'a`.
impl< 'a, D > FormerBegin< 'a, D > for SampleFormer< 'a, D >
where
  D: FormerDefinition<Storage = SampleFormerStorage<'a>>,
  D::Storage: 'a,
  D::Context: 'a,
  D::End: 'a,
{
  fn former_begin( storage: Option<D::Storage>, context: Option<D::Context>, on_end: D::End ) -> Self
  {
    Self { storage: storage.unwrap_or_default(), context, on_end: Some(on_end) }
  }
}

// Add a former impl for SampleFormer to add a setter
impl< 'a, D > SampleFormer< 'a, D >
where D: FormerDefinition<Storage = SampleFormerStorage<'a>>
{
    pub fn field(mut self, value: &'a str) -> Self
    {
        self.storage.field = Some(value);
        self
    }
    pub fn form(mut self) -> D::Formed
    {
        let on_end = self.on_end.take().unwrap();
        on_end.call(self.storage, self.context.take())
    }
}

#[test]
fn reproduces_error_and_passes_after_fix()
{
    // Now that it compiles, we can create and use the former.
    let former: SampleFormer<'_, SampleFormerDefinition<'_, (), _>> = FormerBegin::former_begin(None, None::<()>, ReturnPreformed);
    let instance = former.field("hello").form();
    assert_eq!(instance, Sample { field: "hello" });
}