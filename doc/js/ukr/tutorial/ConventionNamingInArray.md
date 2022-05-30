# Конвенції рутин _.array*: найменування рутин

Принципи найменування рутин сімейства <code>Array</code>.

Рутини `array*` призначені для виконання операцій над масивами. Є п'ять операцій -`Append`, `Prepend`, `Remove`, `Flatten`, `Replace`, котрі утворюють сімейства рутин.

### Призначення сімейств рутин `array*`

- `Append` - додають елементи з другого аргумента в кінець масиву `dstArray`, що переданий в першому аргументі.
- `Prepend` - додають елементи з другого аргумента в початок масиву `dstArray`, що переданий в першому аргументі.
- `Remove` - видаляють елементи з масиву `dstArray`, котрі присутні в другому аргументі.
- `Flatten` - розгортають масиви до плаского стану.
- `Replace` - заміщують елементи масиву `dstArray` новими значеннями.

### Патерн найменування рутин сімейств `Append`, `Prepend`, `Remove`, `Flatten`, `Replace`

Патерн найменування рутин складається з чотирьох частин:

```
array[action][second][how many]
```

- `array` - незмінна частина для позначення того, що рутини виконують операції з масивами;
- `action` - дія, яка буде виконана над масивом переданим в першому аргументі.
  - Згідно назви сімейства може бути `Append`, `Prepend`, `Replace`, `Flatten` i `Replace`, при цьому рутина поверне масив.
  - Назва дії може закінчуватись на `ed` - `Appended`, `Prepended`, `Replaced`, `Flattened` i `Replaced` - в залежності від реалізації рутини може повертатись індекс, кількість елементів або сам елемент;
- `second` - дає зрозуміти як розглядати другий аргумент, що передається рутині.
  - `Element` - скалярна величина.
  - `Array` - масив в якому ведеться перебір окремих елементів.
  - `Arrays` - переданий масив розглядається як масив з вкладеними масивами.
  - Якщо `second` відсутній, то рутина розглядає переданий аргумент як скаляр.
- `how many` - дає зрозуміти скільки раз виконувати дію.
  - Якщо в назві рутини `how many` відсутній, то рутина виконує дію для кожного елементу згідно визначення в `second`.
  - Якщо назва містить `Once` - рутина виконає дію принаймні один раз для кожного елемента визначеного через `second`,
  - `OnceStrictly` - рутина виконає дію тільки один раз або викине помилку. Якщо другий елемент розглядається як масив або масиви, то дія виконується по одному разу для кожного елемента в переданих масивах.

#### Рутини без закінчення `ed`

```js
// array[action]
var result = _.arrayAppend( [ 1, 2 ], [ 3, 4 ] );
console.log( `Without ed : ${ result }` );
// log : Without ed : [ 1, 2, [ 3, 4 ] ]
```
Рутина `arrayAppend` розглядає другий аргумент як скаляр, тому результуючий масив `dstArray` буде містити не розгорнутий елемент `[ 3, 4 ]`.

#### Рутини з закінченням `ed`

```js
// array[action]ed
var result = _.arrayAppended( [ 1, 2 ], [ 3, 4 ] );
console.log( `Without ed : ${ result }` );
// log : Without ed : 2
```

Якщо назва рутини закінчується на `ed`, то рутина виконує зазначену дію, а повертає індекс, елемент, або кількість елементів. Тому при використанні рутини `arrayAppended` виведено індекс доданого елемента.

#### Рутини без суфікса `[how many]`

```js
// array[action][second]
var result = _.arrayRemoveElement( [ 1, 2, 2, 4, 2, 5 ], 2 );
console.log( `Without [how many] : ${ result }` );
// log : Without [how many] : [ 1, 4, 5 ]
```

Рутина `arrayRemoveElement` видаляє з масиву `dstArray` всі елементи, що задано в другому аргументі.

#### Рутини з суфіксом `Once`

```js
// array[action][second]Once
var result = _.arrayRemoveElementOnce( [ 1, 2, 2, 4, 2, 5 ], 2 );
console.log( `With [Once] : ${ result }` );
// log : With [Once] : [ 1, 2, 4, 2, 5 ]
```

