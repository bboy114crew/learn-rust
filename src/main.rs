use std::mem;

fn gcd(mut n: u64, mut m: u64) -> u64 {
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

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let missiles: i32 = 8;
    let ready: i32 = 2;
    println!("Firing {ready} of my {missiles} missiles");

    let result: u64 = gcd(100, 15);
    println!("{result}");

    let result2: i32 = area_of(100, 15);
    println!("{result2}");
}