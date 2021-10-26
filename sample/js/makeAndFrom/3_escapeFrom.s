if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;

let src = _.escape.make( 1 );
let got = _.escape.from( src );

console.log( got === src ); /* log : true */
