/*!
Application logging.

This module wraps a logger to produce line-delimited JSON instead of regular text.
That makes it a bit nicer to consume through some sidecar or ambient environment
that collects and surfaces log events.
*/

use std::{
    fmt::Arguments,
    io::Write,
};

use serde::ser::Serializer;

use log::Level;

use env_logger::{
    fmt::Timestamp,
    Builder,
    Env,
};

/** The environment variable to read the level filter from. */
pub const LOG_LEVEL_ENV: &str = "LOG_LEVEL";
/** The environment variable to read the style info from. */
pub const LOG_STYLE_ENV: &str = "LOG_STYLE";

/** Initialize the global logger. */
pub fn init() {
    let env = Env::default()
        .filter_or(LOG_LEVEL_ENV, "debug")
        .write_style(LOG_STYLE_ENV);

    Builder::from_env(env)
        .format(|mut buf, record| {
            let record = SerializeRecord {
                ts: buf.timestamp(),
                lvl: record.level(),
                module_path: record.module_path(),
                msg: record.args(),
            };

            serde_json::to_writer(&mut buf, &record)?;
            writeln!(buf)
        })
        .init();
}

#[derive(Serialize)]
struct SerializeRecord<'a> {
    #[serde(serialize_with = "serialize_ts")]
    #[serde(rename = "@t")]
    ts: Timestamp,
    #[serde(rename = "@l")]
    lvl: Level,
    #[serde(skip_serializing_if = "Option::is_none")]
    module_path: Option<&'a str>,
    #[serde(rename = "@m")]
    msg: &'a Arguments<'a>,
}

fn serialize_ts<S>(ts: &Timestamp, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_str(ts)
}
