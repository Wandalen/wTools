( function _l0_l3_Path_test_s_( )
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( '../Include3.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

// --
// tests
// --

function is( test )
{

  // Input is path

  test.case = 'Empty string';
  var expected = true;
  var got = _.path.is( '' );
  test.identical( got, expected );

  test.case = 'Empty path';
  var expected = true;
  var got = _.path.is( '/' );
  test.identical( got, expected );

  test.case = 'Simple string';
  var expected = true;
  var got = _.path.is( 'hello' );
  test.identical( got, expected );

  test.case = 'Simple path string';
  var expected = true;
  var got = _.path.is( '/D/work/f' );
  test.identical( got, expected );

  test.case = 'Relative path';
  var expected = true;
  var got = _.path.is( '/home/user/dir1/dir2' );
  test.identical( got, expected );

  test.case = 'Absolute path';
  var expected = true;
  var got = _.path.is( 'C:/foo/baz/bar' );
  test.identical( got, expected );

  test.case = 'Other path';
  var expected = true;
  var got = _.path.is( 'c:\\foo\\' );
  test.identical( got, expected );

  // Input is not path

  test.case = 'No path - regexp';
  var expected = false;
  var got = _.path.is( /foo/ );
  test.identical( got, expected );

  test.case = 'No path - number';
  var expected = false;
  var got = _.path.is( 3 );
  test.identical( got, expected );

  test.case = 'No path - array';
  var expected = false;
  var got = _.path.is( [ '/C/', 'work/f' ] );
  test.identical( got, expected );

  test.case = 'No path - object';
  var expected = false;
  var got = _.path.is( { Path : 'C:/foo/baz/bar' } );
  test.identical( got, expected );

  test.case = 'No path - undefined';
  var expected = false;
  var got = _.path.is( undefined );
  test.identical( got, expected );

  test.case = 'No path - null';
  var expected = false;
  var got = _.path.is( null );
  test.identical( got, expected );

  test.case = 'No path - NaN';
  var expected = false;
  var got = _.path.is( NaN );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.is( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.is( 'a', 'b' ) );
}

//

function isRefined( test )
{
  /* */

  test.case = 'posix path, not refined';

  var path = '/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/a/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/../';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '//foo/bar//baz/asdf/quux/..//';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  /* */

  test.case = 'posix path, refined';

  var path = '/foo/bar//baz/asdf/quux/..';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/../';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '//foo/bar//baz/asdf/quux/..//';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  /* */

  test.case = 'winoows path, not refined';

  var path = 'C:\\temp\\\\foo\\bar\\..\\';
  var expected = false;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = false;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = false;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
  var expected = false;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var expected = false;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
  var expected = false;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  /* */

  test.case = 'winoows path, refined';

  var path = 'C:\\temp\\\\foo\\bar\\..\\';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  test.case = 'empty path,not refined';

  var path = '';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '//';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '///';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/.';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '.';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = './.';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  test.case = 'empty path';

  var path = '';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  test.case = 'here';

  var path = '.';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '//';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '///';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '/.';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '/./.';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '.';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = './.';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the middle';

  var path = 'foo/./bar/baz';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'foo/././bar/baz/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'foo/././bar/././baz/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/foo/././bar/././baz/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  test.case = 'path with "." in the middle,refined';

  var path = 'foo/./bar/baz';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'foo/././bar/baz/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'foo/././bar/././baz/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '/foo/././bar/././baz/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the middle';

  var path = 'foo/../bar/baz';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/baz/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/../../baz/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/foo/../../bar/../../baz/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the middle,refined';

  var path = 'foo/../bar/baz';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'foo/../../bar/baz/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = 'foo/../../bar/../../baz/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '/foo/../../bar/../../baz/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the beginning';

  var path = '../foo/bar';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '../../foo/bar/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '/..//..//foo/bar/';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '..x/foo/bar';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var expected = true;
  var got = _.path.isRefined( path );
  test.identical( got, expected );

  test.case = 'path with ".." in the beginning, refined';

  var path = '../foo/bar';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '../../foo/bar/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '/..//..//foo/bar/';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '..x/foo/bar';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var refined = _.path.refine( path );
  var expected = true;
  var got = _.path.isRefined( refined );
  test.identical( got, expected );

  test.case = 'path with ".." in the beginning, refined and trailed';

  var path = '../foo/bar';
  var refined = _.path.refine( path );
  var trailed = _.path.trail( refined );
  var expected = true;
  var got = _.path.isRefined( trailed );
  test.identical( got, expected );

  var path = '../../foo/bar/';
  var refined = _.path.refine( path );
  var trailed = _.path.trail( refined );
  var expected = true;
  var got = _.path.isRefined( trailed );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var refined = _.path.refine( path );
  var trailed = _.path.trail( refined );
  var expected = true;
  var got = _.path.isRefined( trailed );
  test.identical( got, expected );

  var path = '/..//..//foo/bar/';
  var refined = _.path.refine( path );
  var trailed = _.path.trail( refined );
  var expected = true;
  var got = _.path.isRefined( trailed );
  test.identical( got, expected );

  var path = '..x/foo/bar';
  var refined = _.path.refine( path );
  var trailed = _.path.trail( refined );
  var expected = true;
  var got = _.path.isRefined( trailed );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var refined = _.path.refine( path );
  var trailed = _.path.trail( refined );
  var expected = true;
  var got = _.path.isRefined( trailed );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( 'a', 'b' ) );

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( NaN ) );

}

// //
//
// function isRefined( test )
// {
//   /* */
//
//   test.case = 'posix path, not refined';
//
//   var path = '/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/a/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/foo/bar//baz/asdf/quux/..';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/foo/bar//baz/asdf/quux/../';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '//foo/bar//baz/asdf/quux/..//';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'foo/bar//baz/asdf/quux/..//.';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'posix path, refined';
//
//   var path = '/foo/bar//baz/asdf/quux/..';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '/foo/bar//baz/asdf/quux/../';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '//foo/bar//baz/asdf/quux/..//';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'foo/bar//baz/asdf/quux/..//.';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'winoows path, not refined';
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\';
//   var expected = false;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
//   var expected = false;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
//   var expected = false;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\..';
//   var expected = false;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
//   var expected = false;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
//   var expected = false;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'winoows path, refined';
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\..';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   test.case = 'empty path,not refined';
//
//   var path = '';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '//';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '///';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/.';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/./.';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '.';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = './.';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   test.case = 'empty path';
//
//   var path = '';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   test.case = 'here';
//
//   var path = '.';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '//';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '///';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '/.';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '/./.';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '.';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = './.';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with "." in the middle';
//
//   var path = 'foo/./bar/baz';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'foo/././bar/baz/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'foo/././bar/././baz/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/foo/././bar/././baz/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   test.case = 'path with "." in the middle,refined';
//
//   var path = 'foo/./bar/baz';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'foo/././bar/baz/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'foo/././bar/././baz/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '/foo/././bar/././baz/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." in the middle';
//
//   var path = 'foo/../bar/baz';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'foo/../../bar/baz/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = 'foo/../../bar/../../baz/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/foo/../../bar/../../baz/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." in the middle,refined';
//
//   var path = 'foo/../bar/baz';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'foo/../../bar/baz/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = 'foo/../../bar/../../baz/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '/foo/../../bar/../../baz/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." in the beginning';
//
//   var path = '../foo/bar';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '../../foo/bar/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '..//..//foo/bar/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '/..//..//foo/bar/';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '..x/foo/bar';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   var path = '..x../foo/bar';
//   var expected = true;
//   var got = _.path.isRefined( path );
//   test.identical( got, expected );
//
//   test.case = 'path with ".." in the beginning, refined';
//
//   var path = '../foo/bar';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '../../foo/bar/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '..//..//foo/bar/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '/..//..//foo/bar/';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '..x/foo/bar';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   var path = '..x../foo/bar';
//   var refined = _.path.refine( path );
//   var expected = true;
//   var got = _.path.isRefined( refined );
//   test.identical( got, expected );
//
//   test.case = 'path with ".." in the beginning, refined and trailed';
//
//   var path = '../foo/bar';
//   var refined = _.path.refine( path );
//   var trailed = _.path.trail( refined );
//   var expected = true;
//   var got = _.path.isRefined( trailed );
//   test.identical( got, expected );
//
//   var path = '../../foo/bar/';
//   var refined = _.path.refine( path );
//   var trailed = _.path.trail( refined );
//   var expected = true;
//   var got = _.path.isRefined( trailed );
//   test.identical( got, expected );
//
//   var path = '..//..//foo/bar/';
//   var refined = _.path.refine( path );
//   var trailed = _.path.trail( refined );
//   var expected = true;
//   var got = _.path.isRefined( trailed );
//   test.identical( got, expected );
//
//   var path = '/..//..//foo/bar/';
//   var refined = _.path.refine( path );
//   var trailed = _.path.trail( refined );
//   var expected = true;
//   var got = _.path.isRefined( trailed );
//   test.identical( got, expected );
//
//   var path = '..x/foo/bar';
//   var refined = _.path.refine( path );
//   var trailed = _.path.trail( refined );
//   var expected = true;
//   var got = _.path.isRefined( trailed );
//   test.identical( got, expected );
//
//   var path = '..x../foo/bar';
//   var refined = _.path.refine( path );
//   var trailed = _.path.trail( refined );
//   var expected = true;
//   var got = _.path.isRefined( trailed );
//   test.identical( got, expected );
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'No arguments';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( ) );
//
//   test.case = 'Two arguments';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( 'a', 'b' ) );
//
//   test.case = 'No path - regexp';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( /foo/ ) );
//
//   test.case = 'No path - number';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( 3 ) );
//
//   test.case = 'No path - array';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( [ '/C/', 'work/f' ] ) );
//
//   test.case = 'No path - object';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( { Path : 'C:/foo/baz/bar' } ) );
//
//   test.case = 'No path - undefined';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( undefined ) );
//
//   test.case = 'No path - null';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( null ) );
//
//   test.case = 'No path - NaN';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isRefined( NaN ) );
//
// }

//

