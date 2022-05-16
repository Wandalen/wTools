# Module :: for_each [![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ForEachPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsForEachPush.yml) [![docs.rs](https://img.shields.io/docsrs/for_each?color=e3e8f0&logo=docs.rs)](https://docs.rs/for_each) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Apply a macro for each element of a list.

Macros `$Callback` is called for each element of the passed list, optionally passing prefix `$Prefix` as the first argument(s) and postfix `$Postfix` as the last argument(s).
Macros could be invoked in either function call style or map call style. Prefix and postfix could be passed only in map call style.
In map call style after passing path to macro pass keyword `where` and options in format : `@KEY Value`.

In some cases, the same code may be generated without callback macro, just using prefix and postfix.
That's why `$Callback` is also optional.
To invoke `for_each` without callback use map call style omitting path to callback and keyword `where`.

# Sample :: function-style call

Macro `for_each` may be called either in function-style way or in map-style way.
Pass name of macro to apply to elements as the first arguments and elements after the macro name.
Use comma as delimiter.

```rust
use for_each::for_each;
for_each!( dbg, "a", "b", "c" );

// generates
dbg!( "a" );
dbg!( "b" );
dbg!( "c" );
```

# Sample :: map-style call

Macro `for_each` may be called either in function-style way or in map-style way.
Use keys @Prefix @Postfix @Each to pass options as entries of a map.
Options @Prefix and @Postfix are optional and their entries could be ommited, but entry @Each is mandatory.
Order of options should always be @Prefix, @Postfix, @Each.

```rust
use for_each::for_each;

for_each!
{
  dbg where
  @Prefix { "prefix".to_string() + }
  @Postfix { + "postfix" }
  @Each "a" "b" "c"
};

// generates
dbg!( "prefix".to_string() + "a" + "postfix" );
dbg!( "prefix".to_string() + "b" + "postfix" );
dbg!( "prefix".to_string() + "c" + "postfix" );
```

# Sample :: more than single token

Both prefix and postfix have to be token tree ( `tt` ). But if you need something more complex put it into braces `{ ... }`.
Macros `for_each` will remove outermost braces. Braces are optional in case of prefix/postfix is a singlle token.

```rust
use for_each::for_each;

for_each!
{
  dbg where
  @Prefix { "prefix".to_string() + }
  @Postfix { + "postfix" }
  @Each { "a" + "1" } { "b" + "2" } { "c" + "3" }
};

// generates
dbg!( "prefix".to_string() + "a" + "1" + "postfix" );
dbg!( "prefix".to_string() + "b" + "2" + "postfix" );
dbg!( "prefix".to_string() + "c" + "3" + "postfix" );
```

# Sample :: callbackless

Callback macro is optinal.
Use map call style and omit path to callback macro with keyword `where` to invoke `for_each` without a callback.

```rust
use for_each::for_each;
for_each!
{
  @Prefix { dbg! }
  @Each ( "a" ) ( "b" ) ( "c" )
};
// generates
dbg!( "a" );
dbg!( "b" );
dbg!( "c" );
```

### To add to your project

``` shell
cargo add for_each
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/for_each_trivial_sample
cargo run
```


