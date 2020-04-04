//! Easy path finding tool it wraps up the position of a thing
//! then provides tools to find the nearest other thing.
//! TODO put this in a trait for Vec<Target>

use crate::*;

/// Easy path management
pub struct Finder {
  /// The origin for navigation
  origin: Position,
}

impl Finder {
  /// Returns a new path given anything that has a position
  pub fn new<T>(pos: T) -> Self
  where
    T: HasPosition,
  {
    let origin = pos.pos();
    Finder { origin }
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
    let mut nearest_cost = std::u32::MAX;

    for target in targets {
      let result =
        search(&self.origin, target, std::u32::MAX, SearchOptions::default());
      if !result.incomplete && result.cost < nearest_cost {
        nearest_cost = result.cost;
        nearest = target;
      }
    }
    Some(nearest)
  }
}
