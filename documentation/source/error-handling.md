# Error Handling

Effective error handling is crucial in applications to ensure predictable application behavior.

Rinf expects developers to use Flutter exclusively for the UI layer while keeping all business logic in Rust. This approach encourages handling errors and logging directly in Rust without crossing the language boundary.[^1]

[^1]: Rinf doesn't automatically handle Rust errors for you. By explicitly managing these errors, you can make your app clearer and more robust. Rinf is designed to be a reliable framework without excessive abstractions or implicit behaviors.

Below are recommended practices for managing errors in real-world applications.

## No Panicking

We recommend that you _not_ write panicking code at all, as Rust provides the idiomatic `Result<T, E>`. Additionally, Rust _cannot_ catch panics on the web platform (`wasm32-unknown-unknown`), which can cause callers to wait forever.

```{code-block} rust
:caption: Rust
fn not_good() {
  let option = get_option();
  let value_a = option.unwrap(); // This code can panic
  let result = get_result();
  let value_b = result.expect("This code can panic");
}

fn good() -> Result<(), SomeError> {
  let option = get_option();
  let value_a = option.ok_or(SomeError)?;
  let result = get_result();
  let value_b = result?;
  Ok(())
}
```

As the Rust documentation states, [most errors aren't serious enough](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) to require the program or task to stop entirely.

## Flexible Error Type

To manage Rust errors effectively, using a flexible error type is beneficial.

Developing an app differs from creating a library, as an app may encounter a wide range of error situations. Declaring a distinct error enum variant for each potential failure can be overwhelming unless the error cases are simple enough.

Therefore, it is advisable to utilize a single, flexible error type. You can define your own or use one from `crates.io`:

- [anyhow](https://crates.io/crates/anyhow)

```{code-block} rust
:caption: Rust
use anyhow::{Context, Result};

fn get_cluster_info() -> Result<ClusterMap> {
  // `anyhow::Error` can be created from any error type.
  // By using the `?` operator, the conversion happens automatically.
  let config = std::fs::read_to_string("cluster.json")?;
  // By using the `context` method, you can wrap the original error
  // with additional information.
  let map: ClusterMap = serde_json::from_str(&config)
    .context("Failed to parse cluster configuration as JSON")?;
  Ok(map)
}
```

## Logging

You may want to log errors to the console or a file. Several crates can help with this process:

- [tracing](https://crates.io/crates/tracing)
- [async-log](https://crates.io/crates/async-log)

Using a centralized trait for logging errors can be helpful. By calling a common method for logging, you can handle the propagated error consistently. Rust automatically warns you about unused `Result`s, making it easier to handle all errors in your code.

```{code-block} rust
:caption: Rust
use anyhow::Result;
use tracing::error;

pub trait ReportError {
  fn report(self);
}

impl ReportError for Result<()> {
  fn report(self) {
    if let Err(any_error) = self {
      error!("{any_error}");
    };
  }
}

fn example_function() {
  let result: Result<()> = function_that_returns_result();
  result.report();
}
```
