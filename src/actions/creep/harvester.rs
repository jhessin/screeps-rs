use std::cmp::Ordering;

use screeps::{
    find,
    objects::{HasStore, Structure},
    prelude::*,
    Creep, LookResult, ResourceType, ReturnCode,
};

use crate::actions::CreepAction;
use crate::types::{ActionStates, GeneralError};
use log;

pub struct Harvester;

struct TargetEnergySink {
    x: u32,
    y: u32,
    structure: Structure,
    // structure: Box<dyn HasStore>,
}

impl Harvester {
    fn harvest_energy(creep: &Creep) -> bool {
        // TODO - Work to the nearest source, not just arbitrarily the first.
        let source = &creep.room().find(find::SOURCES)[0];
        let creep_pos = &creep.pos();

        if creep_pos.is_near_to(source) {
            let r = creep.harvest(source);
            if r != ReturnCode::Ok {
                log::warn!("couldn't harvest: {:?}", r);

                false
            } else {
                true
            }
        } else {
            creep.move_to(source);
            false
        }
    }

    fn offload_energy(creep: &Creep) -> bool {
        // Retrieve all the structures in the room.
        let mut targets = Harvester::identify_targets(creep);

        Harvester::_organise_targets(&mut targets);

        let target = match targets.get(0) {
            None => return false,
            Some(val) => val,
        };

        match &target.structure {
            Structure::Container(container) => {
                Ok(creep.transfer_all(container, ResourceType::Energy))
            }
            Structure::Extension(extension) => {
                Ok(creep.transfer_all(extension, ResourceType::Energy))
            }
            Structure::Link(link) => Ok(creep.transfer_all(link, ResourceType::Energy)),
            Structure::Spawn(spawn) => Ok(creep.transfer_all(spawn, ResourceType::Energy)),
            Structure::Storage(storage) => Ok(creep.transfer_all(storage, ResourceType::Energy)),
            Structure::Tower(tower) => Ok(creep.transfer_all(tower, ResourceType::Energy)),
            _ => Err(()),
        }
        .map(|rc| match rc {
            // rc.as_result()
            ReturnCode::Ok => Ok(()),
            ReturnCode::NotInRange => {
                log::debug!(
                    "Creep was not in range to transfer, moving to target: ({},{})",
                    &target.x,
                    &target.y
                );
                creep.move_to_xy(target.x, target.y);
                Ok(())
            }
            ReturnCode::NotEnough => {
                log::debug!("Creep couldn't fill the target completely.");
                Ok(())
            }
            ReturnCode::Full => {
                log::debug!("Target is now full.");
                Ok(())
            }
            r => {
                log::warn!("unhandled return {:?}", r);
                Err(r)
            }
        })
        .is_ok()
    }

