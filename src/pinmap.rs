#[allow(dead_code)]
pub fn pin2gpio(pin: i32) -> Option<i32> {
    // http://pi4j.com/pins/model-3b-rev1.html
    match pin{
        11 => Some(0),
        12 => Some(1),
        13 => Some(2),
        15 => Some(3),
        16 => Some(4),
        18 => Some(5),
        22 => Some(6),
        7  => Some(7),
        3  => Some(8),
        5  => Some(9),
        24 => Some(10),
        26 => Some(11),
        19 => Some(12),
        21 => Some(13),
        23 => Some(14),
        8  => Some(15),
        10 => Some(16),
        // GPIO 17 - 20 do not exist
        29 => Some(21),
        31 => Some(22),
        33 => Some(23),
        35 => Some(24),
        37 => Some(25),
        32 => Some(26),
        36 => Some(27),
        38 => Some(28),
        40 => Some(29),
        27 => Some(30),
        28 => Some(31),
        _ => None
    }
}

#[allow(dead_code)]
pub fn gpio2pin(gpio: i32) -> Option<i32> {
    // http://pi4j.com/pins/model-3b-rev1.html
    match gpio{
        0 =>  Some(11),
        1 =>  Some(12),
        2 =>  Some(13),
        3 =>  Some(15),
        4 =>  Some(16),
        5 =>  Some(18),
        6 =>  Some(22),
        7 =>  Some(7),
        8 =>  Some(3),
        9 =>  Some(5),
        10 => Some(24),
        11 => Some(26),
        12 => Some(19),
        13 => Some(21),
        14 => Some(23),
        15 => Some(8),
        16 => Some(10),
        // GPIO 17 - 20 do not exist
        21 => Some(29),
        22 => Some(31),
        23 => Some(33),
        24 => Some(35),
        25 => Some(37),
        26 => Some(32),
        27 => Some(36),
        28 => Some(38),
        29 => Some(40),
        30 => Some(27),
        31 => Some(28),
        _ => None
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test] 
    fn gpio2pin2gpio_should_be_equal() {
        let gpio_array: [i32; 28] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 
        // GPIO 17 - 20 do not exist
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

        for gpio_test in gpio_array.iter() {

            let pin = gpio2pin(*gpio_test).unwrap();
            let gpio_res = pin2gpio(pin).unwrap();
            assert_eq!(*gpio_test, gpio_res);
        }
    }
}