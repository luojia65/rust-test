use num::bigint::BigUint;
use num::Num;
use core::ops::*;
use core::cmp::Ordering;

#[derive(Debug, Clone)]
struct Big {
    uint: BigUint,
}

impl Big {
    fn new(a: usize) -> Big {
        Big { uint: BigUint::from(a) }
    }
    fn parse_string(string: &str) -> Big {
        Big { uint: BigUint::from_str_radix(string, 10).unwrap() }
    }
    fn max(a: Big, b: Big) -> Big {
        if a > b { a } else { b }
    }
    fn rand() -> Big {
        let a: usize = rand::random();
        Big::new(a)
    }
}

impl Mul<usize> for Big {
    type Output = Big;
    fn mul(mut self, b: usize) -> Big { 
        self.uint = self.uint * BigUint::from(b);
        self
    }
}
impl Div<Big> for Big {
    type Output = Big;
    fn div(mut self, rhs: Big) -> Big {
        self.uint = self.uint / rhs.uint;
        self
    }
}
impl DivAssign<usize> for Big {
    fn div_assign(&mut self, rhs: usize) {
        self.uint /= rhs;
    }
}
impl Rem<Big> for Big {
    type Output = Big;
    fn rem(mut self, rhs: Big) -> Big {
        self.uint %= rhs.uint;
        self
    }
}
impl Rem<usize> for Big {
    type Output = Big;
    fn rem(mut self, rhs: usize) -> Big {
        self.uint = self.uint % rhs;
        self
    }
}
impl RemAssign<Big> for Big {
    fn rem_assign(&mut self, rhs: Big) {
        self.uint %= rhs.uint;
    }
}
impl PartialEq<usize> for Big {
    fn eq(&self, other: &usize) -> bool {
        self.uint.eq(&BigUint::from(*other))
    }
}
impl PartialOrd<usize> for Big {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.uint.partial_cmp(&BigUint::from(*other))
    }
}
impl PartialEq<Big> for Big {
    fn eq(&self, other: &Big) -> bool {
        self.uint.eq(&other.uint)
    }
}
impl PartialOrd<Big> for Big {
    fn partial_cmp(&self, other: &Big) -> Option<Ordering> {
        self.uint.partial_cmp(&other.uint)
    }
}
impl BitAnd<usize> for Big {
    type Output = Big;
    fn bitand(self, rhs: usize) -> Big {
        Big { uint: self.uint.bitand(BigUint::from(rhs)) }
    }
}
impl ShrAssign<usize> for Big {
    fn shr_assign(&mut self, rhs: usize) {
        self.uint.shr_assign(rhs)
    }
}
impl Add<Big> for Big {
    type Output = Big;
    fn add(self, rhs: Big) -> Big {
        Big { uint: self.uint.add(rhs.uint) }
    }
}
impl Add<usize> for Big {
    type Output = Big;
    fn add(self, rhs: usize) -> Big {
        Big { uint: self.uint.add(BigUint::from(rhs)) }
    }
}
impl Sub<usize> for Big {
    type Output = Big;
    fn sub(self, rhs: usize) -> Big {
        Big { uint: self.uint.sub(BigUint::from(rhs as u64)) }
    }
}
impl Sub<Big> for Big {
    type Output = Big;
    fn sub(self, rhs: Big) -> Big {
        Big { uint: self.uint.sub(rhs.uint) }
    }
}

// a^b mod m
fn modpow(mut a: Big, mut b: Big, m: Big) -> Big {
    let mut ans = Big::new(1);
    a %= m.clone();
    while b.clone() > 0 {
        if b.clone() & 1 != 0 {
            ans = modmul(ans, a.clone(), m.clone());
        }
        b >>= 1;
        a = modmul(a.clone(), a.clone(), m.clone());
    }
    ans % m
}

