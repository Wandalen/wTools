#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  //

  fn internals()
  {

    let former = Struct1::former();
    a_id!( former.storage.int_1, None );
    a_id!( former.context, None );
    a_id!( print!( "{:?}", former.on_end ), print!( "{:?}", Some( the_module::ReturnPreformed ) ) );
    let former2 = Struct1Former::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >::new_coercing( former::ReturnPreformed );
    a_id!( ::std::mem::size_of_val( &former ), ::std::mem::size_of_val( &former2 ) );
    let former2 = Struct1Former::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >::new( former::ReturnPreformed );
    a_id!( ::std::mem::size_of_val( &former ), ::std::mem::size_of_val( &former2 ) );

    let command = Struct1::former().form();
    a_id!( command.int_1, 0 );

    let command = Struct1::former().perform();
    a_id!( command.int_1, 0 );

    let command = Struct1::former().end();
    a_id!( command.int_1, 0 );

  }

  //

  fn entity_to()
  {

    let got = < Struct1 as former::EntityToFormer< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > > >::Former::new( former::ReturnPreformed )
    .int_1( 13 )
    .form();
    let exp = Struct1 { int_1 : 13 };
    a_id!( got, exp );

    let got = < Struct1 as former::EntityToStorage >::Storage::default();
    let exp =
    <
      Struct1 as former::EntityToFormer
      <
        Struct1FormerDefinition< (), Struct1, former::ReturnPreformed >
      >
    >::Former::new( former::ReturnPreformed );
    a_id!( got.int_1, exp.storage.int_1 );

    let got = < Struct1 as former::EntityToStorage >::Storage::default();
    let exp =
    <
      Struct1 as former::EntityToFormer
      <
        < Struct1 as former::EntityToDefinition< (), Struct1, former::ReturnPreformed > >::Definition
      >
    >::Former::new( former::ReturnPreformed );
    a_id!( got.int_1, exp.storage.int_1 );

  }

  //

  fn former_begin()
  {

    let former = < Struct1Former as former::FormerBegin< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > > >
    ::former_begin( None, None, former::ReturnPreformed );
    let got = former
    .int_1( 13 )
    .form();
    let exp = Struct1 { int_1 : 13 };
    a_id!( got, exp );

  }

  //

  fn custom_definition_params()
  {

    // custom params
    let got = Struct1Former
    ::
    <
      Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > >
    >
    ::begin_coercing
    (
      None,
      Some( 3 ),
      | storage : Struct1FormerStorage, context : Option< i32 > | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) },
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

    // custom params with into
    let got = Struct1Former
    ::
    <
      Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > >
    >
    ::begin_coercing
    (
      None,
      Some( 3 ),
      | storage : Struct1FormerStorage, context : Option< i32 > | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) },
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

    // custom params begin_coercing
    let got = Struct1Former
    ::
    <

      Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > >
    >
    ::begin_coercing
    (
      None,
      Some( 3 ),
      | storage : Struct1FormerStorage, context : Option< i32 > | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) }
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

    // custom params begin_coercing with Struct1FormerWithClosure
    let got = Struct1Former
    ::
    <

      Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > >
    >
    ::begin_coercing
    (
      None,
      Some( 3 ),
      | storage : Struct1FormerStorage, context : Option< i32 > | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) }
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

    // less explicit
    let got = Struct1Former
    ::
    <

      Struct1FormerDefinition< i32, i32, former::FormingEndClosure< _ > >
    >
    ::begin_coercing
    (
      None,
      Some( 3 ),
      | storage : Struct1FormerStorage, context : Option< i32 > | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) }
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

  }

  //

  fn begin_coercing()
  {

    // begin_coercing with none
    let got = Struct1Former::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >::begin_coercing( None, None, the_module::ReturnPreformed ).int_1( 13 ).form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // begin_coercing with storage
    let mut storage = Struct1FormerStorage::default();
    storage.int_1 = Some( 13 );
    let exp = Struct1Former::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >::begin_coercing( Some( storage ), None, the_module::ReturnPreformed ).form();
    a_id!( got, exp );

    // begin_coercing with context
    let mut storage = Struct1FormerStorage::default();
    storage.int_1 = Some( 13 );
    let exp = Struct1Former
    ::
    <
      Struct1FormerDefinition< (), Struct1, former::ReturnPreformed >
    >
    ::begin_coercing( Some( storage ), Some( () ), the_module::ReturnPreformed )
    .form();
    a_id!( got, exp );

  }

  //

  fn begin()
  {

    // custom params
    let got = Struct1Former
    // ::< Struct1FormerDefinition< i32, i32, _ > >
    ::
    <

      Struct1FormerDefinition< i32, i32, _ >
    >
    ::begin
    (
      None,
      Some( 3 ),
      former::FormingEndClosure::new
      (
        | storage : Struct1FormerStorage, context | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) }
      ),
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

    // custom params with into
    let got = Struct1Former
    // ::< Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > > >
    ::
    <

      Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > >
    >
    ::begin
    (
      None,
      Some( 3 ),
      (
        | storage : Struct1FormerStorage, context : Option< i32 > | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) }
      ).into(),
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

  }

  //

  fn new_coercing()
  {

    // basic case
    let former = Struct1::former();
    let former2 = Struct1Former::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >::new( former::ReturnPreformed );
    a_id!( ::std::mem::size_of_val( &former ), ::std::mem::size_of_val( &former2 ) );
    let exp = former.form();
    let got = former2.form();
    a_id!( got, exp );

    // default explicit params
    let got = Struct1Former
    ::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >
    ::new_coercing( former::ReturnPreformed )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper
    fn f1( storage : Struct1FormerStorage, _context : Option< () > ) -> Struct1
    {
      former::StoragePreform::preform( storage )
    }
    let end_wrapper : former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > = former::FormingEndClosure::new( f1 );
    let got = Struct1Former
    // ::< Struct1FormerDefinition< (), Struct1, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > > >
    ::
    <
      Struct1FormerDefinition< (), Struct1, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > >
    >
    ::new_coercing( end_wrapper )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure
    let got = Struct1Former
    // ::< Struct1FormerDefinition< (), Struct1, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > > >
    ::
    <

      Struct1FormerDefinition< (), Struct1, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > >
    >
    ::new_coercing( former::FormingEndClosure::new( | storage, _context | { former::StoragePreform::preform( storage ) } ) )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure, auto types
    let got = Struct1Former
    // ::< Struct1FormerDefinition< _, _, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > > >
    ::
    <
      Struct1FormerDefinition< (), Struct1, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > >
    >
    ::new_coercing( former::FormingEndClosure::new( | storage, _context : Option< () > | { former::StoragePreform::preform( storage ) } ) )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

  }

  //

  fn new()
  {

    // basic case
    let former = Struct1::former();
    let former2 = Struct1Former::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >::new( former::ReturnPreformed );
    a_id!( ::std::mem::size_of_val( &former ), ::std::mem::size_of_val( &former2 ) );
    let exp = former.form();
    let got = former2.form();
    a_id!( got, exp );

    // default explicit params
    let got = Struct1Former
    // ::< Struct1FormerDefinition< (), Struct1, _ > >
    ::
    <

      Struct1FormerDefinition< (), Struct1, _ >,
    >
    ::new( former::ReturnPreformed )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper
    fn f1( storage : Struct1FormerStorage, _context : Option< () > ) -> Struct1
    {
      former::StoragePreform::preform( storage )
    }
    let end_wrapper : former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > = former::FormingEndClosure::new( f1 );
    let got = Struct1Former
    // ::< Struct1FormerDefinition< (), Struct1, _ > >
    ::
    <
      Struct1FormerDefinition< (), Struct1, _ >,
    >
    ::new( end_wrapper )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    //

    // default explicit params with wrapper and closure
    let got = Struct1Former
    // ::< Struct1FormerWithClosure< (), Struct1 > >
    ::
    <
      Struct1FormerDefinition< (), Struct1, _ >
    >
    ::new( | storage, _context | { former::StoragePreform::preform( storage ) } )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    //

    // default explicit params with wrapper and closure
    let got = Struct1Former
    // ::< Struct1FormerDefinition< (), Struct1, _ > >
    ::
    <
      Struct1FormerDefinition< (), Struct1, _ >,
    >
    ::new( former::FormingEndClosure::new( | storage, _context | { former::StoragePreform::preform( storage ) } ) )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure, auto types
    let got = Struct1Former
    // ::< Struct1FormerDefinition< _, _, _ > >
    ::
    <
      Struct1FormerDefinition< _, _, _ >,
    >
    ::new( former::FormingEndClosure::new( | storage, _context : Option< () > | { former::StoragePreform::preform( storage ) } ) )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

  }

  //

  fn preform()
  {

    // formation should have method preform
    let got = Struct1::former().preform();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // storage should have method preform
    let got = the_module::StoragePreform::preform( Struct1::former().storage );
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // storage should have method preform
    use the_module::StoragePreform;
    let got = Struct1::former().storage.preform();
    let exp = Struct1::former().form();
    a_id!( got, exp );

  }

  //

  fn definition()
  {

    // default is implemented for definition
    let _default = Struct1FormerDefinition::< (), Struct1, former::ReturnPreformed >::default();
    // let _default = Struct1FormerDefinition::default(); // why does not work?

    // definition types exists and has Formed
    let got = < Struct1FormerDefinitionTypes< (), Struct1 > as the_module::FormerDefinitionTypes >::Formed::former().form();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // definition types exists and has Formed
    let got = < Struct1FormerDefinitionTypes< (), Struct1 > as the_module::FormerDefinitionTypes >::Formed::former().form();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // definition types exists and has Storage
    use former::StoragePreform;
    let got = < Struct1FormerDefinitionTypes< (), Struct1 > as the_module::FormerDefinitionTypes >::Storage
    ::preform( Struct1::former().storage );
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // definition exists and has Storage
    let got = < < Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > as the_module::FormerDefinition >::Types as the_module::FormerDefinitionTypes >::Formed
    ::former().form();
    let exp = Struct1::former().form();
    a_id!( got, exp );

  }

  //

  fn storage()
  {

    // definition exists and has Storage
    let got = < Struct1FormerStorage as the_module::StoragePreform >::preform( Struct1::former().storage );
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // default is implemented for Storage
    let got = Struct1FormerStorage::default().preform();
    let exp = Struct1::former().storage.preform();
    a_id!( got, exp );

    // definition exists and has Storage
    use former::StoragePreform;
    let got = Struct1::former().storage.preform();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // storage exists
    let got = < Struct1FormerStorage as the_module::Storage >::Preformed::former().form();
    let exp = Struct1::former().form();
    a_id!( got, exp );

  }

  //

  fn test_int()
  {

    // test.case( "basic" );

    let command = Struct1::former()
    .int_1( 13 )
    .form();
    // dbg!( &command );

    let expected = Struct1
    {
      int_1 : 13,
    };
    a_id!( command, expected );

    // test.case( "rewriting" );

    // should_throw( ||
    // {
    //   let _command = Struct1::former()
    //   .int_1( 1 )
    //   .int_1( 3 )
    //   .form();
    //   Ok( () )
    // })?;
  }

  //

  fn test_underscored_form()
  {
    // test.case( "basic" );
    let command = Struct1::former()
    .int_1( 13 )
    .form();

    let expected = Struct1
    {
      int_1 : 13,
    };
    a_id!( command, expected );
  }

  //


}

//

tests_index!
{
  internals,
  entity_to,
  former_begin,
  custom_definition_params,
  begin_coercing,
  begin,
  new_coercing,
  new,
  preform,
  definition,
  storage,
  test_int,
  test_underscored_form,
}
