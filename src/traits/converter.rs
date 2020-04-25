use crate::*;

/// This allows conversion between different RoomObjects
pub trait Converter {
  /// A base room object
  fn as_room_object(&self) -> Option<RoomObject>;
  /// a Creep
  fn as_creep(&self) -> Option<Creep>;
  /// a Construction Site
  fn as_construction_site(&self) -> Option<ConstructionSite>;
  /// a Mineral Deposit
  fn as_deposit(&self) -> Option<Deposit>;
  /// a Flag
  fn as_flag(&self) -> Option<Flag>;
  /// a Mineral
  fn as_mineral(&self) -> Option<Mineral>;
  /// a Nuke
  fn as_nuke(&self) -> Option<Nuke>;
  /// a Structure
  fn as_structure(&self) -> Option<Structure>;
  /// a Power Creep
  fn as_power_creep(&self) -> Option<PowerCreep>;
  /// a Resource
  fn as_resource(&self) -> Option<Resource>;
  /// a Ruin
  fn as_ruin(&self) -> Option<Ruin>;
  /// a Energy Source
  fn as_source(&self) -> Option<Source>;
  /// a Tombstone
  fn as_tombstone(&self) -> Option<Tombstone>;
}

impl Converter for String {
  fn as_room_object(&self) -> Option<RoomObject> {
    trace!("Attempting to convert {} into a RoomObject", self);
    if let Ok(id) = ObjectId::<RoomObject>::from_str(self) {
      game::get_object_erased(id)
    } else {
      None
    }
  }

  fn as_creep(&self) -> Option<Creep> {
    trace!("Attempting to convert {} into a Creep", self);
    if let Ok(id) = ObjectId::<Creep>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_construction_site(&self) -> Option<ConstructionSite> {
    trace!("Attempting to convert {} into a ConstructionSite", self);
    if let Ok(id) = ObjectId::<ConstructionSite>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_deposit(&self) -> Option<Deposit> {
    trace!("Attempting to convert {} into a Deposit", self);
    if let Ok(id) = ObjectId::<Deposit>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_flag(&self) -> Option<Flag> {
    trace!("Attempting to convert {} into a Flag", self);
    game::flags::get(self)
  }

  fn as_mineral(&self) -> Option<Mineral> {
    trace!("Attempting to convert {} into a Mineral", self);
    if let Ok(id) = ObjectId::<Mineral>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_nuke(&self) -> Option<Nuke> {
    trace!("Attempting to convert {} into a Nuke", self);
    if let Ok(id) = ObjectId::<Nuke>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_structure(&self) -> Option<Structure> {
    trace!("Attempting to convert {} into a Structure", self);
    if let Ok(id) = ObjectId::<Structure>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_power_creep(&self) -> Option<PowerCreep> {
    trace!("Attempting to convert {} into a PowerCreep", self);
    if let Ok(id) = ObjectId::<PowerCreep>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_resource(&self) -> Option<Resource> {
    trace!("Attempting to convert {} into a Resource", self);
    if let Ok(id) = ObjectId::<Resource>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_ruin(&self) -> Option<Ruin> {
    trace!("Attempting to convert {} into a Ruin", self);
    if let Ok(id) = ObjectId::<Ruin>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_source(&self) -> Option<Source> {
    trace!("Attempting to convert {} into a Source", self);
    if let Ok(id) = ObjectId::<Source>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }

  fn as_tombstone(&self) -> Option<Tombstone> {
    trace!("Attempting to convert {} into a Tombstone", self);
    if let Ok(id) = ObjectId::<Tombstone>::from_str(self) {
      if let Ok(result) = id.try_resolve() {
        result
      } else {
        None
      }
    } else {
      None
    }
  }
}
