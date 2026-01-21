pub mod loader;
pub mod executor;
pub mod frames;
pub mod uplc_file_utils;
pub mod diagnostics;

pub use loader::{LoadedProgram, parse_parameter, load_programs_from_file, apply_parameters};
pub use executor::execute_program;
pub use frames::{parse_snapshots_to_frames, Frame};
pub use uplc_file_utils::{create_uplc_file, create_uplc_files};
pub use diagnostics::{analyze_frame, print_diagnostic, Diagnostic, DiagnosticStatus, Severity};