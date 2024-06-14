/*!
Application logging.
*/

use std::time::Duration;

use emit::Emitter;

/** Initialize the global logger. */
pub fn init() {
    let _ = emit::setup().emit_to(emit_term::stdout()).init();
}

/** Flush the global logger. */
pub fn finish() {
    emit::runtime::shared().blocking_flush(Duration::from_secs(5));
}
