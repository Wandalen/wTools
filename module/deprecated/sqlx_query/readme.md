# DEPRECATED: sqlx_query
<!-- {{# generate.module_header{} #}} -->

# Module :: sqlx_query
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=sqlx_query)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/sqlx_query?color=e3e8f0&logo=docs.rs)](https://docs.rs/sqlx_query) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Feature-gated wrapper macros for SQLx that enable fast development builds by allowing compile-time database checks to be disabled, then re-enabled for production.

## Sample

<!-- {{# generate.module{} #}} -->

```rust
use sqlx_query::*;

let user : User = query_as!( User, "SELECT * FROM users LIMIT 1" )
    .fetch_one( executor )
    .await?;

query!( "DELETE FROM users WHERE id = $1", user.id )
    .execute( executor )
    .await?;
}

// Expands to

let user : User =
  {
    #[ cfg( feature = "sqlx_compiletime_checks" ) ]
    let q = ::sqlx::query_as::< _, User >( "SELECT * FROM users LIMIT 1" );
    #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
    let q = ::sqlx::query_as!( User, "SELECT * FROM users LIMIT 1" );
    q
  }
    .fetch_one( executor )
    .await?;
{
  #[ cfg( feature = "sqlx_compiletime_checks" ) ]
  let q = ::sqlx::query( "DELETE FROM users WHERE id = $1" ).bind( user.id );
  #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
  let q = ::sqlx::query!( "DELETE FROM users WHERE id = $1", user.id );
  q
}
    .execute( executor )
    .await?;
```

### To add to your project

```sh
cargo add sqlx_query
```