function isNormalized( test )
{

  var got;

  /* */

  test.case = 'posix path';

  // Not normalized

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/../';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  // Normalized

  var path = '/foo/bar//baz/asdf';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = _.path.normalize( '/foo/bar//baz/asdf/quux/../' );
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = _.path.normalize( 'foo/bar//baz/asdf/quux/..//.' );
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'windows path';

  //Not normalized

  var path = '/C:\\temp\\\\foo\\bar\\..\\';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '\\C:\\temp\\\\foo\\bar\\..\\';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = 'c://temp/foo/bar/';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  // Normalized

  var path = _.path.normalize( '/C:\\temp\\\\foo\\bar\\..\\' );
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/C:/temp//foo';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = _.path.normalize( 'C:\\temp\\\\foo\\bar\\..\\..\\' );
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'empty path';

  var path = '';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '.';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '///';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = './.';
  var expected = false;
  debugger;
  var got = _.path.isNormalized( path );
  debugger;
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the middle';

  var path = 'foo/./bar/baz';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/foo/././bar/././baz/';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/foo/.x./baz';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the beginning';

  var path = './foo/bar';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '.\\foo\\bar';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '././foo/bar/';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/.//.//foo/bar/';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '.x./foo/bar/';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '.x./foo/bar';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the end';

  var path = 'foo/.bar.';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = 'foo/bar/./.';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/foo/baz/.x./';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the middle';

  var path = 'foo/../bar/baz';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/foo/../../bar/../../baz/';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = _.path.normalize( '/foo/../../bar/../../baz/' );
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/../../baz';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the beginning';

  var path = '../../foo/bar';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the end';

  var path = 'foo/..bar..';
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = 'foo/bar/..';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../../..';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." and "." combined';

  var path = '/abc/./.././a/b';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./../.';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = './../.';
  var expected = false;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  var path = _.path.normalize( '/a/b/abc/./../.' );
  var expected = true;
  var got = _.path.isNormalized( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." and "." combined - normalized';

  var path = '/abc/./.././a/b';
  var normalized = _.path.normalize( path );
  var expected = true;
  var got = _.path.isNormalized( normalized );
  test.identical( got, expected );

  var path = '/a/b/abc/./../.';
  var normalized = _.path.normalize( path );
  var expected = true;
  var got = _.path.isNormalized( normalized );
  test.identical( got, expected );

  var path = './../.';
  var normalized = _.path.normalize( path );
  var expected = true;
  var got = _.path.isNormalized( normalized );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." and "." combined - normalized and trailed';

  var path = '/abc/./.././a/b';
  var normalized = _.path.normalize( path );
  var trailed = _.path.trail( normalized );
  var expected = true;
  var got = _.path.isNormalized( trailed );
  test.identical( got, expected );

  var path = '/a/b/abc/./../.';
  var normalized = _.path.normalize( path );
  var trailed = _.path.trail( normalized );
  var expected = true;
  var got = _.path.isNormalized( trailed );
  test.identical( got, expected );

  var path = './../.';
  var normalized = _.path.normalize( path );
  var trailed = _.path.trail( normalized );
  var expected = true;
  var got = _.path.isNormalized( trailed );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( 'a', 'b' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( NaN ) );

}

// //
//
// function isNormalized( test )
// {
//
//   var got;
//
//   /* */
//
//   test.case = 'posix path';
//
//   // Not normalized
//
//   var path = '/foo/bar//baz/asdf/quux/..';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/foo/bar//baz/asdf/quux/../';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = 'foo/bar//baz/asdf/quux/..//.';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   // Normalized
//
//   var path = '/foo/bar//baz/asdf';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = _.path.normalize( '/foo/bar//baz/asdf/quux/../' );
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = _.path.normalize( 'foo/bar//baz/asdf/quux/..//.' );
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'windows path';
//
//   //Not normalized
//
//   var path = '/C:\\temp\\\\foo\\bar\\..\\';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '\\C:\\temp\\\\foo\\bar\\..\\';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\..';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = 'c://temp/foo/bar/';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   // Normalized
//
//   var path = _.path.normalize( '/C:\\temp\\\\foo\\bar\\..\\' );
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/C:/temp//foo';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = _.path.normalize( 'C:\\temp\\\\foo\\bar\\..\\..\\' );
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'empty path';
//
//   var path = '';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '.';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '///';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/./.';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = './.';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with "." in the middle';
//
//   var path = 'foo/./bar/baz';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/foo/././bar/././baz/';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/foo/.x./baz';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with "." in the beginning';
//
//   var path = './foo/bar';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '.\\foo\\bar';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '././foo/bar/';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/.//.//foo/bar/';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '.x./foo/bar';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with "." in the end';
//
//   var path = 'foo/.bar.';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = 'foo/bar/./.';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/foo/baz/.x./';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." in the middle';
//
//   var path = 'foo/../bar/baz';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/foo/../../bar/../../baz/';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = _.path.normalize( '/foo/../../bar/../../baz/' );
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/../../baz';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." in the beginning';
//
//   var path = '../../foo/bar';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '..//..//foo/bar/';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '..x../foo/bar';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." in the end';
//
//   var path = 'foo/..bar..';
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = 'foo/bar/..';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = 'foo/bar/../../../..';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." and "." combined';
//
//   var path = '/abc/./.././a/b';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = '/a/b/abc/./../.';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = './../.';
//   var expected = false;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   var path = _.path.normalize( '/a/b/abc/./../.' );
//   var expected = true;
//   var got = _.path.isNormalized( path );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." and "." combined - normalized';
//
//   var path = '/abc/./.././a/b';
//   var normalized = _.path.normalize( path );
//   var expected = true;
//   var got = _.path.isNormalized( normalized );
//   test.identical( got, expected );
//
//   var path = '/a/b/abc/./../.';
//   var normalized = _.path.normalize( path );
//   var expected = true;
//   var got = _.path.isNormalized( normalized );
//   test.identical( got, expected );
//
//   var path = './../.';
//   var normalized = _.path.normalize( path );
//   var expected = true;
//   var got = _.path.isNormalized( normalized );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'path with ".." and "." combined - normalized and trailed';
//
//   var path = '/abc/./.././a/b';
//   var normalized = _.path.normalize( path );
//   var trailed = _.path.trail( normalized );
//   var expected = true;
//   var got = _.path.isNormalized( trailed );
//   test.identical( got, expected );
//
//   var path = '/a/b/abc/./../.';
//   var normalized = _.path.normalize( path );
//   var trailed = _.path.trail( normalized );
//   var expected = true;
//   var got = _.path.isNormalized( trailed );
//   test.identical( got, expected );
//
//   var path = './../.';
//   var normalized = _.path.normalize( path );
//   var trailed = _.path.trail( normalized );
//   var expected = true;
//   var got = _.path.isNormalized( trailed );
//   test.identical( got, expected );
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'No arguments';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( ) );
//
//   test.case = 'Two arguments';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( 'a', 'b' ) );
//
//   // Input is not path
//
//   test.case = 'No path - regexp';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( /foo/ ) );
//
//   test.case = 'No path - number';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( 3 ) );
//
//   test.case = 'No path - array';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( [ '/C/', 'work/f' ] ) );
//
//   test.case = 'No path - object';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( { Path : 'C:/foo/baz/bar' } ) );
//
//   test.case = 'No path - undefined';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( undefined ) );
//
//   test.case = 'No path - null';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( null ) );
//
//   test.case = 'No path - NaN';
//   test.shouldThrowErrorOfAnyKind( () => _.path.isNormalized( NaN ) );
//
// }

//

function isAbsolute( test )
{

  // Absolute path

  test.case = 'Absolute path';

  var got = _.path.isAbsolute( '/D' );
  test.identical( got, true );

  var got = _.path.isAbsolute( '/D/' );
  test.identical( got, true );

  var got = _.path.isAbsolute( '/D/work' );
  test.identical( got, true );

  var got = _.path.isAbsolute( '/D/work/f' );
  test.identical( got, true );

  var got = _.path.isAbsolute( '/D/work/f/' );
  test.identical( got, true );

  // No absolute path

  test.case = 'Not absolute path';

  var got = _.path.isAbsolute( 'c' );
  test.identical( got, false );

  var got = _.path.isAbsolute( 'c/' );
  test.identical( got, false );

  var got = _.path.isAbsolute( 'c/work' );
  test.identical( got, false );

  var got = _.path.isAbsolute( 'c/work/' );
  test.identical( got, false );

  /* */

  test.case = 'posix path';

  var path = '/foo/bar/baz/asdf/quux/..';
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  /* */

  test.case = 'posix path';

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = false;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  /* */

  test.case = 'windows path';

  test.open( 'not normalized' );

  var path = '/C:/temp/foo/bar/../';
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = 'C:/temp/foo/bar/../';
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = 'c://temp/foo/bar/';
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  test.close( 'not normalized' );
  test.open( 'normalized' );

  var path = _.path.refine( '/C:/temp/foo/bar/../' );
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = _.path.refine( 'C:/temp/foo/bar/../' );
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = _.path.refine( 'c://temp/foo/bar/' );
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  test.close( 'normalized' );

  /* */

  test.case = 'empty path';

  var path = '';
  var expected = false;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = '.';
  var expected = false;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = '/';
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = '///';
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = true;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  var path = './.';
  var expected = false;
  var got = _.path.isAbsolute( path );
  test.identical( got, expected );

  test.open( 'not refined' );

  test.case = '\\ in the beggining';
  var got = _.path.isAbsolute( '\\C:/foo/baz/bar' );
  test.identical( got, true );

  test.case = '\\ in the middle';
  var got = _.path.isAbsolute( 'C:/foo\\baz\\bar' );
  test.identical( got, true );

  test.case = '\\ in the end';
  var got = _.path.isAbsolute( 'C:/foo/baz/bar\\' );
  test.identical( got, true );

  test.close( 'not refined' );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( 'a', 'b' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( NaN ) );

  // // Input is not Refined
  /* */
  // test.case = '\\ in the beggining';
  // test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( '\\C:/foo/baz/bar' ) );
  /* */
  // test.case = '\\ in the middle';
  // test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( 'C:/foo\\baz\\bar' ) );
  /* */
  // test.case = '\\ in the end';
  // test.shouldThrowErrorOfAnyKind( () => _.path.isAbsolute( 'C:/foo/baz/bar\\' ) );

}

//

function isRelative( test )
{

  // Absolute path

  test.case = 'Absolute path';

  var got = _.path.isRelative( '/D' );
  test.identical( got, false );

  var got = _.path.isRelative( '/D/' );
  test.identical( got, false );

  var got = _.path.isRelative( '/D/work' );
  test.identical( got, false );

  var got = _.path.isRelative( '/D/work/f' );
  test.identical( got, false );

  var got = _.path.isRelative( '/D/work/f/' );
  test.identical( got, false );

  // Relative path

  test.case = 'Relative path';

  var got = _.path.isRelative( 'c' );
  test.identical( got, true );

  var got = _.path.isRelative( 'c/' );
  test.identical( got, true );

  var got = _.path.isRelative( 'c/work' );
  test.identical( got, true );

  var got = _.path.isRelative( 'c/work/' );
  test.identical( got, true );

  var got = _.path.isRelative( 'c/work/f/' );
  test.identical( got, true );

  /* */

  test.case = 'posix path';

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = true;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  /* */

  test.case = 'windows path';

  //Not normalized

  var path = '/C:/temp/foo/bar/../';
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = 'C:/temp/foo/bar/../';
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = 'c://temp/foo/bar/';
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  // Normalized

  var path = _.path.normalize( '/C:/temp/foo/bar/../' );
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = _.path.normalize( 'C:/temp/foo/bar/../' );
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = _.path.normalize( 'c://temp/foo/bar/' );
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = _.path.normalize( 'c/temp/foo/bar/' );
  var expected = true;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  test.open( 'not refined' );

  test.case = '\\ in the beggining';
  var got = _.path.isRelative( '\\C:/foo/baz/bar' );
  test.identical( got, false );

  test.case = '\\ in the middle';
  var got = _.path.isRelative( 'C:/foo\\baz\\bar' );
  test.identical( got, false );

  test.case = '\\ in the end';
  var got = _.path.isRelative( 'C:/foo/baz/bar\\' );
  test.identical( got, false );

  test.close( 'not refined' );

  /* */

  test.case = 'empty path';

  var path = '';
  var expected = true;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = '.';
  var expected = true;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = '/';
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = '///';
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = false;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  var path = './.';
  var expected = true;
  var got = _.path.isRelative( path );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( 'a', 'b' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( NaN ) );

  // // Input is not Normalized
  /* */
  // test.case = '\\ in the beggining';
  // test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( '\\C:/foo/baz/bar' ) );
  /* */
  // test.case = '\\ in the middle';
  // test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( 'C:/foo\\baz\\bar' ) );
  /* */
  // test.case = '\\ in the end';
  // test.shouldThrowErrorOfAnyKind( () => _.path.isRelative( 'C:/foo/baz/bar\\' ) );

}

