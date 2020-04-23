//! Utilities for finding a container near a source.

use crate::*;

/// HasContainer is for sources. It lets a source determine if there is a container adjacent to it.
pub trait HasContainer {
  /// Returns a random container adjacent to the source
  fn container(&self) -> Option<StructureContainer>;
}

impl<T: RoomObjectProperties> HasContainer for T {
  fn container(&self) -> Option<StructureContainer> {
    let mut containers: Vec<StructureContainer> =
      self
        .pos()
        .find_in_range(find::STRUCTURES, 1)
        .into_iter()
        .filter_map(|s| {
          if let Structure::Container(c) = s {
            Some(c)
          } else {
            None
          }
        })
        .collect();

    containers.pop()
  }
}