Рутина `arrayRemoveElementOnce` видаляє переданий елемент тільки один раз, тому в `dstArray` залишиться два елементи `2` - `[ 1, 2, 4, 2 ]`.

#### Рутини з суфіксом `OnceStrictly`

```js
// array[action][second]OnceStrictly
var result = _.arrayReplaceElementOnceStrictly( [ 1, 4, 2, 5 ], 2, 3 );
console.log( `With [OnceStrictly] : ${ result }` );
// log : With [OnceStrictly] : [ 1, 4, 3, 5 ]
```

Рутина `arrayReplaceElementOnceStrictly` замінює елемент з масиву `dstArray` на переданий елемент тільки один раз, а якщо в `dstArray` їх більше, то рутина викине помилку.

#### Рутини без позначення `[second]`

```js
// array[action]
var result = _.arrayRemove( [ 1, 2, 2, 4, 2, 5 ], 2 );
console.log( `Without [second] : ${ result }` );
// log : Without [second] : [ 1, 4, 5 ]
```

Рутина `arrayRemove` розглядає другий елемент як скаляр, тому в переданому масиві `dstArray` видалено всі елементи `2`.

#### Рутини з суфіксом `Element`

```js
// array[action]Element
var result = _.arrayRemoveElement( [ 1, 2, 2, 4, 2, 5 ], [ 2, [ 4 ] ] );
console.log( `[second] as element : ${ result }` );
// log : [second] as element : [ 1, 2, 2, 4, 2, 5 ]
```

Рутина `arrayRemoveElement` розглядає другий елемент як скаляр, тому в переданому масиві `dstArray` не знайдено жодного елементу `[ 2, [ 4 ] ]`.

#### Рутини з суфіксом `Array`

```js
// array[action]Array
var result = _.arrayRemoveArray( [ 1, 2, 2, 4, 2, 5 ], [ 2, [ 4 ] ] );
console.log( `[second] as array : ${ result }` );
// log : [second] as array : [ 1, 4, 5 ]
```

Рутина `arrayRemoveArray` розглядає другий аргумент як масив зі скалярами. Тому в другому прикладі рутина поверне масив `[ 1, 4, 5 ]`.

#### Рутини з суфіксом `Arrays`

```js
// array[action]Arrays
var result = _.arrayRemoveArrays( [ 1, 2, 2, 4, 2, 5 ], [ 2, [ 4 ] ] );
console.log( `[second] as arrays : ${ result }` );
// log : [second] as arrays : [ 1, 5 ]
```

Рутина `arrayRemoveArrays` розглядає другий аргумент як масив з масивами, тому елементи у вкладений масивах також враховуються. Таким чином, рутина `arrayRemoveArrays` видалить всі елементи `2` i `4` з масиву `dstArray`.

### Перелік рутин в табличній формі

В першій колонці указано назви рутин в форматі `array[action][second]`, а в наступних додається приставка `how many`. На перетині цих значень указано, що повертає рутина.

<details>
  <summary><u>Таблиця рутин сімейства <code>Prepend</code></u></summary>

| array[action][second]      | -                   | Once                | OnceStrictly               |
|----------------------------|---------------------|---------------------|----------------------------|
| arrayPrepend               | dstArray            | dstArray            | dstArray                   |
| arrayPrependElement        | dstArray            | dstArray            | dstArray                   |
| arrayPrependArray          | dstArray            | dstArray            | dstArray                   |
| arrayPrependArrays         | dstArray            | dstArray            | dstArray                   |
| arrayPrepended             | index               | index               | index                      |
| arrayPrependedElement      | index               | element             | element / index            |
| arrayPrependedArray        | number              | number              | number                     |
| arrayPrependedArrays       | number              | number              | number                     |

</details>
<details>
  <summary><u>Таблиця рутин сімейства <code>Append</code></u></summary>

| array[action][second]     | -                   | Once                | OnceStrictly               |
|---------------------------|---------------------|---------------------|----------------------------|
| arrayAppend               | dstArray            | dstArray            | dstArray                   |
| arrayAppendElement        | dstArray            | dstArray            | dstArray                   |
| arrayAppendArray          | dstArray            | dstArray            | dstArray                   |
| arrayAppendArrays         | dstArray            | dstArray            | dstArray                   |
| arrayAppended             | index               | index               | index                      |
| arrayAppendedElement      | index               | element             | element / index            |
| arrayAppendedArray        | number              | number              | number                     |
| arrayAppendedArrays       | number              | number              | number                     |