//

function isGlobal( test )
{

  // Not global paths

  test.case = 'Empty';

  var expected = false;
  var got = _.path.isGlobal( '' );
  test.identical( got, expected );

  var expected = false;
  var got = _.path.isGlobal( '/' );
  test.identical( got, expected );

  var expected = false;
  var got = _.path.isGlobal( '/.' );
  test.identical( got, expected );

  var expected = false;
  var got = _.path.isGlobal( '///' );
  test.identical( got, expected );

  var expected = false;
  var got = _.path.isGlobal( '.' );
  test.identical( got, expected );

  var expected = false;
  var got = _.path.isGlobal( '/./.' );
  test.identical( got, expected );

  var expected = false;
  var got = _.path.isGlobal( './.' );
  test.identical( got, expected );

  test.case = 'Relative path';
  var expected= false;
  var got = _.path.isGlobal( 'c/work/f/' );
  test.identical( got, expected );

  test.case = 'posix path';

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  /* */

  test.case = 'windows path';

  var path = 'c:/';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = 'C:/temp/foo';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  /* */

  test.case = 'windows path';
  var path = '/C:/temp/foo/bar/../';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  // Global paths

  test.case = 'Empty';

  var got = _.path.isGlobal( '://' );
  var expected = true;
  test.identical( got, expected );

  var got = _.path.isGlobal( '/://' );
  var expected = true;
  test.identical( got, expected );

  var expected = true;
  var got = _.path.isGlobal( '.:///' );
  test.identical( got, expected );

  var expected = true;
  var got = _.path.isGlobal( '/.://./.' );
  test.identical( got, expected );

  var expected= true;
  var got = _.path.isGlobal( 'c/work/f/://' );
  test.identical( got, expected );

  var path = '/foo/bar/://baz/asdf/quux/..';
  var expected = true;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = '://foo/bar//baz/asdf/quux/..//.';
  var expected = true;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = '../abc/**#master';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = '../abc#master';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = '#master';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = '#';
  var expected = false;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  /* */

  test.case = 'windows path';

  var path = 'c://';
  var expected = true;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = 'C://temp/foo';
  var expected = true;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  var path = '/C://temp/foo/bar/../';
  var expected = true;
  var got = _.path.isGlobal( path );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( 'a', 'b' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.isGlobal( NaN ) );

}

//

function isRoot( test )
{

  test.case = 'Is root';

  var path = '/';
  var got = _.path.isRoot( path );
  test.identical( got, true );

  var path = '/.';
  var got = _.path.isRoot( path );
  test.identical( got, true );

  var path = '/./';
  var got = _.path.isRoot( path );
  test.identical( got, true );

  var path = '/./.';
  var got = _.path.isRoot( path );
  test.identical( got, true );

  var path = '/x/..';
  var got = _.path.isRoot( path );
  test.identical( got, true );

  var path = '/x/y/../..';
  var got = _.path.isRoot( path );
  test.identical( got, true );

  test.case = 'Is not root';

  var path = '';
  var got = _.path.isRoot( path );
  test.identical( got, false );

  var path = '.';
  var got = _.path.isRoot( path );
  test.identical( got, false );

  var path = './';
  var got = _.path.isRoot( path );
  test.identical( got, false );

  var path = '/c';
  var got = _.path.isRoot( path );
  test.identical( got, false );

  var path = '/src/a1';
  var got = _.path.isRoot( path );
  test.identical( got, false );

  var path = 'c:/src/a1';
  var got = _.path.isRoot( path );
  test.identical( got, false );

  var path = '/C://src/a1';
  var got = _.path.isRoot( path );
  test.identical( got, false );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var got = _.path.isRoot( path );
  test.identical( got, false );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( 'a', 'b' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.isRoot( NaN ) );

}

//

function isDotted( test )
{
  test.case = 'is single dotted, unix paths';

  var path = '.';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = './';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = './..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = './../';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = './.';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '././';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = './x/..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = './c';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = './src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = './C://src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  test.case = 'is single dotted, Windows paths';

  var path = '.\\';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '.\\..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '.\\..\\';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '.\\.';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '.\\.\\';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '.\\x\\..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '.\\c';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '.\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '.\\C:\\\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  /* */

  test.case = 'is double dotted, unix paths';

  var path = '..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '../';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '../.';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '../..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '../../';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '../x/..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '../c';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '../src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '../C://src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  /* */

  test.case = 'is double dotted, Windows paths';

  var path = '..\\';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '..\\.';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '..\\..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '..\\..\\';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '..\\x\\..';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '..\\c';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '..\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  var path = '..\\C:\\\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, true );

  /* */

  test.case = 'is not dotted, unix paths';

  var path = '';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '/';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '/./.';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '/..';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '/c';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '/src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = 'c:/src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '/C://src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '.c:/src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '.foo/bar';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '.foo/bar//baz/asdf/quux/..//.';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '..c:/src/a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '..foo/bar';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '..foo/bar//baz/asdf/quux/..//.';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  /* */

  test.case = 'is not dotted, Windows paths';

  var path = '\\';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '\\.\\.';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '\\..';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '\\c';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = 'c:\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '\\C:\\\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = 'foo\\bar\\\\baz\\asdf\\quux\\..\\\\.';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '.c:\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '.foo\\bar';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '.foo\\bar\\\\baz\\asdf\\quux\\..\\\\.';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '..c:\\src\\a1';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '..foo\\bar';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  var path = '..foo\\bar\\\\baz\\asdf\\quux\\..\\\\.';
  var got = _.path.isDotted( path );
  test.identical( got, false );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.path.isDotted() );

  test.case = 'two arguments';
  test.shouldThrowErrorSync( () => _.path.isDotted( 'a', 'b' ) );

  test.case = 'wrong type of srcPath';
  test.shouldThrowErrorSync( () => _.path.isDotted( /foo/ ) );
  test.shouldThrowErrorSync( () => _.path.isDotted( 3 ) );
  test.shouldThrowErrorSync( () => _.path.isDotted( { Path : 'C:/foo/baz/bar' } ) );
  test.shouldThrowErrorSync( () => _.path.isDotted( undefined ) );
  test.shouldThrowErrorSync( () => _.path.isDotted( null ) );
  test.shouldThrowErrorSync( () => _.path.isDotted( NaN ) );
}

//

function isTrailed( test )
{

  test.case = 'Is trailed';

  var path = './';
  var got = _.path.isTrailed( path );
  test.identical( got, true );

  var path = '.././';
  var got = _.path.isTrailed( path );
  test.identical( got, true );

  var path = '/../';
  var got = _.path.isTrailed( path );
  test.identical( got, true );

  var path = '/c/';
  var got = _.path.isTrailed( path );
  test.identical( got, true );

  var path = 'src/a1/';
  var got = _.path.isTrailed( path );
  test.identical( got, true );

  var path = 'c:/src/a1/';
  var got = _.path.isTrailed( path );
  test.identical( got, true );

  var path = '/C://src/a1/';
  var got = _.path.isTrailed( path );
  test.identical( got, true );

  var path = 'foo/bar//baz/asdf/quux/..//./';
  var got = _.path.isTrailed( path );
  test.identical( got, true );

  test.case = 'Is not trailed';

  var path = '';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = '/';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = '.';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = '/.';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = './.';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = '././..';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = './x/..';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = './c';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = '.src/a1';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = '.c:/src/a1';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = './C://src/a1';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  var path = '.foo/bar//baz/asdf/quux/..//.';
  var got = _.path.isTrailed( path );
  test.identical( got, false );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( 'a', 'b' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.isTrailed( NaN ) );

}

//

function begins( test )
{

  test.case = 'Same string';

  var got = _.path.begins( 'a', 'a' );
  test.identical( got, true );

  var got = _.path.begins( 'c:/', 'c:/' );
  test.identical( got, true );

  var got = _.path.begins( 'this/is/some/path', 'this/is/some/path' );
  test.identical( got, true );

  var got = _.path.begins( '/this/is/some/path', '/this/is/some/path' );
  test.identical( got, true );

  test.case = 'Begins';

  var got = _.path.begins( 'a/', 'a' );
  test.identical( got, true );

  var got = _.path.begins( 'a/b', 'a' );
  test.identical( got, true );

  var got = _.path.begins( 'this/is/some/path', 'this/is' );
  test.identical( got, true );

  var got = _.path.begins( '/this/is/some/path', '/this/is' );
  test.identical( got, true );

  var got = _.path.begins( '.src/a1', '.src' );
  test.identical( got, true );

  var got = _.path.begins( '.src/a1', '.src/' );
  test.identical( got, true );

  var got = _.path.begins( 'c:/src/a1', 'c:' );
  test.identical( got, true );

  var got = _.path.begins( '/C://src/a1', '/C:' );
  test.identical( got, true );

  var got = _.path.begins( 'foo/bar//baz/asdf/quux/..//.', 'foo' );
  test.identical( got, true );

  var got = _.path.begins( 'foo/bar//baz/asdf/quux/..//.', 'foo/' );
  test.identical( got, true );

  test.case = 'Doesn´t begin';

  var got = _.path.begins( 'ab', 'a' );
  test.identical( got, false );

  var got = _.path.begins( 'a/b', 'b' );
  test.identical( got, false );

  var got = _.path.begins( 'this/is/some/path', '/this/is' );
  test.identical( got, false );

  var got = _.path.begins( 'this/is/some/path', '/this/is/some/path' );
  test.identical( got, false );

  var got = _.path.begins( '/this/is/some/path', 'this/is' );
  test.identical( got, false );

  var got = _.path.begins( '/this/is/some/path', 'this/is/some/path' );
  test.identical( got, false );

  var got = _.path.begins( '/this/is/some/pathpath', '/this/is/some/path' );
  test.identical( got, false );

  var got = _.path.begins( '/this/is/some/path', '/this/is/some/pathpath' );
  test.identical( got, false );

  var got = _.path.begins( 'c:/src/a1', '/c:' );
  test.identical( got, false );

  var got = _.path.begins( '/C://src/a1', 'C:' );
  test.identical( got, false );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( ) );

  test.case = 'One argument';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( 'a' ) );

  test.case = 'Three arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( 'a', 'b', 'c' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( /foo/,  /foo/ ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( [ ], [ ] ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( 3, 3 ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( { Path : 'C:/' }, { Path : 'C:/' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( null ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( null, null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( NaN ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.begins( NaN, NaN ) );

}

//

