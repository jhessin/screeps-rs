//! Utilities for finding a container near a source.

use crate::*;

/// HasExtractor is for Minerals. It lets a Mineral determine if there is an extractor on top of it.
pub trait HasExtractor {
  /// Returns the extractor on top of the mineral
  fn extractor(&self) -> Option<StructureExtractor>;
}

impl HasExtractor for Mineral {
  fn extractor(&self) -> Option<StructureExtractor> {
    let mut containers: Vec<StructureExtractor> =
      self
        .pos()
        .find_in_range(find::STRUCTURES, 0)
        .into_iter()
        .filter_map(|s| {
          if let Structure::Extractor(c) = s {
            Some(c)
          } else {
            None
          }
        })
        .collect();

    containers.pop()
  }
}
