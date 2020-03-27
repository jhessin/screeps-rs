/// Trait used to provide a mechanism of running a task if a creep is assigned one.
use screeps::objects::Creep;

use crate::types::GeneralError;

pub trait Worker {
    fn work(&self) -> Result<(), GeneralError>;
}

impl Worker for Creep {
    fn work(&self) -> Result<(), GeneralError> {
        if self.spawning() {
            // Its impossible to do anything with creeps that are spawning.
            log::debug!("Creep is spawning, skipping");
            return Ok(());
        }
        let _task: String = match self.memory().string("_task") {
            Ok(res) => match res {
                Some(value) => value,
                None => {
                    log::info!("No task for {}", self.name());
                    return Ok(());
                }
            },
            Err(err) => return Err(Box::new(err)),
        };

        // TODO - Deserialise a Task Struct/Trait here and "run" it.
        Ok(())
    }
}