function ends( test )
{

  test.case = 'Doesn´t End';

  var got = _.path.ends( 'a/b', 'a' );
  test.identical( got, false );

  var got = _.path.ends( 'a/b', '/b' );
  test.identical( got, false );

  var got = _.path.ends( 'this/is/some/path', '/some/path' );
  test.identical( got, false );

  var got = _.path.ends( '/this/is/some/path', '/some/path' );
  test.identical( got, false );

  var got = _.path.ends( 'this/is/some/path', 'this/is' );
  test.identical( got, false );

  var got = _.path.ends( 'this/is/some/pathpath', 'path' );
  test.identical( got, false );

  var got = _.path.ends( 'this/is/some/path', '/this/is/some/path' );
  test.identical( got, false );

  var got = _.path.ends( '.src/a1', '.a1' );
  test.identical( got, false );

  var got = _.path.ends( '.src/a1', '/a1' );
  test.identical( got, false );

  var got = _.path.ends( 'c:/src/a1', '/src/a1' );
  test.identical( got, false );

  var got = _.path.ends( 'c:/src/_a1', 'a1' );
  test.identical( got, false );

  var got = _.path.ends( 'foo/bar//baz/asdf/quux/..//.', 'quux' );
  test.identical( got, false );

  var got = _.path.ends( 'foo/bar//baz/asdf/quux/..', 'asdf' );
  test.identical( got, false );

  var got = _.path.ends( 'foo/bar//baz/asdf/quux/..', '/..' );
  test.identical( got, false );

  /* */

  test.case = 'Ends';

  var got = _.path.ends( 'a', 'a' );
  test.identical( got, true );

  var got = _.path.ends( '/a', './a' );
  test.identical( got, true );

  var got = _.path.ends( '.a', 'a' );
  test.identical( got, true );

  var got = _.path.ends( '/C://src/a1/', 'a1/' );
  test.identical( got, true );

  var got = _.path.ends( 'this/is/some/path', 'path' );
  test.identical( got, true );

  var got = _.path.ends( 'this/is/some/path', './path' );
  test.identical( got, true );

  var got = _.path.ends( 'this/is/some/path', 'some/path' );
  test.identical( got, true );

  var got = _.path.ends( 'this/is/some/path', 'is/some/path' );
  test.identical( got, true );

  var got = _.path.ends( 'this/is/some/path', 'this/is/some/path' );
  test.identical( got, true );

  var got = _.path.ends( '/this/is/some/path', 'some/path' );
  test.identical( got, true );

  var got = _.path.ends( '/this/is/some/path', './some/path' );
  test.identical( got, true );

  var got = _.path.ends( '/this/is/some/path', 'this/is/some/path' );
  test.identical( got, true );

  var got = _.path.ends( '/this/is/some/path', '/this/is/some/path' );
  test.identical( got, true );

  var got = _.path.ends( 'c:/file.src_a1', 'src_a1' );
  test.identical( got, true );

  var got = _.path.ends( 'c:/file/src/_a1', '_a1' );
  test.identical( got, true );

  var got = _.path.ends( 'c:/file/src._a1', '_a1' );
  test.identical( got, true );

  var got = _.path.ends( 'foo/bar//baz/asdf/quux/..//.', './/.' );
  test.identical( got, true );

  var got = _.path.ends( 'foo/bar//baz/asdf/quux/..//.', '/.' );
  test.identical( got, true );

  var got = _.path.ends( 'foo/bar//baz/asdf/quux/..//.', '.' );
  test.identical( got, true );

  var got = _.path.ends( 'foo/bar//baz/asdf/quux/...', '.' );
  test.identical( got, true );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( ) );

  test.case = 'One argument';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( 'a' ) );

  test.case = 'Three arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( 'a', 'b', 'c' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( /foo/,  /foo/ ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( [ ], [ ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( { Path : 'C:/' }, { Path : 'C:/' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( undefined ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( undefined, undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( null ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( null, null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( NaN ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.ends( NaN, NaN ) );
}

//

function refine( test )
{

  /* */

  test.case = 'posix path';

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = '/foo/bar//baz/asdf/quux/..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/../';
  var expected = '/foo/bar//baz/asdf/quux/../';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '//foo/bar//baz/asdf/quux/..//';
  var expected = '//foo/bar//baz/asdf/quux/..//';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = 'foo/bar//baz/asdf/quux/..//.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'winoows path';

  var path = 'C:\\\\';
  var expected = '/C//';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'C:\\';
  var expected = '/C';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'C:';
  var expected = '/C';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C/temp//foo/bar/../';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp//foo/bar/..//';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp//foo/bar/..//';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var expected = '/C/temp//foo/bar/../..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
  var expected = '/C/temp//foo/bar/../../';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
  var expected = '/C/temp//foo/bar/../../.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  test.case = 'empty path';
  var path = '';
  var expected = '';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/';
  var expected = '/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\\';
  var expected = '/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\\\\';
  var expected = '//';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\/';
  var expected = '/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '//';
  var expected = '//';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '///';
  var expected = '///';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/.';
  var expected = '/.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = '/./.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '.';
  var expected = '.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = './.';
  var expected = './.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '././';
  var expected = '././';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the middle';

  var path = 'foo/./bar/baz';
  var expected = 'foo/./bar/baz';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/././bar/baz/';
  var expected = 'foo/././bar/baz/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/././bar/././baz/';
  var expected = 'foo/././bar/././baz/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/foo/././bar/././baz/';
  var expected = '/foo/././bar/././baz/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the beginning';

  var path = './foo/bar';
  var expected = './foo/bar';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '././foo/bar/';
  var expected = '././foo/bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = './/.//foo/bar/';
  var expected = './/.//foo/bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/.//.//foo/bar/';
  var expected = '/.//.//foo/bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '.x/foo/bar';
  var expected = '.x/foo/bar';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '.x./foo/bar';
  var expected = '.x./foo/bar';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the end';

  var path = 'foo/bar.';
  var expected = 'foo/bar.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/.bar.';
  var expected = 'foo/.bar.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/bar/.';
  var expected = 'foo/bar/.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/bar/./.';
  var expected = 'foo/bar/./.';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/bar/././';
  var expected = 'foo/bar/././';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/foo/bar/././';
  var expected = '/foo/bar/././';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the middle';

  var path = 'foo/../bar/baz';
  var expected = 'foo/../bar/baz';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/baz/';
  var expected = 'foo/../../bar/baz/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/../../baz/';
  var expected = 'foo/../../bar/../../baz/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/foo/../../bar/../../baz/';
  var expected = '/foo/../../bar/../../baz/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the beginning';

  var path = '../foo/bar';
  var expected = '../foo/bar';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '../../foo/bar/';
  var expected = '../../foo/bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var expected = '..//..//foo/bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/..//..//foo/bar/';
  var expected = '/..//..//foo/bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '..x/foo/bar';
  var expected = '..x/foo/bar';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var expected = '..x../foo/bar';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the end';

  var path = 'foo/bar..';
  var expected = 'foo/bar..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/..bar..';
  var expected = 'foo/..bar..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/bar/..';
  var expected = 'foo/bar/..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/bar/../..';
  var expected = 'foo/bar/../..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../';
  var expected = 'foo/bar/../../';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '/foo/bar/../../';
  var expected = '/foo/bar/../../';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with \\';

  var path = 'foo/bar\\';
  var expected = 'foo/bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/\\bar\\';
  var expected = 'foo//bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\\foo/bar/..';
  var expected = '/foo/bar/..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\\foo\\bar/../..';
  var expected = '/foo/bar/../..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\\foo\\bar/../../';
  var expected = '/foo/bar/../../';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with \/';

  var path = 'foo/bar\/';
  var expected = 'foo/bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = 'foo/\/bar\/';
  var expected = 'foo//bar/';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\/foo/bar/..';
  var expected = '/foo/bar/..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\/foo\/bar/../..';
  var expected = '/foo/bar/../..';
  var got = _.path.refine( path );
  test.identical( got, expected );

  var path = '\/foo\/bar/../../';
  var expected = '/foo/bar/../../';
  var got = _.path.refine( path );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( 'a', 'b' ) );

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.refine( NaN ) );
}

//

function normalize( test )
{
  test.case = 'posix path';

  var path = 'a/foo/../b';
  var expected = 'a/b';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/aa/bb/cc/./';
  var expected = '/aa/bb/cc/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/c/temp/x/foo/../';
  var expected = '/c/temp/x/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/c/temp//foo/../';
  var expected = '/c/temp//';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/c/temp//foo/bar/../..';
  var expected = '/c/temp/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/c/temp//foo/bar/../../';
  var expected = '/c/temp//';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/c/temp//foo/bar/../../.';
  var expected = '/c/temp/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '//b/x/c/../d/..e';
  var expected = '//b/x/d/..e';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '//b//c/../d/..e';
  var expected = '//b//d/..e';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '//b//././c/../d/..e';
  var expected = '//b//d/..e';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/aa/bb/cc/';
  var expected = '/aa/bb/cc/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = '/foo/bar//baz/asdf';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/.';
  var expected = '/foo/bar//baz/asdf/quux';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/./';
  var expected = '/foo/bar//baz/asdf/quux/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/../';
  var expected = '/foo/bar//baz/asdf/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '//foo/bar//baz/asdf/quux/..//';
  var expected = '//foo/bar//baz/asdf//';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = 'foo/bar//baz/asdf/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'windows path';

  var path = '/C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C:/temp//foo/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '\\C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C:/temp//foo/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C/temp//foo/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp//foo//';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp//foo//';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var expected = '/C/temp/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
  var expected = '/C/temp//';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
  var expected = '/C/temp/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'empty path';

  var path = '';
  var expected = '';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/';
  var expected = '/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '//';
  var expected = '//';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '///';
  var expected = '///';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/.';
  var expected = '/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = '/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '.';
  var expected = '.';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = './';
  var expected = './';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = './.';
  var expected = '.';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the middle';

  var path = 'foo/./bar/baz';
  var expected = 'foo/bar/baz';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/././bar/baz/';
  var expected = 'foo/bar/baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/././bar/././baz/';
  var expected = 'foo/bar/baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/././bar/././baz/';
  var expected = '/foo/bar/baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/.x./baz/';
  var expected = '/foo/.x./baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the beginning';

  var path = './foo/bar';
  var expected = './foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '.\\foo\\bar';
  var expected = './foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '././foo/bar/';
  var expected = './foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = './/.//foo/bar/';
  var expected = './//foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/.//.//foo/bar/';
  var expected = '///foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '.x/foo/bar';
  var expected = '.x/foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '.x./foo/bar';
  var expected = '.x./foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = './x/.';
  var expected = './x';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the end';

  var path = 'foo/bar.';
  var expected = 'foo/bar.';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/.bar.';
  var expected = 'foo/.bar.';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar/.';
  var expected = 'foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar/./.';
  var expected = 'foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar/././';
  var expected = 'foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/bar/././';
  var expected = '/foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/baz/.x./';
  var expected = '/foo/baz/.x./';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the middle';

  var path = 'foo/../bar/baz';
  var expected = 'bar/baz';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/baz/';
  var expected = '../bar/baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '../../baz';
  var expected = '../../baz';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/../../baz';
  var expected = '/../../baz';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/../../baz/';
  var expected = '/../../baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/../../baz/';
  var expected = '../../baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/../../bar';
  var expected = '/../bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/../..x/baz/';
  var expected = '/../..x/baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/../x../baz/';
  var expected = '/../x../baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/x../../baz/';
  var expected = '/baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/..x/../baz/';
  var expected = '/baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/../../bar/../../baz/';
  var expected = '/../../baz/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the beginning';

  var path = '../foo/bar';
  var expected = '../foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '../../foo/bar/';
  var expected = '../../foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var expected = '..//foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/../foo/bar';
  var expected = '/../foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/x//../foo/bar';
  var expected = '/x/foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/..//../foo/bar/';
  var expected = '/../foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/..//..//foo/bar/';
  var expected = '/..//foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '..x/foo/bar';
  var expected = '..x/foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var expected = '..x../foo/bar';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the end';

  var path = 'foo/bar..';
  var expected = 'foo/bar..';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/..bar..';
  var expected = 'foo/..bar..';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar/..';
  var expected = 'foo';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar/../..';
  var expected = '.';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../';
  var expected = './';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/../';
  var expected = '/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/foo/bar/../../';
  var expected = '/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../..';
  var expected = '..';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../../..';
  var expected = '../..';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/../bar/../../../..';
  var expected = '../../..';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." and "." combined';

  var path = '/abc/./../a/b';
  var expected = '/a/b';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/abc/.././a/b';
  var expected = '/a/b';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/abc/./.././a/b';
  var expected = '/a/b';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/a/b/abc/../.';
  var expected = '/a/b';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./..';
  var expected = '/a/b';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./../.';
  var expected = '/a/b';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = './../.';
  var expected = '..';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = './.././';
  var expected = '../';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = './..';
  var expected = '..';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '../.';
  var expected = '..';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with \/';

  var path = 'foo/bar\/';
  var expected = 'foo/bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = 'foo/\/bar\/';
  var expected = 'foo//bar/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '\/foo/bar/..';
  var expected = '/foo';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '\/foo\/bar/../..';
  var expected = '/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  var path = '\/foo\/bar/../../';
  var expected = '/';
  var got = _.path.normalize( path );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( 'a', 'b' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.normalize( NaN ) );
}

