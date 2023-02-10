use learn_rust::{gcd, area_of};

fn main() {
    let missiles: i32 = 8;
    let ready: i32 = 2;
    println!("Firing {ready} of my {missiles} missiles");

    let result: u64 = gcd(100, 15);
    println!("{result}");

    let result2: i32 = area_of(100, 15);
    println!("{result2}");

    let num: i32 = if 8 > 10 { 100 } else { 200 };
    println!("{num}");

    let _args: Vec<String> = std::env::args().skip(1).collect();

    for arg in _args {
        if arg == "sum" {
            println!("Sum")
        } else if arg == "double" {
            println!("Double")
        } else {
            println!("Unknown")
        }
    }

    let y = {
        let x = 5_u8;
        x + 1 // if ";" behind x + 1 => y = () https://doc.rust-lang.org/std/primitive.unit.html
    };

    println!("{y}");

}
