# Тип ```Stream```

<code>Stream</code> - абстрактний інтерфейс або його реалізація для роботи з даними розбиваючи їх на послідовність шматків, що йдуть одна за одною та обробляючи так в обробниках подій даного інтерфейсу.

Потік ділить дані, які потрібно отримати або відправити, на малі частини та опрацювання їх одна за одною. Дані можуть бути великими. Використання потоків дозволяє почати обробку початку даних до того як прийдуть всі дані. Абстрактний інтерфейс робить можливим використання декількох типів потоків, наприклад можна брати дані з читаючого потоку та передавати їх у записуючий. Це додає гнучкості і абстрагує алгоритми.

ПОтоки модуть мати:

- Файл для читання чи запису
- Сокети
- IPC
- При запуску процесу йому надаються так звані стандартні потоки

**Потоки в NodeJS:**

* Читаючі(Readable) - потоки, з яких можна прочитати дані(```process.stdin```).
* Записуючі(Writable) - потоки, у які можна записати дані(```process.stdout```).
* Дуплексні(Duplex) - потоки, з яких можна прочитати та у які можна записати(```net.Socket```).
* Трансформуючі(Transform) - потоки, які можуть модифікувати або трансформувати дані, які записуються або читаються(```zlib.createDeflate()```).

**Приклад читаючого потоку:**

```javascript
const { Readable } = require( 'stream' );

const inStream = new Readable
({
  read( size )
  {
    this.push( String.fromCharCode( this.currentCharCode++ ) );
    if( this.currentCharCode > 90 )
    this.push( null );
  }
});

inStream.currentCharCode = 65;
inStream.pipe( process.stdout );

```

**Ресурси:**

* [Node.js документація](https://nodejs.org/api/stream.html)
* [Стаття mozilla.org](https://developer.mozilla.org/en-US/docs/Web/API/Streams_API)
* [Додаткові приклади](https://www.codota.com/code/javascript/modules/stream)
* [Огляд потоків в NodeJS](https://www.freecodecamp.org/news/node-js-streams-everything-you-need-to-know-c9141306be93/)
