//!
//! Smoke test checking health of a module.
//!

// qqq : does not work in parallel, fix
// qqq : make a command for willbe

// xxx2 : use process_tools to build and run rust programs, introduce program_

/// Define a private namespace for all its items.
mod private {
  #[ allow( unused_imports ) ]
  use crate::*;
  use crate::process::environment;
  // zzz : comment out
  // pub mod environment
  // {
  //   pub fn is_cicd() -> bool
  //   {
  //     false
  //   }
  // }

  /// Context for smoke testing of a module.
  #[ derive( Debug ) ]
  pub struct SmokeModuleTest<'a> {
    /// Name of module.
    pub dependency_name: &'a str,
    /// Version of module.
    pub version: &'a str,
    /// Local path to the module.
    pub local_path_clause: &'a str,
    /// Code to run during smoke testing.
    pub code: String,
    /// Path to temp directory to put all files.
    pub test_path: std::path::PathBuf,
    /// Postfix to add to name.
    pub test_postfix: &'a str,
  }

  impl<'a> SmokeModuleTest<'a> {
    /// Constructor of a context for smoke testing.
    #[ must_use ]
    pub fn new(dependency_name: &'a str) -> SmokeModuleTest<'a> {
      use rand::prelude::*;

      let test_postfix = "_smoke_test";
      let mut rng = rand::thread_rng();
      let y: f64 = rng.gen();

      let smoke_test_path = format!("{dependency_name}{test_postfix}_{y}");
      let mut test_path = std::env::temp_dir();
      test_path.push(smoke_test_path);

      SmokeModuleTest {
        dependency_name,
        version: "*",
        local_path_clause: "",
        code: format!("use {dependency_name};").to_string(),
        test_path,
        test_postfix,
      }
    }

    /// Set version.
    pub fn version(&mut self, version: &'a str) -> &mut SmokeModuleTest<'a> {
      self.version = version;
      self
    }

    /// Set local path.
    pub fn local_path_clause(&mut self, local_path_clause: &'a str) -> &mut SmokeModuleTest<'a> {
      self.local_path_clause = local_path_clause;
      self
    }

    /// Set postfix to add to name of test.
    pub fn test_postfix(&mut self, test_postfix: &'a str) -> &mut SmokeModuleTest<'a> {
      use rand::prelude::*;

      self.test_postfix = test_postfix;
      let mut rng = rand::thread_rng();
      let y: f64 = rng.gen();

      let smoke_test_path = format!(
        "{dependency_name}{test_postfix}_{y}",
        dependency_name = self.dependency_name,
        test_postfix = test_postfix,
        y = y
      );
      self.test_path.pop();
      self.test_path.push(smoke_test_path);
      self
    }

    /// Get code.
    pub fn code(&mut self, code: String) -> &mut SmokeModuleTest<'a> {
      self.code = code;
      self
    }

    /// Prepare files at temp dir for smoke testing.
    /// 
    /// Creates a temporary, isolated Cargo project with proper dependency configuration.
    /// Implements FR-4 and FR-5 requirements for project creation and configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if directory creation, project initialization, or file writing fails.
    pub fn form(&mut self) -> Result< (), Box< dyn core::error::Error > > {
      std::fs::create_dir(&self.test_path)
        .map_err(|e| format!("Failed to create test directory: {e}"))?;

      let mut test_path = self.test_path.clone();

      /* create binary test module */
      let test_name = format!("{}{}", self.dependency_name, self.test_postfix);
      // println!( "test_name:{test_name}" );

      // dbg!( &test_path );

      let output = std::process::Command::new("cargo")
        .current_dir(&test_path)
        .args(["new", "--bin", &test_name])
        .output()
        .map_err(|e| format!("Failed to execute cargo new command: {e}"))?;
      
      if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Cargo new failed: {stderr}").into());
      }
      
      if !output.stderr.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
      }

      test_path.push(test_name);

      /* setup config */
      #[ cfg( target_os = "windows" ) ]
      let local_path_clause = if self.local_path_clause.is_empty() {
        String::new()
      } else {
        format!(", path = \"{}\"", self.local_path_clause.escape_default())
      };
      #[cfg(not(target_os = "windows"))]
      let local_path_clause = if self.local_path_clause.is_empty() {
        String::new()
      } else {
        format!(", path = \"{}\"", self.local_path_clause)
      };
      let dependencies_section = format!(
        "{} = {{ version = \"{}\" {} }}",
        self.dependency_name, self.version, &local_path_clause
      );
      let config_data = format!(
        "[package]
        edition = \"2021\"
        name = \"{}_smoke_test\"
        version = \"0.0.1\"

        [dependencies]
        {}",
        &self.dependency_name, &dependencies_section
      );
      let mut config_path = test_path.clone();
      config_path.push("Cargo.toml");
      println!("\n{config_data}\n");
      std::fs::write(config_path, config_data)
        .map_err(|e| format!("Failed to write Cargo.toml: {e}"))?;

      /* write code */
      test_path.push("src");
      test_path.push("main.rs");
      if self.code.is_empty() {
        self.code = format!("use ::{}::*;", self.dependency_name);
      }
      let code = format!(
        "#[ allow( unused_imports ) ]
        fn main()
        {{
          {code}
        }}",
        code = self.code,
      );
      println!("\n{code}\n");
      std::fs::write(&test_path, code)
        .map_err(|e| format!("Failed to write main.rs: {e}"))?;

      Ok(())
    }

    /// Execute smoke testing by running cargo test and cargo run.
    /// 
    /// Implements FR-6 requirement: executes both `cargo test` and `cargo run` 
    /// within the temporary project and ensures both commands succeed.
    ///
    /// # Errors
    ///
    /// Returns an error if either cargo test or cargo run fails.
    pub fn perform(&self) -> Result< (), Box< dyn core::error::Error > > {
      let mut test_path = self.test_path.clone();

      let test_name = format!("{}{}", self.dependency_name, self.test_postfix);
      test_path.push(test_name);

      // Execute cargo test
      let output = std::process::Command::new("cargo")
        .current_dir(test_path.clone())
        .args(["test"])
        .output()
        .map_err(|e| format!("Failed to execute cargo test: {e}"))?;
      
      println!("cargo test status: {}", output.status);
      if !output.stdout.is_empty() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      }
      if !output.stderr.is_empty() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      }
      
      if !output.status.success() {
        return Err(format!("cargo test failed with status: {}", output.status).into());
      }

      // Execute cargo run --release  
      let output = std::process::Command::new("cargo")
        .current_dir(test_path)
        .args(["run", "--release"])
        .output()
        .map_err(|e| format!("Failed to execute cargo run: {e}"))?;
      
      println!("cargo run status: {}", output.status);
      if !output.stdout.is_empty() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      }
      if !output.stderr.is_empty() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      }
      
      if !output.status.success() {
        return Err(format!("cargo run failed with status: {}", output.status).into());
      }

      Ok(())
    }

    /// Clean up temporary directory after testing.
    /// 
    /// Implements FR-7 requirement: cleans up all temporary files and directories 
    /// from the filesystem upon completion, regardless of success or failure.
    ///
    /// # Arguments
    /// 
    /// * `force` - If true, ignores cleanup errors and continues. If false, returns error on cleanup failure.
    ///
    /// # Errors
    ///
    /// Returns an error if cleanup fails and `force` is false.
    pub fn clean(&self, force: bool) -> Result< (), Box< dyn core::error::Error > > {
      if !self.test_path.exists() {
        // Directory already cleaned or never created
        return Ok(());
      }
      
      let result = std::fs::remove_dir_all(&self.test_path);
      match result {
        Ok(()) => Ok(()),
        Err(e) => {
          if force {
            eprintln!("Warning: Failed to remove temporary directory {}: {}", 
                     self.test_path.display(), e);
            Ok(())
          } else {
            Err(format!("Cannot remove temporary directory {}: {}. Consider manual cleanup.", 
                       self.test_path.display(), e).into())
          }
        }
      }
    }
  }

  /// Run smoke test for the module with proper cleanup on failure.
  /// 
  /// Implements comprehensive smoke testing with automatic cleanup regardless of success or failure.
  /// This ensures FR-7 compliance by cleaning up resources even when tests fail.
  ///
  /// # Errors
  ///
  /// Returns error if environment variables are missing, project creation fails, or testing fails.
  ///
  /// # Panics
  ///
  /// This function will panic if the environment variables `CARGO_PKG_NAME` or `CARGO_MANIFEST_DIR` are not set.
  pub fn smoke_test_run(local: bool) -> Result< (), Box< dyn core::error::Error > > {
    let module_name = std::env::var("CARGO_PKG_NAME")
      .map_err(|_| "CARGO_PKG_NAME environment variable not set")?;
    let module_path = std::env::var("CARGO_MANIFEST_DIR")
      .map_err(|_| "CARGO_MANIFEST_DIR environment variable not set")?;
    let test_name = if local { "_local_smoke_test" } else { "_published_smoke_test" };
    println!("smoke_test_run module_name:{module_name} module_path:{module_path}");

    let mut smoke_test = SmokeModuleTest::new(module_name.as_str());
    smoke_test.test_postfix(test_name);
    
    // Always attempt cleanup before starting (force=true to ignore errors)
    let _ = smoke_test.clean(true);

    smoke_test.version("*");
    if local {
      smoke_test.local_path_clause(module_path.as_str());
    }
    
    // Execute the smoke test with proper cleanup on any failure
    let result = (|| -> Result< (), Box< dyn core::error::Error > > {
      smoke_test.form()?;
      smoke_test.perform()?;
      Ok(())
    })();
    
    // Always clean up, regardless of success or failure (FR-7)
    let cleanup_result = smoke_test.clean(false);
    
    // Return the original error if test failed, otherwise cleanup error if any
    match result {
      Ok(()) => cleanup_result,
      Err(e) => {
        // Log cleanup error but preserve original test error
        if let Err(cleanup_err) = cleanup_result {
          eprintln!("Warning: Cleanup failed after test failure: {cleanup_err}");
        }
        Err(e)
      }
    }
  }

  /// Run smoke test for both published and local version of the module.
  /// 
  /// Implements FR-8: conditional execution based on environment variables or CI/CD detection.
  ///
  /// # Errors
  ///
  /// Returns error if either local or published smoke test fails.
  pub fn smoke_tests_run() -> Result< (), Box< dyn core::error::Error > > {
    smoke_test_for_local_run()?;
    smoke_test_for_published_run()?;
    Ok(())
  }

  /// Run smoke test for local version of the module.
  /// 
  /// Implements FR-8: conditional execution triggered by `WITH_SMOKE` environment variable 
  /// or CI/CD environment detection.
  ///
  /// # Errors
  ///
  /// Returns error if smoke test execution fails.
  pub fn smoke_test_for_local_run() -> Result< (), Box< dyn core::error::Error > > {
    println!("smoke_test_for_local_run : {:?}", std::env::var("WITH_SMOKE"));
    
    let should_run = if let Ok(value) = std::env::var("WITH_SMOKE") {
      matches!(value.as_str(), "1" | "local")
    } else {
      environment::is_cicd()
    };
    
    if should_run {
      println!("Running local smoke test (WITH_SMOKE or CI/CD detected)");
      smoke_test_run(true)
    } else {
      println!("Skipping local smoke test (no WITH_SMOKE env var and not in CI/CD)");
      Ok(())
    }
  }

  /// Run smoke test for published version of the module.
  /// 
  /// Implements FR-8: conditional execution triggered by `WITH_SMOKE` environment variable 
  /// or CI/CD environment detection.
  ///
  /// # Errors
  ///
  /// Returns error if smoke test execution fails.
  pub fn smoke_test_for_published_run() -> Result< (), Box< dyn core::error::Error > > {
    println!("smoke_test_for_published_run : {:?}", std::env::var("WITH_SMOKE"));
    
    let should_run = if let Ok(value) = std::env::var("WITH_SMOKE") {
      matches!(value.as_str(), "1" | "published")
    } else {
      environment::is_cicd()
    };
    
    if should_run {
      println!("Running published smoke test (WITH_SMOKE or CI/CD detected)");
      smoke_test_run(false)
    } else {
      println!("Skipping published smoke test (no WITH_SMOKE env var and not in CI/CD)");
      Ok(())
    }
  }
}

// //
// crate::mod_interface!
// {
// //
// //   // exposed use super;
// //   exposed use super::super::smoke_test;
// //
// //   exposed use SmokeModuleTest;
// //   exposed use smoke_test_run;
// //   exposed use smoke_tests_run;
// //   exposed use smoke_test_for_local_run;
// //   exposed use smoke_test_for_published_run;
// //
// // }
//

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own {
  use super::*;

  #[ doc( inline ) ]
  pub use private::{SmokeModuleTest, smoke_test_run, smoke_tests_run, smoke_test_for_local_run, smoke_test_for_published_run};
}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan {
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  pub use super::super::smoke_test;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed {
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use private::{SmokeModuleTest, smoke_test_run, smoke_tests_run, smoke_test_for_local_run, smoke_test_for_published_run};
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude {
  use super::*;

  #[ doc( inline ) ]
  pub use {};
}
