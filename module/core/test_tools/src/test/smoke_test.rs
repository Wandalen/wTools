//!
//! Smoke test checking health of a module.
//!

// qqq : does not work in parallel, fix
// qqq : make a command for willbe

// xxx2 : use process_tools to build and run rust programs, introduce program_

/// Define a private namespace for all its items.
mod private {
  #[allow(unused_imports)]
  use crate::*;
  use process_tools::environment;
  // zzz : comment out
  // pub mod environment
  // {
  //   pub fn is_cicd() -> bool
  //   {
  //     false
  //   }
  // }

  /// Context for smoke testing of a module.
  #[derive(Debug)]
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
    #[must_use]
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
    /// Prepare files at temp dir for smoke testing.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to create the directory or write to the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn form(&mut self) -> Result<(), &'static str> {
      std::fs::create_dir(&self.test_path).unwrap();

      let mut test_path = self.test_path.clone();

      /* create binary test module */
      let test_name = format!("{}{}", self.dependency_name, self.test_postfix);
      // println!( "test_name:{test_name}" );

      // dbg!( &test_path );

      let output = std::process::Command::new("cargo")
        .current_dir(&test_path)
        .args(["new", "--bin", &test_name])
        .output()
        .expect("Failed to execute command");
      println!("{}", core::str::from_utf8(&output.stderr).expect("Invalid UTF-8"));

      test_path.push(test_name);

      /* setup config */
      #[cfg(target_os = "windows")]
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
      std::fs::write(config_path, config_data).unwrap();

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
      std::fs::write(&test_path, code).unwrap();

      Ok(())
    }

    /// Do smoke testing.
    /// Do smoke testing.
    ///
    /// # Panics
    ///
    /// This function will panic if the command execution fails or if the smoke test fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn perform(&self) -> Result<(), &'static str> {
      let mut test_path = self.test_path.clone();

      let test_name = format!("{}{}", self.dependency_name, self.test_postfix);
      test_path.push(test_name);

      let output = std::process::Command::new("cargo")
        .current_dir(test_path.clone())
        .args(["test"])
        .output()
        .unwrap();
      println!("status : {}", output.status);
      println!("{}", core::str::from_utf8(&output.stdout).expect("Invalid UTF-8"));
      println!("{}", core::str::from_utf8(&output.stderr).expect("Invalid UTF-8"));
      assert!(output.status.success(), "Smoke test failed");

      let output = std::process::Command::new("cargo")
        .current_dir(test_path)
        .args(["run", "--release"])
        .output()
        .unwrap();
      println!("status : {}", output.status);
      println!("{}", core::str::from_utf8(&output.stdout).expect("Invalid UTF-8"));
      println!("{}", core::str::from_utf8(&output.stderr).expect("Invalid UTF-8"));
      assert!(output.status.success(), "Smoke test failed");

      Ok(())
    }

    /// Cleaning temp directory after testing.
    /// Cleaning temp directory after testing.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to remove the directory and `force` is set to `false`.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn clean(&self, force: bool) -> Result<(), &'static str> {
      let result = std::fs::remove_dir_all(&self.test_path);
      if force {
        result.unwrap_or_default();
      } else {
        let msg = format!(
          "Cannot remove temporary directory {}. Please, remove it manually",
          &self.test_path.display()
        );
        result.expect(&msg);
      }
      Ok(())
    }
  }

  /// Run smoke test for the module.
  /// Run smoke test for the module.
  ///
  /// # Panics
  ///
  /// This function will panic if the environment variables `CARGO_PKG_NAME` or `CARGO_MANIFEST_DIR` are not set.
  pub fn smoke_test_run(local: bool) {
    let module_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let module_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_name = if local { "_local_smoke_test" } else { "_published_smoke_test" };
    println!("smoke_test_run module_name:{module_name} module_path:{module_path}");

    let mut t = SmokeModuleTest::new(module_name.as_str());
    t.test_postfix(test_name);
    t.clean(true).unwrap();

    t.version("*");
    if local {
      t.local_path_clause(module_path.as_str());
    }
    t.form().unwrap();
    t.perform().unwrap();
    t.clean(false).unwrap();
  }

  /// Run smoke test for both published and local version of the module.
  pub fn smoke_tests_run() {
    smoke_test_for_local_run();
    smoke_test_for_published_run();
  }

  /// Run smoke test for local version of the module.
  pub fn smoke_test_for_local_run() {
    println!("smoke_test_for_local_run : {:?}", std::env::var("WITH_SMOKE"));
    let run = if let Ok(value) = std::env::var("WITH_SMOKE") {
      matches!(value.as_str(), "1" | "local")
    } else {
      // qqq : xxx : use is_cicd() and return false if false
      // true
      environment::is_cicd()
    };
    if run {
      smoke_test_run(true);
    }
  }

  /// Run smoke test for published version of the module.
  pub fn smoke_test_for_published_run() {
    let run = if let Ok(value) = std::env::var("WITH_SMOKE") {
      matches!(value.as_str(), "1" | "published")
    } else {
      environment::is_cicd()
      // qqq : xxx : use is_cicd() and return false if false
      // true
    };
    if run {
      smoke_test_run(false);
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

#[doc(inline)]
#[allow(unused_imports)]
pub use own::*;

/// Own namespace of the module.
#[allow(unused_imports)]
pub mod own {
  use super::*;

  #[doc(inline)]
  pub use private::{SmokeModuleTest, smoke_test_run, smoke_tests_run, smoke_test_for_local_run, smoke_test_for_published_run};
}

/// Shared with parent namespace of the module
#[allow(unused_imports)]
pub mod orphan {
  use super::*;

  #[doc(inline)]
  pub use exposed::*;

  pub use super::super::smoke_test;
}

/// Exposed namespace of the module.
#[allow(unused_imports)]
pub mod exposed {
  use super::*;

  #[doc(inline)]
  pub use prelude::*;

  #[doc(inline)]
  pub use private::{SmokeModuleTest, smoke_test_run, smoke_tests_run, smoke_test_for_local_run, smoke_test_for_published_run};
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[allow(unused_imports)]
pub mod prelude {
  use super::*;

  #[doc(inline)]
  pub use {};
}
