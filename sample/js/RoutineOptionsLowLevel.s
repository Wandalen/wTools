
/* minimal require */

require( '../proto/wtools/abase/l0/Include1.s' );

//

let _ = _global_.wTools;

console.log( 'routineOptions, defaults in routine field' );
var routine = () => true;
routine.defaults = { a : null, b : 1 };
var options = { a : 1, b : undefined };
_.routine.options( routine, options );
console.log( options );
/* log : { a : 1, b : undefined } */

//

console.log( 'assertRoutineOptions, defaults in routine field' );
var routine = () => true;
routine.defaults = { a : null, b : 1 };
var options = { a : 1, b : null };
_.routine.assertOptions( routine, options );
console.log( options );
/* log : { a : 1, b : null } */
