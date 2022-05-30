# Конвенції: мапа дефолтних значень

Як зберігаються та змінюються налаштування в рутинах.

Зберігання налаштувань за замовчуванням в рутинах здійснюється з допомогою спеціальних [мап опцій](../concept/RoutineInput.md) `defaults`. Мапи опцій `defaults` містять повний набір параметрів для рутини.

### Рутина, що не змінює налаштування за замовчуванням

<details>
    <summary><u>Код файла <code>DefaultsInRoutine.js</code></u></summary>

```js
let _ = require( 'wTools' );

//

function nameFull( path )
{
  _.assert( arguments.length === 1 );
  _.assert( _.strIs( path ), 'Expects strings {-path-}' );

  let o = { path : path };
  _.routine.options( nameFull, o );

  let i = o.path.lastIndexOf( '/' );
  if( i !== -1 )
  o.path = o.path.substr( i+1 );

  if( !o.full )
  {
    let i = o.path.lastIndexOf( '.' );
    if( i !== -1 ) o.path = o.path.substr( 0, i );
  }

  return o.path;
}

nameFull.defaults =
{
  path : null,
  full : 1,
}

//

console.log( nameFull( '/foo/bar/baz.js' ) );
```

</details>

Внесіть в файл `DefaultsInRoutine.js` приведений вище код.

Рутина `nameFull` - приклад встановлення статичних налаштувань за замовчуванням. Рутина приймає єдиний аргумент `path` типу `String`, а мапа опцій `defaults` є лише контейнером для зберігання налаштувань.

Рутина `routineOptions` порівнює встановлені в рутині ( або передані ) опції з опціями в мапі `defaults`. Якщо в мапі `defaults` є опції, що не встановлені в рутині, то `routineOptions` присвоює їх рутині. Опція `full` в мапі `defaults` має постійне значення `1`, вона не передається через аргумент, тому `routineOptions` присвоює опцію рутині `nameFull`.

<details>
  <summary><u>Вивід команди <code>node DefaultsInRoutine.js</code></u></summary>

```
$ node DefaultsInRoutine.js
baz.js
```

</details>

Запустіть виконання файла командою `node DefaultsInRoutine.js`. Порівняйте вивід з приведеним вище.

Рутина `nameFull` відділила повну назву файла з указаного шляху. Змініть значення опції `full : 1` на `full : 0` та повторіть виконання файла.

<details>
  <summary><u>Вивід команди <code>node DefaultsInRoutine.js</code></u></summary>

```
$ node DefaultsInRoutine.js
baz
```

</details>

Тепер рутина виділяє лише ім'я файла, без розширення.

Статичні налаштування за замовчуванням зручні при створенні рутин, що мають спільний код, а поведінка відрізняється за рахунок встановлених опцій. Як створювати множину рутин з відмінними налаштуваннями за замовчуванням можна дізнатись в [туторіалі](./RoutineFromPreAndBody.md).


### Рутини, котрі змінюють налаштування за замовчуванням

Заміна використання масиву аргументів на мапу опцій веде до збільшення гнучкості. Це досягається за рахунок спрощеного інтерфейсу передачі значень та спрощеного механізму встановлення опцій за замовчуванням.

<details>
    <summary><u>Код файла <code>DefaultsInRoutine.js</code></u></summary>

```js
let _ = require( 'wTools' );

//

function name( o )
{
  _.assert( arguments.length === 1 );
  _.assert( _.mapIs( o ), 'Expects map' );

  _.routine.options( name, o );

  let i = o.path.lastIndexOf( '/' );
  if( i !== -1 )
  o.path = o.path.substr( i+1 );

  if( !o.full )
  {
    let i = o.path.lastIndexOf( '.' );
    if( i !== -1 ) o.path = o.path.substr( 0, i );
  }

  return o.path;
}

name.defaults =
{
  path : null,
  full : 0,
}

//

console.log( name( { path : '/foo/bar/baz.js' } ) );
console.log( name( { path : '/foo/bar/baz.js', full : 1 } ) );
```

</details>

Замініть код в файлі `DefaultsInRoutine.js` на приведений вище.

Оновлений код описує рутину `name`. Рутина приймає мапу опції, це дозволяє передавати опції `path` i `full` відразу в мапі, та динамічно змінювати поведінку рутини `name`. Порядок запису опцій не має значення, розробнику достатньо знати назви опцій.

<details>
  <summary><u>Вивід команди <code>node DefaultsInRoutine.js</code></u></summary>

```
$ node DefaultsInRoutine.js
baz
baz.js
```

</details>

Запустіть виконання файла командою `node DefaultsInRoutine.js`. Порівняйте вивід з приведеним вище.

В першому рядку виводу показано, що при передачі шляху без указання опції `full`, присвоюється значення за замовчуванням і рутина повертає ім'я файла без розширення. В другому рядку було встановлено опцію `full` i тому рутина вивела повне ім'я.

### Підсумок

- Мапа опцій `defaults` зберігає налаштування за замовчуванням, в ній міститься повний перелік параметрів рутини.
- Опції в мапі `defaults` можуть змінюватись в залежності від переданих аргументів або залишатись незмінними (статичними).
- Зміною статичних налаштувань в мапі `defaults` можна створювати нові рутини, що мають спільний код, але різну поведінку.
- Налаштування в рутину можуть передаватись через масив аргументів або мапу опцій. Мапа опцій зручніша в використанні.
- Опції, що передаються через аргументи рутини або мапу опцій забезпечують високу гнучкість при використанні рутин.

[Повернутись до змісту](../README.md#Туторіали)
