pub mod loader;
pub mod executor;
pub mod frames;
pub mod error_interpreter;

pub use loader::{LoadedProgram, parse_parameter};
pub use executor::execute_program;
pub use frames::{parse_raw_frames, Frame};
pub use error_interpreter::interpret_error;
