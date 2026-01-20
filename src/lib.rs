pub mod loader;
pub mod executor;
pub mod frames;

pub use loader::{LoadedProgram, parse_parameter};
pub use executor::execute_program;
pub use frames::{parse_snapshots_to_frames, Frame};  // Changed from parse_raw_frames