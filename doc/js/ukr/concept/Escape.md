# Ескейпінг

Ескейпінг - процес маскування певного значення. Реалізується рутинами і засобами, щоб вирішити проблему обмеженості значень.
Неймспейс <code>_.escape</code> містить засоби для створення, перевірки та маніпулювання таких значень.

## План презентації

* Проблема обмеженості значень
  * Приклад - створення рутини <code>objectSetField</code>.
  * Приклад - використання рутини <code>objectSetField</code>
* Вирішення проблеми обмеженості значень : додавання 4 аргументу <code>skip</code>
  * Приклад - створення рутини <code>objectSetFieldWith4Params</code>.
  * Приклад - використання рутини <code>objectSetFieldWith4Params</code>.
* Вирішення проблеми обмеженості значень : третій аргумент - <code>valueDescriptor</code>
  * Приклад - створення рутини <code>objectSetFieldWithValueDescriptor</code>.
  * Приклад - використання рутини <code>objectSetFieldWithValueDescriptor</code>.
* Модифікована версія оригінальної проблеми : додавання константного поля
  * Приклад - створення рутини <code>objectSetFieldConstant</code>.
  * Приклад - використання рутини <code>objectSetFieldConstant</code>.
* Вирішення проблеми обмеженості значень : використання неймспейсу - <code>_.escape</code>
  * Приклад - створення рутини <code>objectSetFieldConstantWithEscape</code>.
  * Приклад - використання рутини <code>objectSetFieldConstantWithEscape</code>.
* Висновки

## Проблема обмеженості значень

Якщо значення зарезервоване в рутині для вибору алгоритму, воно не може бути використане, як звичайне.<br><br>
<em><b>Приклад</b> - створення рутини <code>objectSetField</code>.</em><br>
Рутина присвоює значення <code>value</code> ( якщо <code>value !== null</code> ) полю <code>field</code> об'єкта <code>obj</code>.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>function objectSetField( obj, field, value )
{
  if( value === null )
  return;
  else
  obj[ field ] = value;
}

module.exports = objectSetField;</code></pre>
</details>

<em><b>Приклад</b> - використання рутини <code>objectSetField</code>.</em><br>
❌ Проблема : неможливо присвоїти значення <code>null</code>, так як воно використовується в рутині, як спеціальне.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let objectSetField = require( './0_Implementation.s' );

let obj = {};

objectSetField( obj, 'field1', 1 );
objectSetField( obj, 'field2', 2 );
objectSetField( obj, 'field3', null );

console.log( obj );
/* log : { field1: 1, field2: 2 } */</code></pre>
</details>

## Обмеження накладені на реалізацію

* Не використовувати назви полів, як ідентифікатор
* Не переписувати логіку рутини
* Використовувати існуючий інтерфейс
* Значення визначене параметром, якщо не вказане інше
* Не змінювати перший аргумент
* Не змінювати другий аргумент

## Вирішення проблеми обмеженості значень : додавання 4 аргументу <code>skip</code>

<em><b>Приклад</b> - створення рутини <code>objectSetFieldWith4Params</code>.</em><br>
Рутина присвоює значення <code>value</code> ( якщо <code>skip !== trueLike</code> ) полю <code>field</code> об'єкта <code>obj</code>.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>function objectSetFieldWith4Params( obj, field, value, skip )
{
  if( skip === true )
  return;
  else
  obj[ field ] = value;
}
module.exports = objectSetFieldWith4Params;</code></pre>
</details>

<em><b>Приклад</b> - використання рутини <code>objectSetFieldWith4Params</code>.</em><br>
❌ Проблема : наявність додаткового 4 параметра
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let objectSetFieldWith4Params = require( './1_ImplementationWith4Arguments.s' );

let obj = {};

objectSetFieldWith4Params( obj, 'field1', 1 );
objectSetFieldWith4Params( obj, 'field2', 2 );
objectSetFieldWith4Params( obj, 'field3', null );
objectSetFieldWith4Params( obj, 'field4', null, true );

console.log( obj );
/* log : { field1: 1, field2: 2, field3: null } */</code></pre>
</details>

## Вирішення проблеми обмеженості значень : третій аргумент - <code>valueDescriptor</code>

<em><b>Приклад</b> - створення рутини <code>objectSetFieldWithValueDescriptor</code>.</em><br>
Рутина присвоює значення <code>valueDescriptor.value</code> ( якщо <code>valueDescriptor.skip !== trueLike</code> ) полю <code>field</code> об'єкта <code>obj</code>.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>function objectSetFieldWithValueDescriptor( obj, field, valueDescriptor )
{
  if( valueDescriptor.skip === true )
  return;
  else
  obj[ field ] = valueDescriptor.value;
}

module.exports = objectSetFieldWithValueDescriptor;</code></pre>
</details>

