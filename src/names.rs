//! Holds global constants that can be used throughout the program.

use crate::*;

/// Holds all the names we will rotate through.
const NAMES: [&str; 20] = [
  "Jim",
  "Crystal",
  "Nathan",
  "Samuel",
  "Anna",
  "Tom",
  "Diane",
  "Tina",
  "David",
  "Andy",
  "Joey",
  "Gary",
  "Pat",
  "Chris",
  "Sarah",
  "Elizabeth",
  "Luke",
  "Bethany",
  "Deborah",
  "Ezra",
];

/// This gets a random name from my NAMES constant
pub fn get_random_name(room: Room) -> String {
  for name in NAMES.iter() {
    let name = format!("{}_{}", name, room.name());
    if let Some(_) = game::creeps::get(&name) {
      continue;
    }
    return name;
  }
  // We have run out of names:
  String::new()
}
