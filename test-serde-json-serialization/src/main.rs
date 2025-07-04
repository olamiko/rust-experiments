//! A simple crate to test the use of serde-json serialization.
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

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

    // File serialization
    let file = File::create("serialized.txt").unwrap();
    let _ = serde_json::to_writer(file, &a);

    let file = File::open("serialized.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let b: Move = serde_json::from_reader(buf_reader).unwrap();

    println!("(File) Movement parameters: {:?}", a);
    println!("(File) Deserialized Movement parameters: {:?}", b);

    // vector serialization
    let ser_vec = serde_json::to_vec(&a);
    let unwrapped_vec = ser_vec.unwrap();

    let b: Move = serde_json::from_slice(&unwrapped_vec).unwrap();

    println!("(Vec serialization) Movement parameters: {:?}", a);
    println!(
        "(Vec serialization) Deserialized Movement parameters: {:?}",
        b
    );
}
