#[ allow( unused_imports ) ]
use super :: *;
use super ::the_module ::string ::parse_request as parse;
use std ::collections ::HashMap;

//

#[ test ]
fn op_type_from_into()
{
  let got = parse ::OpType ::from( 1 );
  let exp = parse ::OpType ::Primitive( 1 );
  assert_eq!( got, exp );

  let got: parse ::OpType< i32 > = parse ::OpType ::from( vec![ 1, 2 ] );
  let exp = parse ::OpType ::Vector( vec![ 1, 2 ] );
  assert_eq!( got, exp );

  /* */

  let op = parse ::OpType ::from( vec![ 1, 2 ] );
  let got: Vec< isize > = op.into();
  assert_eq!( got, vec![ 1, 2 ] );

  /* */

  let op = parse ::OpType ::from( 1 );
  let got = op.primitive(); /* rrr: for Dmytro: does not work properly, find better way to convert types */
  assert_eq!( got.unwrap(), 1 );

  let op = parse ::OpType ::from( vec![ 1, 2 ] );
  let got: Vec< isize > = op.vector().unwrap();
  assert_eq!( got, vec![ 1, 2 ] );

  let op = parse ::OpType ::from( 1 );
  let got = op.vector();
  assert_eq!( got, None );

  let op: parse ::OpType< usize > = parse ::OpType ::from( vec![ 1, 2 ] );
  let got = op.primitive();
  assert_eq!( got, None );
}

//

