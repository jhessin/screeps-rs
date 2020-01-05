/// Bring Capabilities relating to rooms into a package.
use log;
use screeps::{game, objects::Room, prelude::*, Creep, Part, ReturnCode};

use crate::actions::creep::Default;
use crate::actions::CreepAction;
use crate::types::GeneralError;

pub fn room_manager(room: Room) -> Result<(), GeneralError> {
    log::debug!("Managing room: {}", room.name());
    manage_spawn(&room)?;
    manage_creeps(&room)?;

    Ok(())
}

fn manage_spawn(room: &Room) -> Result<(), GeneralError> {
    log::debug!("running spawns");

    for spawn in game::spawns::values() {
        if spawn.room().name() != room.name() {
            continue;
        }

        log::debug!("running spawn {}", spawn.name());
        let body = [Part::Move, Part::Move, Part::Carry, Part::Work];

        if spawn.energy() >= body.iter().map(|p| p.cost()).sum() {
            // create a unique name, spawn.
            let name_base = game::time();
            let mut additional = 0;
            let res = loop {
                let name = format!("{}-{}", name_base, additional);
                let res = spawn.spawn_creep(&body, &name);

                if res == ReturnCode::NameExists {
                    additional += 1;
                } else {
                    break res;
                }
            };

            if res != ReturnCode::Ok {
                log::warn!("couldn't spawn: {:?}", res);
            }
        }
    }
    Ok(())
}

fn manage_creeps(room: &Room) -> Result<(), GeneralError> {
    let room_name = room.name();
    let creeps_in_room: Vec<Creep> = game::creeps::values()
        .into_iter()
        .filter(|creep| creep.room().name() == room_name)
        .collect();

    log::debug!("running creeps");
    for creep in creeps_in_room {
        let name = creep.name();
        log::debug!("running creep {}", name);

        Default::tick(creep)?;
    }
    Ok(())
}
