//! A simple crate to test the use of serde-json serialization. 
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

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
    let j = serde_json::to_string(&a);

    // write to file
    let mut file = File::create("serialized.txt").unwrap();
    file.write_all(j.as_ref().unwrap().to_owned().as_bytes())
        .unwrap();

    // read from file
    let file = File::open("serialized.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    // deserialize
    let b: Move = serde_json::from_str(&contents).unwrap();

    println!("Movement parameters: {:?}", a);
    println!("Deserialized Movement parameters: {:?}", b);
}
