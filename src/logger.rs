/*!
Application logging.

This module wraps a logger to produce line-delimited JSON instead of regular text.
That makes it a bit nicer to consume through some sidecar or ambient environment
that collects and surfaces log events.
*/

use std::time::Duration;

use emit::Emitter;

/** The environment variable to read the level filter from. */
pub const LOG_LEVEL_ENV: &str = "LOG_LEVEL";
/** The environment variable to read the style info from. */
pub const LOG_STYLE_ENV: &str = "LOG_STYLE";

/** Initialize the global logger. */
pub fn init() {
    let _ = emit::setup().emit_to(emit_term::stdout()).init();
}

/** Flush the global logger. */
pub fn finish() {
    emit::runtime::shared().blocking_flush(Duration::from_secs(5));
}
