//! Manual version of the minimal test case to isolate the E0223 error
//! This implements what the macro should generate

use super::*;

#[ derive( Default, Debug, PartialEq ) ]
pub struct MinimalStructManual {
  vec_1: Vec<String>,
}

// Manual implementation of what the Former macro should generate
#[ derive( Default ) ]
pub struct MinimalStructManualFormerStorage {
  pub vec_1: Option<Vec<String>>,
}


impl former::Storage for MinimalStructManualFormerStorage {
  type Preformed = MinimalStructManual;
}

impl former::StoragePreform for MinimalStructManualFormerStorage {
  fn preform(mut self) -> Self::Preformed {
    let vec_1 = if self.vec_1.is_some() { 
      self.vec_1.take().unwrap() 
    } else { 
      Vec::new() // Default value
    };
    MinimalStructManual { vec_1 }
  }
}

#[ derive( Debug ) ]
pub struct MinimalStructManualFormerDefinitionTypes<__Context = (), __Formed = MinimalStructManual> {
  _phantom: core::marker::PhantomData<(*const __Context, *const __Formed)>,
}

impl<__Context, __Formed> Default for MinimalStructManualFormerDefinitionTypes<__Context, __Formed> {
  fn default() -> Self {
    Self { _phantom: core::marker::PhantomData }
  }
}

impl<__Context, __Formed> former::FormerDefinitionTypes for MinimalStructManualFormerDefinitionTypes<__Context, __Formed> {
  type Storage = MinimalStructManualFormerStorage;
  type Formed = __Formed;
  type Context = __Context;
}

#[ derive( Debug ) ]
pub struct MinimalStructManualFormerDefinition<
  __Context = (),
  __Formed = MinimalStructManual,
  __End = former::ReturnPreformed,
> {
  _phantom: core::marker::PhantomData<(*const __Context, *const __Formed, *const __End)>,
}

impl<__Context, __Formed, __End> Default for MinimalStructManualFormerDefinition<__Context, __Formed, __End> {
  fn default() -> Self {
    Self { _phantom: core::marker::PhantomData }
  }
}

impl<__Context, __Formed, __End> former::FormerDefinition for MinimalStructManualFormerDefinition<__Context, __Formed, __End>
where 
  __End: former::FormingEnd<MinimalStructManualFormerDefinitionTypes<__Context, __Formed>>
{
  type Types = MinimalStructManualFormerDefinitionTypes<__Context, __Formed>;
  type End = __End;
  type Storage = MinimalStructManualFormerStorage;
  type Formed = __Formed;
  type Context = __Context;
}

pub struct MinimalStructManualFormer<Definition = MinimalStructManualFormerDefinition<(), MinimalStructManual, former::ReturnPreformed>>
where
  Definition: former::FormerDefinition<Storage = MinimalStructManualFormerStorage>,
  Definition::Types: former::FormerDefinitionTypes<Storage = MinimalStructManualFormerStorage>
{
  pub storage: Definition::Storage,
  pub context: Option<Definition::Context>,
  pub on_end: Option<Definition::End>,
}

impl<Definition> MinimalStructManualFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = MinimalStructManualFormerStorage>,
  Definition::Types: former::FormerDefinitionTypes<Storage = MinimalStructManualFormerStorage>
{
  pub fn new(on_end: Definition::End) -> Self {
    Self {
      storage: Default::default(),
      context: None,
      on_end: Some(on_end),
    }
  }

  pub fn form(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    let on_end = self.on_end.take().unwrap();
    let mut context = self.context.take();
    <Definition::Types as former::FormerMutator>::form_mutation(&mut self.storage, &mut context);
    former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
  }

  // Collection setter for vec_1 field
  pub fn vec_1(self) -> former::CollectionFormer<
    <Vec<String> as former::Collection>::Entry,
    former::VectorDefinition<String, Self, Self, MinimalStructManualSubformCollectionVec1End<Definition>>,
  >
  where
    former::VectorDefinition<String, Self, Self, MinimalStructManualSubformCollectionVec1End<Definition>>: 
      former::FormerDefinition<
        Storage = Vec<String>,
        Context = Self,
        End = MinimalStructManualSubformCollectionVec1End<Definition>,
      >
  {
    self._vec_1_subform_collection::<former::CollectionFormer<_, _>>()
  }

  pub fn _vec_1_subform_collection<'a, Former2>(self) -> Former2
  where
    Former2: former::FormerBegin<
      'a,
      former::VectorDefinition<String, Self, Self, MinimalStructManualSubformCollectionVec1End<Definition>>,
    >,
    former::VectorDefinition<String, Self, Self, MinimalStructManualSubformCollectionVec1End<Definition>>:
      former::FormerDefinition<
        Storage = Vec<String>,
        Context = Self,
        End = MinimalStructManualSubformCollectionVec1End<Definition>,
      >,
    <former::VectorDefinition<String, Self, Self, MinimalStructManualSubformCollectionVec1End<Definition>> as former::FormerDefinition>::Storage: 'a,
    <former::VectorDefinition<String, Self, Self, MinimalStructManualSubformCollectionVec1End<Definition>> as former::FormerDefinition>::Context: 'a,
    <former::VectorDefinition<String, Self, Self, MinimalStructManualSubformCollectionVec1End<Definition>> as former::FormerDefinition>::End: 'a,
    Definition: 'a,
  {
    Former2::former_begin(
      None,
      Some(self),
      MinimalStructManualSubformCollectionVec1End::<Definition>::default(),
    )
  }
}

// End callback for vec_1 subform collection
pub struct MinimalStructManualSubformCollectionVec1End<Definition> {
  _phantom: core::marker::PhantomData<(Definition,)>,
}

impl<Definition> Default for MinimalStructManualSubformCollectionVec1End<Definition> {
  fn default() -> Self {
    Self { _phantom: core::marker::PhantomData }
  }
}

impl<Definition> former::FormingEnd<former::VectorDefinitionTypes<String, MinimalStructManualFormer<Definition>, MinimalStructManualFormer<Definition>>> 
  for MinimalStructManualSubformCollectionVec1End<Definition>
where
  Definition: former::FormerDefinition<Storage = MinimalStructManualFormerStorage>,
  Definition::Types: former::FormerDefinitionTypes<Storage = MinimalStructManualFormerStorage>
{
  fn call(
    &self,
    storage: Vec<String>,
    super_former: Option<MinimalStructManualFormer<Definition>>,
  ) -> MinimalStructManualFormer<Definition> {
    let mut super_former = super_former.unwrap();
    if let Some(ref mut field) = super_former.storage.vec_1 {
      former::CollectionAssign::assign(field, storage);
    } else {
      super_former.storage.vec_1 = Some(storage);
    }
    super_former
  }
}

impl<__Context, __Formed> former::FormerMutator for MinimalStructManualFormerDefinitionTypes<__Context, __Formed> {}

impl MinimalStructManual {
  pub fn former() -> MinimalStructManualFormer<MinimalStructManualFormerDefinition<(), MinimalStructManual, former::ReturnPreformed>> {
    MinimalStructManualFormer::new(former::ReturnPreformed)
  }
}

#[ test ]
fn manual_test() {
  let _instance = MinimalStructManual::former()
    .vec_1()
    .add("test".to_string())
    .end()
    .form();
}