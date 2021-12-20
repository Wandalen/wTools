( function _l0_l9_Date_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
}

const _ = _global_.wTools;

//--
// tests
//--

function is( test )
{
  test.case = 'undefined';
  var src = undefined;
  test.identical( _.date.is( src ), false );

  test.case = 'null';
  var src = null;
  test.identical( _.date.is( src ), false );

  test.case = 'string';
  var src = 'str';
  test.identical( _.date.is( src ), false );

  test.case = 'number';
  var src = 13;
  test.identical( _.date.is( src ), false );

  test.case = 'symbol';
  var src = Symbol.for( 'a' );
  test.identical( _.date.is( src ), false );

  test.case = 'boolean false';
  var src = true;
  test.identical( _.date.is( src ), false );

  test.case = 'boolean true';
  var src = true;
  test.identical( _.date.is( src ), false );

  test.case = 'empty array';
  var src = [];
  test.identical( _.date.is( src ), false );

  test.case = 'date empty';
  var src = new Date();
  test.identical( _.date.is( src ), true );

  test.case = 'date string';
  var src = new Date( 'December 17, 1995 03:24:00' );
  test.identical( _.date.is( src ), true );

  test.case = 'date y:m:d';
  var src = new Date( 1995, 11, 17 );
  test.identical( _.date.is( src ), true );

  test.case = 'date y:m:d time';
  var src = new Date( 1995, 11, 17, 3, 24, 0 )
  test.identical( _.date.is( src ), true );

  test.case = 'date milliseconds';
  var src = new Date( 628021800000 );
  test.identical( _.date.is( src ), true );

  test.case = 'empty arguments array';
  var src = _.argumentsArray.make();
  test.identical( _.date.is( false ), false );

  test.case = 'pure empty map';
  var src = Object.create( null );
  test.identical( _.date.is( src ), false );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  test.identical( _.date.is( src ), false );

  test.case = 'pure map with constructor';
  var src = Object.create( null );
  test.identical( _.date.is( src ), false );

  test.case = 'from pure with iterator';
  var src = Object.create( null );
  src[ Symbol.iterator ] = function(){};
  test.identical( _.date.is( src ), false );

  test.case = 'empty polluted map';
  var src = {};
  test.identical( _.date.is( src ), false );

  test.case = 'polluted map';
  var src = { a : 7, b : 13 };
  test.identical( _.date.is( src ), false );

  test.case = 'polluted map with constructor';
  var src = {};
  test.identical( _.date.is( src ), false );

  test.case = 'from polluted with iterator';
  var src = {};
  src[ Symbol.iterator ] = function(){};
  test.identical( _.date.is( src ), false );

  test.case = 'new object';
  var src = new Object();
  test.identical( _.date.is( src ), false );

  test.case = 'routine';
  var src = routine;
  test.identical( _.date.is( src ), false );

  test.case = 'prototyped from pure map';
  var prototype = Object.create( null );
  var src = Object.create( prototype );
  test.identical( _.date.is( src ), false );

  test.case = 'prototyped from pure map deep';
  var prototype1 = Object.create( null );
  var prototype2 = Object.create( prototype1 );
  var src = Object.create( prototype1 );
  test.identical( _.date.is( src ), false );

  test.case = 'prototyped from pure map deep with props';
  var prototype1 = Object.create( null );
  prototype1.a = 1;
  var prototype2 = Object.create( prototype1 );
  prototype2.b = 1;
  var src = Object.create( prototype1 );
  src.c = 1;
  test.identical( _.date.is( src ), false );

  test.case = 'prototyped from polluted map';
  var prototype = {};
  var src = Object.create( prototype );
  test.identical( _.date.is( src ), false );

  test.case = 'prototyped from polluted map with constructor';
  var prototype = {};
  prototype.constructor = function(){}
  var src = Object.create( prototype );
  test.identical( _.date.is( src ), false );

  test.case = 'prototyped from polluted map deep';
  var prototype1 = {};
  var prototype2 = Object.create( prototype1 );
  var src = Object.create( prototype1 );
  test.identical( _.date.is( src ), false );

  test.case = 'prototyped from polluted map deep with props';
  var prototype1 = {};
  prototype1.a = 1;
  var prototype2 = Object.create( prototype1 );
  prototype2.b = 1;
  var src = Object.create( prototype1 );
  src.c = 1;
  test.identical( _.date.is( src ), false );

  /* - */

  function routine () {}

}

//

