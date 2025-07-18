// traits -> same as interfaces in typescript

mod serialize_deserialize_macro;

use serialize_deserialize_macro::{Deserialize, Serializable, Swap};

struct Rect {
    width: u32,
    height: u32,
}

struct Square {
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}

// 2nd user example
// -> they always exist in pair , there is a trait and they implement it
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Name is : {}, Age is : {})", self.name, self.age)
    }
}

// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "(Name is : {}, Age is : {})", self.name, self.age)
//     }
// }

// serialization and deserialization

fn main() {
    println!("Hello, world!");
    let rect = Rect {
        width: 10,
        height: 20,
    };
    let square = Square { side: 5 };

    // 2nd user example
    let user = User {
        name: String::from("Lord Subh"),
        age: 25,
    };

    println!("{}", user); // Display trait -> its for end users so they wrote macro
    println!("{:?}", user); // Debug trait -> its for developers so they wrote macro

    println!("Area of rect: {}", rect.area());
    println!("Area of square: {}", square.area());
    println!(
        "area and perimeter of rect: {:?}",
        get_area_and_perimeter(rect)
    );
    println!(
        "area and perimeter of square: {:?}",
        get_area_and_perimeter(square)
    );

    fn get_area_and_perimeter(s: impl Shape) -> (u32, u32) {
        (s.area(), s.perimeter())
    }

    // new example 3rd

    let swap = Swap {
        qty_1: 100,
        qty_2: 200,
    };

    // Serialize
    let serialized = swap.serialize();
    println!("Serialized Swap: {:?}", serialized);

    // Deserialize
    match Swap::deserialize(serialized) {
        Ok(deserialized) => {
            println!(
                "Deserialized Swap: qty_1 = {}, qty_2 = {}",
                deserialized.qty_1, deserialized.qty_2
            );
        }
        Err(e) => {
            println!("Deserialization failed: {}", e);
        }
    }
}
