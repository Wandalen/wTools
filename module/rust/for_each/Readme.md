# Module :: for_each

Apply macro for each element of a list.

Macros $Callback is called for each element of passed list, optinally passing prefix $Prefix as first argument(s) and postfix $Postfix as the last argument(s).
Macros could be invoked in either function call style or map call style. Prefix and postfix could be passed only in map call style.
In map call style after passing path to macro pass keyword `where` and options in format : `@KEY Value`.

In some cases same code may be generated without callback macro, just using prefix and postfix.
That's why $Callback is also optional.
To invoke `for_each` without callback use map call style omitting path to callback and keyword `where`.

# Sample :: function-style call

Macro `for_each` may be called either in function-style way or in map-style way.
Pass name of macro to apply to elements as the first arguments and elements after the macro name.
Use comma as delimeter.

```rust
use for_each::for_each;
for_each!( dbg, "a", "b", "c" );
```
Generates:
```rust
// generates
dbg!( "a" );
dbg!( "b" );
dbg!( "c" );
```

# Sample :: map-style call

Macro `for_each` may be called either in function-style way or in map-style way.
Use keys @PREFIX @POSTFIX @EACH to pass options as entries of a map.
Options @PREFIX and @POSTFIX are optional and their entries could be ommited, but entry @EACH is mandatory.
Order of options should always be @PREFIX, @POSTFIX, @EACH.

```rust
use for_each::for_each;
for_each!
{
  dbg where
  @PREFIX { "prefix".to_string() + }
  @POSTFIX { + "postfix" }
  @EACH "a" "b" "c"
};
// generates
dbg!( "prefix".to_string() + "a" + "postfix" );
dbg!( "prefix".to_string() + "b" + "postfix" );
dbg!( "prefix".to_string() + "c" + "postfix" );
```
Generates:
```rust
dbg!( "a" );
dbg!( "b" );
dbg!( "b" );
```

# Sample :: more than single token

Both prefix and postfix have to be token tree ( `tt` ). But if you need something more complex put it into braces `{ ... }`.
Macros `for_each` will remove outermost braces. Braces are optional in case of prefix/postfix is a singlle token.

```rust
use for_each::for_each;
for_each!
{
  dbg where
  @PREFIX { "prefix".to_string() + }
  @POSTFIX { + "postfix" }
  @EACH { "a" + "1" } { "b" + "2" } { "c" + "3" }
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
  @PREFIX { dbg! }
  @EACH ( "a" ) ( "b" ) ( "c" )
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
