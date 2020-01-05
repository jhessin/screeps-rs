//! Screeps AI powered by rust!.

#![warn(missing_docs)]

use stdweb::js;

mod actions;
mod game;
mod logging;
mod rooms;
mod types;

use game::{game_loop};


/// Bootstrap the game AI
///
///This is the entrypoint for the Screeps AI, it sets up the JS environment to load in and run
///the game environment from rust.
fn main() {
    logging::setup_logging(logging::Info);

    js! {
        var game_loop = @{game_loop};

        module.exports.loop = function() {
            // Provide actual error traces.
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}


