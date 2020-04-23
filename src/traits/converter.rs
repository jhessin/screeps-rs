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
    if let Ok(id) = ObjectId::<RoomObject>::from_str(self) {
      game::get_object_erased(id)
    } else {
      None
    }
  }

  fn as_creep(&self) -> Option<Creep> {
    if let Ok(id) = ObjectId::<Creep>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_construction_site(&self) -> Option<ConstructionSite> {
    if let Ok(id) = ObjectId::<ConstructionSite>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_deposit(&self) -> Option<Deposit> {
    if let Ok(id) = ObjectId::<Deposit>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_flag(&self) -> Option<Flag> {
    game::flags::get(self)
  }

  fn as_mineral(&self) -> Option<Mineral> {
    if let Ok(id) = ObjectId::<Mineral>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_nuke(&self) -> Option<Nuke> {
    if let Ok(id) = ObjectId::<Nuke>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_structure(&self) -> Option<Structure> {
    if let Ok(id) = ObjectId::<Structure>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_power_creep(&self) -> Option<PowerCreep> {
    if let Ok(id) = ObjectId::<PowerCreep>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_resource(&self) -> Option<Resource> {
    if let Ok(id) = ObjectId::<Resource>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_ruin(&self) -> Option<Ruin> {
    if let Ok(id) = ObjectId::<Ruin>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_source(&self) -> Option<Source> {
    if let Ok(id) = ObjectId::<Source>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }

  fn as_tombstone(&self) -> Option<Tombstone> {
    if let Ok(id) = ObjectId::<Tombstone>::from_str(self) {
      id.resolve()
    } else {
      None
    }
  }
}

impl Converter for RoomObject {
  fn as_room_object(&self) -> Option<RoomObject> {
    Some(self.clone())
  }

  fn as_creep(&self) -> Option<Creep> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_construction_site(&self) -> Option<ConstructionSite> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_deposit(&self) -> Option<Deposit> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_flag(&self) -> Option<Flag> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_mineral(&self) -> Option<Mineral> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_nuke(&self) -> Option<Nuke> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_structure(&self) -> Option<Structure> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_power_creep(&self) -> Option<PowerCreep> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_resource(&self) -> Option<Resource> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_ruin(&self) -> Option<Ruin> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_source(&self) -> Option<Source> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }

  fn as_tombstone(&self) -> Option<Tombstone> {
    if let Ok(target) = self.as_ref().clone().into_expected_type() {
      Some(target)
    } else {
      None
    }
  }
}