normalize.timeOut = 10000;

//

function normalizeTolerant( test )
{

  /* */

  test.case = 'posix path';

  var path = '/aa/bb/cc/./';
  var expected = '/aa/bb/cc/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '//b//././c/../d/..e';
  var expected = '/b/d/..e';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/aa/bb/cc/';
  var expected = '/aa/bb/cc/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = '/foo/bar/baz/asdf';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/.';
  var expected = '/foo/bar/baz/asdf/quux';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/./';
  var expected = '/foo/bar/baz/asdf/quux/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/../';
  var expected = '/foo/bar/baz/asdf/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '//foo/bar//baz/asdf/quux/..//';
  var expected = '/foo/bar/baz/asdf/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = 'foo/bar/baz/asdf';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'winoows path';

  var path = 'C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C/temp/foo/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp/foo/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp/foo/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var expected = '/C/temp';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
  var expected = '/C/temp/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
  var expected = '/C/temp';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'empty path';

  var path = '';
  var expected = '';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/';
  var expected = '/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '//';
  var expected = '/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '///';
  var expected = '/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/.';
  var expected = '/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = '/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '.';
  var expected = '.';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = './.';
  var expected = '.';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = './';
  var expected = './';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the middle';

  var path = 'foo/./bar/baz';
  var expected = 'foo/bar/baz';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/././bar/baz/';
  var expected = 'foo/bar/baz/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/././bar/././baz/';
  var expected = 'foo/bar/baz/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/././bar/././baz/';
  var expected = '/foo/bar/baz/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/.x./baz/';
  var expected = '/foo/.x./baz/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the beginning';

  var path = './foo/bar';
  var expected = './foo/bar';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '././foo/bar/';
  var expected = './foo/bar/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = './/.//foo/bar/';
  var expected = './foo/bar/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/.//.//foo/bar/';
  var expected = '/foo/bar/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '.x/foo/bar';
  var expected = '.x/foo/bar';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '.x./foo/bar';
  var expected = '.x./foo/bar';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = './x/.';
  var expected = './x';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the end';

  var path = 'foo/bar.';
  var expected = 'foo/bar.';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/.bar.';
  var expected = 'foo/.bar.';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/.';
  var expected = 'foo/bar';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/./.';
  var expected = 'foo/bar';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/././';
  var expected = 'foo/bar/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar/././';
  var expected = '/foo/bar/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/baz/.x./';
  var expected = '/foo/baz/.x./';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the middle';

  var path = 'foo/../bar/baz';
  var expected = 'bar/baz';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/baz/';
  var expected = '../bar/baz/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/../../baz/';
  var expected = '../../baz/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/../../bar/../../baz/';
  var expected = '/../../baz/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the beginning';

  var path = '../foo/bar';
  var expected = '../foo/bar';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '../../foo/bar/';
  var expected = '../../foo/bar/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var expected = '../../foo/bar/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/..//..//foo/bar/';
  var expected = '/../../foo/bar/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '..x/foo/bar';
  var expected = '..x/foo/bar';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var expected = '..x../foo/bar';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the end';

  var path = 'foo/bar..';
  var expected = 'foo/bar..';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/..bar..';
  var expected = 'foo/..bar..';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/..';
  var expected = 'foo';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../';
  var expected = './';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/../..';
  var expected = '.';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar/../../';
  var expected = '/';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../..';
  var expected = '..';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../../..';
  var expected = '../..';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/../bar/../../../..';
  var expected = '../../..';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." and "." combined';

  var path = '/abc/./../a/b';
  var expected = '/a/b';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/abc/.././a/b';
  var expected = '/a/b';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/abc/./.././a/b';
  var expected = '/a/b';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/a/b/abc/../.';
  var expected = '/a/b';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./..';
  var expected = '/a/b';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./../.';
  var expected = '/a/b';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = './../.';
  var expected = '..';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = './.././';
  var expected = '../';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = './..';
  var expected = '..';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '../.';
  var expected = '..';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );

  var path = '.././';
  var expected = '../';
  var got = _.path.normalizeTolerant( path );
  test.identical( got, expected );
}

//

function canonize( test )
{

  var path = 'a/foo/../b';
  var expected = 'a/b';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'posix path';

  var path = '/aa/bb/cc/./';
  var expected = '/aa/bb/cc';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '//b//././c/../d/..e';
  var expected = '//b//d/..e';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/aa/bb/cc/';
  var expected = '/aa/bb/cc';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = '/foo/bar//baz/asdf';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/.';
  var expected = '/foo/bar//baz/asdf/quux';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/./';
  var expected = '/foo/bar//baz/asdf/quux';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/../';
  var expected = '/foo/bar//baz/asdf';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '//foo/bar//baz/asdf/quux/..//';
  var expected = '//foo/bar//baz/asdf//';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/.';
  var expected = 'foo/bar//baz/asdf/quux';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = 'foo/bar//baz/asdf';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'windows path';

  var path = '/C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C:/temp//foo';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '\\C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C:/temp//foo';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C/temp//foo';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp//foo//';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp//foo//';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var expected = '/C/temp';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var expected = '/C/temp';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
  var expected = '/C/temp';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
  var expected = '/C/temp';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'empty path';

  var path = '';
  var expected = '';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/';
  var expected = '/';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '//';
  var expected = '//';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '///';
  var expected = '///';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/.';
  var expected = '/';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = '/';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '.';
  var expected = '.';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = './';
  var expected = '.';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = './.';
  var expected = '.';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the middle';

  var path = 'foo/./bar/baz';
  var expected = 'foo/bar/baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/././bar/baz/';
  var expected = 'foo/bar/baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/././bar/././baz/';
  var expected = 'foo/bar/baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/././bar/././baz/';
  var expected = '/foo/bar/baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/.x./baz/';
  var expected = '/foo/.x./baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the beginning';

  var path = './foo/bar';
  var expected = 'foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '././foo/bar/';
  var expected = 'foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = './././foo/bar/';
  var expected = 'foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '.\\foo\\bar';
  var expected = 'foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = './/.//foo/bar/';
  var expected = './//foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/.//.//foo/bar/';
  var expected = '///foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '.x/foo/bar';
  var expected = '.x/foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '.x./foo/bar';
  var expected = '.x./foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = './x/.';
  var expected = 'x';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the end';

  var path = 'foo/bar.';
  var expected = 'foo/bar.';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/.bar.';
  var expected = 'foo/.bar.';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar/.';
  var expected = 'foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar/./.';
  var expected = 'foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar/././';
  var expected = 'foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/bar/././';
  var expected = '/foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/baz/.x./';
  var expected = '/foo/baz/.x.';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the middle';

  var path = 'foo/../bar/baz';
  var expected = 'bar/baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/baz/';
  var expected = '../bar/baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/../../baz/';
  var expected = '../../baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/../../bar/../../baz/';
  var expected = '/../../baz';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the beginning';

  var path = '../foo/bar';
  var expected = '../foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '../../foo/bar/';
  var expected = '../../foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var expected = '..//foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/..//..//foo/bar/';
  var expected = '/..//foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '..x/foo/bar';
  var expected = '..x/foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var expected = '..x../foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the end';

  var path = 'foo/bar..';
  var expected = 'foo/bar..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/..bar..';
  var expected = 'foo/..bar..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar/..';
  var expected = 'foo';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar/../..';
  var expected = '.';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../';
  var expected = '.';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/foo/bar/../../';
  var expected = '/';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../..';
  var expected = '..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../../..';
  var expected = '../..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/../bar/../../../..';
  var expected = '../../..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." and "." combined';

  var path = '/abc/./../a/b';
  var expected = '/a/b';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/abc/.././a/b';
  var expected = '/a/b';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/abc/./.././a/b';
  var expected = '/a/b';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/a/b/abc/../.';
  var expected = '/a/b';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./..';
  var expected = '/a/b';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./../.';
  var expected = '/a/b';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = './../.';
  var expected = '..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = './.././';
  var expected = '..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = './..';
  var expected = '..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '../.';
  var expected = '..';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with \/';

  var path = 'foo/bar\/';
  var expected = 'foo/bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = 'foo/\/bar\/';
  var expected = 'foo//bar';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '\/foo/bar/..';
  var expected = '/foo';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '\/foo\/bar/../..';
  var expected = '/';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  var path = '\/foo\/bar/../../';
  var expected = '/';
  var got = _.path.canonize( path );
  test.identical( got, expected );

  /* */

  if( !Config.debug )
  return;

  test.case = 'No arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( ) );

  test.case = 'Two arguments';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( 'a', 'b' ) );

  // Input is not path

  test.case = 'No path - regexp';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( /foo/ ) );

  test.case = 'No path - number';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( 3 ) );

  test.case = 'No path - array';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( [ '/C/', 'work/f' ] ) );

  test.case = 'No path - object';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( { Path : 'C:/foo/baz/bar' } ) );

  test.case = 'No path - undefined';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( undefined ) );

  test.case = 'No path - null';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( null ) );

  test.case = 'No path - NaN';
  test.shouldThrowErrorOfAnyKind( () => _.path.canonize( NaN ) );
}

//

