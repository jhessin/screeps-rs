/// Bring Capabilities relating to rooms into a package.
use log;
use screeps::{ResourceType, find, game, objects::Room, prelude::*, Part, ReturnCode};

pub fn room_manager(room: Room) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Managing room: {}", room.name());
    manage_spawn(&room)?;
    manage_creeps()?;

    Ok(())
}

fn manage_spawn(room: &Room) -> Result<(), Box<dyn std::error::Error>> {
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

fn manage_creeps() -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("running creeps");
    for creep in screeps::game::creeps::values() {
        let name = creep.name();
        log::debug!("running creep {}", name);
        if creep.spawning() {
            continue;
        }

        if creep.memory().bool("harvesting") {
            if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
                creep.memory().set("harvesting", false);
            }
        } else {
            if creep.store_used_capacity(None) == 0 {
                creep.memory().set("harvesting", true);
            }
        }

        if creep.memory().bool("harvesting") {
            let source = &creep.room().find(find::SOURCES)[0];
            if creep.pos().is_near_to(source) {
                let r = creep.harvest(source);
                if r != ReturnCode::Ok {
                    log::warn!("couldn't harvest: {:?}", r);
                }
            } else {
                creep.move_to(source);
            }
        } else {
            if let Some(c) = creep.room().controller() {
                let r = creep.upgrade_controller(&c);
                if r == ReturnCode::NotInRange {
                    creep.move_to(&c);
                } else if r != ReturnCode::Ok {
                    log::warn!("couldn't upgrade: {:?}", r);
                }
            } else {
                log::warn!("creep room has no controller!");
            }
        }
    }

    Ok(())
}

