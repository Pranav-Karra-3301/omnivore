use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use super::detector::CodebaseInfo;
use super::filter::FilteredFile;

pub struct CodeOrganizer {
    codebase_info: CodebaseInfo,
    files: Vec<FilteredFile>,
}

impl CodeOrganizer {
    pub fn new(codebase_info: CodebaseInfo, files: Vec<FilteredFile>) -> Self {
        Self {
            codebase_info,
            files,
        }
    }

    pub fn organize(&self) -> OrganizedCode {
        let mut organized = OrganizedCode {
            metadata: self.generate_metadata(),
            sections: Vec::new(),
        };

        let categorized = self.categorize_files();
        
        for (category, files) in categorized {
            if !files.is_empty() {
                organized.sections.push(CodeSection {
                    name: category.clone(),
                    description: self.get_category_description(&category),
                    files: files.iter().cloned().cloned().collect(),
                });
            }
        }

        organized.sections.sort_by(|a, b| {
            let order = self.get_section_priority(&a.name);
            let order_b = self.get_section_priority(&b.name);
            order.cmp(&order_b)
        });

        organized
    }

    fn generate_metadata(&self) -> ProjectMetadata {
        ProjectMetadata {
            project_type: format!("{:?}", self.codebase_info.project_type),
            description: self.codebase_info.description.clone(),
            main_language: self
                .codebase_info
                .main_language
                .as_ref()
                .map(|l| format!("{:?}", l))
                .unwrap_or_else(|| "Unknown".to_string()),
            frameworks: self
                .codebase_info
                .frameworks
                .iter()
                .map(|f| format!("{:?}", f))
                .collect(),
            build_tools: self
                .codebase_info
                .build_tools
                .iter()
                .map(|b| format!("{:?}", b))
                .collect(),
            total_files: self.files.len(),
        }
    }

    fn categorize_files(&self) -> HashMap<String, Vec<&FilteredFile>> {
        let mut categories: HashMap<String, Vec<&FilteredFile>> = HashMap::new();

        for file in &self.files {
            let category = self.determine_category(&file.relative_path);
            categories
                .entry(category)
                .or_insert_with(Vec::new)
                .push(file);
        }

        for files in categories.values_mut() {
            files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        }

        categories
    }

    fn determine_category(&self, path: &Path) -> String {
        let path_str = path.to_string_lossy().to_lowercase();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        if path_str.contains("test") || path_str.contains("spec") {
            return "Tests".to_string();
        }

        if file_name == "readme.md"
            || file_name == "license"
            || file_name == "contributing.md"
            || file_name == "changelog.md"
        {
            return "Documentation".to_string();
        }

        if file_name == "dockerfile"
            || file_name == "docker-compose.yml"
            || file_name == ".dockerignore"
            || path_str.contains("k8s/")
            || path_str.contains("kubernetes/")
        {
            return "Infrastructure".to_string();
        }

        if file_name.starts_with('.')
            || file_name == "package.json"
            || file_name == "cargo.toml"
            || file_name == "pyproject.toml"
            || file_name == "go.mod"
            || file_name == "pom.xml"
            || file_name == "build.gradle"
            || file_name == "gemfile"
            || file_name == "composer.json"
            || file_name == "makefile"
            || file_name == "cmakelists.txt"
        {
            return "Configuration".to_string();
        }

        if path_str.contains("migrations/") || path_str.contains("schema") {
            return "Database".to_string();
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext {
                "html" | "htm" => return "Templates".to_string(),
                "css" | "scss" | "sass" | "less" => return "Styles".to_string(),
                "sql" => return "Database".to_string(),
                "yml" | "yaml" if !file_name.contains("docker") => {
                    return "Configuration".to_string()
                }
                _ => {}
            }
        }

        let components = path.components().collect::<Vec<_>>();
        if components.len() > 1 {
            if let Some(first_dir) = components.first() {
                let dir_str = first_dir.as_os_str().to_string_lossy().to_lowercase();
                return match dir_str.as_str() {
                    "src" | "lib" => "Source Code".to_string(),
                    "app" => "Application".to_string(),
                    "pages" => "Pages".to_string(),
                    "components" => "Components".to_string(),
                    "utils" | "helpers" => "Utilities".to_string(),
                    "services" => "Services".to_string(),
                    "api" => "API".to_string(),
                    "models" => "Models".to_string(),
                    "controllers" => "Controllers".to_string(),
                    "views" => "Views".to_string(),
                    "public" | "static" => "Static Assets".to_string(),
                    "scripts" => "Scripts".to_string(),
                    "bin" | "cmd" => "Binaries".to_string(),
                    _ => "Source Code".to_string(),
                };
            }
        }

        "Source Code".to_string()
    }

    fn get_category_description(&self, category: &str) -> String {
        match category {
            "Configuration" => "Project configuration and build files".to_string(),
            "Source Code" => "Main application source code".to_string(),
            "Tests" => "Test files and specifications".to_string(),
            "Documentation" => "Project documentation and guides".to_string(),
            "Infrastructure" => "Deployment and infrastructure configuration".to_string(),
            "Database" => "Database schemas and migrations".to_string(),
            "Templates" => "HTML templates and views".to_string(),
            "Styles" => "CSS and styling files".to_string(),
            "Components" => "Reusable UI components".to_string(),
            "Pages" => "Application pages and routes".to_string(),
            "API" => "API endpoints and handlers".to_string(),
            "Services" => "Business logic and service layers".to_string(),
            "Models" => "Data models and entities".to_string(),
            "Controllers" => "Request controllers and handlers".to_string(),
            "Views" => "View templates and presentations".to_string(),
            "Utilities" => "Helper functions and utilities".to_string(),
            "Static Assets" => "Static files and resources".to_string(),
            "Scripts" => "Build and utility scripts".to_string(),
            "Binaries" => "Executable files and commands".to_string(),
            "Application" => "Application entry points and core logic".to_string(),
            _ => format!("{} files", category),
        }
    }