function canonizeTolerant( test )
{
  /* */

  test.case = 'posix path';

  var path = '/aa/bb/cc/./';
  var expected = '/aa/bb/cc';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '//b//././c/../d/..e';
  var expected = '/b/d/..e';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/aa/bb/cc/';
  var expected = '/aa/bb/cc';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/..';
  var expected = '/foo/bar/baz/asdf';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/.';
  var expected = '/foo/bar/baz/asdf/quux';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/./';
  var expected = '/foo/bar/baz/asdf/quux';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar//baz/asdf/quux/../';
  var expected = '/foo/bar/baz/asdf';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '//foo/bar//baz/asdf/quux/..//';
  var expected = '/foo/bar/baz/asdf';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar//baz/asdf/quux/..//.';
  var expected = 'foo/bar/baz/asdf';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'winoows path';

  var path = 'C:\\temp\\\\foo\\bar\\..\\';
  var expected = '/C/temp/foo';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp/foo';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\\\';
  var expected = '/C/temp/foo';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..';
  var expected = '/C/temp';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\';
  var expected = '/C/temp';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'C:\\temp\\\\foo\\bar\\..\\..\\.';
  var expected = '/C/temp';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'empty path';

  var path = '';
  var expected = '';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/';
  var expected = '/';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '//';
  var expected = '/';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '///';
  var expected = '/';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/.';
  var expected = '/';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/./.';
  var expected = '/';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '.';
  var expected = '.';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = './.';
  var expected = '.';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = './';
  var expected = '.';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the middle';

  var path = 'foo/./bar/baz';
  var expected = 'foo/bar/baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/././bar/baz/';
  var expected = 'foo/bar/baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/././bar/././baz/';
  var expected = 'foo/bar/baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/././bar/././baz/';
  var expected = '/foo/bar/baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/.x./baz/';
  var expected = '/foo/.x./baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the beginning';

  var path = './foo/bar';
  var expected = 'foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '././foo/bar/';
  var expected = 'foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = './/.//foo/bar/';
  var expected = 'foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/.//.//foo/bar/';
  var expected = '/foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '.x/foo/bar';
  var expected = '.x/foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '.x./foo/bar';
  var expected = '.x./foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = './x/.';
  var expected = 'x';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with "." in the end';

  var path = 'foo/bar.';
  var expected = 'foo/bar.';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/.bar.';
  var expected = 'foo/.bar.';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/.';
  var expected = 'foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/./.';
  var expected = 'foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/././';
  var expected = 'foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar/././';
  var expected = '/foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/baz/.x./';
  var expected = '/foo/baz/.x.';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the middle';

  var path = 'foo/../bar/baz';
  var expected = 'bar/baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/baz/';
  var expected = '../bar/baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/../../bar/../../baz/';
  var expected = '../../baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/../../bar/../../baz/';
  var expected = '/../../baz';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the beginning';

  var path = '../foo/bar';
  var expected = '../foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '../../foo/bar/';
  var expected = '../../foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '..//..//foo/bar/';
  var expected = '../../foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/..//..//foo/bar/';
  var expected = '/../../foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '..x/foo/bar';
  var expected = '..x/foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '..x../foo/bar';
  var expected = '..x../foo/bar';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." in the end';

  var path = 'foo/bar..';
  var expected = 'foo/bar..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/..bar..';
  var expected = 'foo/..bar..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/..';
  var expected = 'foo';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../';
  var expected = '.';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/../..';
  var expected = '.';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/foo/bar/../../';
  var expected = '/';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../..';
  var expected = '..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/bar/../../../..';
  var expected = '../..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = 'foo/../bar/../../../..';
  var expected = '../../..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  /* */

  test.case = 'path with ".." and "." combined';

  var path = '/abc/./../a/b';
  var expected = '/a/b';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/abc/.././a/b';
  var expected = '/a/b';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/abc/./.././a/b';
  var expected = '/a/b';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/a/b/abc/../.';
  var expected = '/a/b';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./..';
  var expected = '/a/b';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '/a/b/abc/./../.';
  var expected = '/a/b';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = './../.';
  var expected = '..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = './.././';
  var expected = '..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = './..';
  var expected = '..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '../.';
  var expected = '..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );

  var path = '.././';
  var expected = '..';
  var got = _.path.canonizeTolerant( path );
  test.identical( got, expected );
}

//

function nativize( test )
{

  var src = 'A:\\';
  debugger;
  var got = _.path.nativize( src );
  var expected = 'A:\\';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );
  debugger;

  var src = '/A/';
  var got = _.path.nativize( src );
  var expected = 'A:\\';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );

  var src = '/A';
  var got = _.path.nativize( src );
  var expected = 'A:\\';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );

  var src = '/A/b';
  var got = _.path.nativize( src );
  var expected = 'A:\\b';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );

  var src = '/A/b/';
  var got = _.path.nativize( src );
  var expected = 'A:\\b\\';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );

}

//

function nativizeEscaping( test )
{
  var src = '/A/b/';
  var got = _.path.nativizeEscaping( src );
  var expected = 'A:\\b\\';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );

  var src = '"/A/b/"';
  var got = _.path.nativizeEscaping( src );
  var expected = '"A:\\b\\"';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );

  var src = '"/A/b c/"';
  var got = _.path.nativizeEscaping( src );
  var expected = '"A:\\b c\\"';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );

  var src = '"/A/b';
  var got = _.path.nativizeEscaping( src );
  var expected = '"A:\\b';
  if( process.platform === 'win32' )
  test.identical( got, expected );
  else
  test.identical( got, src );
}

//

function _nativizeEscapingWindows( test )
{
  var src = '/A/b/';
  var got = _.path._nativizeEscapingWindows( src );
  var expected = 'A:\\b\\';
  test.identical( got, expected );

  var src = '"/A/b/"';
  var got = _.path._nativizeEscapingWindows( src );
  var expected = '"A:\\b\\"';
  test.identical( got, expected );

  var src = '"/A/b c/"';
  var got = _.path._nativizeEscapingWindows( src );
  var expected = '"A:\\b c\\"';
  test.identical( got, expected );

  var src = '"/A/b';
  var got = _.path._nativizeEscapingWindows( src );
  var expected = '"A:\\b';
  test.identical( got, expected );
}

// //
//
// function escape( test )
// {
//
//   test.case = `"a`;
//   var src = `"a`;
//   var dst = `""a`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var got2 = _.path._unescape( got1 );
//   var exp =
//   {
//     wasEscaped : false,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `"#`;
//   var src = `"#`;
//   var dst = `"""#"`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `"!`;
//   var src = `"!`;
//   var dst = `"""!"`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var got2 = _.path._unescape( got1 );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `a"`;
//   var src = `a"`;
//   var dst = `a""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : false,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `#"`;
//   var src = `#"`;
//   var dst = `"#"""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `!"`;
//   var src = `!"`;
//   var dst = `"!"""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `"a"`;
//   var src = `"a"`;
//   var dst = `""a""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : false,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `"#"`;
//   var src = `"#"`;
//   var dst = `"""#"""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `"!"`;
//   var src = `"!"`;
//   var dst = `"""!"""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `""a""`;
//   var src = `""a""`;
//   var dst = `""""a""""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : false,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `""#""`;
//   var src = `""#""`;
//   var dst = `"""""#"""""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = `""!""`;
//   var src = `""!""`;
//   var dst = `"""""!"""""`
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
//   test.case = 'a1#/a2@/a3!/a4?/#a5/@a6/!a7/?a8/File1.txt';
//   var src = 'a1#/a2@/a3!/a4?/#a5/@a6/!a7/?a8/File1.txt';
//   var dst = `"a1#"/"a2@"/"a3!"/"a4?"/"#a5"/"@a6"/"!a7"/"?a8"/File1.txt`;
//   var got1 = _.path.escape( src );
//   test.identical( got1, dst );
//   var exp =
//   {
//     wasEscaped : true,
//     unescaped : src,
//   }
//   var got2 = _.path._unescape( got1 );
//   test.identical( got2, exp );
//   var got3 = _.path.unescape( got1 );
//   test.identical( got3, src );
//
// }

//

function _nativizeWindows( test )
{
  test.case = '\'\'';
  var src = '';
  var expected = '';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  test.case = '.';
  var src = '.';
  var expected = '.';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  test.case = './';
  var src = './';
  var expected = '.\\';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  test.case = 'a/b';
  var src = 'a/b';
  var expected = 'a\\b';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  test.case = 'a/b/';
  var src = 'a/b/';
  var expected = 'a\\b\\';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  /* */

  test.case = 'simple absolute path';
  var src = '/foo';
  var expected = '\\foo';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  /* */

  test.case = 'absolute path : nested dirs';
  var src = '/foo/bar/baz/text.txt';
  var expected = '\\foo\\bar\\baz\\text.txt';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = '/aa/bb';
  var expected = '\\aa\\bb';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = '/aa';
  var expected = '\\aa';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = '/';
  var expected = '\\';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  /* */

  test.case = 'relative path';

  var src = 'aa/bb';
  var expected = 'aa\\bb';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = 'aa';
  var expected = 'aa';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = 'aa/.';
  var expected = 'aa\\.';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = '.';
  var expected = '.';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = '..';
  var expected = '..';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  /* - */

  test.open( 'already native path' );

  test.case = 'simple absolute path';
  var src = '\\foo';
  var expected = '\\foo';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  /* */

  test.case = 'absolute path : nested dirs';
  var src = '\\foo\\bar\\baz\\text.txt';
  var expected = '\\foo\\bar\\baz\\text.txt';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = '\\aa\\bb';
  var expected = '\\aa\\bb';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = '\\aa';
  var expected = '\\aa';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  var src = '\\';
  var expected = '\\';
  var got = _.path._nativizeWindows( src );
  test.identical( got, expected );

  test.close( 'already native path' );

  if( !Config.debug )
  return;

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( () => _.path._nativizeWindows( 1 ) );
}

//

function _nativizePosix( test )
{
  test.case = '\'\'';
  var src = '';
  var expected = '';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = '.';
  var src = '.';
  var expected = '.';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = '..';
  var src = '..';
  var expected = '..';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = './';
  var src = './';
  var expected = './';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = 'a/b';
  var src = 'a/b';
  var expected = 'a/b';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = './a/b';
  var src = './a/b';
  var expected = './a/b';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = 'a/b/';
  var src = 'a/b/';
  var expected = 'a/b/';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = './a/b/';
  var src = './a/b/';
  var expected = './a/b/';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  /* */

  test.case = 'simple absolute path';
  var src = '/foo';
  var expected = '/foo';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = 'absolute path : nested dirs';
  var src = '/foo/bar/baz/text.txt';
  var expected = '/foo/bar/baz/text.txt';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  var src = '/aa/bb';
  var expected = '/aa/bb';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  var src = '/';
  var expected = '/';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  /* */

  test.case = '.';
  var src = '.';
  var expected = '.';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = '..';
  var src = '..';
  var expected = '..';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  /* - */

  test.open( 'windows native path' );

  test.case = '.\\';
  var src = '.\\';
  var expected = './';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = 'a\\b';
  var src = 'a\\b';
  var expected = 'a/b';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = 'a/b/';
  var src = 'a\\b\\';
  var expected = 'a/b/';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.case = 'simple absolute path';
  var src = '\\foo';
  var expected = '/foo';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  /* */

  test.case = 'absolute path : nested dirs';
  var src = '\\foo\\bar\\baz\\text.txt';
  var expected = '/foo/bar/baz/text.txt';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  var src = '\\aa\\bb';
  var expected = '/aa/bb';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  var src = '\\aa';
  var expected = '/aa';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  var src = '\\';
  var expected = '/';
  var got = _.path._nativizePosix( src );
  test.identical( got, expected );

  test.close( 'windows native path' );

  /* - */


  if( !Config.debug )
  return;

  test.case = 'wrong type of argument';
  test.shouldThrowErrorSync( () => _.path._nativizePosix( 1 ) );
}

