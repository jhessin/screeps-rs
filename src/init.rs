use crate::*;

/// This initializes logging and anything else that needs it only once.
pub fn init() {
  logging::setup_logging(logging::Debug);
}
