use core::ops::Add;

#[derive(Debug)]
struct Wrap(u32);

impl Add<u32> for Wrap {
    type Output = Wrap;
    fn add(self, rhs: u32) -> Wrap {
        Wrap(self.0 + rhs)
    }
}

impl Add<Wrap> for Wrap {
    type Output = Wrap;
    fn add(self, rhs: Wrap) -> Wrap {
        Wrap(self.0 + rhs.0)
    }
}

impl Add<Wrap> for u32 { // rust牛逼
    type Output = Wrap;
    fn add(self, rhs: Wrap) -> Wrap {
        Wrap(self + rhs.0)
    }
}

fn main() {
    let a = Wrap(1);
    let b = 2;
    println!("{:?}", a + b);
}
