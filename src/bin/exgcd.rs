use core::ops::{MulAssign, Add, Sub};
use core::fmt;

// group <T; *> has zero and identity element
trait Exgcd: MulAssign<Self> + Sub<Output = Self> + Sized {
    fn zero() -> Self;
    fn identity() -> Self;
    fn is_zero(&self) -> bool;
    fn div_rem(a: &Self, b: &Self) -> (Self, Self);
}

fn exgcd<T: Exgcd + Clone>(a: T, b: T) -> (T, T) {
    if T::is_zero(&b) {
        return (T::identity(), T::zero())
    }     
    let (mut d, r) = T::div_rem(&a, &b);
    let (x, y) = exgcd(b, r);
    d *= y.clone();
    return (y, T::sub(x, d))
}

impl Exgcd for i32 {
    fn zero() -> Self { 0 }
    fn identity() -> Self { 1 }
    fn is_zero(&self) -> bool {
        *self == 0
    }
    fn div_rem(a: &Self, b: &Self) -> (Self, Self) {
        (a / b, a % b)
    }
}

#[derive(Clone)]
struct Polynome {
    parts: Vec<f64>,
}

impl Polynome {
    const fn new(parts: Vec<f64>) -> Self {
        Self { parts }
    }
    
    fn len(&self) -> usize {
        let mut ans = self.parts.len();
        let mut iter = self.parts.iter().rev();
        while let Some(a) = iter.next() {
            if a == &0.0 {
                ans -= 1;
            } else {
                break
            }
        }
        ans
    }
}

impl Add for Polynome {
    type Output = Polynome;
    
    fn add(self, other: Polynome) -> Polynome {
        let len = usize::max(self.len(), other.len());
        let mut v = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.parts.get(i).unwrap_or(&0.0);
            let b = other.parts.get(i).unwrap_or(&0.0);
            v.push(*a + *b)
        }
        v.into()
    }
}

impl Sub for Polynome {
    type Output = Polynome;
    
    fn sub(self, other: Polynome) -> Polynome {
        let len = usize::max(self.len(), other.len());
        let mut v = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.parts.get(i).unwrap_or(&0.0);
            let b = other.parts.get(i).unwrap_or(&0.0);
            v.push(*a - *b)
        }
        v.into()
    }
}

impl MulAssign<Polynome> for Polynome {
    fn mul_assign(&mut self, rhs: Polynome) {
        let mut ans = vec![0.0; self.len() + rhs.len()];
        for (i, n1) in self.parts.iter().enumerate() {
            for (j, n2) in rhs.parts.iter().enumerate() {
                ans[i + j] += n1 * n2; 
            }
        }
        self.parts = ans;
    }
}

impl Exgcd for Polynome {
    fn zero() -> Self { Polynome::new(vec![]) }
    fn identity() -> Self { Polynome::new(vec![1.0]) }
    fn is_zero(&self) -> bool {
        self.len() == 0
    }
    fn div_rem(a: &Self, b: &Self) -> (Self, Self) {
        let mut ans = Polynome::new(vec![]);
        let mut a = a.clone();
        while a.len() >= b.len() {
            let n = a.len() - b.len();
            // println!("{}; {}", a.len(), b.len());
            let num = a.parts[a.len() - 1] / b.parts[b.len() - 1];
            // println!("{:?} {:?}", a.parts, b.parts);
            let mut v = vec![0.0; n + 1];
            v[n] = num;
            // println!("{:?}", v);
            let d = Polynome::new(v);
            // println!("{}", d.clone());
            let mut dd = d.clone();
            dd *= b.clone();
            a = a - dd;
            ans = ans + d;
        }
        (ans, a)
    }
}

impl From<Vec<f64>> for Polynome {
    fn from(src: Vec<f64>) -> Self {
        Polynome { parts: src }
    }
}

const CHARS: &'static str = "⁰¹²³⁴⁵⁶⁷⁸⁹";

impl fmt::Display for Polynome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, num) in self.parts.iter().enumerate().take(self.len()) {
            if *num == 0.0 {
                continue;
            }
            let mut a = String::new();
            if i > 1 {
                let mut u = i;
                while u > 0 {
                    a.push(CHARS.chars().nth(u % 10).unwrap());
                    u /= 10;
                }
            }
            if i != 0 {
                a.push('x');
            }
            write!(f, "{}{}", num, a.chars().rev().collect::<String>())?;
            if i != self.len() - 1 {
                write!(f, " + ")?;
            }
        }
        Ok(())
    }
}

fn main() {
    let a = 12345;
    let b = 345;
    println!("{:?}", exgcd(a, b));    
    // let p = Polynome::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    // println!("{}", p);
    // let p1 = Polynome::from(vec![1, 2, 3, 4]);
    // let p2 = Polynome::from(vec![1, 2]);
    // println!("{}", p1.clone());
    // println!("{}", p2.clone());
    // println!("{}", p1.clone() - p2.clone());
    // println!("{}", p2.clone() - p1.clone());
    // let mut p3 = p2.clone();
    // println!("p3 = {}", p3.clone());
    // p3 *= p3.clone();
    // println!("p3^2 = {}", p3.clone());
    
    let aa = Polynome::from(vec![1.0, 3.0, 3.0, 1.0, 20.0]);
    let bb = Polynome::from(vec![0.0, 0.0, 7.0]);
    let (d, r) = Polynome::div_rem(&aa, &bb);
    println!("{} …… {}", d, r);
    let (x, y) = exgcd(aa, bb);
    println!("x={}; y={}", x, y);
}

// #[cfg(test)]
// mod tests {
//     use super::Polynome;
//     #[test]
//     fn po_len() {
//         assert_eq!(Polynome::from(vec![]).len(), 0);
//         assert_eq!(Polynome::from(vec![0]).len(), 0);
//         assert_eq!(Polynome::from(vec![1, 0, 0]).len(), 1);
//         assert_eq!(Polynome::from(vec![0, 1, 0]).len(), 2);
//     }
// }
