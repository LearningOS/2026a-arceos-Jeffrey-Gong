#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

use axstd::os::arceos::modules::axlog::{__print_colored, ColorCode};
#[cfg(feature = "axstd")]

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    __print_colored(ColorCode::Green, "[WithColor]: Hello, Arceos!\n");
}
