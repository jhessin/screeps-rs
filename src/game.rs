/// Define the outermost loop for run the game from.
// Provide any required functionality to manage the AI.
use std::collections::HashSet;

use log;
use screeps::{game, RoomObjectProperties};

use crate::rooms;

pub fn game_loop() {
    log::debug!("loop starting! CPU: {}", screeps::game::cpu::get_used());

    log::debug!("Managing rooms.");
    for spawn in game::spawns::values() {
        let room = spawn.room();
        log::debug!("Managing room {}", room.name());

        let result = rooms::room_manager(room);

        match result {
            Ok(()) => {
                log::info!("Finished managing rooms.");
            }
            Err(err) => {
                log::error!("Encountered an error during this tick. {}", err);
            }
        }
    }

    let time = screeps::game::time();

    if time % 32 == 3 {
        log::info!("running memory cleanup");
        cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
    }

    log::info!("done! cpu: {}", screeps::game::cpu::get_used());
}

pub fn cleanup_memory() -> Result<(), Box<dyn std::error::Error>> {
    let alive_creeps: HashSet<String> = screeps::game::creeps::keys().into_iter().collect();

    let screeps_memory = match screeps::memory::root().dict("creeps")? {
        Some(v) => v,
        None => {
            log::warn!("not cleaning game creep memory: no Memory.creeps dict");
            return Ok(());
        }
    };

    for mem_name in screeps_memory.keys() {
        if !alive_creeps.contains(&mem_name) {
            log::debug!("cleaning up creep memory of dead creep {}", mem_name);
            screeps_memory.del(&mem_name);
        }
    }

    Ok(())
}
