# Guide: Daemon Monitoring

### Scope

- **Purpose:** Show how to combine the `daemon` and `check` sub-modules to manage the full lifecycle of a Unix daemon from startup through monitoring and cleanup.
- **Responsibility:** Documents the end-to-end workflow: PID file creation at daemon startup, liveness polling from a monitor process, and waiting for exit.
- **In Scope:** Using `daemon::write_pidfile()`, `check::is_pidfile_alive()`, `check::wait_for_exit()`, and `daemon::remove_pidfile()` together in sequence.
- **Out of Scope:** Signal delivery; supervisor restart logic; Windows process management; `daemonize()` internals (→ `api/006`).

### Abstract

A typical Unix daemon writes its PID to a file at startup and removes it on clean exit. A monitor reads the PID file to check liveness or wait for termination. This guide shows both sides using `process_tools`.

### Daemon Side

Inside the daemonized process — write PID after startup, remove on clean exit:

```rust,no_run
# #[ cfg( unix ) ]
# {
use process_tools::lifecycle::daemon;
use std::path::Path;

let pid_path = Path::new( "/var/run/myapp.pid" );

// Write PID after daemonization (or use DaemonizeOptions::pid_file for automatic write)
daemon::write_pidfile( pid_path, std::process::id() ).expect( "write pidfile" );

// ... do work ...

// Remove PID file on clean exit
daemon::remove_pidfile( pid_path ).ok();
# }
```

### Monitor Side

From a separate process — check liveness or wait for exit:

```rust,no_run
# #[ cfg( unix ) ]
# {
use process_tools::lifecycle::{ check, daemon };
use std::path::Path;
use std::time::Duration;

let pid_path = Path::new( "/var/run/myapp.pid" );

// One-shot liveness check
match check::is_pidfile_alive( pid_path ) {
  Ok( true )  => println!( "daemon is running" ),
  Ok( false ) => println!( "daemon has exited" ),
  Err( e ) if e.kind() == std::io::ErrorKind::NotFound =>
    println!( "no PID file — daemon never started or already cleaned up" ),
  Err( e )    => eprintln!( "check failed: {e}" ),
}

// Wait up to 30 seconds for daemon to exit
let pid = daemon::read_pidfile( pid_path ).expect( "read pidfile" );
let pid_i32 = i32::try_from( pid ).expect( "pid fits i32" );
match check::wait_for_exit( pid_i32, Duration::from_secs( 30 ) )
{
  Ok( () )  => println!( "daemon exited within 30 s" ),
  Err( e ) if e.kind() == std::io::ErrorKind::TimedOut =>
    println!( "daemon still running after 30 s" ),
  Err( e )  => eprintln!( "error: {e}" ),
}
# }
```

### Notes

- `is_pidfile_alive` returns `Err(NotFound)` when the PID file does not exist. Distinguish "daemon never started" from other errors by checking `e.kind()`.
- `EPERM` from `kill(pid, 0)` is treated as alive — privileged daemons will correctly appear alive to unprivileged monitors. See invariant `004_eperm_means_alive.md`.
- The PID file format is a plain decimal integer. Files written by system init scripts via `echo $PID > file` are compatible due to whitespace trimming. See invariant `003_pidfile_format.md`.
- Use `DaemonizeOptions::pid_file` with `daemonize()` to have the PID file written automatically with `flock` singleton protection — preventing duplicate daemon instances at the OS level.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [api/005_check_api.md](../api/005_check_api.md) | `is_process_alive()`, `wait_for_exit()`, `is_pidfile_alive()` |
| doc | [api/006_daemon_api.md](../api/006_daemon_api.md) | `write_pidfile()`, `read_pidfile()`, `remove_pidfile()`, `daemonize()` |
| doc | [invariant/003_pidfile_format.md](../invariant/003_pidfile_format.md) | PID file decimal format contract |
| doc | [invariant/004_eperm_means_alive.md](../invariant/004_eperm_means_alive.md) | EPERM interpretation for privileged daemons |
