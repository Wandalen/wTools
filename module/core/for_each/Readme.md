<!-- {{# generate.module_header{} #}} -->

# Module :: for_each
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_for_each_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_for_each_push.yml) [![docs.rs](https://img.shields.io/docsrs/for_each?color=e3e8f0&logo=docs.rs)](https://docs.rs/for_each) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ffor_each%2Fexamples%2Ffor_each_trivial.rs,RUN_POSTFIX=--example%20for_each_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Apply a macro for each element of a list.

Macros `$Callback` is called for each element of the passed list, optionally passing prefix `$Prefix` as the first argument(s) and postfix `$Postfix` as the last argument(s).
Macros could be invoked in either function call style or map call style. Prefix and postfix could be passed only in map call style.
In map call style after passing path to macro pass keyword `where` and options in format : `@KEY Value`.

In some cases, the same code may be generated without callback macro, just using prefix and postfix.
That's why `$Callback` is also optional.
To invoke `for_each` without callback use map call style omitting path to callback and keyword `where`.

### Basic Use Case :: function-style call

Apply a macro for each element of a list.

Macro `for_each` may be called either in function-style way or in map-style way.
Pass name of macro to apply to elements as the first arguments and elements after the macro name.
Use comma as delimiter.

<!-- {{# generate.module{} #}} -->

```rust
use for_each::for_each;
for_each!( dbg, "a", "b", "c" );

// generates
dbg!( "a" );
dbg!( "b" );
dbg!( "c" );
```

### Basic Use Case :: map-style call

Macro `for_each` may be called either in function-style way or in map-style way.
Use keys @Prefix @Postfix @Each to pass options as entries of a map.
Options @Prefix and @Postfix are optional and their entries could be omitted, but entry @Each is mandatory.
Order of options should always be @Prefix, @Postfix, @Each.

<!-- {{# generate.module{} #}} -->

```rust
use for_each::for_each;

for_each!
{
  dbg where
  @Prefix { "prefix".to_string() + }
  @Postfix { + "postfix" }
  @Each "a" "b" "c"
};
```

It generates:

```rust
// generated
dbg!( "prefix".to_string() + "a" + "postfix" );
dbg!( "prefix".to_string() + "b" + "postfix" );
dbg!( "prefix".to_string() + "c" + "postfix" );
```

### Basic Use Case :: more than single token

Both prefix and postfix have to be token tree ( `tt` ). But if you need something more complex put it into braces `{ ... }`.
Macros `for_each` will remove outermost braces. Braces are optional in case of prefix/postfix is a single token.

<!-- {{# generate.module{} #}} -->

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

### Basic Use Case :: callbackless

Callback macro is optional.
Use map call style and omit path to callback macro with keyword `where` to invoke `for_each` without a callback.

<!-- {{# generate.module{} #}} -->

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
cd examples/for_each_trivial
cargo run
```
<!-- xxx : qqq2 : fix each example instruction -->>