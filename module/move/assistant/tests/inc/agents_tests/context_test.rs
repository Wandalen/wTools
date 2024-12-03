use super::*;

use the_module::agents::
{
  path::Path,
  context::
  {
    ContextDir,
    ContextEntry,
  },
};

#[ test ]
fn context_dir_add_terminal()
{
  let mut ctx : ContextDir< () > = ContextDir::new();
  let entry = ContextEntry::Terminal( () );
  let name = "test";
  
  let res = ctx.add( name, entry.clone() );

  assert!( res );
  assert_eq!( ctx.get( name ), Some( &entry ) );
}

#[ test ]
fn context_dir_add_dir()
{
  let mut ctx : ContextDir< () > = ContextDir::new();
  let entry : ContextEntry< () > = ContextDir::new().into();
  let name = "test";
  
  let res = ctx.add( name, entry.clone() );

  assert!( res );
  assert_eq!( ctx.get( name ), Some( &entry ) );
}

#[ test ]
fn context_dir_add_duplicate()
{
  let name = "test";
  let orig_entry = ContextEntry::Terminal( 1 );

  let mut ctx : ContextDir< usize > = ContextDir::new();
  ctx.add( name, orig_entry.clone() );

  let res = ctx.add( name, ContextEntry::Terminal( 2 ) );

  assert!( !res );
  assert_eq!( ctx.get( name ), Some( &orig_entry ) );
}

#[ test ]
fn context_dir_get()
{
  let mut ctx : ContextDir< usize > = ContextDir::new();
  ctx.add( "test_1", ContextEntry::Terminal( 1 ) );
  ctx.add( "test_2", ContextEntry::Terminal( 2 ) );
  ctx.add( "test_3", ContextEntry::Terminal( 3 ) );

  assert_eq!( ctx.get( "test_1" ), Some( &ContextEntry::Terminal( 1 ) ) );
  assert_eq!( ctx.get( "test_2" ), Some( &ContextEntry::Terminal( 2 ) ) );
  assert_eq!( ctx.get( "test_3" ), Some( &ContextEntry::Terminal( 3 ) ) );
}

#[ test ]
fn context_dir_get_non_existing()
{
  let ctx : ContextDir< () > = ContextDir::new();

  let res = ctx.get( "test" );

  assert!( res.is_none() );
}

#[ test ]
fn context_dir_get_by_path_relative()
{
  let value_1 = ContextEntry::Terminal( 1 );
  let value_2 = ContextEntry::Terminal( 2 );
  let value_3 = ContextEntry::Terminal( 3 );

  let mut dir_1 : ContextDir< usize > = ContextDir::new();
  dir_1.add( "value_1", value_1.clone() );
  dir_1.add( "value_2", value_2.clone() );

  let mut dir_3 : ContextDir< usize > = ContextDir::new();
  dir_3.add( "value_3", value_3.clone() );

  let mut dir_2 : ContextDir< usize > = ContextDir::new();
  dir_2.add( "dir_3", dir_3.into() );

  let mut ctx : ContextDir< usize > = ContextDir::new();
  ctx.add( "dir_1", dir_1.into() );
  ctx.add( "dir_2", dir_2.into() );

  let got_value_1 = ctx.get_by_path( &Path::try_from( "dir_1::value_1" ).unwrap() );
  let got_value_2 = ctx.get_by_path( &Path::try_from( "dir_1::value_2" ).unwrap() );
  let got_value_3 = ctx.get_by_path( &Path::try_from( "dir_2::dir_3::value_3" ).unwrap() );

  assert_eq!( got_value_1, Some( &value_1 ) );
  assert_eq!( got_value_2, Some( &value_2 ) );
  assert_eq!( got_value_3, Some( &value_3 ) );
}

#[ test ]
fn context_dir_get_by_path_absolute()
{
  let entry = ContextEntry::Terminal( () );
  let mut ctx : ContextDir< () > = ContextDir::new();
  ctx.add( "test", entry.clone() );

  let res = ctx.get_by_path( &&Path::try_from( "::test" ).unwrap() );

  assert!( res.is_some() );
  assert_eq!( res.unwrap(), &entry );
}

#[ test ]
fn context_dir_get_by_path_non_existing()
{
  let ctx : ContextDir< () > = ContextDir::new();

  let res = ctx.get_by_path( &Path::try_from( "test" ).unwrap() );

  assert!( res.is_none() );
}