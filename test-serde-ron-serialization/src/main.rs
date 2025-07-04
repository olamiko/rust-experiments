//! A simple crate to test the use of serde-ron serialization.
use serde::{Deserialize, Serialize};
use std::str;

/// Struct indicating a single movement of a chess piece indicating its direction and the number of squares moved
#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    number_of_squares: u16,
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Move {
    fn new(number_of_squares: u16, direction: Direction) -> Self {
        Move {
            number_of_squares,
            direction,
        }
    }
}

// Heavy use of unwrap()! No error handling due to its experimental nature
fn main() {
    let a = Move::new(1, Direction::SOUTH);
    let j = ron::to_string(&a).unwrap();

    println!("Vector serialization (string) is {:?}", j);

    // convert to vec buffer. Can't find a method to write directly to vec,
    // so conversion is needed
    let buffer = j.as_bytes();

    println!("Vector serialization (vec<u8> buffer) is {:?}", buffer);

    // convert back to string
    let str_value = str::from_utf8(buffer).unwrap();

    // deserialize
    let b: Move = ron::from_str(str_value).unwrap();

    println!("Movement parameters: {:?}", a);
    println!("Deserialized Movement parameters: {:?}", b);
}
