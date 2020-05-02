use crate::*;
use screeps::pathfinder::{search_many, SearchResults};
use screeps::Terrain;
use std::ops::Deref;

/// This holds data common to all room objects and generates terrain data to go with it.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct CommonData {
  pos: Position,
}

impl Display for CommonData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({}, {})", self.pos.x(), self.pos.y())
  }
}

/// A New Type for a Vec<CommonData>
pub struct CommonDataVec(pub Vec<CommonData>);

impl From<Vec<CommonData>> for CommonDataVec {
  fn from(vec: Vec<CommonData>) -> Self {
    CommonDataVec(vec)
  }
}

impl Deref for CommonDataVec {
  type Target = Vec<CommonData>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Default for CommonDataVec {
  fn default() -> Self {
    CommonDataVec(vec![])
  }
}

impl IntoIterator for CommonDataVec {
  type Item = (Position, u32);
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    let mut vec = vec![];
    let mut source = self.0;
    while !source.is_empty() {
      let next = source.pop().unwrap().pos;
      vec.push((next, std::u32::MAX));
    }
    vec.into_iter()
  }
}

impl<T: HasPosition> From<T> for CommonData {
  fn from(p: T) -> Self {
    let pos = p.pos();
    CommonData { pos }
  }
}

impl CommonData {
  /// Get the underlying terrain
  pub fn terrain(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x(), self.pos.y())
  }

  /// Get the terrain above
  pub fn terrain_top(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x(), self.pos.y() - 1)
  }

  /// Get the terrain bellow
  pub fn terrain_bottom(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x(), self.pos.y() + 1)
  }

  /// Get the terrain left
  pub fn terrain_left(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x() - 1, self.pos.y())
  }

  /// Get the terrain right
  pub fn terrain_right(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x() + 1, self.pos.y())
  }

  /// Get the terrain up and to the right
  pub fn terrain_top_right(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x() + 1, self.pos.y() - 1)
  }

  /// Get the terrain down and to the right
  pub fn terrain_bottom_right(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x() + 1, self.pos.y() + 1)
  }

  /// Get the terrain down and to the left
  pub fn terrain_bottom_left(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x() - 1, self.pos.y() + 1)
  }

  /// Get the terrain up and to the left
  pub fn terrain_top_left(&self) -> Terrain {
    let data = RoomTerrain::constructor(self.pos.room_name());
    data.get(self.pos.x() - 1, self.pos.y() - 1)
  }
}

impl CommonData {
  /// Find a direct path ignoring swamps for a scout
  pub fn scout_path_to<T: Into<CommonDataVec>>(
    &self,
    other: T,
  ) -> SearchResults {
    let options = SearchOptions::new().swamp_cost(1);
    let other = other.into();
    search_many(&self.pos, other, options)
  }

  /// Find a path for a hauler to drop off resources amidst multiple targets
  pub fn hauler_path_to<T: Into<CommonDataVec>>(
    &self,
    other: T,
  ) -> SearchResults {
    let other = other.into();
    search_many(&self.pos, other, SearchOptions::default())
  }

  /// Find a basic path
  pub fn path_to(&self, other: &CommonData) -> SearchResults {
    search(&self.pos, &other.pos, std::u32::MAX, SearchOptions::default())
  }
}