</details>

Рутини сімейств `Prepend` i `Append` приймають від двох до чотирьох аргументів. Не містять додаткових рутин поведінка яких би відрізнялась від описаної в алгоритмі вище.

<details>
  <summary><u>Таблиця рутин сімейства <code>Remove</code></u></summary>

| array[action][second]     | -                   | Once                | OnceStrictly               |
|---------------------------|---------------------|---------------------|----------------------------|
| arrayRemove               | dstArray            | dstArray            | dstArray                   |
| arrayRemoveElement        | dstArray            | dstArray            | dstArray                   |
| arrayRemoveArray          | dstArray            | dstArray            | dstArray                   |
| arrayRemoveArrays         | dstArray            | dstArray            | dstArray                   |
| arrayRemoved              | number              | index               | index                      |
| arrayRemovedElement       | number              | index               | element                    |
| arrayRemovedArray         | number              | number              | number                     |
| arrayRemovedArrays        | number              | number              | number                     |
| arrayRemoveDuplicates     | dstArray            | -                   | -                          |

</details>

Рутина `arrayRemoveDuplicates`, що входить до сімейства `Remove` видаляє з переданого масиву елементи, що повторюються та повертає оновлений масив `dstArray`.

<details>
  <summary><u>Таблиця рутин сімейства <code>Flatten</code></u></summary>

| array[action][second]     | -                   | Once                | OnceStrictly               |
|---------------------------|---------------------|---------------------|----------------------------|
| arrayFlatten              | dstArray            | dstArray            | dstArray                   |
| arrayFlattenDefined       | dstArray            | dstArray            | dstArray                   |
| arrayFlattened            | number / dstArray   | number / dstArray   | number / dstArray          |
| arrayFlattenedDefined     | number / dstArray   | number / dstArray   | number / dstArray          |

</details>

Рутини сімейства `Flatten`, котрі містять приставку `Defined` викидають помилку, якщо другий аргумент містить значення `undefined`.

<details>
  <summary><u>Таблиця рутин сімейства <code>Replace</code></u></summary>

| array[action][second]     | -                   | Once                | OnceStrictly               |
|---------------------------|---------------------|---------------------|----------------------------|
| arrayReplace              | dstArray            | dstArray            | dstArray                   |
| arrayReplaceElement       | dstArray            | dstArray            | dstArray                   |
| arrayReplaceArray         | dstArray            | dstArray            | dstArray                   |
| arrayReplaceArrays        | dstArray            | dstArray            | dstArray                   |
| arrayReplaced             | number              | index               | index                      |
| arrayReplacedElement      | number              | element             | element                    |
| arrayReplacedArray        | number              | number              | number                     |
| arrayReplacedArrays       | number              | number              | number                     |

</details>

Рутини сімейства `Replace` приймають не менше трьох параметрів - масив призначення `dstArray`, елемент, що потрібно замінити `ins`, елемент, що буде додано `sub`.

Рутини, що виконують операції з масивами можуть містити додаткові [фільтри-колбеки в вигляді еквалайзера або евалуатора](ConventionCallbecksInArray.md).

### Підсумок

- Для виконання операцій над масивами використовуються рутини сімейств `Append`, `Prepend`, `Remove`, `Flatten`, `Replace`. Вони мають власний патерн найменування - `array[action][second][how many]`.
- `action` визначається за назвою сімейства рутин та може містити закінчення `ed`. Закінчення дає зрозуміти який результат буде повернено рутиною. Якщо рутина не містить закінчення `ed`, вона повертає масив `dstArray`, інакше індекс, або елемент чи кількість елементів.
- `second` дає зрозуміти як розглядати другий аргумент переданий рутині - як скаляр, чи як вектор.
- `how many` дає зрозуміти скільки раз буде виконуватись процедура - за кількістю знайдених елементів чи один раз.

[Повернутись до змісту](../README.md#Туторіали)