    fn get_section_priority(&self, section: &str) -> usize {
        match section {
            "Documentation" => 0,
            "Configuration" => 1,
            "Source Code" => 2,
            "Application" => 3,
            "Pages" => 4,
            "Components" => 5,
            "API" => 6,
            "Services" => 7,
            "Models" => 8,
            "Controllers" => 9,
            "Views" => 10,
            "Utilities" => 11,
            "Templates" => 12,
            "Styles" => 13,
            "Database" => 14,
            "Tests" => 15,
            "Scripts" => 16,
            "Infrastructure" => 17,
            "Static Assets" => 18,
            "Binaries" => 19,
            _ => 99,
        }
    }
}

#[derive(Debug)]
pub struct OrganizedCode {
    pub metadata: ProjectMetadata,
    pub sections: Vec<CodeSection>,
}

#[derive(Debug)]
pub struct ProjectMetadata {
    pub project_type: String,
    pub description: String,
    pub main_language: String,
    pub frameworks: Vec<String>,
    pub build_tools: Vec<String>,
    pub total_files: usize,
}

#[derive(Debug)]
pub struct CodeSection {
    pub name: String,
    pub description: String,
    pub files: Vec<FilteredFile>,
}

impl OrganizedCode {
    pub fn to_formatted_text(&self, include_content: bool, _root_path: &Path) -> Result<String> {
        let mut output = String::new();

        output.push_str(&format!(
            r#"================================================================================
                          OMNIVORE CODE ANALYSIS REPORT
================================================================================

PROJECT INFORMATION
-------------------
Type:         {}
Description:  {}
Language:     {}
Frameworks:   {}
Build Tools:  {}
Total Files:  {}

================================================================================
"#,
            self.metadata.project_type,
            self.metadata.description,
            self.metadata.main_language,
            if self.metadata.frameworks.is_empty() {
                "None detected".to_string()
            } else {
                self.metadata.frameworks.join(", ")
            },
            if self.metadata.build_tools.is_empty() {
                "None detected".to_string()
            } else {
                self.metadata.build_tools.join(", ")
            },
            self.metadata.total_files
        ));

        output.push_str("\nPROJECT STRUCTURE\n");
        output.push_str("-----------------\n\n");

        for section in &self.sections {
            output.push_str(&format!("📁 {} ({})\n", section.name, section.files.len()));
            output.push_str(&format!("   {}\n\n", section.description));

            for file in &section.files {
                output.push_str(&format!("   • {}\n", file.relative_path.display()));
            }
            output.push_str("\n");
        }

        if include_content {
            output.push_str("\n");
            output.push_str("================================================================================\n");
            output.push_str("                              SOURCE CODE\n");
            output.push_str("================================================================================\n\n");

            for section in &self.sections {
                if section.files.is_empty() {
                    continue;
                }

                output.push_str(&format!(
                    "\n╔══════════════════════════════════════════════════════════════════════════════╗\n"
                ));
                output.push_str(&format!(
                    "║ {} - {} file(s)\n",
                    section.name.to_uppercase(),
                    section.files.len()
                ));
                output.push_str(&format!(
                    "╚══════════════════════════════════════════════════════════════════════════════╝\n\n"
                ));

                for file in &section.files {
                    output.push_str(&format!(
                        "┌─────────────────────────────────────────────────────────────────────────────┐\n"
                    ));
                    output.push_str(&format!("│ File: {}\n", file.relative_path.display()));
                    output.push_str(&format!(
                        "└─────────────────────────────────────────────────────────────────────────────┘\n\n"
                    ));

                    if let Ok(content) = std::fs::read_to_string(&file.path) {
                        let lines: Vec<&str> = content.lines().collect();
                        for (i, line) in lines.iter().enumerate() {
                            output.push_str(&format!("{:4} │ {}\n", i + 1, line));
                        }
                    } else {
                        output.push_str("[Unable to read file content]\n");
                    }
                    output.push_str("\n");
                }
            }
        }

        output.push_str("\n================================================================================\n");
        output.push_str("                       Generated by Omnivore Code Extractor\n");
        output.push_str("================================================================================\n");

        Ok(output)
    }

    pub fn to_json(&self) -> Result<String> {
        let mut json_output = serde_json::json!({
            "metadata": {
                "project_type": self.metadata.project_type,
                "description": self.metadata.description,
                "main_language": self.metadata.main_language,
                "frameworks": self.metadata.frameworks,
                "build_tools": self.metadata.build_tools,
                "total_files": self.metadata.total_files,
            },
            "sections": []
        });

        let sections = json_output["sections"].as_array_mut().unwrap();
        
        for section in &self.sections {
            let mut section_json = serde_json::json!({
                "name": section.name,
                "description": section.description,
                "files": []
            });

            let files = section_json["files"].as_array_mut().unwrap();
            for file in &section.files {
                let content = std::fs::read_to_string(&file.path).unwrap_or_default();
                files.push(serde_json::json!({
                    "path": file.relative_path.display().to_string(),
                    "content": content
                }));
            }

            sections.push(section_json);
        }

        Ok(serde_json::to_string_pretty(&json_output)?)
    }
}