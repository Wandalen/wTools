const _ = require( '../../out/release/Main.s' );

var src = [ 1, 2, 3 ];
var arr = _.array.make( src );
console.log( arr );
/* log : [ 1, 2, 3 ] */
console.log( src === arr );
/* log : false */

console.log( _.map.is( Object.create( null ) ) );
/* log : true */
