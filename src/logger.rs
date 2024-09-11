/*!
Application logging.
*/

use std::time::Duration;

use emit::Emitter;

/** Initialize the global logger. */
pub fn init() {
    let _ = emit::setup()
        .emit_to(emit_term::stdout())
        .emit_to(emit_otlp::new()
            .logs(emit_otlp::logs_grpc_proto("http://localhost:4319"))
            .traces(emit_otlp::traces_grpc_proto("http://localhost:4319"))
            .metrics(emit_otlp::metrics_grpc_proto("http://localhost:4319"))
            .spawn()
         )
        .init();
}

/** Flush the global logger. */
pub fn finish() {
    emit::runtime::shared().blocking_flush(Duration::from_secs(5));
}
