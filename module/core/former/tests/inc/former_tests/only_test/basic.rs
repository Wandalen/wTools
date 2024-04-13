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
    let former2 = Struct1Former::< Struct1FormerDefinition >::new( former::ReturnPreformed );
    a_id!( std::mem::size_of_val( &former ), std::mem::size_of_val( &former2 ) );

    let command = Struct1::former().form();
    a_id!( command.int_1, 0 );

    let command = Struct1::former().perform();
    a_id!( command.int_1, 0 );

    let command = Struct1::former().end();
    a_id!( command.int_1, 0 );

  }

  //

  fn custom_definition_params()
  {
    // zzz : make example of that

    // custom params
    let got = Struct1Former
    ::< Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > > >
    ::begin
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
    ::< Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > > >
    ::begin
    (
      None,
      Some( 3 ),
      | storage : Struct1FormerStorage, context : Option< i32 > | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) },
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

    // custom params begin
    let got = Struct1Former
    ::< Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > > >
    ::begin
    (
      None,
      Some( 3 ),
      | storage : Struct1FormerStorage, context : Option< i32 > | { 2 * ( storage.int_1.unwrap() + context.unwrap() ) }
    )
    .int_1( 13 )
    .form();
    a_id!( got, 32 );

    // custom params begin with Struct1FormerWithClosure
    let got = Struct1Former
    ::< Struct1FormerWithClosure< i32, i32 > >
    ::begin
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

  fn begin()
  {

    // begin with none
    let got = Struct1Former::< Struct1FormerDefinition >::begin( None, None, the_module::ReturnPreformed ).int_1( 13 ).form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // begin with storage
    let mut storage = Struct1FormerStorage::default();
    storage.int_1 = Some( 13 );
    let exp = Struct1Former::< Struct1FormerDefinition >::begin( Some( storage ), None, the_module::ReturnPreformed ).form();
    a_id!( got, exp );

    // begin with context
    let mut storage = Struct1FormerStorage::default();
    storage.int_1 = Some( 13 );
    let exp = Struct1Former::< Struct1FormerDefinition >
    ::begin( Some( storage ), Some( () ), the_module::ReturnPreformed )
    .form();
    a_id!( got, exp );

  }

  //

  fn _begin_precise()
  {

    // custom params
    let got = Struct1Former
    ::< Struct1FormerDefinition< i32, i32, _ > >
    ::_begin_precise
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
    ::< Struct1FormerDefinition< i32, i32, former::FormingEndClosure< Struct1FormerDefinitionTypes< i32, i32 > > > >
    ::_begin_precise
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

  fn new()
  {

    // basic case
    let former = Struct1::former();
    let former2 = Struct1Former::< Struct1FormerDefinition >::_new_precise( former::ReturnPreformed );
    a_id!( std::mem::size_of_val( &former ), std::mem::size_of_val( &former2 ) );
    let exp = former.form();
    let got = former2.form();
    a_id!( got, exp );

    // default explicit params
    let got = Struct1Former
    ::< Struct1FormerDefinition< (), Struct1, former::ReturnPreformed > >
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
    ::< Struct1FormerDefinition< (), Struct1, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > > >
    ::new( end_wrapper )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure
    let got = Struct1Former
    ::< Struct1FormerDefinition< (), Struct1, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > > >
    ::new( former::FormingEndClosure::new( | storage, _context | { former::StoragePreform::preform( storage ) } ) )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure, auto types
    let got = Struct1Former
    ::< Struct1FormerDefinition< _, _, former::FormingEndClosure< Struct1FormerDefinitionTypes< (), Struct1 > > > >
    ::new( former::FormingEndClosure::new( | storage, _context : Option< () > | { former::StoragePreform::preform( storage ) } ) )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure
    let got = Struct1Former
    ::< Struct1FormerWithClosure< (), Struct1 > >
    ::new( former::FormingEndClosure::new( | storage, _context | { former::StoragePreform::preform( storage ) } ) )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure
    let got = Struct1Former
    ::< Struct1FormerWithClosure< (), Struct1 > >
    ::new( | storage, _context | { former::StoragePreform::preform( storage ) } )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

  }

  //

  fn _new_precise()
  {

    // basic case
    let former = Struct1::former();
    let former2 = Struct1Former::< Struct1FormerDefinition >::_new_precise( former::ReturnPreformed );
    a_id!( std::mem::size_of_val( &former ), std::mem::size_of_val( &former2 ) );
    let exp = former.form();
    let got = former2.form();
    a_id!( got, exp );

    // default explicit params
    let got = Struct1Former
    ::< Struct1FormerDefinition< (), Struct1, _ > >
    ::_new_precise( former::ReturnPreformed )
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
    ::< Struct1FormerDefinition< (), Struct1, _ > >
    ::_new_precise( end_wrapper )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure
    let got = Struct1Former
    ::< Struct1FormerDefinition< (), Struct1, _ > >
    ::_new_precise( former::FormingEndClosure::new( | storage, _context | { former::StoragePreform::preform( storage ) } ) )
    .int_1( 13 )
    .form();
    let exp = Struct1::former().int_1( 13 ).form();
    a_id!( got, exp );

    // default explicit params with wrapper and closure, auto types
    let got = Struct1Former
    ::< Struct1FormerDefinition< _, _, _ > >
    ::_new_precise( former::FormingEndClosure::new( | storage, _context : Option< () > | { former::StoragePreform::preform( storage ) } ) )
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
    let _default = Struct1FormerDefinition::< () >::default();
    // let _default = Struct1FormerDefinition::default(); // why does not work?

    // definition types exists and has Formed
    let got = < Struct1FormerDefinitionTypes< (), Struct1 > as the_module::FormerDefinitionTypes >::Formed::former().form();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // definition types exists and has Formed
    let got = < Struct1FormerDefinitionTypes as the_module::FormerDefinitionTypes >::Formed::former().form();
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // definition types exists and has Storage
    use former::StoragePreform;
    let got = < Struct1FormerDefinitionTypes as the_module::FormerDefinitionTypes >::Storage::preform( Struct1::former().storage );
    let exp = Struct1::former().form();
    a_id!( got, exp );

    // definition exists and has Storage
    let got = < < Struct1FormerDefinition as the_module::FormerDefinition >::Types as the_module::FormerDefinitionTypes >::Formed::former().form();
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
    let got = < Struct1FormerStorage as the_module::Storage >::Formed::former().form();
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
  custom_definition_params,
  begin,
  _begin_precise,
  new,
  _new_precise,
  preform,
  definition,
  storage,
  test_int,
  test_underscored_form,
}