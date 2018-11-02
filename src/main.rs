// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

extern crate dotenv;

mod pinmap;

use pinmap::gpio2pin;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let gpio = 4;
    println!("Hello, world! GPIO #{} maps to pin #{}", 
        gpio, gpio2pin(4).unwrap());
}
