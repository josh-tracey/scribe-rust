# scribe-rust
Scribe-rust is a straightforward and color-coded logging library for Rust applications. It provides an easy-to-use API to log messages with different severity levels. Here's what you can expect from scribe-rust:

- **Flexible Log Levels:** With five different levels (Trace, Debug, Info, Warn, Error), you have granular control over the logging output based on the severity of the messages. You can use these levels to differentiate critical errors from minor debugging information.
- **Intuitive Logging Methods:** The library provides a set of logging methods corresponding to each level (trace, debug, info, warn, error). You just need to call the appropriate method with your message, and scribe-rust takes care of the rest.
- **Color-Coded Output:** Scribe-rust makes reading logs easier by color-coding the output based on the log level. For example, 'Error' messages are displayed in red, and 'Info' messages in green. This visual cue can help you spot critical issues more quickly.
- **Environment Variable Support:** You can control the log level of your application at runtime using the LOG_LEVEL environment variable. This feature makes it easy to adjust the verbosity of your logs without changing the code.

#### As an Example

```rust
let logger = Logger::default();

logger.info("Starting My Service!");

#[derive(Debug)]
pub enum ActionType {
  Lambda,
  Webhook,
}

let action_type = ActionType::Lambda;

logger.debug(&format!("Executing {:?} Action...", action_type));
```


**Please note: as of the current version, scribe-rust is designed for simplicity and ease of use. It focuses on console output and does not yet support logging to files, remote systems, or custom message formatting. Future enhancements may add these and other advanced features.
