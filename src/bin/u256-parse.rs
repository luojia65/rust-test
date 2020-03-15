use core::fmt;

struct U256 {
    h: u128,
    l: u128,
}

impl U256 {
    fn new(decimal: &str) -> U256 {
        let mut h = 0;
        let mut l = 0;
        for ch in decimal.chars() {
            let a = ch.to_digit(10).unwrap();
            if l > u128::max_value() / 10 {
                h += l % (u128::max_value() / 10);
                l %= u128::max_value() / 10;
            } 
            l *= 10;
            l += a as u128;
        }
        U256 { h, l }
    }
}

impl fmt::Debug for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

fn main() {

}