/*
  The problem of limited values :
  Set object field to a value.
  If value === null, do nothing

  What if we want to add `null` as the value ?
*/

console.log( '0_Imlementation.s' );

function objectSetField( obj, field, value )
{
  if( value === null )
  return;
  else
  obj[ field ] = value;
}

module.exports = objectSetField;
