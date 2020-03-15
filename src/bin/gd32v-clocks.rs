// const CK_USBFS: usize = 48_000_000;
const IRC8M: usize = 8_000_000;

#[derive(Default, Debug)]
struct Ans {
    pllsel: Option<usize>,
    pllmf: Option<usize>,
    scs: Option<usize>,
    predv0: Option<usize>,
    predv0sel: Option<usize>,
    pll1mf: Option<usize>,
    pll2mf: Option<usize>,
    predv1: Option<usize>,
    i2s12sel: Option<usize>,
    usb_ok: bool,
    ck_sys: usize,
    ck_i2s: usize,
}

fn diff(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

const PLLMF: [usize; 30] = [
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
]; // duplicate 16; special 6.5

const PREDV0: [usize; 16] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
];

const PLL1: [usize; 10] = [8, 9, 10, 11, 12, 13, 14, 15, 16, 20];

const PLL2: [usize; 10] = [8, 9, 10, 11, 12, 13, 14, 15, 16, 20];

const PREDV1: [usize; 16] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
];

fn calculate(ck_sys: usize, ck_i2s: usize, hxtal: Option<usize>) -> (Ans, usize) {
    let mut ans = Ans::default();
    let err;
    if let Some(hxtal) = hxtal {
        let err01 = diff(ck_sys, hxtal);
        let mut err10 = usize::max_value();
        let mut min_idx = (0, 0);
        for (j, div) in PREDV0.iter().enumerate() {
            for (i, mul) in PLLMF.iter().enumerate() {
                let f = hxtal * mul / div;
                let e = diff(ck_sys, f);
                if e <= err10 {
                    min_idx = (i, j);
                    err10 = e;
                    ans.ck_sys = f;
                }
            }
            let e_65 = diff(ck_sys, hxtal / 4 * 13); // * 6.5
            if e_65 <= err10 {
                min_idx = (31, j);
                err10 = e_65;
                ans.ck_sys = hxtal / 4 * 13;
            }
        }
        let mut err11 = usize::max_value();
        let mut min_idx2 = (0, 0);
        for (k, div1) in PREDV1.iter().enumerate() {
            for (l, mul1) in PLL1.iter().enumerate() {
                for (j, div) in PREDV0.iter().enumerate() {
                    for (i, mul) in PLLMF.iter().enumerate() {
                        let f = hxtal * mul * mul1 / div / div1;
                        let e = diff(ck_sys, f);
                        if e <= err11 {
                            min_idx = (i, j);
                            min_idx2 = (k, l);
                            err11 = e;
                            ans.ck_sys = f;
                        }
                    }
                    let e_65 = diff(ck_sys, hxtal * mul1 * 13 / 4 / div1); // * 6.5
                    if e_65 <= err11 {
                        min_idx = (31, j);
                        min_idx2 = (k, l);
                        err11 = e_65;
                        ans.ck_sys = hxtal * mul1 * 13 / 4 / div1;
                    }
                }
            }
        }
        let mut predv1_unset = false;
        if err10 < err01 && err10 < err11 { 
            ans.pllsel = Some(1);
            ans.pllmf = Some(min_idx.0);
            ans.predv0sel = Some(0);
            ans.predv0 = Some(min_idx.1);
            ans.scs = Some(10);
            err = err10;
        } else if err01 < err10 && err01 < err11 { 
            ans.scs = Some(01);
            err = err01;
        } else {
            ans.pllsel = Some(1);
            ans.pllmf = Some(min_idx.0);
            ans.predv0sel = Some(1);
            ans.predv0 = Some(min_idx.1);
            ans.predv1 = Some(min_idx2.0);
            ans.pll1mf = Some(min_idx2.1);
            ans.scs = Some(11);
            err = err11;
            predv1_unset = true;
            let mut err_i2s = usize::max_value();
            let mut ans_j = 0;
            let div1 = PREDV1[min_idx2.0];
            for (j, mul2) in PLL2.iter().enumerate() {
                let f = hxtal * mul2 * 2 / div1;
                let e = diff(ck_i2s, f);
                if e <= err_i2s {
                    err_i2s = e;
                    ans_j = j;
                    ans.ck_i2s = f;
                }
            }
            if diff(ans.ck_sys, ck_i2s) < err_i2s {
                ans.ck_i2s = ans.ck_sys;
                ans.i2s12sel = Some(0);
            } else {
                ans.pll2mf = Some(ans_j);
                ans.i2s12sel = Some(1);
            }
        }
        if !predv1_unset {
            let mut err_i2s = usize::max_value();
            let mut ans_i = (0, 0);
            for (i, div1) in PREDV1.iter().enumerate() {
                for (j, mul2) in PLL2.iter().enumerate() {
                    let f = hxtal * mul2 * 2 / div1;
                    let e = diff(ck_i2s, f);
                    if e <= err_i2s {
                        err_i2s = e;
                        ans_i = (i, j);
                        ans.ck_i2s = f;
                    }
                }
            }
            ans.predv1 = Some(ans_i.0);
            ans.pll2mf = Some(ans_i.1);
            ans.i2s12sel = Some(1);
        }
    } else {
        let err00 = diff(ck_sys, IRC8M);
        let mut err10 = usize::max_value();
        let mut min_idx = 0;
        for (i, n) in PLLMF.iter().enumerate() {
            let e = diff(ck_sys, IRC8M / 2 * n);
            if e <= err10 {
                min_idx = i;
                err10 = e;
                ans.ck_sys = IRC8M / 2 * n;
            }
        }
        let e_65 = diff(ck_sys, IRC8M / 4 * 13); // * 6.5
        if e_65 <= err10 {
            min_idx = 31;
            err10 = e_65;
            ans.ck_sys = IRC8M / 4 * 13;
        }
        if err10 < err00 {
            ans.pllsel = Some(0);
            ans.pllmf = Some(min_idx);
            ans.scs = Some(10);
            err = err10;
        } else {
            ans.scs = Some(00);
            ans.ck_sys = IRC8M;
            err = err00;
        }
        ans.usb_ok = false; // IRC8M
        ans.i2s12sel = Some(0);
        ans.ck_i2s = ans.ck_sys;
    }
    (ans, err)
}

fn main() {
    let ck_sys = 69_000_000;
    let ck_i2s = 44_000_000;
    let hxtal = Some(32_000_000);
    println!("{:?}", calculate(ck_sys, ck_i2s, hxtal));
}
