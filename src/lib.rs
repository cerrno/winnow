#![warn(
    unreachable_pub,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    rust_2018_idioms,
    missing_debug_implementations
)]

const DEBUG: bool = false;

#[macro_use]
macro_rules! dprintln {
    ($($arg:tt)*) => {
        if crate::DEBUG {
            println!($($arg)*)
        }
    };
}

pub mod detector;
pub mod winnowing;
