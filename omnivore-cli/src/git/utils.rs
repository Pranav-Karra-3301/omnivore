use anyhow::Result;
use std::path::Path;

pub fn is_text_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return is_text_extension(ext_str);
        }
    }
    
    if let Some(file_name) = path.file_name() {
        if let Some(name_str) = file_name.to_str() {
            return is_text_filename(name_str);
        }
    }
    
    false
}

fn is_text_extension(ext: &str) -> bool {
    matches!(
        ext.to_lowercase().as_str(),
        "txt" | "md" | "markdown" | "rst" | "adoc" | "org" |
        "rs" | "go" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" |
        "java" | "kt" | "scala" | "groovy" | "clj" | "cljs" |
        "py" | "pyi" | "rb" | "lua" | "perl" | "pl" | "pm" |
        "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" |
        "html" | "htm" | "xml" | "xhtml" | "svg" |
        "css" | "scss" | "sass" | "less" | "styl" |
        "json" | "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" |
        "sh" | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd" |
        "sql" | "graphql" | "gql" |
        "dockerfile" | "makefile" | "cmake" |
        "vue" | "svelte" | "elm" | "dart" | "swift" |
        "r" | "matlab" | "julia" | "nim" | "zig" | "v" |
        "php" | "asp" | "aspx" | "jsp" |
        "proto" | "thrift" | "avro" |
        "tf" | "tfvars" | "hcl" |
        "nix" | "dhall" | "jsonnet" |
        "vim" | "el" | "lisp" |
        "asm" | "s" |
        "tex" | "bib" | "sty" | "cls"
    )
}

fn is_text_filename(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "readme" | "license" | "contributing" | "changelog" | "authors" |
        "todo" | "notes" | "install" | "copyright" | "patents" |
        "makefile" | "dockerfile" | "gemfile" | "rakefile" | "gulpfile" |
        "gruntfile" | "package.json" | "cargo.toml" | "go.mod" | "go.sum" |
        "requirements.txt" | "setup.py" | "setup.cfg" | "pyproject.toml" |
        ".gitignore" | ".dockerignore" | ".npmignore" | ".eslintrc" |
        ".prettierrc" | ".editorconfig" | ".gitattributes" | ".env" |
        ".env.example" | ".env.sample" | ".env.template"
    ) || name.starts_with('.') && !name.contains('.')
}

pub fn parse_size_string(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_uppercase();
    
    if let Ok(bytes) = size_str.parse::<u64>() {
        return Ok(bytes);
    }
    
    let (number_part, unit_part) = split_size_string(&size_str)?;
    let number: f64 = number_part
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid number: {}", number_part))?;
    
    let multiplier = match unit_part.as_str() {
        "B" | "" => 1_u64,
        "K" | "KB" => 1_024,
        "M" | "MB" => 1_048_576,
        "G" | "GB" => 1_073_741_824,
        "T" | "TB" => 1_099_511_627_776,
        _ => return Err(anyhow::anyhow!("Unknown size unit: {}", unit_part)),
    };
    
    Ok((number * multiplier as f64) as u64)
}

fn split_size_string(s: &str) -> Result<(String, String)> {
    let mut number_part = String::new();
    let mut unit_part = String::new();
    let mut found_unit = false;
    
    for ch in s.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            if found_unit {
                return Err(anyhow::anyhow!("Invalid size format: {}", s));
            }
            number_part.push(ch);
        } else if ch.is_ascii_alphabetic() {
            found_unit = true;
            unit_part.push(ch);
        } else if !ch.is_whitespace() {
            return Err(anyhow::anyhow!("Invalid character in size: {}", ch));
        }
    }
    
    if number_part.is_empty() {
        return Err(anyhow::anyhow!("No number found in size: {}", s));
    }
    
    Ok((number_part, unit_part))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_string() {
        assert_eq!(parse_size_string("1024").unwrap(), 1024);
        assert_eq!(parse_size_string("10KB").unwrap(), 10240);
        assert_eq!(parse_size_string("10 KB").unwrap(), 10240);
        assert_eq!(parse_size_string("1.5MB").unwrap(), 1572864);
        assert_eq!(parse_size_string("2GB").unwrap(), 2147483648);
        assert_eq!(parse_size_string("2 GB").unwrap(), 2147483648);
    }

    #[test]
    fn test_is_text_file() {
        assert!(is_text_file(Path::new("test.rs")));
        assert!(is_text_file(Path::new("README.md")));
        assert!(is_text_file(Path::new("Dockerfile")));
        assert!(is_text_file(Path::new(".gitignore")));
        assert!(!is_text_file(Path::new("image.png")));
        assert!(!is_text_file(Path::new("binary.exe")));
    }
}