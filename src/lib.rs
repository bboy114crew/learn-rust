use std::mem;

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // let t:u64 = m;
            // m = n;
            // n = t;
            mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n // return n;
}

pub fn area_of(x: i32, y: i32) -> i32 {
    x * y
}
