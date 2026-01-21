use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Creates an UPLC validator file with proper UTF-8 encoding (no BOM)
/// Works on all platforms: Windows, macOS, Linux
/// 
/// # Arguments
/// * `filename` - Path and filename for the UPLC file
/// * `content` - UPLC program content
///
/// # Example
/// ```ignore
/// use cek_debugger::create_uplc_file;
/// create_uplc_file("my_validator.uplc", "(program 1.0.0 (lam x x))").unwrap();
/// ```
pub fn create_uplc_file<P: AsRef<Path>>(filename: P, content: &str) -> std::io::Result<()> {
    let path = filename.as_ref();
    let mut file = File::create(path)?;
    
    // Write with explicit UTF-8 encoding (no BOM)
    file.write_all(content.as_bytes())?;
    file.flush()?;
    
    println!("✓ Created UPLC file: {}", path.display());
    Ok(())
}

/// Creates multiple UPLC files at once
/// 
/// # Example
/// ```ignore
/// use cek_debugger::create_uplc_files;
/// let validators = vec![
///     ("validator1.uplc", "(program 1.0.0 (con integer 42))"),
///     ("validator2.uplc", "(program 1.0.0 (lam x x))"),
/// ];
/// create_uplc_files(&validators).unwrap();
/// ```
pub fn create_uplc_files(files: &[(&str, &str)]) -> std::io::Result<()> {
    for (filename, content) in files {
        create_uplc_file(filename, content)?;
    }
    println!("✓ Created {} UPLC file(s)", files.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_uplc_file() {
        let test_file = "test_create.uplc";
        let content = "(program 1.0.0 (lam x x))";
        
        create_uplc_file(test_file, content).unwrap();
        
        let read_content = fs::read_to_string(test_file).unwrap();
        assert_eq!(read_content, content);
        
        fs::remove_file(test_file).unwrap();
    }
}
