//! The role is the role that a creep will take.
use crate::*;

/// This is an enum that lists the different roles
#[derive(
  Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd,
)]
pub enum Role {
  /// Harvest energy and place it into Extensions, Spawns, Towers, Storage
  /// fallback: -> Upgrader
  Harvester,
  /// Mine from source and drop on the ground on into a container.
  Miner,
  /// Upgrade the room controller
  Upgrader,
  /// Builds anything it finds except walls or ramparts
  /// fallback: -> Repair -> Upgrader
  Builder,
  /// Repairs anything damaged except walls or ramparts
  /// fallback: -> Upgrader
  Repairer,
  /// Builds walls and ramparts
  /// Repairs the most damaged wall or rampart
  /// fallback: -> Upgrader
  WallRepairer,
  /// Ferries resources from containers or the ground and places it in
  /// Extensions, Spawns, Towers, or Storage
  /// fallback: -> Repair -> Upgrader
  Lorry,
  /// Ferries resources to links
  /// fallback: -> Repair -> Upgrader
  LinkLoader,
  /// This is a claimer to claim new rooms
  Claimer,
  /// A reserve new rooms
  Reserver,
  /// A scout
  Scout,
  /// A soldier
  Soldier,
  /// A healer
  Healer,
}

/// This gives me to_string functionality for easy debugging
impl Display for Role {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", to_string(self).unwrap())
  }
}

/// General helper methods
impl Role {
  /// Returns the appropriate body for this role as well as if it should be expanded.
  pub fn body(&self) -> Vec<Part> {
    use Part::*;
    match self {
      Role::Harvester => vec![Work, Carry, Move, Move],
      Role::Miner => vec![Move, Work, Work, Work, Work, Work],
      Role::Upgrader => vec![Work, Carry, Move, Move],
      Role::Builder => vec![Work, Carry, Move, Move],
      Role::Repairer => vec![Work, Carry, Move, Move],
      Role::WallRepairer => vec![Work, Carry, Move, Move],
      Role::Lorry => vec![Carry, Move],
      Role::LinkLoader => vec![Carry, Carry, Move, Move],
      Role::Claimer => vec![Move, Move, Move, Move, Move, Claim],
      Role::Reserver => vec![Move, Move, Move, Move, Move, Claim],
      Role::Scout => vec![Move],
      Role::Soldier => vec![Tough, Attack, RangedAttack, Move, Move, Move],
      Role::Healer => vec![RangedAttack, Heal, Move, Move],
    }
  }

  /// Returns the minimum cost to spawn this creep
  pub fn cost(&self) -> u32 {
    let body = self.body();
    cost(&body)
  }

  /// Returns the creeps body expanded to the appropriate amount of energy
  pub fn expand(&self, energy: u32) -> Vec<Part> {
    let base = self.body();
    use Role::*;

    match self {
      // exceptions for roles that don't expand
      Miner => {
        let mut body = base.clone();
        while cost(&body) > energy {
          body.pop();
          if body.len() == 2 {
            return body;
          }
        }
        body
      }
      Claimer => base,
      Scout => base,
      _ => {
        let mut body = vec![];
        let num_parts = energy / self.cost();
        if num_parts == 0 {
          base
        } else {
          for part in base {
            for _ in 0..num_parts {
              body.push(part);
            }
          }
          body
        }
      }
    }
  }

  /// This works the role
  pub fn run(&self, creep: &Creep) -> ReturnCode {
    todo!("Run each role here: {}", creep.id().to_string())
  }

  /// Spawn the specified role
  pub fn spawn(&self, spawn: StructureSpawn) -> ReturnCode {
    let energy = spawn.room().unwrap().energy_capacity_available();

    let body = self.expand(energy);

    let name = get_random_name(spawn.room().unwrap());

    let opts = SpawnOptions::new()
      .memory(MemoryReference::new().set_value(Values::Role(*self)));

    spawn.spawn_creep_with_options(&body, &name, &opts)
  }

  /// Spawn the role NOW (emergency mode)
  pub fn spawn_now(&self, spawn: &StructureSpawn) -> ReturnCode {
    let energy = spawn.room().unwrap().energy_available();

    let body = self.expand(energy);

    let name = get_random_name(spawn.room().unwrap());

    let opts = SpawnOptions::new()
      .memory(MemoryReference::new().set_value(Values::Role(*self)));

    spawn.spawn_creep_with_options(&body, &name, &opts)
  }

  /// Spawn all emergency situations
  /// Return true if something is spawned
  pub fn spawn_emergencies(spawn: &StructureSpawn) -> bool {
    let room = spawn.room().unwrap();

    use Role::*;
    if room.creeps_with_role(Lorry).len() == 0
      && room.creeps_with_role(Harvester).len() == 0
    {
      // No harvesters or lorries
      if room.creeps_with_role(Miner).len() == 0 {
        Harvester.spawn_now(spawn);
      } else {
        Lorry.spawn_now(spawn);
      }
      true
    } else {
      false
    }
  }

  /// Spawn the minimum of each role
  pub fn spawn_min(spawn: &StructureSpawn) -> bool {
    todo!("{}", spawn.id().to_string())
  }

  /// Spawn any specials or extras
  pub fn spawn_extras(spawn: &StructureSpawn) -> ReturnCode {
    todo!("{}", spawn.id().to_string())
  }
}

fn cost(body: &Vec<Part>) -> u32 {
  let mut cost = 0;
  for part in body {
    cost += part.cost();
  }
  cost
}
