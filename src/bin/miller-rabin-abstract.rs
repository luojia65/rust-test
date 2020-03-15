trait ModPow: Sized {
    fn zero() -> Self;
    fn identity() -> Self;
    fn is_zero(&self) -> bool;
    fn lowest_bit(&self) -> bool;
    fn shr_assign_one(&mut self);
    fn shl_assign_one(&mut self);
    fn rem_assign(&mut self, m: &Self);
    fn add_assign(&mut self, m: &Self);
}

impl ModPow for usize {
    fn zero() -> Self { 0 }
    fn identity() -> Self { 1 }
    fn is_zero(&self) -> bool { *self == 0 }
    fn lowest_bit(&self) -> bool { self & 1 == 0 }
    fn shr_assign_one(&mut self) { *self >>= 1 }
    fn shl_assign_one(&mut self) { *self <<= 1 }
    fn rem_assign(&mut self, m: &Self) { *self %= *m }
    fn add_assign(&mut self, m: &Self) { *self += *m }
}

// a^b mod m
fn modpow<T: ModPow + Clone>(mut a: T, mut b: T, m: &T) -> T {
    let mut ans = T::identity();
    a.rem_assign(&m);
    while !b.is_zero() {
        if b.lowest_bit() {
            ans = modmul(ans, a.clone(), m);
        }
        b.shr_assign_one();
        a = modmul(a.clone(), a, m);
    }
    ans.rem_assign(&m);
    ans
}

// ab mod m
fn modmul<T: ModPow>(mut a: T, mut b: T, m: &T) -> T {
    let mut ans = T::zero();
    a.rem_assign(&m);
    while !b.is_zero() {
        if b.lowest_bit() {
            ans.add_assign(&a);
            ans.rem_assign(m); 
        }
        b.shr_assign_one();
        a.shl_assign_one();
        a.rem_assign(&m);
    }
    ans
}

fn main() {}
