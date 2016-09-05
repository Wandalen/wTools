( function _String_toStr2_test_s_( ) {

'use strict';

/*

 to run this test
 from the project directory run

 npm install
 node ./staging/abase/z.test/String.toStr2.test.s

 */

if( typeof module !== 'undefined' )
{

  require( '../wTools.s' );
  require( '../component/StringTools.s' );

  if( require( 'fs' ).existsSync( __dirname + '/../object/Testing.debug.s' ) )
    require( '../object/Testing.debug.s' );
  else
    require( 'wTesting' );

}

var _ = wTools;
var Self = {};


//
/*

\' 	single quote 	byte 0x27 in ASCII encoding
\" 	double quote 	byte 0x22 in ASCII encoding
\\ 	backslash 	byte 0x5c in ASCII encoding
\b 	backspace 	byte 0x08 in ASCII encoding
\f 	form feed - new page 	byte 0x0c in ASCII encoding
\n 	line feed - new line 	byte 0x0a in ASCII encoding
\r 	carriage return 	byte 0x0d in ASCII encoding
\t 	horizontal tab 	byte 0x09 in ASCII encoding
\v 	vertical tab 	byte 0x0b in ASCII encoding
\nnn 	arbitrary octal value 	byte nnn
\xnn 	arbitrary hexadecimal value 	byte nn
\unnnn 	universal character name
(arbitrary Unicode value);
may result in several characters 	code point U+nnnn
\Unnnnnnnn 	universal character name
(arbitrary Unicode value);
may result in several characters 	code point U+nnnnnnnn

source : http://en.cppreference.com/w/cpp/language/escape

*/
//

var reportChars = function()
{

  debugger;
  var r = '';
  for( var i = 0 ; i < 512 ; i++ )
  {
    var d = _.strDup( '0',4-i.toString( 16 ).length ) + i.toString( 16 );
    var u = eval( `"\\u${d}"` );
    r += `${d} : "${u}"\n`;
  }

  console.log( r );
  debugger;

}

/*reportChars();*/

//

var toStr = function( test )
{
  var cases =
  [
    {
      desc :  'empty arguments',
      src : [ {}, '', [] ],
      options : [ {} ],
      expected :[ '{}', '""', '[]' ]
    },

    {
      desc :  'Symbol test',
      src :
      [
        Symbol(),
        Symbol('sm'),
        Symbol('sx'),
        Symbol('sy')


      ],
      options :
      [
        {},
        {},
        { levels : 0 },
        { noAtomic : 1 },

      ],
      expected :
      [
        'Symbol()',
        'Symbol(sm)',
        'Symbol(sx)',
        ''
      ]
    },

    {
      desc :  'Number test',
      src :
      [
        Number(),
        5,
        15000,
        1222.222,
        1234.4321,
        15,
        99,
        22


      ],
      options :
      [
        {},
        {},
        { precision : 3 },
        { fixed : 1 },
        { noNumber : 1 },
        { noAtomic : 1 },
        { levels : 0 },
        { noRoutine : 1 }
      ],
      expected :
      [
        '0',
        '5',
        '1.50e+4',
        '1222.2',
        '',
        '',
        '99',
        '22'

      ]
    },

    {
      desc :  'String test',
      src :
      [
        String(),
        'sample',
        'sample2',
        'sample3',
        '\nsample4\n',
        'sample5',
        'sample6',
        '\nsample7'

      ],
      options :
      [
        { },
        { },
        { noAtomic : 1 },
        { noString : 1 },
        { escaping : 1 },
        { tab : '---' },
        { levels : 0 },
        { },
      ],
      expected :
      [
        '""',
        '"sample"',
        '',
        '',
        '"\\nsample4\\n"',
        '"sample5"',
        '"sample6"',
        '"\nsample7"'
      ]
    },

    {
      desc :  'boolean, null, undefined test',
      src :
      [
        Boolean(),
        true,
        false,
        1!=2,

        null,
        null,

        undefined,
        undefined
      ],
      options :
      [
        { },
        { },
        { levels : 0 },
        { onlyRoutines : 1 },

        { },
        { levels : 3 },

        { },
        { noAtomic : 1 }

      ],
      expected :
      [
        'false',
        'true',
        'false',
        '',

        'null',
        'null',

        'undefined',
        ''
      ]
    },

    {
      desc :  'Date test',
      src :
      [
        new Date( Date.UTC( 1993, 12, 12 ) ),
        new Date( 1990, 0, 0 ),
        new Date( 2016, 12, 8 ),
        new Date( 2016, 1, 2 ),
      ],
      options :
      [
        { },
        { },
        { levels : 0 },
        { noDate : 1 }
      ],
      expected :
      [
        '1994-01-12T00:00:00.000Z',
        '1989-12-30T22:00:00.000Z',
        '2017-01-07T22:00:00.000Z',
        '',
      ]
    },

    {
      desc :  'Error test',
      src :
      [
        new Error(),
        new Error('msg'),
        new Error('msg2'),
        new Error('message'),
      ],
      options :
      [
        { },
        { },
        { levels : 0 },
        { noError : 1 }

      ],
      expected :
      [
        'Error',
        'Error: msg',
        '[object Error]',
        ''
      ]
    },

    {
      desc :  'Routine test',
      src :
      [
        function rr( ){ },
        function rx( ){ },
        [ function ry( ){ } , 1],
      ],
      options :
      [
        { },
        { noRoutine : 1 },
        { onlyRoutines : 1 },

      ],
      expected :
      [
        '[ routine rr ]',
        '',
        '',
      ]
    },

    {
      desc : 'Array test',
      src :
      [
      /*01*/ [ 1, 2, 3 ],
      /*02*/ [ { a : 1 }, { b : 2 } ],
      /*03*/ [ function( ){ }, function add( ){ } ],
      /*04*/ [ 1000, 2000, 3000 ],
      /*05*/ [ 1.1111, 2.2222, 3.3333 ],
      /*06*/ [ 0, 1, 2 ],
      /*07*/ [  7, { v : 0 }, 1, 'x' ],
      /*08*/ [ 'e', 'e', 'e' ],
      /*09*/ [ '\n\nEscaping test' ],
      /*10*/ [ 0, 1, 2 ],
      /*11*/ [ 'a','b','c', 1, 2, 3 ],
      /*12*/ [ { x : 1 }, { y : 2 } ],
      /*13*/ [ 1, [ 2, 3, 4 ], 5 ],
      /*14*/ [ 6, [ 7, 8, 9 ], 10 ],
      /*15*/ [ { k : 3 }, { l : 4 } ],
      /*16*/ [ 1, { a : 2 }, 5 ],
      /*17*/ [ 0, { b : 1 }, 3 ],
      /*18*/ [ 'a', 7, { u : 2 }, 8, 'b' ],
      /*19*/ [ function f1( ){ }, function ( ){ } ],
      /*20*/ [ function f2( ){ }, function f3( ){ } ],
      /*21*/ [ { a : { a : '1' } } ],
      /*22*/ [ { c : 1 }, { d : 2 } ],
      /*23*/ [ new Date( Date.UTC( 1993, 12, 12 ) ), { d : 2 }, new Error( 'error' ) ],
      /*24*/ [ function ff( ){ }, { d : 3 }, 15 ],
      /*25*/ [ [ 1, 2, 3 ],[ 4, 5, 6 ] ],
      /*26*/ [ [ 1, 2, 3 ],[ '4, 5, 6' ], undefined, false ],
      /*27*/ [ 'e', 'e', 'e' ],
      /*28*/ [ 'a','b','c', 1, 2, 3 ],
      /*29*/ [ 15, 16, 17, 18 ],
      /*30*/ [ { a : 5, b : 6, c : 7 } ],
      /*31*/ [ 'a', 1, function () { }, false ],
      /*32*/ [ 'b', 2, function () { }, true ],
      /*33*/ [ function () { } ],
      /*34*/ [ 'a', 1000, 2000, 3000 ],
      /*35*/ [ 1.1111, 2.2222, 3.3333 ],
      /*36*/ [  7, { v : 0 }, 1, 'x' ],
      /*37*/ [ '\n\nEscaping & wrap test' ],
      /*38*/ [ 0, [ 1, 2, 3, 4 ], 5, { a : 6 } ],
      /*39*/ [ ['a','b','c'],'d','e' ],
      /*40*/ [ { a : 0 }, { b : 1 }, [ 2, 3 ] ],
      /*41*/ [ { a : 'a', b : 'b', c : 'c' } ],
      /*42*/ [ 'a', 7, { u : 2 }, 8, 'b' ],
      /*43*/ [ 0.1111, 0.2222, 0.3333 ],
      /*44*/ [ 'x', 2000, 3000, 4000 ],
      /*45*/ [ 0, { b : 1 }, 3 ],
      /*46*/ [ { a : '\na', b : { d : '\ntrue' } } ],
      /*47*/ [ { x : '\na', y : { z : '\ntrue' } } ],
      /*48*/ [ 1, { a : 2 }, '03' ],
      /*49*/ [ 0, [ 1, 2, 3, 4 ], 5, { a : 6 } ],
      /*50*/ [ { a : 'string' }, [ true ], 1 ],
      /*51*/ [ [ 5, 4, [ 3 ] ],[ 2, 1, 0 ] ],
      /*52*/ [ { a : 0 }, { b : 1 }, [ 2, 3 ] ],
      /*53*/ [ [ 1.100,1.200 ], [ 2, 3 ] ],
      /*54*/ [ 9000,[ 8000, 6000], 7000 ],
      /*55*/ [ { a : '\\test' }, { b : '\ntest' }, { c : 'test' } ],
      /*56*/ [ { a : function func ( ){ } }, 0, 1, 'a' ],
      /*57*/ [ { b : function f ( ){ } }, 1, 2 , 3 ],
      /*58*/ [ new Error('msg'),new Date(1990, 0, 0),'test' ],
      /*59*/ [ 1, [ 2, 3, 4 ], 2 ],
      /*60*/ [ 1, [ '2', null,undefined, '4' ], 2 ],
      /*61*/ [ [ 1, 2 ],'string',{ a : true, b : null }, undefined ],
      /*62*/ [ [ 0, 1 ],'test',{ a : Symbol() }, undefined ],
      /*63*/ [ 0, 'str', { a : Symbol() },function test(){ }, null ],
      /*64*/ [ 0, 'str', { a : Symbol() },function test( ){ }, true, new Date(1990, 0, 0) ],
      /*65*/ [ [ 0, 1 ],'test', { a : 'a' } ],
      /*66*/ [ [ 1, 2 ],'sample', { a : 'b' } ],
      /*67*/ [ 11,22,function routine( ){ }, { a : 'string' } ],
      /*68*/ [ ['a',100],['b',200] ],
      /*69*/ [ ['aa',300],['bb',400] ],
      /*70*/ [ [ 1.00, 2.00 ], [ 3.00, 4.00],'str sample' ],
      /*71*/ [ '1', [ 2, 3, 4 ], '2' ],
      /*72*/ [ '1', [ 2.00, 3.00, 4.00 ], '2' ],
      /*73*/ [ 'o', [ 90, 80, 70 ], 'o' ],
      /*74*/ [ 'o', 1, { a : true, b : undefined, c : null } ],
      /*75*/ [ 'a', 2, { a : '\\true', b : true, c : null } ],
      /*76*/ [ [ 'a', 1 ], new Error( 'err msg' ), new Date(1990, 0, 0) ],
      /*77*/ [ [ 'a', 1 ], new Date(1999, 1, 1) ],
      /*78*/ [ [ 1, 2, 3 ], 'a' ],
      /*79*/ [ function routine( ){ }, 'a' ],

      ],
      options :
      [
      /*01*/ { },
      /*02*/ { },
      /*03*/ { },
      /*04*/ { precision : 2, multiline : 1 },
      /*05*/ { fixed : 2 },
      /*06*/ { noArray : 1 },
      /*07*/ { noAtomic : 1 },
      /*08*/ { noArray : 1 },
      /*09*/ { escaping : 1, levels : 2 },
      /*10*/ { levels : 0 },
      /*11*/ { levels : 2, noString : 1 },
      /*12*/ { levels : 2, dtab : '-' },
      /*13*/ { levels : 2, multiline : 1 },
      /*14*/ { levels : 2, noNumber : 1 },
      /*15*/ { levels : 2, colon : '->' },
      /*16*/ { levels : 2, noObject : 1 },
      /*17*/ { levels : 2, noNumber : 1 },
      /*18*/ { levels : 2, noAtomic : 1 },
      /*19*/ { levels : 2 },
      /*20*/ { levels : 2, noRoutine : 1},
      /*21*/ { levels : 3, noSubObject : 1 },
      /*22*/ { levels : 2, tab : '|', prependTab : 0 },
      /*23*/ { levels : 2, noError : 1, noDate : 1 },
      /*24*/ { levels : 2, noRoutine : 1, noSubObject : 1 },

      /*25*/  { wrap : 0, comma : ' | ' },
      /*26*/  { wrap : 0, noString : 1, noNumber: 1, comma : ', ' },
      /*27*/  { wrap : 0, comma : ' ' },
      /*28*/  { wrap : 0, prependTab : 0, comma : ', ' },
      /*29*/  { wrap : 0, tab : '| ', dtab : '', comma : '. ' },
      /*30*/  { wrap : 0, colon : '->', comma : '.' },
      /*31*/  { wrap : 0, noRoutine : 1, comma : '. ' },
      /*32*/  { wrap : 0, noAtomic : 1, comma : '. ' },
      /*33*/  { wrap : 0, onlyRoutines : 1, comma : '| ' },
      /*34*/  { wrap : 0, precision : 3, comma : '* ' },
      /*35*/  { wrap : 0, fixed : 3, comma : ', ' },
      /*36*/  { wrap : 0, multiline : 1, comma : '. ' },
      /*37*/  { wrap : 0, escaping : 1, comma : '. ' },

      /*38*/  { levels : 2, wrap : 0, comma : '- ' },
      /*39*/  { levels : 2, wrap : 0, comma : '. '},
      /*40*/  { levels : 2, wrap : 0, tab : '| ', dtab : '', comma : ', ' },
      /*41*/  { levels : 2, wrap : 0, colon : ' - ', comma : '| ' },
      /*42*/  { levels : 2, wrap : 0, prependTab : 0, comma : ', ' },
      /*43*/  { levels : 2, wrap : 0, fixed : 1, comma : '* ' },
      /*44*/  { levels : 2, wrap : 0, precision : 1, comma : ', ' },
      /*45*/  { levels : 2, wrap : 0, multiline : 1, comma : '| ' },
      /*46*/  { levels : 3, wrap : 0, escaping : 1, comma : '. ' },
      /*47*/  { levels : 4, wrap : 0, escaping : 1, comma : ', ' },
      /*48*/  { levels : 3, wrap : 0, noAtomic : 1, comma : ' ,' },
      /*49*/  { levels : 2, wrap : 0, noSubObject : 1, noArray : 1, comma : ' ..' },
      /*50*/  { levels : 2, wrap : 0, noString : 1, noNumber : 1, comma : '/ ' },

      /*51*/  { levels : 3, wrap : 0, comma : '||' },
      /*52*/  { levels : 2, wrap : 0, comma : ',, ', tab :'  |', colon : '->' },
      /*53*/  { levels : 2, prependTab : 0, fixed : 2 },
      /*54*/  { levels : 2, prependTab : 0, precision : 1 },
      /*55*/  { levels : 2, multiline : 1, escaping : 1 },
      /*56*/  { levels : 2, noRoutine : 1,},
      /*57*/  { levels : 3, noRoutine : 1,},
      /*58*/  { levels : 3, noError : 1, noDate : 1},
      /*59*/  { levels : 2, noArray : 1},
      /*60*/  { levels : 2, noNumber : 1, noString : 1},
      /*61*/  { levels : 2, noNumber : 1, noString : 1, noObject : 1 },
      /*62*/  { levels : 3, noNumber : 1, noString : 1, noObject : 1 },
      /*63*/  { levels : 2, noNumber : 1, noString : 1, noObject : 1, noRoutine : 1 },
      /*64*/  { levels : 2, noNumber : 1, noString : 1, noObject : 1, noRoutine : 1, noDate : 1 },
      /*65*/  { levels : 2, noNumber : 1, noString : 1, noSubObject : 1 },
      /*66*/  { levels : 3, noNumber : 1, noString : 1, noSubObject : 1 },
      /*67*/  { levels : 2, noNumber : 1, noString : 1, onlyRoutines : 1 },
      /*68*/  { levels : 2, noString : 1, precision : 2 },
      /*69*/  { levels : 3, noString : 1, precision : 3 },
      /*70*/  { levels : 2, noString : 1, fixed : 3 },
      /*71*/  { levels : 2, noString : 1, noNumber : 1, precision : 1 },
      /*72*/  { levels : 2, noString : 1, noNumber : 1, fixed : 1 },
      /*73*/  { levels : 3, noString : 1, noNumber : 1, precision : 1 },
      /*74*/  { levels : 2, noString : 1, noNumber : 1, multiline : 1 },
      /*75*/  { levels : 2, noString : 1, noNumber : 1, multiline : 1, escaping : 1 },
      /*76*/  { levels : 2, noString : 1, noNumber : 1, noError : 1 },
      /*77*/  { levels : 2, noString : 1, noNumber : 1, tab : '|', prependTab : 0 },
      /*78*/  { levels : 3, noAtomic : 1, noNumber : 0 },
      /*79*/  { levels : 2, onlyRoutines : 1, noRoutine : 1 },


      ],
      expected :
      [
      /*01*/ '[ 1, 2, 3 ]',
      /*02*/
      [
        '[',
        '  [ Object with 1 elements ], ',
        '  [ Object with 1 elements ]',
        ']'
      ].join( '\n' ),

      /*03*/ '[ [object Function], [object Function] ]',
      /*04*/
      [
        '[',
        '  1.0e+3, ',
        '  2.0e+3, ',
        '  3.0e+3',
        ']',
      ].join( '\n' ),

      /*05*/ '[ 1.11, 2.22, 3.33 ]',

      /*06*/ '',


      /*07*/
      [
        '[',
        '  7, ',
        '  [ Object with 1 elements ], ',
        '  1, ',
        '  "x"',
        ']'
      ].join( '\n' ),

      /*08*/ '',


      /*09*/ '[ "\\n\\nEscaping test" ]',

      /*10*/ '[ Array with 3 elements ]',
      /*11*/ '[ 1, 2, 3 ]',
      /*12*/
      [
        '[',
        '-{ x : 1 }, ',
        '-{ y : 2 }',
        ']'
      ].join( '\n' ),

      /*13*/
      [
        '[',
        '  1, ',
        '  [',
        '    2, ',
        '    3, ',
        '    4',
        '  ], ',
        '  5',
        ']',

      ].join( '\n' ),

      /*14*/
      [
        '[',
        '  [ 7, 8, 9 ]',
        ']'
      ].join( '\n' ),

      /*15*/
      [
        '[',
        '  { k->3 }, ',
        '  { l->4 }',
        ']'
      ].join( '\n' ),

      /*16*/
      [
        '[',
        '  1, ',
        '  5',
        ']',
      ].join( '\n' ),

      /*17*/
      [
        '[',
        '  { b : 1 }',
        ']'
      ].join( '\n' ),

      /*18*/
      [
        '[',
        '  { u : 2 }',
        ']'
      ].join( '\n' ),

      /*19*/ '[ [ routine f1 ], [ routine without name ] ]',
      /*20*/ '[  ]',
      /*21*/
      [
        '[',
        '  {',
        '    ',
        '  }',
        ']'
      ].join( '\n' ),

      /*22*/
      [
        '[',
        '|  { c : 1 }, ',
        '|  { d : 2 }',
        '|]',
      ].join( '\n' ),

      /*23*/
      [
        '[',
        '  { d : 2 }',
        ']',
      ].join( '\n' ),

      /*24*/
      [
        '[',
        '  { d : 3 }, ',
        '  15',
        ']',
      ].join( '\n' ),

      /*25*/
      [
        '  [ Array with 3 elements ] | ',
        '  [ Array with 3 elements ]',
      ].join( '\n' ),

      /*26*/
      [
        '  [ Array with 3 elements ], ',
        '  [ Array with 1 elements ], ',
        '  undefined, ',
        '  false',
      ].join( '\n' ),

      /*27*/
      '  "e" "e" "e"',

      /*28*/
      '"a", "b", "c", 1, 2, 3',
      /*29*/
      '| 15. 16. 17. 18',
      /*30*/
      '  [ Object with 3 elements ]',
      /*31*/
      '  "a". 1. [object Function]. false',
      /*32*/
      '  "b". 2. [object Function]. true',
      /*33*/
      '',
      /*34*/
      '  "a"* 1.00e+3* 2.00e+3* 3.00e+3',

      /*35*/
      '  1.111, 2.222, 3.333',
      /*36*/
      [
        '  7. ',
        '  [ Object with 1 elements ]. ',
        '  1. ',
        '  "x"',
      ].join( '\n' ),

      /*37*/
      '  ""...',

      /*38*/
      [
        '  0- ',
        '  1- 2- 3- 4- ',
        '  5- ',
        '  a : 6',
      ].join( '\n' ),

      /*39*/
      [
        '  "a". "b". "c". ',
        '  "d". ',
        '  "e"'
      ].join( '\n' ),

      /*40*/
      [
        '| a : 0, ',
        '| b : 1, ',
        '| 2, 3'
      ].join( '\n' ),

      /*41*/
      [
        '  a - "a"| b - "b"| c - "c"'
      ].join( '\n' ),

      /*42*/
      [
        '"a", ',
        '  7, ',
        '  u : 2, ',
        '  8, ',
        '  "b"',
      ].join( '\n' ),

      /*43*/
      [
        '  0.1* 0.2* 0.3',
      ].join( '\n' ),

      /*44*/
      [
        '  "x", 2e+3, 3e+3, 4e+3',
      ].join( '\n' ),

      /*45*/
      [
        '  0| ',
        '  b : 1| ',
        '  3',
      ].join( '\n' ),

      /*46*/
      [
        '  a : "\\na". ',
        '    b : d : ""...',
      ].join( '\n' ),

      /*47*/
      [
        '  x : "\\na", ',
        '    y : z : "\\ntrue"',
      ].join( '\n' ),

      /*48*/
      [
        '  ',
      ].join( '\n' ),

      /*49*/
      [
        '',
      ].join( '\n' ),

      /*50*/
      [
        '  a : "string"/ ',
        '  true'
      ].join( '\n' ),

      /*51*/
      [
        '  5||',
        '    4||',
        '    3||',
        '  2||1||0',
      ].join( '\n' ),

      /*52*/
      [
        '  |  a->0,, ',
        '  |  b->1,, ',
        '  |  2,, 3',
      ].join( '\n' ),

      /*53*/
      [
        '[',
        '  [ 1.10, 1.20 ], ',
        '  [ 2.00, 3.00 ]',
        ']',
      ].join( '\n' ),

      /*54*/
      [
        '[',
        '  9e+3, ',
        '  [ 8e+3, 6e+3 ], ',
        '  7e+3',
        ']',
      ].join( '\n' ),

      /*55*/
      [
        '[',
        '  {',
        '    a : "\\test"',
        '  }, ',
        '  {',
        '    b : ""...',
        '  }, ',
        '  {',
        '    c : "test"',
        '  }',
        ']',
      ].join( '\n' ),

      /*56*/
      [
        '[',
        '  { a : [object Function] }, ',
        '  0, ',
        '  1, ',
        '  "a"',
        ']',
      ].join( '\n' ),

      /*57*/
      [
        '[',
        '  {  }, ',
        '  1, ',
        '  2, ',
        '  3',
        ']',
      ].join( '\n' ),

      /*58*/
      [ '[',
        '  "test"',
        ']',
      ].join( '\n' ),

      /*59*/
       '',

      /*60*/

      [ '[',
        '  [ "2", null, undefined, "4" ]',
        ']',
      ].join( '\n' ),

      /*61*/

      [ '[',
        '  [ 1, 2 ], ',
        '  undefined',
        ']',
      ].join( '\n' ),

      /*62*/

      [ '[',
        '  [  ], ',
        '  undefined',
        ']',
      ].join( '\n' ),

      /*63*/

      [ '[',
        '  null',
        ']',
      ].join( '\n' ),

      /*64*/

      [ '[',
        '  true',
        ']',
      ].join( '\n' ),

      /*65*/

      [ '[',
        '  [ 0, 1 ], ',
        '  { a : "a" }',
        ']',
      ].join( '\n' ),

      /*66*/

      [ '[',
        '  [  ], ',
        '  {  }',
        ']',
      ].join( '\n' ),

      /*67*/

      [
        ''
      ].join( '\n' ),

      /*68*/

      [
        '[',
        '  [ "a", 1.0e+2 ], ',
        '  [ "b", 2.0e+2 ]',
        ']'
      ].join( '\n' ),

      /*69*/

      [
        '[',
        '  [ 300 ], ',
        '  [ 400 ]',
        ']'
      ].join( '\n' ),

      /*70*/

      [
        '[',
        '  [ 1.000, 2.000 ], ',
        '  [ 3.000, 4.000 ]',
        ']'
      ].join( '\n' ),

      /*71*/

      [
        '[',
        '  [ 2, 3, 4 ]',
        ']'
      ].join( '\n' ),

      /*72*/

      [
        '[',
        '  [ 2.0, 3.0, 4.0 ]',
        ']'
      ].join( '\n' ),

      /*73*/

      [
        '[',
        '  [  ]',
        ']'
      ].join( '\n' ),

      /*74*/

      [
        '[',
        '  {',
        '    a : true, ',
        '    b : undefined, ',
        '    c : null',
        '  }',
        ']'
      ].join( '\n' ),

      /*75*/

      [
        '[',
        '  {',
        '    a : "\\true", ',
        '    b : true, ',
        '    c : null',
        '  }',
        ']'
      ].join( '\n' ),

      /*76*/

      [
        '[',
        '  [ "a", 1 ], ',
        '  1989-12-30T22:00:00.000Z',
        ']'
      ].join( '\n' ),

      /*77*/

      [
        '[',
        '|  [ "a", 1 ], ',
        '|  1999-01-31T22:00:00.000Z',
        '|]'
      ].join( '\n' ),

      /*78*/

      [
        '[',
        '  [  ]',
        ']'
      ].join( '\n' ),

      /*79*/
       '',





      ]
    },

      {
        desc :  'Object test',
        src :
        [
        /*01*/  { a : 1, b : 2, c : 3 },
        /*02*/  { x : 3, y : 5, z : 5 },
        /*03*/  { q : 6, w : 7, e : 8 },
        /*04*/  { u : 12, i : { o : 13 }, p : 14 },
        /*05*/  { r : 9, t : { a : 10 }, y : 11 },
          /* redundant */
        /*06*/  { z : '01', x : { c : { g : 4 } }, v : '03' },
        /*07*/  { u : 12, i : { o : { x : { y : [ 1, 2, 3 ] } } }, p : 14 },
          /* redundant */
        /*08*/  { q : { a : 1 }, w : 'c', e : [1] },
        /*09*/  { z : '02', x : { c : { g : 6 } }, v : '01' },
        /*10*/  { h : { d : 1 }, g : 'c', c : [2] },
        /*11*/  { a : 6, b : 7, c : 1 },
        /*12*/  { a : true, b : '2', c : 3, d : undefined },
        /*13*/  { a : null, b : 1, c : '2', d : undefined, e : true, f : Symbol( 'symbol' ) },
        /*14*/  { a : 'true', b : 2, c : false, d : undefined },
        /*15*/  { e : new Error('msg') },
        /*16*/  { f : 1, g : function f(  ) { } },
        /*17*/  { x : function y(  ) { } },
        /*18*/  { a : null, b : 1, c : '2', d : undefined },
        /*19*/  { e : function r( ) { }, f : 1, g : '2', h : [ 1 ] },
        /*20*/  { i : 0, k : 1, g : 2, l : 3 },
        /*21*/  { o : 4, p : 5, r : 6, s : 7 },
        /*22*/  { m : 8, n : 9 },
        /*23*/  { x : '\n10', z : '\\11' },
        /*24*/  { a : 1, b : { d : 2 }, c : 3 },
        /*25*/  { a : 3, b : { d : 2 }, c : 1 },
        /*26*/  { a : 4, b : { d : 5 }, c : 6 },
        /*27*/  { a : 7, b : { d : 8 }, c : 9 },
        /*28*/  { a : 9, b : { d : 8 }, c : 7 },
        /*29*/  { a : 10, b : { d : 20 }, c : 30 },
        /*30*/  { a : 10.00, b : { d : 20.00 }, c : 30.00 },
        /*31*/  { a : 'a', b : { d : false }, c : 3 },
        /*32*/  { a : '\na', b : { d : '\ntrue' }, c : '\n' },
        /*33*/  { a : 'a', b : { d : false }, c : 3 },
        /*34*/  { a : 'aa', b : { d : true }, c : 40 },
        /*35*/  { a : ['a','b'], b : { d : 'true' }, c : 1 },
        /*36*/  { a : ['a','b'], b : { d : 'true' }, c : 1 },
        /*37*/  { a : 1, b : { d : 2 }, c : 3 },
        /*38*/  { a : 3, b : { d : 2 }, c : 1 },
        /*39*/  { a : 'bb', b : { d : false }, c : 30 },
        /*40*/  { a : 100, b : { d : 110 }, c : 120 },
        /*41*/  { a : '\na', b : { d : '\ntrue' } },
        /*42*/  { a : 'aa', b : { d : function(){ } } },
        /*43*/  { a : 'bb', b : { d : function(){ } } },
        /*44*/  { a : new Date( Date.UTC( 1993, 12, 12 ) ), b : { d : new Error('msg') }, c : 1 },
        /*45*/  { "sequence" : "\u001b[A", "name" : "undefined", "shift" : false, "code" : "[A"  },
        /*46*/  { "sequence" : "\x7f[A", "name" : "undefined", "shift" : false, "code" : "[A"  },
        /*47*/  { "sequence" : "<\u001cb>text<\u001cb>", "data" : { "name" : "myname", "age" : 1 }, "shift" : false, "code" : "<b>text<b>"  },
        /*48*/  { "sequence" : "\u0068\u0065\u004C\u004C\u006F", "shift" : false, "code" : "heLLo"  },
        /*49*/  { "sequence" : "\n\u0061\u0062\u0063", "shift" : false, "code" : "abc"  },
        /*50*/  { "sequence" : "\t\u005b\u0063\u0062\u0061\u005d\t", "data" : 100, "code" : "\n[cba]\n"  },
        /*51*/  { "sequence" : "\u005CABC\u005C", "data" : 100, "code" : "\\ABC\\"  },
        /*52*/  { "sequence" : "\u000Aline\u000A", "data" : null, "code" : "\nline\n"  },
        /*53*/  { "sequence" : "\rspace\r",  },
        /*54*/  { "sequence" : "\btest",  },
        /*55*/  { "sequence" : "\vsample",  },
        /*56*/  { "sequence" : "\ftest",  },
        /*57*/  { a : 1, b : { d : 'string' }, c : true },
        /*58*/  { a : 1, b : { d : 'string' }, c : new Date() },
        /*59*/  { a : 1000, b : { d : 'string' }, c : 1.500 },
        /*60*/  { a : 1000, b : 'text', c : 1.500 },
        /*61*/  { a : 1000, b : 'text', c : false, d : undefined, e : null},
        /*62*/  { a : 1001, b : 'text', c : false, d : undefined, e : null},
        /*63*/  { a : function routine( ){ }, b : 'a'},

        /*64*/  ( function( ) //own:0 option test
                {
                  var x = { a : 1, b : 2 },
                  y = Object.create( x );
                  y.c = 3;
                  return y;
                } )( ),

        /*65*/  ( function( ) //own:1 option test
                {
                  var x = { a : '0', b : '1' },
                  y = Object.create( x );
                  y.c = '3';
                  return y;
                } )( ),




        ],
        options :
        [
        /*01*/  { },
        /*02*/  { levels : 0 },
        /*03*/  { levels : 1 },
        /*04*/  { levels : 1 },
        /*05*/  { levels : 2 },

          /* redundant */
        /*06*/  { levels : 3 },
        /*07*/  { levels : 5 },
          /* redundant */

        /*08*/  { levels : 2, noSubObject : 1, noArray : 1 },
        /*09*/  { levels : 3, noAtomic : 1 },
        /*10*/  { levels : 2, noObject : 1 },


        /*11*/  { wrap : 0, comma : ' | ' },
        /*12*/  { wrap : 0, noString : 1, noNumber: 1, comma : ', ' },
        /*13*/  { wrap : 0, comma : '* ' },
        /*14*/  { wrap : 0, prependTab : 0, comma : '-> ' },
        /*15*/  { wrap : 0, tab : '| ', dtab : '', comma : '> ' },
        /*16*/  { wrap : 0, colon : '', comma : ' ' },
        /*17*/  { wrap : 0, noRoutine : 1, comma : '.. ' },
        /*18*/  { wrap : 0, noAtomic : 1, comma : ', ' },
        /*19*/  { wrap : 0, onlyRoutines : 1, comma : '<< ' },
        /*20*/  { wrap : 0, precision : 3, comma : '| ' },
        /*21*/  { wrap : 0,  fixed : 3, comma : '^ ' },
        /*22*/  { wrap : 0,  multiline : 1, comma : ', ' },
        /*23*/  { wrap : 0,  escaping : 1, comma : '| ' },

        /*24*/  { levels : 2, wrap : 0, comma : '. ' },
        /*25*/  { levels : 2, wrap : 0, comma : '. '},
        /*26*/  { levels : 2, wrap : 0, tab : '| ', dtab : '', comma : '@ ' },
        /*27*/  { levels : 2, wrap : 0, colon : ' - ', comma : '-? ' },
        /*28*/  { levels : 2, wrap : 0, prependTab : 0, comma : ', ' },
        /*29*/  { levels : 2, wrap : 0, fixed : 1, comma : '| ' },
        /*30*/  { levels : 2, wrap : 0, precision : 1, comma : '/ ' },
        /*31*/  { levels : 2, wrap : 0, multiline : 1, comma : ',, ' },
        /*32*/  { levels : 3, wrap : 0, escaping : 1, comma : '| ' },
        /*33*/  { levels : 2, wrap : 0, noAtomic : 1, comma : '< ' },
        /*34*/  { levels : 3, wrap : 0, noAtomic : 1, comma : ', ' },
        /*35*/  { levels : 2, wrap : 0, noSubObject : 1, noArray : 1, comma : '' },
        /*36*/  { levels : 2, wrap : 0, noString : 1, noNumber : 1, comma : '. ' },
        /*37*/  { levels : 2, wrap : 0, comma : '. '},
        /*38*/  { levels : 2, wrap : 0, comma : ',, ', tab :'  |', colon : '->' },
        /*39*/  { levels : 2, prependTab : 0, fixed : 5 },
        /*40*/  { levels : 2, prependTab : 0, precision : 5 },
        /*41*/  { levels : 2, multiline : 1, escaping : 1 },
        /*42*/  { levels : 2, noRoutine : 1,},
        /*43*/  { levels : 3, noRoutine : 1,},
        /*44*/  { levels : 3, noError : 1, noDate : 1},
        /*45*/  { },
        /*46*/  { },
        /*47*/  { },
        /*48*/  { multiline : 1 },
        /*49*/  { levels : 2, multiline : 1, escaping : 1 },
        /*50*/  { levels : 2, multiline : 1, escaping : 1 },
        /*51*/  { levels : 2, multiline : 1, escaping : 1 },
        /*52*/  { levels : 2, multiline : 1, escaping : 1 },
        /*53*/  { levels : 2, escaping : 1 },
        /*54*/  { levels : 2, escaping : 1 },
        /*55*/  { levels : 2, escaping : 1 },
        /*56*/  { levels : 2, escaping : 1 },
        /*57*/  { levels : 3, noNumber : 1, noString : 1},
        /*58*/  { levels : 3, noNumber : 1, noString : 1, noDate : 1},
        /*59*/  { levels : 2, noString : 1, fixed : 1},
        /*60*/  { levels : 2, noString : 1, precision : 1},
        /*61*/  { levels : 2, noString : 1, noNumber :1, tab : '-', prependTab : 0 },
        /*62*/  { levels : 2, noAtomic : 1, noNumber : 0 },
        /*63*/  { levels : 2, onlyRoutines : 1, noRoutine : 1 },
        /*64*/  { own : 0},
        /*65*/  {  },



        ],
        expected :
        [
        /*01*/  '{ a : 1, b : 2, c : 3 }',
        /*02*/  '[ Object with 3 elements ]',
        /*03*/  '{ q : 6, w : 7, e : 8 }',

        /*04*/
          [
            '{',
            '  u : 12, ',
            '  i : [ Object with 1 elements ], ',
            '  p : 14',
            '}'
          ].join( '\n' ),

        /*05*/
          [
            '{',
            '  r : 9, ',
            '  t : { a : 10 }, ',
            '  y : 11',
            '}'
          ].join( '\n' ),

          /* redundant */
        /*06*/
          [
            '{',
            '  z : "01", ',
            '  x : ',
            '  {',
            '    c : { g : 4 }',
            '  }, ',
            '  v : "03"',
            '}'
          ].join( '\n' ),

        /*07*/
          [
            '{',
            '  u : 12, ',
            '  i : ',
            '  {',
            '    o : ',
            '    {',
            '      x : ',
            '      {',
            '        y : [ 1, 2, 3 ]',
            '      }',
            '    }',
            '  }, ',
            '  p : 14',
            '}'
          ].join( '\n' ),
        /* redundant */

        /*08*/
          [
            '{',
            '  w : "c"',
            '}'
          ].join( '\n' ),

        /*09*/
          [
            '{',
            '  x : ',
            '  {',
            '    c : { g : 6 }',
            '  }',
            '}'
          ].join( '\n' ),

        /*10*/  '',





        /*11*/  '  a : 6 | b : 7 | c : 1',

        /*12*/
          [
            '  a : true, ',
            '  b : "2", ',
            '  c : 3, ',
            '  d : undefined'

          ].join( '\n' ),

        /*13*/
          [
            '  a : null* ',
            '  b : 1* ',
            '  c : "2"* ',
            '  d : undefined* ',
            '  e : true* ',
            '  f : Symbol(symbol)'

          ].join( '\n' ),

        /*14*/
          [
            'a : "true"-> ',
            '  b : 2-> ',
            '  c : false-> ',
            '  d : undefined'

          ].join( '\n' ),

        /*15*/

          '| e : [object Error]',

        /*16*/

          '  f1 g[object Function]',

        /*17*/

          '  x : [object Function]',

        /*18*/
        [
          '  a : null, ',
          '  b : 1, ',
          '  c : "2", ',
          '  d : undefined'

        ].join( '\n' ),

        /*19*/

        '',

        /*20*/

        [
          '  i : 0.00| ',
          '  k : 1.00| ',
          '  g : 2.00| ',
          '  l : 3.00'

        ].join( '\n' ),

        /*21*/

        [
          '  o : 4.000^ ',
          '  p : 5.000^ ',
          '  r : 6.000^ ',
          '  s : 7.000'

        ].join( '\n' ),

        /*22*/

        [
          '  m : 8, ',
          '  n : 9'

        ].join( '\n' ),

        /*23*/

        '  x : ""...| z : "\\11"',

        /*24*/
        [
          '  a : 1. ',
          '  b : d : 2. ',
          '  c : 3'

        ].join( '\n' ),

        /*25*/
        [
          '  a : 3. ',
          '  b : d : 2. ',
          '  c : 1'

        ].join( '\n' ),

        /*26*/
        [
          '| a : 4@ ',
          '| b : d : 5@ ',
          '| c : 6'

        ].join( '\n' ),

        /*27*/
        [
          '  a - 7-? ',
          '  b - d - 8-? ',
          '  c - 9'

        ].join( '\n' ),

        /*28*/
        [
          'a : 9, ',
          '  b : d : 8, ',
          '  c : 7'

        ].join( '\n' ),

        /*29*/
        [
          '  a : 10.0| ',
          '  b : d : 20.0| ',
          '  c : 30.0'

        ].join( '\n' ),

        /*30*/
        [
          '  a : 1e+1/ ',
          '  b : d : 2e+1/ ',
          '  c : 3e+1'

        ].join( '\n' ),

        /*31*/
        [
          '  a : "a",, ',
          '  b : ',
          '  d : false,, ',
          '  c : 3'

        ].join( '\n' ),

        /*32*/
        [
          '  a : "\\na"| ',
          '  b : d : "\\ntrue"| ',
          '  c : "\\n"'

        ].join( '\n' ),

        /*33*/
        '  b : d : false',

        /*34*/
        '  b : ',

        /*35*/
        '  c : 1',

        /*36*/
        [
          '  a : "a". "b". ',
          '  b : d : "true"'

        ].join( '\n' ),

        /*37*/
        [
          '  a : 1. ',
          '  b : d : 2. ',
          '  c : 3',
        ].join( '\n' ),

        /*38*/
        [
          '  |  a->3,, ',
          '  |  b->d->2,, ',
          '  |  c->1',
        ].join( '\n' ),

        /*39*/
        [
          '{',
          '  a : "bb", ',
          '  b : { d : false }, ',
          '  c : 30.00000',
          '}'

        ].join( '\n' ),

        /*40*/
        [
          '{',
          '  a : 100.00, ',
          '  b : { d : 110.00 }, ',
          '  c : 120.00',
          '}'

        ].join( '\n' ),

        /*41*/
        [
          '{',
          '  a : "\\na", ',
          '  b : ',
          '  {',
          '    d : ""...',
          '  }',
          '}'

        ].join( '\n' ),

        /*42*/
        [
          '{',
          '  a : "aa", ',
          '  b : { d : [object Function] }',
          '}'

        ].join( '\n' ),

        /*43*/
        [
          '{',
          '  a : "bb", ',
          '  b : {  }',
          '}'

        ].join( '\n' ),

        /*44*/
        [
          '{',
          '  b : ',
          '  {',
          '    ',
          '  }, ',
          '  c : 1',
          '}'

        ].join( '\n' ),

        /*45*/
        [
          '{',
          '  sequence : "\\u001b[A", ',
          '  name : "undefined", ',
          '  shift : false, ',
          '  code : "[A"',
          '}'

        ].join( '\n' ),




        /*46*/
        [
          '{',
          '  sequence : "\\x7f[A", ',
          '  name : "undefined", ',
          '  shift : false, ',
          '  code : "[A"',
          '}'

        ].join( '\n' ),

        /*47*/
        [
          '{',
          '  sequence : "<\\u001cb>text<\\u001cb>", ',
          '  data : [ Object with 2 elements ], ',
          '  shift : false, ',
          '  code : "<b>text<b>"',
          '}'

        ].join( '\n' ),

        /*48*/
        [
          '{',
          '  sequence : "heLLo", ',
          '  shift : false, ',
          '  code : "heLLo"',
          '}'

        ].join( '\n' ),

        /*49*/
        [
          '{',
          '  sequence : "\\nabc", ',
          '  shift : false, ',
          '  code : "abc"',
          '}'

        ].join( '\n' ),

        /*50*/
        [
          '{',
          '  sequence : "\\t[cba]\\t", ',
          '  data : 100, ',
          '  code : "\\n[cba]\\n"',
          '}'

        ].join( '\n' ),

        /*51*/
        [
          '{',
          '  sequence : "\\\\ABC\\\\", ',
          '  data : 100, ',
          '  code : "\\\\ABC\\\\"',
          '}'

        ].join( '\n' ),

        /*52*/
        [
          '{',
          '  sequence : "\\nline\\n", ',
          '  data : null, ',
          '  code : "\\nline\\n"',
          '}'

        ].join( '\n' ),

        /*53*/
        [
          '{ sequence : "\\rspace\\r" }'

        ].join( '\n' ),

        /*54*/
        [
          '{ sequence : "\\btest" }'

        ].join( '\n' ),

        /*55*/
        [
          '{ sequence : "\\vsample" }'

        ].join( '\n' ),

        /*56*/
        [
          '{ sequence : "\\ftest" }'

        ].join( '\n' ),

        /*57*/
        [
          '{',
          '  b : {  }, ',
          '  c : true',
          '}'


        ].join( '\n' ),

        /*58*/
        [
          '{',
          '  b : {  }',
          '}'


        ].join( '\n' ),

        /*59*/
        [
          '{',
          '  a : 1000.0, ',
          '  b : { d : "string" }, ',
          '  c : 1.5',
          '}'


        ].join( '\n' ),

        /*60*/
        [
          '{ a : 1e+3, c : 2 }',

        ].join( '\n' ),

        /*61*/
        [
          '{',
          '-  c : false, ',
          '-  d : undefined, ',
          '-  e : null',
          '-}'


        ].join( '\n' ),

        /*62*/
        [
          '{',
          '  ',
          '}'

        ].join( '\n' ),

        /*63*/
        [
          '',

        ].join( '\n' ),

        /*64*/
        [
          '{ c : 3, a : 1, b : 2 }',

        ].join( '\n' ),

        /*65*/
        [
          '{ c : "3" }',

        ].join( '\n' ),


        ],


      },
      {
       desc :  'wrapString test',
       src :
       [
       /*01*/ { a : "string",b : 1, c : null , d : undefined },
       /*02*/ { a : "sample",b : 0, c : false , d : [ "a" ] },
       /*03*/ { a : [ "example" ],b : 1, c : null , d : [ "b" ] },
       /*04*/ { a : "test", b : new Error( "err" ) },
       /*05*/ { a : "a", b : "b", c : { d : "d" } },
       /*06*/ { a : { h : "a" }, b : "b", c : { d : "d" } },



       ],
       options :
       [
       /*01*/ { wrapString : 0 },
       /*02*/ { levels : 2, wrapString : 0 },
       /*03*/ { levels : 3, wrapString : 0 },
       /*04*/ { levels : 2 },
       /*05*/ { wrapString: 0, levels : 1 },
       /*06*/ { wrapString: 0, levels : 2 },


       ],

       expected :
       [
        /*01*/
          [
           '{',
           '  a : "string", ',
           '  b : 1, ',
           '  c : null, ',
           '  d : undefined',
           '}'

         ].join( '\n' ),

        /*02*/
          [
           '{',
           '  a : sample, ',
           '  b : 0, ',
           '  c : false, ',
           '  d : [ "a" ]',
           '}'

         ].join( '\n' ),

        /*03*/
          [
           '{',
           '  a : [ example ], ',
           '  b : 1, ',
           '  c : null, ',
           '  d : [ b ]',
           '}'

         ].join( '\n' ),

        /*04*/
          [
           '{',
           '  a : "test", ',
           '  b : Error: err',
           '}'

         ].join( '\n' ),

        /*05*/
          [
           '{',
           '  a : a, ',
           '  b : b, ',
           '  c : [ Object with 1 elements ]',
           '}'

         ].join( '\n' ),

        /*05*/
          [
           '{',
           '  a : { h : a }, ',
           '  b : b, ',
           '  c : { d : d }',
           '}'

         ].join( '\n' ),


       ]



      },

      {
       desc :  'json test',
       src :
       [
       /*01*/ { "a" : 100, "b" : "c", "c" : { "d" : true, "e" : null } },
       /*02*/ { "b" : "a", "c" : 50, "d" : { "a" : "undefined", "e" : null } },
       /*03*/ [ { "a" : 100, "b" : "x", "c" : { "d" : true, "e" : null } } ],
       /*04*/ { a : "aa", b : [ 1,2,3 ], c : function r( ){ } },
       /*05*/ [ { a : 1, b : 2, c : { d : [ null,undefined ] } } ],
       /*06*/ { a : new Date( Date.UTC( 1993, 12, 12 ) ) },
       /*07*/ { a : new Error( "r" ) },
       /*08*/ { a : Symbol('sm') },

       ],
       options :
       [
       /*01*/ { json : 1 },
       /*02*/ { json : 1 },
       /*03*/ { json : 1 },
       /*04*/ { json : 1 },
       /*05*/ { json : 1 },
       /*06*/ { json : 1 },
       /*07*/ { json : 1 },
       /*08*/ { json : 1 },

       ],

       expected :
       [
        /*01*/
          [
           '{',
           ' "a" : 100, ',
           ' "b" : "c", ',
           ' "c" : { "d" : true, "e" : null }',
           '}'

         ].join( '\n' ),

        /*02*/
        [
          '{',
          '  "b" : "a", ',
          '  "c" : 50, ',
          '  "d" : { "a" : "undefined", "e" : null }',
          '}'

        ].join( '\n' ),

        /*03*/
        [
          '[',
          '  {',
          '    "a" : 100, ',
          '    "b" : "x", ',
          '    "c" : { "d" : true, "e" : null }',
          '  }',
          ']'

        ].join( '\n' ),

        /*04*/
        [
          '  {',
          '  "a" : "aa", ',
          '  "b" : [ 1, 2, 3 ], ',
          '  "c" : [ "routine r" ]',
          '  }',

        ].join( '\n' ),

        /*05*/
        [
          '[',
          '  {',
          '    "a" : 1, ',
          '    "b" : 2, ',
          '    "c" : ',
          '    {',
          '      "d" : [ null, null ]',
          '    }',
          '  }',
          ']'

        ].join( '\n' ),

        /*06*/
        [

          '{',
          '  "a" : "1994-01-12T00:00:00.000Z"',
          '}',


        ].join( '\n' ),

        /*07*/
        [

          '{',
          '  "a" : "Error: r"',
          '}',


        ].join( '\n' ),

        /*08*/
        [

          '{ "a" : "Symbol(sm)" }'


        ].join( '\n' ),

       ]



      },



  ];


  /**/

  if( Config.debug )
{

  test.description = 'wrong type of argument';
  test.shouldThrowError( function()
  {
    _.toStr( { a : 1 }, null );
  });

  test.description = '( o.precision ) is not between 1 and 21';
  test.shouldThrowError( function()
  {
    _.toStr( { a : 1 }, { precision : 0 } );
  });

  test.description = '( o.fixed ) is not between 0 and 20';
  test.shouldThrowError( function()
  {
    _.toStr( { a : 1 }, { fixed : 22 } );
  });

  test.description = 'if json : 1, wrapString must be equal to 1';
  test.shouldThrowError( function()
  {
    _.toStr( { a : 1 }, { json : 1, wrapString : 0 } );
  });

  test.description = 'wrong arguments count';
  test.shouldThrowError( function()
  {
    _.toStr( { a : 1 }, { b : 1 }, { json : 1 } );
  });



}

  debugger;
  for( var i = 0; i < cases.length; ++i )
  {
    var _case = cases[ i ];
    var src = _case['src'];
    var exp = _case['expected'];
    var o = _case['options'];
    var got = null;
    var result = null;
    var expected = null;
    for( var k = 0; k < src.length; ++k  )
    {
      test.description = _case.desc;
      got = _.toStr( src[ k ], o[ k ] || o[ 0 ]);

      if( test.description === 'json test' && o[ k ].json )
      {
        //if JSON.parse is OK,compare source vs parse result
        // else compare toStr() result vs expected
        expected = src[ k ]; //source obj
        try
        {
          result = JSON.parse( got );

        }
        catch( e )
        {
          console.log( e );
          result = got;
          expected = exp[k];
        }
        test.identical( result, expected );

      }

      else
      test.identical( got, exp[ k ] );



    }


  }
  debugger;

}

// node ./staging/abase/z.test/String.toStr2.test.s

//

var Proto =
{

  name : 'toStr',

  tests:
  {
      toStr : toStr


  }

};

_.mapExtend( Self,Proto );

if( typeof module !== 'undefined' && !module.parent )
  _.testing.test( Self );

} )( );
