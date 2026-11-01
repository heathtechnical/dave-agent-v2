//! Diagnostic output via `tracing`, written to stderr.

use tracing::level_filters::LevelFilter;

/// The most verbose level that will be emitted.
pub fn max_level(debug: bool) -> LevelFilter {
    if debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    }
}

/// Install the global subscriber. Call once, early in `main`.
///
/// Fails only if a global subscriber is already set.
pub fn init(debug: bool) -> Result<(), tracing_subscriber::util::TryInitError> {
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(max_level(debug))
        .with_target(debug)
        .finish()
        .try_init()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_enables_debug_level() {
        assert_eq!(max_level(true), LevelFilter::DEBUG);
    }

    #[test]
    fn default_is_info() {
        assert_eq!(max_level(false), LevelFilter::INFO);
    }

    #[test]
    fn second_init_fails_without_panicking() {
        // Whether or not another test got there first, a second call must fail cleanly.
        let _ = init(false);
        assert!(init(false).is_err());
    }
}
