if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;


// Input element
var got = _.arrayReplace( [ 1, 2, 3, 3, 4 ], 3, 4 );
logger.log( 'Two elements Replaced', got )

var got = _.arrayReplaceOnce( [ 1, 2, 3, 3, 4 ], 3, 4 );
logger.log( 'Two elements Replaced', got )

var got = _.arrayReplaceOnceStrictly( [ 1, 2, 3, 4 ], 3, 5 );
logger.log( 'Two elements Replaced', got )

var got = _.arrayReplaced( [ 1, 2, 3, 3, 4 ], 3, 4 );
logger.log( 'Two elements Replaced', got )

var got = _.arrayReplacedOnce( [ 1, 2, 3, 3, 4 ], 3, 4 );
logger.log( 'Two elements Replaced', got )

var got = _.arrayReplacedOnceStrictly( [ 1, 2, 3, 4 ], 3, 5 );
logger.log( 'Two elements Replaced', got )


logger.log( 'Question' )

//Q Pablo
var dst = [ 0, 1, 2 ];
var got = _.arrayReplaceArrayOnce( dst, [ 0, 1, 2 ], [ 2, 0, 1 ] );
logger.log( 'Arrays Replaced', got ); // Output: [ 1, 0, 2 ] - Shouldn´t the expected output be [ 2, 0, 1 ]?

var dst = [ 0, 1, 2 ];
var got = _.arrayReplaceArraysOnce( dst, [ 0, 1, 2 ], [ 2, 0, 1 ] );
logger.log( 'Arrays Replaced', got ); // Output: [ 2, 0, 1 ] - but need to slice the array

var dst = [ 'a', 'b', 'c', 'd' ];
var got = _.arrayReplaceArrayOnce( dst, [ 'a', 'b', 'c' ], [ undefined, undefined, undefined ] );
logger.log( 'Arrays Replaced', got ); // Output: [ 2, 0, 1 ] - but need to slice the array