function exportStringDiagnosticShallow( test )
{
  test.case = 'new Date( 1000 )';
  var src = new Date( 1000 );
  var expected = '1970-01-01T00:00:01.000Z';
  var got = _.date.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'new Date( \'01 Jan 1970 00:00:00 GMT\' )';
  var src = new Date( '01 Jan 1970 00:00:00 GMT' );
  var expected = '1970-01-01T00:00:00.000Z';
  var got = _.date.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'new Date( 2011, 1 )';
  var src = new Date( Date.UTC( 2011, 1 ) );
  var expected = '2011-02-01T00:00:00.000Z';
  var got = _.date.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'new Date( Date.UTC( 2011, 1, 1 )';
  var src = new Date( Date.UTC( 2011, 1, 1 ) );
  var expected = '2011-02-01T00:00:00.000Z';
  var got = _.date.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'new Date( Date.UTC( 2011, 1, 1, 1 ) )';
  var src = new Date( Date.UTC( 2011, 1, 1, 1 ) );
  var expected = '2011-02-01T01:00:00.000Z';
  var got = _.date.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'new Date( Date.UTC( 2011, 1, 1, 1, 1 ) )';
  var src = new Date( Date.UTC( 2011, 1, 1, 1, 1 ) );
  var expected = '2011-02-01T01:01:00.000Z';
  var got = _.date.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'new Date( Date.UTC( 2011, 1, 1, 1, 1, 1 ) )';
  var src = new Date( Date.UTC( 2011, 1, 1, 1, 1, 1 ) );
  var expected = '2011-02-01T01:01:01.000Z';
  var got = _.date.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'new Date( Date.UTC( 2011, 1, 1, 1, 1, 1, 1 ) )';
  var src = new Date( Date.UTC( 2011, 1, 1, 1, 1, 1, 1 ) );
  var expected = '2011-02-01T01:01:01.001Z';
  var got = _.date.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  if( !Config.debug )
  return;

  test.case = 'without argument';
  test.shouldThrowErrorSync( () => _.bigInt.exportStringDiagnosticShallow() );

  test.case = 'not a date';
  test.shouldThrowErrorSync( () => _.bigInt.exportStringDiagnosticShallow( 11314 ) );

}

exportStringDiagnosticShallow.timeOut = 10000;

//

function exportStringCodeShallow( test )
{
  test.case = 'new Date( 1000 )';
  var src = new Date( 1000 );
  var expected = `new Date( '1970-01-01T00:00:01.000Z' )`;
  var got = _.date.exportStringCodeShallow( src );
  test.identical( got, expected );
  test.identical( strToDate( got ), src );

  test.case = 'new Date( \'01 Jan 1970 00:00:00 GMT\' )';
  var src = new Date( '01 Jan 1970 00:00:00 GMT' );
  var expected = `new Date( '1970-01-01T00:00:00.000Z' )`;
  var got = _.date.exportStringCodeShallow( src );
  test.identical( got, expected );
  test.identical( strToDate( got ), src );

  test.case = 'new Date( 2011, 1 )';
  var src = new Date( Date.UTC( 2011, 1 ) );
  var expected = `new Date( '2011-02-01T00:00:00.000Z' )`;
  var got = _.date.exportStringCodeShallow( src );
  test.identical( got, expected );
  test.identical( strToDate( got ), src );

  test.case = 'new Date( 2011, 1, 1';
  var src = new Date( Date.UTC( 2011, 1, 1 ) );
  var expected = `new Date( '2011-02-01T00:00:00.000Z' )`;
  var got = _.date.exportStringCodeShallow( src );
  test.identical( got, expected );
  test.identical( strToDate( got ), src );

  test.case = 'new Date( 2011, 1, 1, 1 )';
  var src = new Date( Date.UTC( 2011, 1, 1, 1 ) );
  var expected = `new Date( '2011-02-01T01:00:00.000Z' )`;
  var got = _.date.exportStringCodeShallow( src );
  test.identical( got, expected );
  test.identical( strToDate( got ), src );

  test.case = 'new Date( 2011, 1, 1, 1, 1 )';
  var src = new Date( Date.UTC( 2011, 1, 1, 1, 1 ) );
  var expected = `new Date( '2011-02-01T01:01:00.000Z' )`;
  var got = _.date.exportStringCodeShallow( src );
  test.identical( got, expected );
  test.identical( strToDate( got ), src );

  test.case = 'new Date( 2011, 1, 1, 1, 1, 1 )';
  var src = new Date( Date.UTC( 2011, 1, 1, 1, 1, 1 ) );
  var expected = `new Date( '2011-02-01T01:01:01.000Z' )`;
  var got = _.date.exportStringCodeShallow( src );
  test.identical( got, expected );
  test.identical( strToDate( got ), src );

  test.case = 'new Date( 2011, 1, 1, 1, 1, 1, 1 )';
  var src = new Date( Date.UTC( 2011, 1, 1, 1, 1, 1, 1 ) );
  var expected = `new Date( '2011-02-01T01:01:01.001Z' )`;
  var got = _.date.exportStringCodeShallow( src );
  test.identical( got, expected );
  test.identical( strToDate( got ), src );

  if( !Config.debug )
  return;

  test.case = 'without argument';
  test.shouldThrowErrorSync( () => _.bigInt.exportStringCodeShallow() );

  test.case = 'not a date';
  test.shouldThrowErrorSync( () => _.bigInt.exportStringCodeShallow( 11314 ) );

  /* - */

  function strToDate( stringDate )
  {
    return Function( `return ${stringDate}` )();
  }
}

// --
//
// --

const Proto =
{

  name : 'Tools.Date.l0.l9',
  silencing : 1,
  enabled : 1,

  tests :
  {
    is,
    exportStringDiagnosticShallow,
    exportStringCodeShallow
  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
