let arrayFrom = require( './5_implementArrayFrom.s' );

let src = [ 1, 2, 3 ];
let got = arrayFrom( src );

console.log( got ); /* log : [ 1, 2, 3 ] */
console.log( got === src ); /* log : true */
