# Конвенції рутин _.array*: аргумент dst

Про що свідчить використання аргументів <code>dst</code> і <code>src</code>.

### Використання рутин з аргументами `src` i `dst`

Найменування аргументів рутин `_.array*` може розповісти про те, як використовується переданий масив рутиною.

```js
let srcArray = [ 1, 2, 'a', 'b', true, 1,  [ 1 ] ];
let result = _.arraySlice( srcArray );

console.log( `The original array is : ${ srcArray }` );
// log : The original array is : [ 1, 2, 'a', 'b', true, 1, [ 1 ] ]
console.log( `The copied array is : ${ result }` );
// log : The copied array is : [ 1, 2, 'a', 'b', true, 1, [ 1 ] ]
console.log( srcArray === result );
// log : false
```

Рутина `arraySlice` приймає масив `srcArray` ( назву аргумента відповідає назві параметра рутини ) та повертає його копію. Приставка `src` в назві параметра говорить про те, що переданий масив використовується як джерело даних. Дані джерела при виконанні рутин не змінюються, про це свідчить вивід консолі. 

```js
let dstArray = [ 1, 2, 'a', 'b', true, 1,  [ 1 ] ];
let result = _.arrayEmpty( dstArray );

console.log( `The original array is : ${ dstArray }` );
// log : The original array is : []
console.log( `The copied array is : ${ result }` );
// log : The copied array is : []
console.log( dstArray === result );
// log : true
```

Рутина `arrayEmpty` приймає масив `dstArray` ( назву аргумента відповідає назві параметра рутини ) та очищає його від елементів. Приставка `dst` в назві параметра говорить про те, що переданий масив використовується як контейнер призначення і результат виконання операції записується в нього. Вивід консолі свідчить про те, що рутина очистила масив `dstArray`, а змінна `result` є ще одним посиланням на даний масив.

```js
let dstArray = [ 1, 2 ];
let srcArray = [ [ 3 ], [ 4 ] ];
let result = _.arrayFlatten( dstArray, srcArray );

console.log( `The destination array is : ${ dstArray }` );
// log : The destination array is : [ 1, 2, 3, 4 ]
console.log( `The source array is : ${ srcArray }` );
// log : The source array is : [ [ 3 ], [ 4 ] ]
console.log( dstArray === result );
// log : true
```

Рутина одночасно може містити параметри з приставкою `dst` і `src`. В такому разі контейнер призначення завжди записується першим. В приведеному вище прикладі рутина `arrayFlatten` додала до масиву `dstArray` значення з розгорнутого масиву `srcArray`. При цьому масив `srcArray` не змінився.

### Підсумок

- Опис аргументів рутини визначає її поведінку.
- Якщо аргумент в своїй назві містить `dst`, це означає що аргумент буде змінено згідно алгоритму рутини.
- Якщо рутина містить аргумент з приставкою `dst`, то вона може повернути як змінений аргумент так і інше значення, визначене її призначенням.
- Аргументи, що містять в своїй назві `src` використовуються рутиною як джерело даних та не змінюються в процесі виконання рутини.

[Повернутись до змісту](../README.md#Туторіали)
