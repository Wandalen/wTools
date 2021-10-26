let arrayMake = require( './4_implementArrayMake.s' );

let src = [ 1, 2, 3 ];
let got = arrayMake( src );

console.log( got ); /* log : [ 1, 2, 3 ] */
console.log( got === src ); /* log : false */
