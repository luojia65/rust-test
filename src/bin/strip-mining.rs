const L: usize = 8;

fn strip_mining(n: usize) {
    println!("=== Begin with n={} ===", n);
    let mut lo = 0;
    for _ in 0..n/L {
        println!("Processing {} to {}", lo, lo + L - 1);
        lo += L;
    }
    for i in lo..n {
        println!("Processing {}", i);
    }
}

fn main() {
    strip_mining(17);
    strip_mining(16);
    strip_mining(15);
    strip_mining(0);
    strip_mining(1);
}