// ab mod m
fn modmul(mut a: Big, mut b: Big, m: Big) -> Big {
    let mut ans =  Big::new(0);
    a %= m.clone();
    while b.clone() > 0 {
        if b.clone() & 1 != 0 {
            ans = (ans + a.clone()) % m.clone(); 
        }
        b >>= 1;
        a = (a.clone() + a.clone()) % m.clone();
    }
    ans
}

fn is_prime_miller_rabin(n: Big) -> bool {
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    if n.clone() % 2 == 0 || n.clone() % 3 == 0 || n.clone() % 5 == 0 || n.clone() % 7 == 0 {
        return false;
    }
    let mut cnt: usize = 0;
    let mut d: Big = n.clone() - 1;
    while d.clone() % 2 == 0 {
        cnt += 1;
        d /= 2;
    }
    for &i in [0, 2, 7, 61].iter() {
        if n.clone() == i {
            return true;
        }
        let i = Big::new(i);
        let mut x = modpow(i, d.clone(), n.clone());
        if x.clone() % n.clone() != 1 && x.clone() % n.clone() != n.clone() - 1 {
            let mut flag = false;
            for _ in 0..cnt {
                x = modmul(x.clone(), x.clone(), n.clone());
                if x == d.clone() - 1 {
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

fn gcd(a: Big, b: Big) -> Big {
    if b == 0 { a } else { gcd(b.clone(), a % b.clone()) }
}

fn diff(a: Big, b: Big) -> Big {
    if a > b { a - b } else { b - a }
}

fn modmuladd(a: Big, b: Big, c: Big, m: Big) -> Big {
    (modmul(a, b, m.clone()) + c) % m
}

fn pollard_rho(n: Big, mut pre: Big) -> Big {
    if is_prime_miller_rabin(n.clone()) {
        return Big::max(pre, n.clone());
    }
    let mut t1 = Big::rand() % (n.clone() - 1) + 1; // [1, n)
    let b = Big::rand() % (n.clone() - 1) + 1; 
    let mut t2 = modmuladd(t1.clone(), t1.clone(), b.clone(), n.clone());
    while t1 != t2 {
        let t = gcd(diff(t1.clone(), t2.clone()), n.clone());
        if t != 1 && t != n {
            pre = pollard_rho(t.clone(), pre);
            pre = pollard_rho(n.clone() / t.clone(), pre);
        }
        t1 = modmuladd(t1.clone(), t1.clone(), b.clone(), n.clone());
        t2 = modmuladd(t2.clone(), t2.clone(), b.clone(), n.clone());
        t2 = modmuladd(t2.clone(), t2.clone(), b.clone(), n.clone());
    }
    pre
}

fn main() {
    let mut a = Big::new(0);
    while a == 0 {
        a = pollard_rho(Big::parse_string("92633224202293464529490877582423584974729"), a);
    }
    println!("{:?}", a);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_modpow() {
//         assert_eq!(modpow(1, 1, 1), 0);
//         assert_eq!(modpow(2, 100, 7), 2);
//     }

//     fn is_prime_sqrt(a: u128) -> bool {
//         for i in 2..((a as f64).sqrt() as u128) {
//             if a % i == 0 {
//                 return false;
//             }
//         }
//         return true;
//     }

//     #[test]
//     fn test_is_prime() {
//         assert!(is_prime_miller_rabin(2));
//         assert!(is_prime_miller_rabin(3));
//         let mut tests = 0;
//         let mut matches = 0;
//         for i in 2..1000000 {
//             if is_prime_sqrt(i) == is_prime_miller_rabin(i) {
//                 matches += 1;
//             }
//             tests += 1;
//         }
//         println!("tests: {}, matches: {}", tests, matches);
//         assert!(matches as f32 > 0.9 * tests as f32);
//     }

//     #[test]
//     fn test_pollard_rho() {
//         let mut a = 0;
//         while a == 0 {
//             a = pollard_rho(15, a);
//         }
//         println!("{}", a);
//     }
// }
