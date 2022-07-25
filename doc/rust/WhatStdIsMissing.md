# What is missing in STD of Rust

Rust provides very profound standard library. However it lacks few things.

## Iteration pattern

Iterator is a behavioral design pattern that lets you traverse elements of a collection without exposing its underlying representation.
Rust supports it. Collections expose methods [iter()](https://doc.rust-lang.org/1.56.1/std/collections/struct.HashMap.html#method.iter) and [iter_mut()](https://doc.rust-lang.org/1.56.1/std/collections/struct.HashMap.html#method.iter_mut) to iterate its elements. Method `iter()` returns something implementing abstract interface of trat [Iterator](https://doc.rust-lang.org/1.56.1/std/iter/trait.Iterator.html). Beauty of that pattern is sepeartion of data and algorithm. Hundreds of algorithms are implemented for based on the simple interface, indeed in Rust it has only one method [next()](https://doc.rust-lang.org/1.56.1/std/iter/trait.Iterator.html#tymethod.next). Having that method implemented for any particualr data type make possible to use any algorithm which is based on the interface.

All collections implement interface of trait [IntoIterator](https://doc.rust-lang.org/1.56.1/core/iter/trait.IntoIterator.html) with help of which it's possible to convert the collection into a iterator giving up elements of it. Structures which implements interface of trait [Extend](https://doc.rust-lang.org/1.56.1/core/iter/trait.Extend.html) can consume another collection adopting its elements. Traits [ExactSizeIterator](https://doc.rust-lang.org/1.56.1/core/iter/trait.ExactSizeIterator.html) and [FusedIterator](https://doc.rust-lang.org/1.56.1/core/iter/trait.FusedIterator.html) give additional promises rectricting implementor of these interface. [Peekable](https://doc.rust-lang.org/1.56.1/std/iter/struct.Peekable.html) makes possible to iterate speculatively, looking forward the next element without advancing. Use function [repeat](https://doc.rust-lang.org/1.56.1/std/iter/fn.repeat.html) to construct an iterator which repeat the same element endlessly. Use [zip](https://doc.rust-lang.org/1.56.1/std/iter/fn.zip.html) if you want to pair elements of two collections with the same length. But there is the catch. Currently [zip](https://doc.rust-lang.org/1.56.1/std/iter/fn.zip.html) is experimental and it is not available in stable Rust.

All that is good. But what if you need to peek more than one element forward? What if you need to repeat an element finite number of times?
At this moment you cant do anything of that easily with Rust STD library.
To speculatively peek more than one element use [multipeek](https://docs.rs/itertools/0.10.3/itertools/fn.multipeek.html). To repeat the same element `n` times use [repeat_n](https://docs.rs/itertools/0.10.3/itertools/fn.repeat_n.html). To zip more than two collection use [multizip](https://docs.rs/itertools/0.10.3/itertools/fn.multizip.html). More over all this functionality available in stable Rust! There are many more algorithm whih are based on the interface, but is not part of stable Rust STD library in [crate itertools](https://docs.rs/itertools/0.10.3/itertools/index.html).

So why such useful algorithms is not part of STD?
<!-- xxx : answer please -->
Does lack of advance algorithms and data types has positive effect on time of compilation and size of compiled executable?
I don't know. <!-- xxx : answer please -->

<!-- - literally -->
<!-- - drive more -->
<!-- - prelude does not include hashmap and other containers, but include ...  -->
<!-- - std does not implement variadic constructor of hashmap, but vector ...  -->
<!-- - variadic make -->
<!-- - type constructors -->

## Derive more

Fundamental problem of a programming language is ability to write programs on the same dialect. C++ has 3 way to express algorithms:
- statically with help of macroses
- statically with help of templates
- dynamically with help of ordinary C++
Disadvantage is developer should often has his algorithm for 3 different domains, express each in 3 different dialects and it's not possible to have it in one dialect.
That is the biggest advantage of such language as JS nad Python over C++ and Rust, developer don't need to have 3 copies of the same algorithm.

In Rust situation is better than in C++. [Declarative macroses](https://www.youtube.com/watch?v=q6paRBbLgNw) has its niche, but for generating code usually used [procedural macroses](https://www.youtube.com/watch?v=geovSK3wMB8). Procedural macroses are written in pure Rust. Although in Rust stage of compilation has explicit boundary and developer have to write programs which generates programs. It's called metaprogramming.

In JS there is no boundary between stage of generation of a program and program itself and metapgram on JS looks exactly the same as ordinary program. In Rust there is several means to container a metaprogram. [Derive](https://doc.rust-lang.org/stable/reference/attributes/derive.html#derive) is the primaraly mean to do that. Derive is written on Rust and it generates Rust code from Rust code on input.

Rust have many derives shiped with Rust in STD library. Some of them are
