// use learn_rust::{gcd, area_of};

enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    // Here is a method for the `Shot` enum you just defined.
    fn points(self) -> i32 {
        // 1b. Implement this method to convert a Shot into points
        // - return 5 points if `self` is a `Shot::Bullseye`
        // - return 2 points if `self` is a `Shot::Hit(x)` where x < 3.0
        // - return 1 point if `self` is a `Shot::Hit(x)` where x >= 3.0
        // - return 0 points if `self` is a Miss
        match self {
            Shot::Bullseye => 5,
            Shot::Hit(x) => {
                if x < 3.0 {
                    2
                } else {
                    1
                }
            }
            Shot::Miss => 0,
        }
    }
}

fn main() {
    // let missiles: i32 = 8;
    // let ready: i32 = 2;
    // println!("Firing {ready} of my {missiles} missiles");

    // let result: u64 = gcd(100, 15);
    // println!("{result}");

    // let result2: i32 = area_of(100, 15);
    // println!("{result2}");

    // let num: i32 = if 8 > 10 { 100 } else { 200 };
    // println!("{num}");

    // let _args: Vec<String> = std::env::args().skip(1).collect();

    // for arg in _args {
    //     if arg == "sum" {
    //         println!("Sum")
    //     } else if arg == "double" {
    //         println!("Double")
    //     } else {
    //         println!("Unknown")
    //     }
    // }

    // let y = {
    //     let x = 5_u8;
    //     x + 1 // if ";" behind x + 1 => y = () https://doc.rust-lang.org/std/primitive.unit.html
    // };

    // println!("{y}");

    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // 2. For each coord in arrow_coords:
    //
    //   A. Call `coord.print_description()`
    //   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
    //   `coord.distance_from_center()`
    //      - Less than 1.0 -- `Shot::Bullseye`
    //      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
    //      - Greater than 5.0 -- `Shot::Miss`
    for coord in arrow_coords {
        coord.print_description();
        let distance = coord.distance_from_center();
        let variant = match distance {
            x if x < 1.0 => Shot::Bullseye,
            x if x <= 5.0 => Shot::Hit(x),
            _ => Shot::Miss,
        };
        // let variant = if distance < 1.0 {
        //     Shot::Bullseye
        // } else if distance <= 5.0 {
        //     Shot::Hit(distance)
        // } else {
        //     Shot::Miss
        // };
        shots.push(variant);
    }

    let mut total = 0;
    // 3. Finally, loop through each shot in shots and add its points to total
    for shot in shots {
        total += shot.points();
    }

    println!("Final point total is: {}", total);

}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y
        );
    }
}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}