    fn identify_targets(creep: &Creep) -> Vec<TargetEnergySink> {
        let targets: Vec<TargetEnergySink> = creep
            .room()
            .look_at_area(0, 0, 49, 49)
            .into_iter()
            .map(|item| match &item.look_result {
                // Look only at structures
                LookResult::Structure(structure) => match structure {
                    // Look only at these types of structure, all of which implement Transferable
                    // and HasStore
                    Structure::Container(container) => {
                        if container.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            Some(TargetEnergySink {
                                x: item.x,
                                y: item.y,
                                structure: Structure::Container(container.clone()),
                            })
                        } else {
                            None
                        }
                    }
                    Structure::Extension(extension) => {
                        if extension.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            Some(TargetEnergySink {
                                x: item.x,
                                y: item.y,
                                structure: Structure::Extension(extension.clone()),
                                // structure: Box::new(extension.clone()),
                            })
                        } else {
                            None
                        }
                    }
                    Structure::Link(link) => {
                        if link.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            Some(TargetEnergySink {
                                x: item.x,
                                y: item.y,
                                structure: Structure::Link(link.clone()),
                                // structure: Box::new(link.clone()),
                            })
                        } else {
                            None
                        }
                    }
                    Structure::Spawn(spawn) => {
                        if spawn.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            Some(TargetEnergySink {
                                x: item.x,
                                y: item.y,
                                structure: Structure::Spawn(spawn.clone()),
                                // structure: Box::new(spawn.clone()),
                            })
                        } else {
                            None
                        }
                    }
                    Structure::Storage(storage) => {
                        if storage.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            Some(TargetEnergySink {
                                x: item.x,
                                y: item.y,
                                structure: Structure::Storage(storage.clone()),
                                // structure: Box::new(storage.clone()),
                            })
                        } else {
                            None
                        }
                    }
                    Structure::Tower(tower) => {
                        if tower.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            Some(TargetEnergySink {
                                x: item.x,
                                y: item.y,
                                structure: Structure::Tower(tower.clone()),
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            })
            .filter(|item| item.is_some())
            .map(|item| item.unwrap())
            .collect();

        targets
    }
    fn _compare_structure_free_energy(
        struct_a: &dyn HasStore,
        struct_b: &dyn HasStore,
    ) -> Ordering {
        let b_free = struct_b.store_free_capacity(Some(ResourceType::Energy));
        let a_free = struct_a.store_free_capacity(Some(ResourceType::Energy));
        b_free.cmp(&a_free)
        // if b_free < a_free {
        //     Ordering::Less
        // } else if b_free > a_free {
        //     Ordering::Greater
        // } else {
        //     Ordering::Equal
        // }
    }

    fn _organise_targets(targets: &mut Vec<TargetEnergySink>) {
        // Organise the targets by their priority, then select the most urgent/closest one.
        targets.sort_by(|a, b| {
            let a_struct_ref = &a.structure;
            let b_struct_ref = &b.structure;

            // Desired priority:
            //  - Towers
            //  - Spawn
            //  - Extension
            //  - Link
            //  - Container
            //  - Storage
            //
            //  Where both sides are the same structure type, sort by the one with less energy
            match b_struct_ref {
                Structure::Tower(struct_b) => match a_struct_ref {
                    Structure::Tower(struct_a) => {
                        Harvester::_compare_structure_free_energy(struct_b, struct_a)
                    }

                    // In this branch, the Tower is the most significant item, so anything that
                    // isn't a tower is less important.
                    _ => Ordering::Less,
                },
                Structure::Spawn(struct_b) => match a_struct_ref {
                    Structure::Spawn(struct_a) => {
                        Harvester::_compare_structure_free_energy(struct_b, struct_a)
                    }

                    // The tower is the only thing more important than the spawn
                    Structure::Tower(_) => Ordering::Greater,

                    // Everything else is less important
                    _ => Ordering::Less,
                },
                Structure::Extension(struct_b) => match a_struct_ref {
                    Structure::Extension(struct_a) => {
                        Harvester::_compare_structure_free_energy(struct_b, struct_a)
                    }

                    // The tower and the spawn are more important than the extension
                    Structure::Spawn(_) | Structure::Tower(_) => Ordering::Greater,

                    // Everything else is less important
                    _ => Ordering::Less,
                },
                Structure::Link(struct_b) => match a_struct_ref {
                    Structure::Link(struct_a) => {
                        Harvester::_compare_structure_free_energy(struct_b, struct_a)
                    }

                    // The tower, spawn and extension are more important than the link
                    Structure::Extension(_) | Structure::Spawn(_) | Structure::Tower(_) => {
                        Ordering::Greater
                    }

                    // Everything else is less important
                    _ => Ordering::Less,
                },
                Structure::Container(struct_b) => match a_struct_ref {
                    Structure::Container(struct_a) => {
                        Harvester::_compare_structure_free_energy(struct_b, struct_a)
                    }

                    // The tower, spawn, extension and link are more important than the container
                    Structure::Link(_)
                    | Structure::Extension(_)
                    | Structure::Spawn(_)
                    | Structure::Tower(_) => Ordering::Greater,

                    // Everything else is less important
                    _ => Ordering::Less,
                },
                Structure::Storage(struct_b) => match a_struct_ref {
                    Structure::Storage(struct_a) => {
                        Harvester::_compare_structure_free_energy(struct_b, struct_a)
                    }

                    // In this branch, the second item is a spawn, so anything other than the first
                    // item being a Spawn means we'll be the greater side.
                    _ => Ordering::Greater,
                },

                // Should never be called, but anything other than the above comparisons should
                // never exist.  If it somehow does, default to being more significant than it.
                _ => Ordering::Less,
            }
        });
    }
}

impl CreepAction for Harvester {
    fn tick(creep: Creep) -> Result<(), GeneralError> {
        if creep.spawning() {
            // Its impossible to do anything with creeps that are spawning.
            log::debug!("Creep is spawning, skipping");
            return Ok(());
        }

        let creep_mem = creep.memory();
        let creep_state: ActionStates = match creep_mem.string("state") {
            Ok(res) => {
                log::debug!("Found Action {:?} for {}", res, creep.name());
                match res {
                    Some(state_str) => {
                        log::debug!("State for {} is: {}", creep.name(), state_str);
                        serde_json::from_str::<ActionStates>(&state_str)
                            .unwrap_or(ActionStates::Undefined)
                    }
                    None => ActionStates::Undefined,
                }
            }
            Err(err) => {
                log::debug!("Could not find a state on {}", creep.name());
                log::warn!(
                    "Encountered error when searching for {}'s state: {}",
                    creep.name(),
                    err
                );
                return Err(Box::new(err));
            }
        };

        // Update creeps state
        match creep_state {
            ActionStates::Undefined | ActionStates::Idle => creep
                .memory()
                .set("state", serde_json::to_string(&ActionStates::Harvesting)?),

            ActionStates::Harvesting => {
                // If we have all the energy we can carry, go deliver it.
                if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
                    creep
                        .memory()
                        .set("state", serde_json::to_string(&ActionStates::Offloading)?);
                }
            }

            ActionStates::Offloading => {
                // If we have no energy left, go get some more.
                if creep.store_used_capacity(Some(ResourceType::Energy)) == 0 {
                    creep
                        .memory()
                        .set("state", serde_json::to_string(&ActionStates::Harvesting)?);
                }
            }
        };

        // Act on Creeps state
        match creep_state {
            ActionStates::Harvesting => {
                Harvester::harvest_energy(&creep);
            }
            ActionStates::Offloading => {
                Harvester::offload_energy(&creep);
            }

            // If the state isn't something we expect, do nothing.
            _ => {
                log::info!("Not acting on this creep as its state doen't match our expectations.");
                return Ok(());
            }
        };
        Ok(())
    }
}
