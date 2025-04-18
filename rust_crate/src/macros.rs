#[macro_export]
/// Writes the interface code
/// needed to communicate with Dart.
/// This should be used once, and only once,
/// at the root of the `hub` crate.
macro_rules! write_interface {
  () => {
    #[cfg(not(target_family = "wasm"))]
    #[unsafe(no_mangle)]
    extern "C" fn rinf_start_rust_logic_extern() {
      use rinf::debug_print;
      let result = $crate::start_rust_logic(main);
      if let Err(err) = result {
        debug_print!("{}", err);
      }
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen::prelude::wasm_bindgen]
    pub fn rinf_start_rust_logic_extern() {
      use rinf::debug_print;
      let result = $crate::start_rust_logic(main);
      if let Err(err) = result {
        debug_print!("{}", err);
      }
    }
  };
}

/// Delegates the printing operation to Flutter,
/// which excels at handling various platforms
/// including web and mobile emulators.
/// When debugging, using this macro is recommended over `println!`,
/// as it seamlessly adapts to different environments.
/// Note that this macro does nothing in release mode.
#[macro_export]
macro_rules! debug_print {
  ( $( $t:tt )* ) => {
    {
      let rust_report = format!( $( $t )* );
      #[cfg(debug_assertions)]
      {
        let result = $crate::send_rust_signal(
          "RinfOut", // Special message ID for Rust output
          Vec::new(),
          rust_report.clone().into_bytes(),
        );
        if let Err(err) = result {
          println!("{}\n{}", err, rust_report);
        }
      }
      #[cfg(not(debug_assertions))]
      let _ = rust_report;
    }
  }
}
