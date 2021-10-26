/*
  A fourth argument can be provided to use as identifier whether to assign value or not
*/

console.log( '1_ImplementationWith4Arguments.s' );

function objectSetFieldWith4Params( /* obj, field, value, skip */ )
{
  let obj = arguments[ 0 ];
  let field = arguments[ 1 ];
  let value = arguments[ 2 ];
  let skip = arguments[ 3 ];

  if( skip === true )
  return;
  else
  obj[ field ] = value;
}

module.exports = objectSetFieldWith4Params;
