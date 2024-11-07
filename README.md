# scribe-rust

Scribe-rust is a straightforward and color-coded logging library for Rust applications. It provides an easy-to-use API to log messages with different severity levels. Here's what you can expect from scribe-rust:

- **Flexible Log Levels:** With five different levels (Trace, Debug, Info, Warn, Error), you have granular control over the logging output based on the severity of the messages. This allows you to differentiate critical errors from minor debugging information.

- **Intuitive Logging Methods:** The library provides a set of logging methods corresponding to each level (`trace`, `debug`, `info`, `warn`, `error`). You can call the appropriate method with your message, and scribe-rust handles the rest.

- **Color-Coded Output:** Scribe-rust enhances readability by color-coding the output based on the log level. For example, 'Error' messages are displayed in red, and 'Info' messages in green. This visual cue helps you spot critical issues more quickly.

- **Environment Variable Support:** You can control the log level of your application at runtime using the `LOG_LEVEL` environment variable. This feature makes it easy to adjust the verbosity of your logs without changing the code.

## Integration with `tracing`

Scribe-rust integrates seamlessly with the [`tracing`](https://docs.rs/tracing/latest/tracing/) crate, a framework for instrumenting Rust programs to collect structured, event-based diagnostic information. By implementing a custom `Layer`, scribe-rust can process and log events emitted by `tracing`.

### Custom Logger Layer

The `CustomLoggerLayer` struct implements the `Layer` trait from `tracing_subscriber`, allowing it to intercept and log events. It utilizes a `FieldVisitor` to extract structured data from events, which is then serialized into JSON for detailed logging.

Here's an example of how to set up the custom logger layer:

```rust
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::prelude::*;

// Assuming Logger and CustomLoggerLayer are defined as in your code

fn main() {
    // Initialize the logger
    let logger = Logger::default();

    // Create the custom layer
    let custom_layer = CustomLoggerLayer {
        logger: Arc::clone(&logger),
    };

    // Set up the subscriber with the custom layer
    let subscriber = tracing_subscriber::Registry::default().with(custom_layer);

    // Set the global default subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    // Example usage
    info!("This is an info message.");
    logger.warn("This is a warning message.");
}
