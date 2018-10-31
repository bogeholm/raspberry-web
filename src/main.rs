mod pinmap;
use pinmap::gpio2pin;

fn main() {
    let gpio = 4;
    println!("Hello, world! GPIO #{} maps to pin #{}", 
        gpio, gpio2pin(4).unwrap());
}
