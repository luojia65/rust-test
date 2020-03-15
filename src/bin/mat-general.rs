// use core::ops::{Mul, MulAssign};

// #[derive(Debug)]
// struct MatBuf<T> {
//     size: [usize; 2],
//     data: Vec<T>,
// }

// const fn FPTR_SIZE<T>() -> usize {
//     core::mem::size_of::<Vec<T>>()
// } 

// struct Mat<T> {
//     size: [usize; 2],
//     data: [T; FPTR_SIZE::<T>()],
// }

// impl<'a, T> AsRef<Mat<T>> for MatBuf<T> {
//     fn as_ref(&self) -> &Mat<T> {
//         &Mat { size: self.size, data: self.data }
//     }
// }

// impl<T: Mul<Output = T>> Mul<T> for MatBuf<T> { // MatBufrix * number
//     type Output = MatBuf<T>;
//     fn mul(self, rhs: T) -> MatBuf<T> {
//         let MatBuf { size, mut data } = self;
//         for i in data.iter_mut() {
//             let mut ll: T = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
//             let mut rr: T = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
//             unsafe { core::ptr::copy(i as *const T, &mut ll as *mut T, 1) };
//             unsafe { core::ptr::copy(&rhs as *const T, &mut rr as *mut T, 1) };
//             *i = ll * rr;
//         }
//         MatBuf { size, data }
//     }
// }

// impl<'a, T: Mul<Output = T>> Mul<T> for &'a mut Mat<T> { // MatBufrix * number
//     type Output = &'a mut Mat<T>;
//     fn mul(self, rhs: T) -> &'a mut Mat<T> {
//         for i in self.data.iter_mut() {
//             let mut ll: T = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
//             let mut rr: T = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
//             unsafe { core::ptr::copy(i as *const T, &mut ll as *mut T, 1) };
//             unsafe { core::ptr::copy(&rhs as *const T, &mut rr as *mut T, 1) };
//             *i = ll * rr;
//         }
//         self
//     }
// }


// // impl<T: Mul<Output = T>> Mul<MatBuf<T>> for T {
// //     type Output = MatBuf<T>;
// //     fn mul(self, rhs: MatBuf<T>) -> MatBuf<T> {
// //         let MatBuf { size, mut data } = rhs;
// //         for i in data.iter_mut() {
// //             let mut ll: T = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
// //             let mut rr: T = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
// //             unsafe { core::ptr::copy(i as *const T, &mut ll as *mut T, 1) };
// //             unsafe { core::ptr::copy(&self as *const T, &mut rr as *mut T, 1) };
// //             *i = ll * rr;
// //         }
// //         MatBuf { size, data }
// //     }
// // }

// impl<T: MulAssign<T>> MulAssign<T> for MatBuf<T> {
//     fn mul_assign(&mut self, rhs: T) {
//         for i in self.data.iter_mut() {
//             let mut rr: T = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
//             unsafe { core::ptr::copy(&rhs as *const T, &mut rr as *mut T, 1) };
//             *i *= rr;
//         }
//     }
// }

// impl<'a, T: MulAssign<&'a T>> MulAssign<&'a T> for MatBuf<T> {
//     fn mul_assign(&mut self, rhs: &'a T) {
//         for i in self.data.iter_mut() {
//             *i *= rhs;
//         }
//     }
// }

// fn main() {
//     let mut ans = MatBuf { size: [2, 3], data: vec![1, 2, 3, 4, 5, 6] };
//     // let ans = 2 * ans;
//     ans *= 2;
//     dbg!(ans);
// }
fn main () {}
