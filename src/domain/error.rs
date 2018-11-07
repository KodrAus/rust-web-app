use failure;

/** The main error type */
pub type Error = failure::Error;

pub use failure::err_msg;

macro_rules! err {
    ($($err:tt)*) => {{
        error!($($err)*);
        Err(err_msg(format!($($err)*)))
    }};
}
