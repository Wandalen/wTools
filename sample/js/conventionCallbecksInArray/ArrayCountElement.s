require( 'wTools' );

// without equalizer or evalutor

let arr1 = [ 1, 2, 'a', 'b', true, 6, [ 1 ] ];
let elem1_1 = 1;
let elem1_2 = [ 1 ];

console.log( 'The number of element 1 in the array arr1:' );
console.log( wTools.longCountElement( arr1, elem1_1 ) );
// returns 1
console.log( 'The number of element [ 1 ] in the array arr1:' );
console.log( wTools.longCountElement( arr1, elem1_2 ) );
// returns 0

// with equilizer

let arr2 = [ 'a', 4, 6, 1, 3, true, 'd', [ 'str' ] ];
let elem2_1 = 'str';
let elem2_2 = [ 'str' ];
let equalizer = ( elem, arrEl ) => typeof elem !== typeof arrEl;

console.log( 'The number of element \'str\' in the array arr2:' );
console.log( wTools.longCountElement( arr2, elem2_1, equalizer ) );
// returns 6
console.log( 'The number of element [ \'str\' ] in the array arr2:' );
console.log( wTools.longCountElement( arr2, elem2_2, equalizer ) );
// returns 7

// with evalutor, one function

let arr3 = [ { v : 1 }, { v : 2 }, { v : 'str' }, { v : 2, e : 'str' } ];
let elem3 =  { v : 2 };
let evalutor = ( elem ) => elem.v;

console.log( 'The number of elements with pair \'v : 2\' in the array arr3:' );
console.log( wTools.longCountElement( arr3, elem3, evalutor ) );

// with evalutor, one function

let arr4 = [ { v : 1 }, { v : 2 }, { v : 'str' }, { v : 2, e : 'str' } ];
let elem4 =  'str';
let evalutor1 = ( arrEl ) => arrEl.v;
let evalutor2 = ( elem ) => elem;

console.log( 'The number of elements, which has value \'str\' in the array arr4:' );
console.log( wTools.longCountElement( arr3, elem4, evalutor1, evalutor2 ) );
// returns 1
