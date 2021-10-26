/*
  Add/convert to a constant field
  What if we want to add `null` as the value ?
*/

console.log( '5_ImplementationAddConstantField.s' );

function objectSetFieldConstant( object, field, value )
{
  if( value === null )
  {
    Object.defineProperty
    (
      object,
      field,
      {
        enumerable : true,
        configurable : false,
        writable : false,
        value : object[ field ]
      }
    );
  }
  else
  {
    Object.defineProperty
    (
      object,
      field,
      {
        enumerable : true,
        configurable : false,
        writable : false,
        value
      }
    );
  }
}

module.exports = objectSetFieldConstant;
