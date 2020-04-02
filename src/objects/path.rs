//! Easy path finding tool it wraps up the position of a thing
//! then provides tools to find the nearest other thing.
use std::u32::MAX;

use crate::*;

/// Easy path management
pub struct Path {
  /// The origin for navigation
  origin: Position,
}

impl Path {
  /// Returns a new path given anything that has a position
  pub fn new<T>(pos: T) -> Self
  where
    T: HasPosition,
  {
    let origin = pos.pos();
    Path { origin }
  }

  /// Returns the nearest from a provided array.
  pub fn find_nearest_of<'a, T>(&self, targets: Vec<&'a T>) -> Option<&'a T>
  where
    T: RoomObjectProperties + ?Sized,
  {
    if targets.is_empty() {
      return None;
    }

    let mut nearest = *targets.get(0).unwrap();
    let mut nearest_cost = MAX;

    for target in targets {
      let result = search(&self.origin, target, MAX, SearchOptions::default());
      if !result.incomplete && result.cost < nearest_cost {
        nearest_cost = result.cost;
        nearest = target;
      }
    }
    Some(nearest)
  }
}