<em><b>Приклад</b> - використання рутини <code>objectSetFieldWithValueDescriptor</code>.</em><br>
❌ Проблема : необхідність змінювати інтерфейс, третій параметр - мапа з полями <code>skip</code> та <code>value</code>
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let objectSetFieldWithValueDescriptor = require( './4_ImplementationWithValueDescriptor.s' );

let obj = {};

objectSetFieldWithValueDescriptor( obj, 'field1', { value : 1, skip : false } );
objectSetFieldWithValueDescriptor( obj, 'field2', { value : 2, skip : false } );
objectSetFieldWithValueDescriptor( obj, 'field3', { value : null, skip : false } );
objectSetFieldWithValueDescriptor( obj, 'field4', { value : null, skip : true } );

console.log( obj );
/* log : { field1: 1, field2: 2, field3: null } */</code></pre>
</details>

## Модифікована версія оригінальної проблеми : додавання константного поля

<em><b>Приклад</b> - створення рутини <code>objectSetFieldConstant</code>.</em><br>
Ускладнена версія <code>objectSetField</code>, яка додає константне поле об'єкту <code>object</code>, якщо <code>value === null</code> - перетворює поле <code>field</code> на константне.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>function objectSetFieldConstant( object, field, value )
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

module.exports = objectSetFieldConstant;</code></pre>
</details>

<em><b>Приклад</b> - використання рутини <code>objectSetFieldConstant</code>.</em><br>
❌ Проблема : ідентична <code>objectSetField</code>.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let objectSetFieldConstant = require( './5_ImplementationAddConstantField.s' );

let obj = { fieldToBeRemained : 1, fieldToBeChanged : 2 };

objectSetFieldConstant( obj, 'field1', 1 );
objectSetFieldConstant( obj, 'fieldToBeChanged', 'changed' );
objectSetFieldConstant( obj, 'fieldToBeRemained', null );

console.log( Object.getOwnPropertyDescriptors( obj ) );
/*
log :
{
  fieldToBeRemained:
  {
    value: 1,
    writable: false,
    enumerable: true,
    configurable: false
  },
  fieldToBeChanged:
  {
    value: 'changed',
    writable: false,
    enumerable: true,
    configurable: false
  },
  field1:
  {
    value: 1,
    writable: false,
    enumerable: true,
    configurable: false
  }
}
*/</code></pre>
</details>

## Вирішення проблеми обмеженості значень : використання неймспейсу - <code>_.escape</code>

<em><b>Приклад</b> - створення рутини <code>objectSetFieldConstantWithEscape</code>.</em><br>
Рутина додає константне поле об'єкту <code>object</code>, якщо <code>value === null</code> - перетворює поле <code>field</code> на константне.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let _ = require( '../..' );

function objectSetFieldConstantWithEscape( object, field, value )
{
  if( _.escape.is( value ) )
  {
    Object.defineProperty
    (
      object,
      field,
      {
        enumerable : true,
        configurable : false,
        writable : false,
        value : _.escape.right( value )
      }
    );
  }
  else if( value === null )
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
  return object;
}

module.exports = objectSetFieldConstantWithEscape;
</code></pre>
</details>

<em><b>Приклад</b> - використання рутини <code>objectSetFieldConstantWithEscape</code>.</em><br>
✅  Переваги : інтерфейс не змінюється, уніфікація коду, вирішена проблема обмеженності значень.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let _ = require( '../..' );
let objectSetFieldConstantWithEscape = require( './6_ImplementationAddConstantFieldWithEscape.s' )

var src = { 'fieldToBeRemained' : 1 };
objectSetFieldConstantWithEscape( src, 'fieldToBeRemained', null );
console.log( 'src1 : ', Object.getOwnPropertyDescriptors( src ) );
/*
log :

src1 :
{
  fieldToBeRemained:
  {
    value: 1,
    writable: false,
    enumerable: true,
    configurable: false
  }
}
*/

var src2 = { 'fieldToBeChanged' : 1 };
objectSetFieldConstantWithEscape( src2, 'fieldToBeChanged', 'changed' );
console.log( 'src2 : ', Object.getOwnPropertyDescriptors( src2 ) );
/*
log :

src1 :
{
  fieldToBeChanged:
  {
    value: 'changed',
    writable: false,
    enumerable: true,
    configurable: false
  }
}
*/

var src3 = { 'fieldToBeChangedWithNull' : 1 };
objectSetFieldConstantWithEscape( src3, 'fieldToBeChangedWithNull', _.escape.make( null ) );
console.log( 'src3 : ', Object.getOwnPropertyDescriptors( src3 ) );
/*
log :

src1 :
{
  fieldToBeChangedWithNull:
  {
    value: null,
    writable: false,
    enumerable: true,
    configurable: false
  }
}
*/</code></pre>
</details>

## Висновки
  
Проблема може бути вирішена декількома способами, але ескейпінг дає універсальний метод. Не потрібно змінювати інтерфейс, код стає уніфікованим за рахунок використання одного паттерну та неймспейсу <code>_.escape</code>, а виконання очікуваним.</em>

<br>

[Повернутись до змісту](../README.md#Концепції) 
