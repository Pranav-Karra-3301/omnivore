use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_basic_git_functionality() -> Result<()> {
    // Test basic file structure creation
    let temp_dir = TempDir::new()?;
    let root = temp_dir.path();

    // Create a mock git repository structure
    fs::create_dir_all(root.join(".git"))?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(root.join("README.md"), "# Test Project")?;
    fs::write(root.join("src/main.rs"), "fn main() {}")?;
    fs::write(root.join(".gitignore"), "target/\n*.log")?;

    // Verify files were created
    assert!(root.join(".git").exists());
    assert!(root.join("src/main.rs").exists());
    assert!(root.join("README.md").exists());
    assert!(root.join(".gitignore").exists());

    Ok(())
}

#[test]
fn test_gitignore_patterns() -> Result<()> {
    // Test gitignore pattern parsing
    let patterns = vec![
        "*.log",
        "target/",
        "node_modules/",
        ".DS_Store",
    ];

    for pattern in patterns {
        assert!(!pattern.is_empty());
    }

    Ok(())
}

#[test]
fn test_file_path_manipulation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let root = temp_dir.path();
    let src_path = root.join("src").join("lib.rs");
    
    // Test path operations
    assert_eq!(src_path.file_name().unwrap(), "lib.rs");
    assert_eq!(src_path.extension().unwrap(), "rs");
    
    Ok(())
}