#[cfg(test)]
mod tests {
    use std::path::Path;
    use cek_debugger::loader::{load_programs_from_file, parse_parameter};
    
    #[tokio::test]
    async fn test_load_uplc_file() {
        let path = Path::new("crates/uplc/test_data/basic/integer/integer.uplc");
        let result = load_programs_from_file(path).await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parameter_parsing() {
        let hex = "d8799f0102ff"; // Example Plutus data
        let result = parse_parameter(0, hex.to_string());
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod full_workflow_tests {
    use std::path::PathBuf;
    use cek_debugger::loader::{load_programs_from_file, apply_parameters};
    use cek_debugger::executor::execute_program;

    #[tokio::test]
    async fn test_full_workflow_with_params() {
        let path = PathBuf::from("crates/uplc/test_data/basic/integer/integer.uplc");
        let programs = load_programs_from_file(&path).await.unwrap();
        let program = programs.into_iter().next().unwrap();
        
        let params = vec![];
        let applied = apply_parameters(program, params).unwrap();
        let snapshots = execute_program(applied.program).unwrap();
        
        assert!(snapshots.len() > 0);
    }
}