_nativizePosix.experimental = true;

//

function dot( test )
{

  test.case = 'src - empty path';
  var src = '';
  var got = _.path.dot( src );
  var exp = './';
  test.identical( got, exp );

  test.case = 'src - filename';
  var src = 'a';
  var got = _.path.dot( src );
  var exp = './a';
  test.identical( got, exp );

  test.case = 'src - current directory';
  var src = '.';
  var got = _.path.dot( src );
  var exp = '.';
  test.identical( got, exp );

  test.case = 'src - filename begins with dot';
  var src = '.a';
  var got = _.path.dot( src );
  var exp = './.a';
  test.identical( got, exp );

  test.case = 'src - file in current directory';
  var src = './a';
  var got = _.path.dot( src );
  var exp = './a';
  test.identical( got, exp );

  test.case = 'src - up to current directory';
  var src = '..';
  var got = _.path.dot( src );
  var exp = '..';
  test.identical( got, exp );

  test.case = 'src - filename begins by double dot';
  var src = '..a';
  var got = _.path.dot( src );
  var exp = './..a';
  test.identical( got, exp );

  test.case = 'src - file in up to current directory';
  var src = '../a';
  var got = _.path.dot( src );
  var exp = '../a';
  test.identical( got, exp );

  if( !Config.debug )
  return;

  test.case = 'root directory';
  test.shouldThrowErrorSync( () => _.path.dot( '/' ) );

  test.case = 'file in root directory';
  test.shouldThrowErrorSync( () => _.path.dot( '/a' ) );
}

//

function undot( test )
{
  /* aaa : unwrap array and normalize all tests in this test suite */
  /* Dmytro : unwrapped and normalized this test routine and in all test suite */

  test.case = 'src - current directory with slash';
  var src = './';
  var got =  _.path.undot( src );
  var exp = './';
  test.identical( got, exp );

  test.case = 'src - current directory';
  var src = '.';
  var got =  _.path.undot( src );
  var exp = '.';
  test.identical( got, exp );

  test.case = 'src - up from current directory';
  var src = '..';
  var got =  _.path.undot( src );
  var exp = '..';
  test.identical( got, exp );

  test.case = 'src - current directory with filename';
  var src = './a';
  var got =  _.path.undot( src );
  var exp = 'a';
  test.identical( got, exp );

  test.case = 'src - filename';
  var src = 'a';
  var got =  _.path.undot( src );
  var exp = 'a';
  test.identical( got, exp );

  test.case = 'src - current directory, filename with dot at begin';
  var src = './.a';
  var got =  _.path.undot( src );
  var exp = '.a';
  test.identical( got, exp );

  test.case = 'src - current directory, filename with double dot at begin';
  var src = './..a';
  var got =  _.path.undot( src );
  var exp = '..a';
  test.identical( got, exp );

  test.case = 'src - file in directory with name ".", after root';
  var src = '/./a';
  var got =  _.path.undot( src );
  var exp = '/./a';
  test.identical( got, exp );
}

//

function absolute( test )
{
  test.case = '.';
  var src = '.';
  var expected = '/.';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  test.case = './';
  var src = './';
  var expected = '/./';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  test.case = 'a/b';
  var src = 'a/b';
  var expected = '/a/b';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  test.case = 'a/b/';
  var src = 'a/b/';
  var expected = '/a/b/';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  /* */

  test.case = 'simple absolute path';
  var src = '/foo';
  var expected = '/foo';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  /* */

  test.case = 'absolute path : nested dirs';
  var src = '/foo/bar/baz/text.txt';
  var expected = '/foo/bar/baz/text.txt';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  var src = '/aa/bb';
  var expected = '/aa/bb';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  var src = '/aa';
  var expected = '/aa';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  var src = '/';
  var expected = '/';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  /* */

  test.case = 'relative path';

  var src = 'aa/bb';
  var expected = '/aa/bb';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  var src = 'aa';
  var expected = '/aa';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  var src = 'aa/.';
  var expected = '/aa/.';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  var src = '.';
  var expected = '/.';
  var got = _.path.absolute( src );
  test.identical( got, expected );

  var src = '..';
  var expected = '/..';
  var got = _.path.absolute( src );
  test.identical( got, expected );
}

absolute.experimental = true;

//

function unabsolute( test )
{
  test.case = '/.';
  var src = '/.';
  var expected = '.';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  test.case = '/./';
  var src = './';
  var expected = './';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  test.case = '/a/b';
  var src = '/a/b';
  var expected = 'a/b';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  /* */

  test.case = 'simple absolute path';
  var src = '/foo';
  var expected = 'foo';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  /* */

  test.case = 'absolute path : nested dirs';
  var src = '/foo/bar/baz/text.txt';
  var expected = 'foo/bar/baz/text.txt';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  var src = '/aa/bb';
  var expected = 'aa/bb';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  var src = '/aa';
  var expected = 'aa';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  var src = '/';
  var expected = '';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  /* */

  test.case = 'relative path';

  var src = 'aa/bb';
  var expected = 'aa/bb';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  var src = 'aa';
  var expected = 'aa';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  var src = 'aa/.';
  var expected = 'aa/.';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  var src = '.';
  var expected = '.';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );

  var src = '..';
  var expected = '..';
  var got = _.path.unabsolute( src );
  test.identical( got, expected );
}

//

