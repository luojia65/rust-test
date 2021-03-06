trait ModPow: Sized {
    fn zero() -> Self;
    fn identity() -> Self;
    fn is_zero(&self) -> bool;
    fn has_lowest_bit(&self) -> bool;
    fn shr_assign_one(&mut self);
    fn shl_assign_one(&mut self);
    fn rem_assign(&mut self, m: &Self);
    fn add_assign(&mut self, m: &Self);
}

// a^b mod m
fn modpow<T: ModPow + Clone>(mut a: T, mut b: T, m: &T) -> T {
    let mut ans = T::identity();
    a.rem_assign(&m);
    while !b.is_zero() {
        if b.has_lowest_bit() {
            ans = modmul(ans, a.clone(), &m);
        }
        b.shr_assign_one();
        a = modmul(a.clone(), a, &m);
    }
    ans.rem_assign(&m);
    ans
}

// ab mod m
fn modmul<T: ModPow>(mut a: T, mut b: T, m: &T) -> T {
    let mut ans = T::zero();
    a.rem_assign(&m);
    while !b.is_zero() {
        if b.has_lowest_bit() {
            ans.add_assign(&a);
            ans.rem_assign(&m); 
        }
        b.shr_assign_one();
        a.shl_assign_one();
        a.rem_assign(&m);
    }
    ans
}

impl ModPow for usize {
    fn zero() -> Self { 0 }
    fn identity() -> Self { 1 }
    fn is_zero(&self) -> bool { *self == 0 }
    fn has_lowest_bit(&self) -> bool { *self & 1 != 0 }
    fn shr_assign_one(&mut self) { *self >>= 1 }
    fn shl_assign_one(&mut self) { *self <<= 1 }
    fn rem_assign(&mut self, m: &Self) { *self %= *m }
    fn add_assign(&mut self, m: &Self) { *self += *m }
}

trait MillerRabin: 'static + ModPow {
    fn is_simple_prime(&self) -> bool;
    fn is_simple_composite(&self) -> bool;
    fn dec(&self) -> Self;
    fn simple_primes() -> &'static [Self];
    fn eq(&self, other: &Self) -> bool;
    fn rem(&self, other: &Self) -> Self;
}

fn miller_rabin<T: MillerRabin + Clone>(n: &T) -> bool {
    if n.is_simple_prime() { return true }
    if n.is_simple_composite() { return false }
    let mut cnt = 0;
    let mut d = n.dec();
    while !d.has_lowest_bit() {
        cnt += 1;
        d.shr_assign_one();
    }
    for i in T::simple_primes().iter() {
        if n.eq(i) {
            return true;
        }
        let mut x = modpow(i.clone(), d.clone(), &n);
        let rem = x.rem(&n);
        if !rem.eq(&T::identity()) && !rem.eq(&n.dec()) {
            let mut flag = false;
            for _ in 0..cnt {
                x = modmul(x.clone(), x, &n);
                if x.eq(&d.dec()) {
                    flag = true;
                    break;
                }
            }
            if !flag {
                return false;
            }
        }
    }
    return true;
}

impl MillerRabin for usize {
    fn is_simple_prime(&self) -> bool { *self == 2 || *self == 3 }
    fn is_simple_composite(&self) -> bool { self % 2 == 0 || self % 3 == 0 }
    fn dec(&self) -> Self { self - 1 }
    fn simple_primes() -> &'static [Self] { &[0, 2, 7, 61] }
    fn eq(&self, other: &Self) -> bool { self == other }
    fn rem(&self, other: &Self) -> Self { self % other }
}

trait PollardRho: MillerRabin {
    fn rand_region(limit: &Self) -> Self;
    fn gcd(a: &Self, b: &Self) -> Self;
    fn diff(a: &Self, b: &Self) -> Self;
    fn max(a: &Self, b: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
}

fn pollard_rho<T: PollardRho + Clone>(n: T, c: T) -> T {
    let mut x = T::rand_region(&n.dec()); //[1, n-2]
    let mut y = x.clone(); 
    let mut i = 1;
    let mut k = 2;
    loop {
        i += 1;
        x = modmul(x.clone(), x.clone(), &n);
        x.add_assign(&c);
        x.add_assign(&n);
        let d = T::gcd(&T::diff(&y, &x), &n);
        if !d.eq(&T::identity()) && !d.eq(&n.dec()) {
            return d;
        }
        if y.eq(&x) {
            return n;
        }
        if i == k {
            y = x.clone();
            k <<= 1;
        }
    }
}

fn not_really_random() -> usize {
    static mut SEED: usize = 1233344556;
    unsafe {
        SEED = usize::wrapping_mul(SEED, 27755);
        SEED = usize::wrapping_add(SEED, 9987);
        SEED
    }
}

impl PollardRho for usize {
    fn rand_region(limit: &Self) -> Self { 
        not_really_random() % (limit - 1) + 1
    }
    fn gcd(a: &Self, b: &Self) -> Self {     
        if *b == 0 { *a } else { Self::gcd(b, &(a % b)) } 
    }
    fn diff(a: &Self, b: &Self) -> Self {
        if a > b { a - b } else { b - a }
    }
    fn max(a: &Self, b: &Self) -> Self {
        if a > b { *a } else { *b }
    }
    fn div(&self, other: &Self) -> Self {
        self / other
    }
}

fn do_find(n: usize, mut c: usize) {
    if n == 1 {
        return;
    }
    if miller_rabin(&n) {
        println!("{}", n);
        return;
    }
    let mut a = n;
    while a >= n {
        a = pollard_rho(a, c);
        c -= 1;
    }
    do_find(a, c);
    do_find(n / a, c);
}

fn main() {
    do_find(99999997, 10000000);
}