#[ test ]
fn basic()
{
  let src = "";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let exp = parse ::Request
  {
    original : "",
    subject : String ::new(),
    subjects : vec![],
    map : HashMap ::new(),
    maps : vec![],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = " ";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let exp = parse ::Request
  {
    original : " ",
    subject : String ::new(),
    subjects : vec![],
    map : HashMap ::new(),
    maps : vec![],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "  \t ";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let exp = parse ::Request
  {
    original : "  \t ",
    subject : String ::new(),
    subjects : vec![],
    map : HashMap ::new(),
    maps : vec![],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );
}

//

#[ test ]
fn with_subject_and_map_single_command()
{
  let src = "subj";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let exp = parse ::Request
  {
    original : "subj",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : HashMap ::new(),
    maps : vec![ HashMap ::new() ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj with space";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let exp = parse ::Request
  {
    original : "subj with space",
    subject : "subj with space".to_string(),
    subjects : vec![ "subj with space".to_string() ],
    map : HashMap ::new(),
    maps : vec![ HashMap ::new() ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj v: 1";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Primitive( String ::from( "1" ) ) ) ] );
  let exp = parse ::Request
  {
    original : "subj v: 1",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj v: 1 r: some";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let m = HashMap ::from(
  [
    ( String ::from( "v" ), parse ::OpType ::Primitive( String ::from( "1" ) ) ),
    ( String ::from( "r" ), parse ::OpType ::Primitive( String ::from( "some" ) ) ),
  ] );
  let exp = parse ::Request
  {
    original : "subj v: 1 r: some",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );
}

//

#[ test ]
fn with_subject_and_map_multi_command()
{
  let src = "subj1 ; subj2";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let exp = parse ::Request
  {
    original : "subj1 ; subj2",
    subject : "subj1".to_string(),
    subjects : vec![ "subj1".to_string(), "subj2".to_string() ],
    map : HashMap ::new(),
    maps : vec![ HashMap ::new(), HashMap ::new() ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj1 v: 1 ; subj2";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Primitive( String ::from( "1" ) ) ) ] );
  let exp = parse ::Request
  {
    original : "subj1 v: 1 ; subj2",
    subject : "subj1".to_string(),
    subjects : vec![ "subj1".to_string(), "subj2".to_string() ],
    map : m.clone(),
    maps : vec![ m, HashMap ::new() ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj1 v: 1 ; subj2 v: 2";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let m1 = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Primitive( String ::from( "1" ) ) ) ] );
  let m2 = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Primitive( String ::from( "2" ) ) ) ] );
  let exp = parse ::Request
  {
    original : "subj1 v: 1 ; subj2 v: 2",
    subject : "subj1".to_string(),
    subjects : vec![ "subj1".to_string(), "subj2".to_string() ],
    map : m1.clone(),
    maps : vec![ m1, m2 ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj1 v: 1 ne: -2 ; subj2 v: 2 r: some";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  let req = options.parse();
  let m1 = HashMap ::from(
  [
    ( String ::from( "v" ), parse ::OpType ::Primitive( String ::from( "1" ) ) ),
    ( String ::from( "ne" ), parse ::OpType ::Primitive( String ::from( "-2" ) ) ),
  ] );
  let m2 = HashMap ::from(
  [
    ( String ::from( "v" ), parse ::OpType ::Primitive( String ::from( "2" ) ) ),
    ( String ::from( "r" ), parse ::OpType ::Primitive( String ::from( "some" ) ) ),
  ] );
  let exp = parse ::Request
  {
    original : "subj1 v: 1 ne: -2 ; subj2 v: 2 r: some",
    subject : "subj1".to_string(),
    subjects : vec![ "subj1".to_string(), "subj2".to_string() ],
    map : m1.clone(),
    maps : vec![ m1, m2 ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );
}

//

#[ test ]
fn with_several_values()
{
  let src = "subj v: 1 v: 2";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  options.several_values = the_module ::string ::parse_request ::private ::ParseSeveralValues( false );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Primitive( "2".to_string() ) ) ] );
  let exp = parse ::Request
  {
    original : "subj v: 1 v: 2",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj v: 1 v: 2";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  options.several_values = the_module ::string ::parse_request ::private ::ParseSeveralValues( true );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ] );
  let exp = parse ::Request
  {
    original : "subj v: 1 v: 2",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );
}

//

#[ test ]
fn with_parsing_arrays()
{
  let src = "subj v: [1,2]";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  options.parsing_arrays = the_module ::string ::parse_request ::private ::ParseParsingArrays( false );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Primitive( "[1,2]".to_string() ) ) ] );
  let exp = parse ::Request
  {
    original : "subj v: [1,2]",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj v: [1,2]";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  options.parsing_arrays = the_module ::string ::parse_request ::private ::ParseParsingArrays( true );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Vector( vec![ "1".to_string(), "2".to_string() ] ) ) ] );
  let exp = parse ::Request
  {
    original : "subj v: [1,2]",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  /* */

  let src = "subj v: [1,2] v: 3";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  options.parsing_arrays = the_module ::string ::parse_request ::private ::ParseParsingArrays( true );
  options.several_values = the_module ::string ::parse_request ::private ::ParseSeveralValues( true );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string() ] ) ) ] );
  let exp = parse ::Request
  {
    original : "subj v: [1,2] v: 3",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj v: 3 v: [1,2]";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  options.parsing_arrays = the_module ::string ::parse_request ::private ::ParseParsingArrays( true );
  options.several_values = the_module ::string ::parse_request ::private ::ParseSeveralValues( true );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Vector( vec![ "3".to_string(), "1".to_string(), "2".to_string() ] ) ) ] );
  let exp = parse ::Request
  {
    original : "subj v: 3 v: [1,2]",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );

  let src = "subj v: [1,2] v: [3,4]";
  let mut options = the_module ::string ::request_parse();
  options.src = the_module ::string ::parse_request ::private ::ParseSrc( src );
  options.parsing_arrays = the_module ::string ::parse_request ::private ::ParseParsingArrays( true );
  options.several_values = the_module ::string ::parse_request ::private ::ParseSeveralValues( true );
  let req = options.parse();
  let m = HashMap ::from( [ ( String ::from( "v" ), parse ::OpType ::Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string() ] ) ) ] );
  let exp = parse ::Request
  {
    original : "subj v: [1,2] v: [3,4]",
    subject : "subj".to_string(),
    subjects : vec![ "subj".to_string() ],
    map : m.clone(),
    maps : vec![ m ],
    key_val_delimiter : ": ",
    commands_delimiter : ";",
  };
  assert_eq!( req, exp );
}