function dir( test )
{

  test.case = '.';
  var src = '.';
  var expected = '..';
  var got = _.path.dir( src );
  test.identical( got, expected );

  test.case = './';
  var src = './';
  var expected = '../';
  var got = _.path.dir( src );
  test.identical( got, expected );

  test.case = 'a/b';
  var src = 'a/b';
  var expected = 'a';
  var got = _.path.dir( src );
  test.identical( got, expected );

  test.case = 'a/b/';
  var src = 'a/b/';
  var expected = 'a/';
  var got = _.path.dir( src );
  test.identical( got, expected );

  /* */

  test.case = 'simple absolute path';
  var src = '/foo';
  var expected = '/';
  var got = _.path.dir( src );
  test.identical( got, expected );

  /* */

  test.case = 'absolute path : nested dirs';
  var src = '/foo/bar/baz/text.txt';
  var expected = '/foo/bar/baz';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = '/aa/bb';
  var expected = '/aa';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = '/aa';
  var expected = '/';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = '/';
  var expected = '/..';
  var got = _.path.dir( src );
  test.identical( got, expected );

  /* */

  test.case = 'relative path : nested dirs';

  var src = 'aa/bb';
  var expected = 'aa';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = 'aa';
  var expected = '.';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = 'aa/.';
  var expected = '.';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = '.';
  var expected = '..';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = '..';
  var expected = '../..';
  var got = _.path.dir( src );
  test.identical( got, expected );

  /* - */

  test.open( 'trailing slash' );

  var src = '/a/b/';
  var expected = '/a/';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = '/a/b/.';
  var expected = '/a';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = '/a/b/./';
  var expected = '/a/';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = 'a/b/';
  var expected = 'a/';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = 'a/b/.';
  var expected = 'a';
  var got = _.path.dir( src );
  test.identical( got, expected );

  var src = 'a/b/./';
  var expected = 'a/';
  var got = _.path.dir( src );
  test.identical( got, expected );

  test.close( 'trailing slash' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty path';
  test.shouldThrowErrorSync( function()
  {
    var got = _.path.dir( '' );
  });

  test.case = 'redundant argument';
  test.shouldThrowErrorSync( function()
  {
    var got = _.path.dir( 'a', 'b' );
  });

  test.case = 'passed argument is non string';
  test.shouldThrowErrorSync( function()
  {
    _.path.dir( {} );
  });
}

//

function dirDepthOption( test )
{
  test.case = 'root directory, depth - 0';
  var src = '/';
  var got = _.path.dir( src, 0 );
  var exp = '/';
  test.identical( got, exp );

  test.case = 'root directory, depth - 1';
  var src = '/';
  var got = _.path.dir( src, 1 );
  var exp = '/..';
  test.identical( got, exp );

  test.case = 'root directory, depth - 3';
  var src = '/';
  var got = _.path.dir( src, 3 );
  var exp = '/../../..';
  test.identical( got, exp );

  /* */

  test.case = 'absolute path, depth - 0';
  var src = '/a/b/c/d';
  var got = _.path.dir( src, 0 );
  var exp = '/a/b/c/d';
  test.identical( got, exp );

  test.case = 'absolute path, depth - 1';
  var src = '/a/b/c/d';
  var got = _.path.dir( src, 1 );
  var exp = '/a/b/c';
  test.identical( got, exp );

  test.case = 'absolute path, depth - 4';
  var src = '/a/b/c/d';
  var got = _.path.dir( src, 4 );
  var exp = '/';
  test.identical( got, exp );

  test.case = 'absolute path, depth > levels of nesting';
  var src = '/a/b/c/d';
  var got = _.path.dir( src, 6 );
  var exp = '/../..';
  test.identical( got, exp );

  /* */

  test.case = 'absolute path, depth - 0';
  var src = '/a/b/c/d/';
  var got = _.path.dir( src, 0 );
  var exp = '/a/b/c/d';
  test.identical( got, exp );

  test.case = 'absolute path, depth - 1';
  var src = '/a/b/c/d/';
  var got = _.path.dir( src, 1 );
  var exp = '/a/b/c/';
  test.identical( got, exp );

  test.case = 'absolute path, depth - 4';
  var src = '/a/b/c/d/';
  var got = _.path.dir( src, 4 );
  var exp = '/';
  test.identical( got, exp );

  test.case = 'absolute path, depth > levels of nesting';
  var src = '/a/b/c/d/';
  var got = _.path.dir( src, 6 );
  var exp = '/../..';
  test.identical( got, exp );

  /* */

  test.case = 'relative path, depth - 0';
  var src = './a/b/c/d';
  var got = _.path.dir( src, 0 );
  var exp = 'a/b/c/d';
  test.identical( got, exp );

  test.case = 'relative path, depth - 1';
  var src = './a/b/c/d';
  var got = _.path.dir( src, 1 );
  var exp = 'a/b/c';
  test.identical( got, exp );

  test.case = 'relative path, depth - 4';
  var src = './a/b/c/d';
  var got = _.path.dir( src, 4 );
  var exp = '.';
  test.identical( got, exp );

  test.case = 'relative path, depth > levels of nesting';
  var src = './a/b/c/d';
  var got = _.path.dir( src, 6 );
  var exp = '../..';
  test.identical( got, exp );

  /* */

  test.case = 'relative path, depth - 0';
  var src = 'a/b/c/d';
  var got = _.path.dir( src, 0 );
  var exp = 'a/b/c/d';
  test.identical( got, exp );

  test.case = 'relative path, depth - 1';
  var src = 'a/b/c/d';
  var got = _.path.dir( src, 1 );
  var exp = 'a/b/c';
  test.identical( got, exp );

  test.case = 'relative path, depth - 4';
  var src = 'a/b/c/d';
  var got = _.path.dir( src, 4 );
  var exp = '.';
  test.identical( got, exp );

  test.case = 'relative path, depth > levels of nesting';
  var src = 'a/b/c/d';
  var got = _.path.dir( src, 6 );
  var exp = '../..';
  test.identical( got, exp );

  /* */

  test.case = 'relative path, depth - 0';
  var src = './a/b/c/d/';
  var got = _.path.dir( src, 0 );
  var exp = 'a/b/c/d';
  test.identical( got, exp );

  test.case = 'relative path, depth - 1';
  var src = './a/b/c/d/';
  var got = _.path.dir( src, 1 );
  var exp = 'a/b/c/';
  test.identical( got, exp );

  test.case = 'relative path, depth - 4';
  var src = './a/b/c/d/';
  var got = _.path.dir( src, 4 );
  var exp = '.';
  test.identical( got, exp );

  test.case = 'relative path, depth > levels of nesting';
  var src = './a/b/c/d/';
  var got = _.path.dir( src, 6 );
  var exp = '../..';
  test.identical( got, exp );

  /* */

  test.case = 'relative path, depth - 0';
  var src = '../a/b/c/d';
  var got = _.path.dir( src, 0 );
  var exp = '../a/b/c/d';
  test.identical( got, exp );

  test.case = 'relative path, depth - 1';
  var src = '../a/b/c/d';
  var got = _.path.dir( src, 1 );
  var exp = '../a/b/c';
  test.identical( got, exp );

  test.case = 'relative path, depth - 4';
  var src = '../a/b/c/d';
  var got = _.path.dir( src, 4 );
  var exp = '..';
  test.identical( got, exp );

  test.case = 'relative path, depth > levels of nesting';
  var src = '../a/b/c/d';
  var got = _.path.dir( src, 6 );
  var exp = '../../..';
  test.identical( got, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'depth < 0';
  test.shouldThrowErrorSync( () => _.path.dir( '/a/b/c', -1 ) );
}

//

function dirFirst( test )
{

  test.case = '.';
  var src = '.';
  var expected = '../';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  test.case = './';
  var src = './';
  var expected = './';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  test.case = 'a/b';
  var src = 'a/b';
  var expected = 'a/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  test.case = 'a/b/';
  var src = 'a/b/';
  var expected = 'a/b/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  /* */

  test.case = 'simple absolute path';
  var src = '/foo';
  var expected2 = '/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected2 );

  /* */

  test.case = 'absolute path : nested dirs';
  var src = '/foo/bar/baz/text.txt';
  var expected = '/foo/bar/baz/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = '/aa/bb';
  var expected = '/aa/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = '/aa';
  var expected = '/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = '/';
  var expected = '/../';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  /* */

  test.case = 'relative path : nested dirs';

  var src = 'aa/bb';
  var expected = 'aa/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = 'aa';
  var expected = './';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = 'aa/.';
  var expected = './';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = '.';
  var expected = '../';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = '..';
  var expected = '../../';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  /* - */

  test.open( 'trailing slash' );

  var src = '/a/b/';
  var expected = '/a/b/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = '/a/b/.';
  var expected = '/a/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = '/a/b/./';
  var expected = '/a/b/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = 'a/b/';
  var expected = 'a/b/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = 'a/b/.';
  var expected = 'a/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  var src = 'a/b/./';
  var expected = 'a/b/';
  var got = _.path.dirFirst( src );
  test.identical( got, expected );

  test.close( 'trailing slash' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty path';
  test.shouldThrowErrorSync( function()
  {
    var got = _.path.dirFirst( '' );
  });

  test.case = 'redundant argument';
  test.shouldThrowErrorSync( function()
  {
    var got = _.path.dirFirst( 'a', 'b' );
  });

  test.case = 'passed argument is non string';
  test.shouldThrowErrorSync( function()
  {
    _.path.dirFirst( {} );
  });
}

//

function dirFirstDepthOption( test )
{
  test.case = 'root directory, depth - 1';
  var src = '/';
  var got = _.path.dirFirst( src, 1 );
  var exp = '/../';
  test.identical( got, exp );

  test.case = 'root directory, depth - 3';
  var src = '/';
  var got = _.path.dirFirst( src, 3 );
  var exp = '/../../../';
  test.identical( got, exp );

  /* */

  test.case = 'absolute path, depth - 1';
  var src = '/a/b/c/d';
  var got = _.path.dirFirst( src, 1 );
  var exp = '/a/b/c/';
  test.identical( got, exp );

  test.case = 'absolute path, depth - 4';
  var src = '/a/b/c/d';
  var got = _.path.dirFirst( src, 4 );
  var exp = '/';
  test.identical( got, exp );

  test.case = 'absolute path, depth > levels of nesting';
  var src = '/a/b/c/d';
  var got = _.path.dirFirst( src, 6 );
  var exp = '/../../';
  test.identical( got, exp );

  /* */

  test.case = 'absolute path, depth - 1';
  var src = '/a/b/c/d/';
  var got = _.path.dirFirst( src, 1 );
  var exp = '/a/b/c/d/';
  test.identical( got, exp );

  test.case = 'absolute path, depth - 4';
  var src = '/a/b/c/d/';
  var got = _.path.dirFirst( src, 4 );
  var exp = '/';
  test.identical( got, exp );

  test.case = 'absolute path, depth > levels of nesting';
  var src = '/a/b/c/d/';
  var got = _.path.dirFirst( src, 6 );
  var exp = '/../../';
  test.identical( got, exp );

  /* */

  test.case = 'relative path, depth - 1';
  var src = './a/b/c/d';
  var got = _.path.dirFirst( src, 1 );
  var exp = './a/b/c/';
  test.identical( got, exp );

  test.case = 'relative path, depth - 4';
  var src = './a/b/c/d';
  var got = _.path.dirFirst( src, 4 );
  var exp = './';
  test.identical( got, exp );

  test.case = 'relative path, depth > levels of nesting';
  var src = './a/b/c/d';
  var got = _.path.dirFirst( src, 6 );
  var exp = './../../';
  test.identical( got, exp );

  /* */

  test.case = 'relative path, depth - 1';
  var src = 'a/b/c/d';
  var got = _.path.dirFirst( src, 1 );
  var exp = 'a/b/c/';
  test.identical( got, exp );

  test.case = 'relative path, depth - 4';
  var src = 'a/b/c/d';
  var got = _.path.dirFirst( src, 4 );
  var exp = '.';
  test.identical( got, exp );

  test.case = 'relative path, depth > levels of nesting';
  var src = 'a/b/c/d';
  var got = _.path.dirFirst( src, 6 );
  var exp = './../../';
  test.identical( got, exp );

  /* */

  test.case = 'relative path, depth - 1';
  var src = './a/b/c/d/';
  var got = _.path.dirFirst( src, 1 );
  var exp = './a/b/c/d/';
  test.identical( got, exp );

  test.case = 'relative path, depth - 4';
  var src = './a/b/c/d/';
  var got = _.path.dirFirst( src, 4 );
  var exp = './';
  test.identical( got, exp );

  test.case = 'relative path, depth > levels of nesting';
  var src = './a/b/c/d/';
  var got = _.path.dirFirst( src, 6 );
  var exp = './../../';
  test.identical( got, exp );

  /* */

  test.case = 'relative path, depth - 1';
  var src = '../a/b/c/d';
  var got = _.path.dirFirst( src, 1 );
  var exp = '../a/b/c/';
  test.identical( got, exp );

  test.case = 'relative path, depth - 4';
  var src = '../a/b/c/d';
  var got = _.path.dirFirst( src, 4 );
  var exp = '../';
  test.identical( got, exp );

  test.case = 'relative path, depth > levels of nesting';
  var src = '../a/b/c/d';
  var got = _.path.dirFirst( src, 6 );
  var exp = '../../../';
  test.identical( got, exp );
}

//

/* qqq : extend */
function name( test )
{

  var src = '/foo/bar/baz.asdf';
  var expected = 'baz';
  test.identical( _.path.name( src ), expected );

  // test.case = 'get paths name';
  // var src =
  // [
  //   '',
  //   'some.txt',
  //   '/foo/bar/baz.asdf',
  //   '/foo/bar/.baz',
  //   '/foo.coffee.md',
  //   '/foo/bar/baz'
  // ];
  // var expected =
  // [
  //   '',
  //   'some',
  //   'baz',
  //   '',
  //   'foo.coffee',
  //   'baz'
  // ];

  /* - */

  if( !Config.debug )
  return;

  test.case = 'incorrect input';
  test.shouldThrowErrorSync( () => _.paths.name() );
  test.shouldThrowErrorSync( () => _.paths.name( [ 'aa/bb/file.txt', 1 ] ) );

  test.case = 'inner array';
  test.shouldThrowErrorSync( () => _.paths.name( [ 'a/b/file.txt', [ '/a/d/file.js' ] ] ) );

  test.case = 'two arguments';
  test.shouldThrowErrorSync( () => _.paths.name( [ 'a/b/file.txt' ], [ 'a/c/file.js' ] ) );
}

//

/* qqq : extend */
function fullName( test )
{

  var src = '/foo/bar/baz.asdf';
  var expected = 'baz.asdf';
  test.identical( _.path.fullName( src ), expected );

  // test.case = 'get paths name with extension';
  // var src =
  // [
  //   '',
  //   'some.txt',
  //   '/foo/bar/baz.asdf',
  //   '/foo/bar/.baz',
  //   '/foo.coffee.md',
  //   '/foo/bar/baz'
  // ];
  // var expected =
  // [
  //   '',
  //   'some.txt',
  //   'baz.asdf',
  //   '.baz',
  //   'foo.coffee.md',
  //   'baz'
  // ];
  // test.identical( _.paths.fullName( src ), expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'incorrect input';
  test.shouldThrowErrorSync( () => _.paths.fullName() );
  test.shouldThrowErrorSync( () => _.paths.fullName( [ 'aa/bb/file.txt', 1 ] ) );

  test.case = 'inner array';
  test.shouldThrowErrorSync( () => _.paths.fullName( [ 'a/b/file.txt', [ '/a/d/file.js' ] ] ) );

  test.case = 'two arguments';
  test.shouldThrowErrorSync( () => _.paths.fullName( [ 'a/b/file.txt' ], [ 'a/c/file.js' ] ) );
}

// --
// declare
// --

const Proto =
{

  name : 'Tools.Path.l0.l3',
  silencing : 1,

  tests :
  {

    is,
    isRefined,
    isNormalized,
    isAbsolute,
    isRelative,
    isGlobal,
    isRoot,
    isDotted,
    isTrailed,

    begins,
    ends,

    refine,

    normalize,
    normalizeTolerant,
    canonize,
    canonizeTolerant,
    nativize,
    nativizeEscaping,
    _nativizeEscapingWindows,
    // escape,

    _nativizeWindows,
    _nativizePosix,

    dot,
    undot,
    absolute,
    unabsolute,
    dir,
    dirDepthOption,
    dirFirst,
    dirFirstDepthOption,
    name,
    fullName,

  },

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
