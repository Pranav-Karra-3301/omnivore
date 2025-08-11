use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseInfo {
    pub project_type: ProjectType,
    pub languages: Vec<Language>,
    pub frameworks: Vec<Framework>,
    pub build_tools: Vec<BuildTool>,
    pub main_language: Option<Language>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectType {
    WebApplication,
    Library,
    CLI,
    MobileApp,
    API,
    Documentation,
    Monorepo,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Java,
    CSharp,
    CPlusPlus,
    C,
    Ruby,
    PHP,
    Swift,
    Kotlin,
    Scala,
    Elixir,
    Haskell,
    Shell,
    HTML,
    CSS,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Framework {
    NextJS,
    React,
    Vue,
    Angular,
    Svelte,
    Django,
    Flask,
    FastAPI,
    Rails,
    Laravel,
    Spring,
    Express,
    NestJS,
    Actix,
    Rocket,
    Gin,
    Echo,
    DotNet,
    Flutter,
    ReactNative,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuildTool {
    Npm,
    Yarn,
    Pnpm,
    Cargo,
    Maven,
    Gradle,
    Pip,
    Poetry,
    Composer,
    Bundler,
    Go,
    DotNet,
    Make,
    CMake,
    Other(String),
}

pub struct CodebaseDetector {
    root_path: PathBuf,
}

impl CodebaseDetector {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    pub fn detect(&self) -> Result<CodebaseInfo> {
        let mut info = CodebaseInfo {
            project_type: ProjectType::Unknown,
            languages: Vec::new(),
            frameworks: Vec::new(),
            build_tools: Vec::new(),
            main_language: None,
            description: String::new(),
        };

        self.detect_by_config_files(&mut info)?;
        self.detect_by_file_extensions(&mut info)?;
        self.determine_project_type(&mut info);
        self.determine_main_language(&mut info);
        self.generate_description(&mut info);

        Ok(info)
    }

    fn detect_by_config_files(&self, info: &mut CodebaseInfo) -> Result<()> {
        let config_checks: Vec<(&str, fn(&Path, &mut CodebaseInfo) -> Result<()>)> = vec![
            ("package.json", Self::check_nodejs_project),
            ("Cargo.toml", Self::check_rust_project),
            ("go.mod", Self::check_go_project),
            ("requirements.txt", Self::check_python_requirements),
            ("pyproject.toml", Self::check_python_pyproject),
            ("Gemfile", Self::check_ruby_project),
            ("composer.json", Self::check_php_project),
            ("pom.xml", Self::check_maven_project),
            ("build.gradle", Self::check_gradle_project),
            (".csproj", Self::check_dotnet_project),
            ("CMakeLists.txt", Self::check_cmake_project),
            ("Makefile", Self::check_makefile_project),
        ];

        for (file, checker) in config_checks {
            let path = self.root_path.join(file);
            if path.exists() {
                checker(&path, info)?;
            } else if file.contains('.') {
                for entry in fs::read_dir(&self.root_path)? {
                    let entry = entry?;
                    let filename = entry.file_name();
                    let filename_str = filename.to_string_lossy();
                    if filename_str.ends_with(file) {
                        checker(&entry.path(), info)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn check_nodejs_project(path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let json: serde_json::Value = serde_json::from_str(&content)?;

        if !info.languages.contains(&Language::JavaScript) {
            info.languages.push(Language::JavaScript);
        }

        if json.get("dependencies").is_some() || json.get("devDependencies").is_some() {
            if !info.build_tools.contains(&BuildTool::Npm) {
                info.build_tools.push(BuildTool::Npm);
            }
        }

        let deps = json.get("dependencies").and_then(|d| d.as_object());
        let dev_deps = json.get("devDependencies").and_then(|d| d.as_object());

        let check_dep = |name: &str| -> bool {
            deps.map_or(false, |d| d.contains_key(name))
                || dev_deps.map_or(false, |d| d.contains_key(name))
        };

        if check_dep("next") {
            info.frameworks.push(Framework::NextJS);
            info.languages.push(Language::TypeScript);
        } else if check_dep("react") {
            info.frameworks.push(Framework::React);
        } else if check_dep("vue") {
            info.frameworks.push(Framework::Vue);
        } else if check_dep("@angular/core") {
            info.frameworks.push(Framework::Angular);
            info.languages.push(Language::TypeScript);
        } else if check_dep("svelte") {
            info.frameworks.push(Framework::Svelte);
        } else if check_dep("express") {
            info.frameworks.push(Framework::Express);
        } else if check_dep("@nestjs/core") {
            info.frameworks.push(Framework::NestJS);
            info.languages.push(Language::TypeScript);
        } else if check_dep("react-native") {
            info.frameworks.push(Framework::ReactNative);
        }

        if path.parent().and_then(|p| p.join("yarn.lock").exists().then_some(())).is_some() {
            info.build_tools.push(BuildTool::Yarn);
        }
        if path.parent().and_then(|p| p.join("pnpm-lock.yaml").exists().then_some(())).is_some() {
            info.build_tools.push(BuildTool::Pnpm);
        }

        Ok(())
    }

    fn check_rust_project(path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let toml: toml::Value = toml::from_str(&content)?;

        info.languages.push(Language::Rust);
        info.build_tools.push(BuildTool::Cargo);

        if let Some(deps) = toml.get("dependencies").and_then(|d| d.as_table()) {
            if deps.contains_key("actix-web") {
                info.frameworks.push(Framework::Actix);
            } else if deps.contains_key("rocket") {
                info.frameworks.push(Framework::Rocket);
            }
        }

        Ok(())
    }

    fn check_go_project(_path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        info.languages.push(Language::Go);
        info.build_tools.push(BuildTool::Go);
        Ok(())
    }

    fn check_python_requirements(_path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        info.languages.push(Language::Python);
        info.build_tools.push(BuildTool::Pip);
        Ok(())
    }

    fn check_python_pyproject(path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let toml: toml::Value = toml::from_str(&content)?;

        info.languages.push(Language::Python);

        if toml.get("tool").and_then(|t| t.get("poetry")).is_some() {
            info.build_tools.push(BuildTool::Poetry);
        } else {
            info.build_tools.push(BuildTool::Pip);
        }

        if let Some(deps) = toml
            .get("tool")
            .and_then(|t| t.get("poetry"))
            .and_then(|p| p.get("dependencies"))
            .and_then(|d| d.as_table())
        {
            if deps.contains_key("django") {
                info.frameworks.push(Framework::Django);
            } else if deps.contains_key("flask") {
                info.frameworks.push(Framework::Flask);
            } else if deps.contains_key("fastapi") {
                info.frameworks.push(Framework::FastAPI);
            }
        }

        Ok(())
    }

    fn check_ruby_project(path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        info.languages.push(Language::Ruby);
        info.build_tools.push(BuildTool::Bundler);
        
        // Check for Rails by looking for config/application.rb relative to Gemfile
        if let Some(parent) = path.parent() {
            if parent.join("config").join("application.rb").exists() {
                info.frameworks.push(Framework::Rails);
            }
        }
        
        Ok(())
    }

    fn check_php_project(path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let json: serde_json::Value = serde_json::from_str(&content)?;

        info.languages.push(Language::PHP);
        info.build_tools.push(BuildTool::Composer);

        if let Some(require) = json.get("require").and_then(|r| r.as_object()) {
            if require.contains_key("laravel/framework") {
                info.frameworks.push(Framework::Laravel);
            }
        }

        Ok(())
    }

    fn check_maven_project(_path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        info.languages.push(Language::Java);
        info.build_tools.push(BuildTool::Maven);
        Ok(())
    }

    fn check_gradle_project(_path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        info.languages.push(Language::Java);
        info.build_tools.push(BuildTool::Gradle);
        Ok(())
    }

    fn check_dotnet_project(_path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        info.languages.push(Language::CSharp);
        info.build_tools.push(BuildTool::DotNet);
        info.frameworks.push(Framework::DotNet);
        Ok(())
    }

    fn check_cmake_project(_path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        info.languages.push(Language::CPlusPlus);
        info.build_tools.push(BuildTool::CMake);
        Ok(())
    }

    fn check_makefile_project(_path: &Path, info: &mut CodebaseInfo) -> Result<()> {
        if !info.build_tools.contains(&BuildTool::Make) {
            info.build_tools.push(BuildTool::Make);
        }
        Ok(())
    }

    fn detect_by_file_extensions(&self, info: &mut CodebaseInfo) -> Result<()> {
        let mut language_counts: HashMap<Language, usize> = HashMap::new();

        for entry in walkdir::WalkDir::new(&self.root_path)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                let lang = match ext {
                    "rs" => Some(Language::Rust),
                    "js" | "mjs" | "cjs" => Some(Language::JavaScript),
                    "ts" | "tsx" => Some(Language::TypeScript),
                    "py" => Some(Language::Python),
                    "go" => Some(Language::Go),
                    "java" => Some(Language::Java),
                    "cs" => Some(Language::CSharp),
                    "cpp" | "cc" | "cxx" => Some(Language::CPlusPlus),
                    "c" | "h" => Some(Language::C),
                    "rb" => Some(Language::Ruby),
                    "php" => Some(Language::PHP),
                    "swift" => Some(Language::Swift),
                    "kt" | "kts" => Some(Language::Kotlin),
                    "scala" => Some(Language::Scala),
                    "ex" | "exs" => Some(Language::Elixir),
                    "hs" => Some(Language::Haskell),
                    "sh" | "bash" | "zsh" => Some(Language::Shell),
                    "html" | "htm" => Some(Language::HTML),
                    "css" | "scss" | "sass" | "less" => Some(Language::CSS),
                    _ => None,
                };

                if let Some(lang) = lang {
                    *language_counts.entry(lang.clone()).or_insert(0) += 1;
                    if !info.languages.contains(&lang) {
                        info.languages.push(lang);
                    }
                }
            }
        }

        if let Some((main_lang, _)) = language_counts.iter().max_by_key(|(_, count)| *count) {
            info.main_language = Some(main_lang.clone());
        }

        Ok(())
    }

    fn determine_project_type(&self, info: &mut CodebaseInfo) {
        if !info.frameworks.is_empty() {
            if matches!(
                info.frameworks.first(),
                Some(Framework::NextJS | Framework::React | Framework::Vue | Framework::Angular | Framework::Svelte)
            ) {
                info.project_type = ProjectType::WebApplication;
            } else if matches!(
                info.frameworks.first(),
                Some(Framework::Express | Framework::FastAPI | Framework::Django | Framework::Flask | Framework::Actix | Framework::Rocket)
            ) {
                info.project_type = ProjectType::API;
            } else if matches!(
                info.frameworks.first(),
                Some(Framework::ReactNative | Framework::Flutter)
            ) {
                info.project_type = ProjectType::MobileApp;
            }
        } else if self.root_path.join("src").join("main.rs").exists()
            || self.root_path.join("cmd").exists()
            || self.root_path.join("cli.py").exists()
        {
            info.project_type = ProjectType::CLI;
        } else if self.root_path.join("lib.rs").exists()
            || self.root_path.join("index.js").exists()
            || self.root_path.join("__init__.py").exists()
        {
            info.project_type = ProjectType::Library;
        } else if self.root_path.join("docs").exists()
            || self.root_path.join("README.md").exists()
        {
            info.project_type = ProjectType::Documentation;
        } else if self.root_path.join("packages").exists()
            || self.root_path.join("apps").exists()
        {
            info.project_type = ProjectType::Monorepo;
        }
    }

    fn determine_main_language(&self, info: &mut CodebaseInfo) {
        if info.main_language.is_none() && !info.languages.is_empty() {
            info.main_language = Some(info.languages[0].clone());
        }
    }

    fn generate_description(&self, info: &mut CodebaseInfo) {
        let project_type = match &info.project_type {
            ProjectType::WebApplication => "web application",
            ProjectType::Library => "library",
            ProjectType::CLI => "command-line tool",
            ProjectType::MobileApp => "mobile application",
            ProjectType::API => "API service",
            ProjectType::Documentation => "documentation project",
            ProjectType::Monorepo => "monorepo",
            ProjectType::Unknown => "project",
        };

        let main_lang = info
            .main_language
            .as_ref()
            .map(|l| format!("{:?}", l))
            .unwrap_or_else(|| "Unknown".to_string());

        let framework_str = if !info.frameworks.is_empty() {
            format!(" using {:?}", info.frameworks[0])
        } else {
            String::new()
        };

        info.description = format!(
            "{} {} written in {}{}",
            if matches!(info.project_type, ProjectType::API | ProjectType::Unknown) {
                "A"
            } else {
                "An"
            },
            project_type,
            main_lang,
            framework_str
        );
    }
}

pub fn get_default_include_patterns(info: &CodebaseInfo) -> Vec<String> {
    let mut patterns = Vec::new();

    for lang in &info.languages {
        match lang {
            Language::Rust => {
                patterns.extend(vec![
                    "**/*.rs".to_string(),
                    "**/Cargo.toml".to_string(),
                    "**/Cargo.lock".to_string(),
                ]);
            }
            Language::JavaScript | Language::TypeScript => {
                patterns.extend(vec![
                    "**/*.js".to_string(),
                    "**/*.jsx".to_string(),
                    "**/*.ts".to_string(),
                    "**/*.tsx".to_string(),
                    "**/*.mjs".to_string(),
                    "**/*.cjs".to_string(),
                    "**/package.json".to_string(),
                    "**/tsconfig.json".to_string(),
                    "**/.eslintrc.json".to_string(),
                    "**/.babelrc".to_string(),
                    "**/webpack.config.js".to_string(),
                ]);
            }
            Language::Python => {
                patterns.extend(vec![
                    "**/*.py".to_string(),
                    "**/requirements.txt".to_string(),
                    "**/pyproject.toml".to_string(),
                    "**/setup.py".to_string(),
                ]);
            }
            Language::Go => {
                patterns.extend(vec![
                    "**/*.go".to_string(),
                    "**/go.mod".to_string(),
                    "**/go.sum".to_string(),
                ]);
            }
            Language::Java => {
                patterns.extend(vec![
                    "**/*.java".to_string(),
                    "**/pom.xml".to_string(),
                    "**/build.gradle".to_string(),
                ]);
            }
            Language::CSharp => {
                patterns.extend(vec![
                    "**/*.cs".to_string(),
                    "**/*.csproj".to_string(),
                    "**/*.sln".to_string(),
                ]);
            }
            Language::Ruby => {
                patterns.extend(vec![
                    "**/*.rb".to_string(),
                    "**/Gemfile".to_string(),
                    "**/Gemfile.lock".to_string(),
                ]);
            }
            Language::PHP => {
                patterns.extend(vec![
                    "**/*.php".to_string(),
                    "**/composer.json".to_string(),
                    "**/composer.lock".to_string(),
                ]);
            }
            _ => {}
        }
    }

    patterns.extend(vec![
        "**/README.md".to_string(),
        "**/.env.example".to_string(),
        "**/Dockerfile".to_string(),
        "**/docker-compose.yml".to_string(),
        "**/.gitignore".to_string(),
    ]);

    patterns.sort();
    patterns.dedup();
    patterns
}

pub fn get_smart_exclude_patterns(info: &CodebaseInfo) -> Vec<String> {
    let mut patterns = vec![
        // Package and dependency directories (NEVER needed for code analysis)
        "**/node_modules/**".to_string(),
        "**/bower_components/**".to_string(),
        "**/jspm_packages/**".to_string(),
        "**/vendor/**".to_string(),
        "**/.pnp/**".to_string(),
        "**/.yarn/**".to_string(),
        
        // Build outputs
        "**/target/**".to_string(),
        "**/dist/**".to_string(),
        "**/build/**".to_string(),
        "**/out/**".to_string(),
        "**/output/**".to_string(),
        "**/.next/**".to_string(),
        "**/.nuxt/**".to_string(),
        "**/.output/**".to_string(),
        "**/.svelte-kit/**".to_string(),
        "**/public/build/**".to_string(),
        
        // Version control
        "**/.git/**".to_string(),
        "**/.svn/**".to_string(),
        "**/.hg/**".to_string(),
        
        // Python specific
        "**/__pycache__/**".to_string(),
        "**/venv/**".to_string(),
        "**/.venv/**".to_string(),
        "**/env/**".to_string(),
        "**/.env/**".to_string(),
        "**/site-packages/**".to_string(),
        "**/.tox/**".to_string(),
        "**/*.egg-info/**".to_string(),
        "**/pip-wheel-metadata/**".to_string(),
        
        // Test coverage and reports
        "**/coverage/**".to_string(),
        "**/.coverage/**".to_string(),
        "**/htmlcov/**".to_string(),
        "**/.nyc_output/**".to_string(),
        "**/test-results/**".to_string(),
        "**/.pytest_cache/**".to_string(),
        
        // IDE and editor files
        "**/.idea/**".to_string(),
        "**/.vscode/**".to_string(),
        "**/.vs/**".to_string(),
        "**/*.swp".to_string(),
        "**/*.swo".to_string(),
        "**/*~".to_string(),
        "**/.DS_Store".to_string(),
        "**/Thumbs.db".to_string(),
        
        // Logs and temporary files
        "**/logs/**".to_string(),
        "**/*.log".to_string(),
        "**/tmp/**".to_string(),
        "**/temp/**".to_string(),
        "**/.tmp/**".to_string(),
        "**/.temp/**".to_string(),
        "**/.cache/**".to_string(),
        
        // Minified and compiled files
        "**/*.min.js".to_string(),
        "**/*.min.css".to_string(),
        "**/*.map".to_string(),
        "**/bundle.js".to_string(),
        "**/chunk.*.js".to_string(),
        "**/*.bundle.js".to_string(),
        
        // Lock files (usually not needed for code understanding)
        "**/package-lock.json".to_string(),
        "**/yarn.lock".to_string(),
        "**/pnpm-lock.yaml".to_string(),
        "**/composer.lock".to_string(),
        "**/Gemfile.lock".to_string(),
        "**/poetry.lock".to_string(),
        "**/Pipfile.lock".to_string(),
        
        // Documentation build outputs
        "**/docs/_build/**".to_string(),
        "**/site/**".to_string(),
        "**/_site/**".to_string(),
        
        // Database files
        "**/*.sqlite".to_string(),
        "**/*.sqlite3".to_string(),
        "**/*.db".to_string(),
        "**/*.mdb".to_string(),
        "**/*.accdb".to_string(),
        
        // Large binary and archive files
        "**/*.zip".to_string(),
        "**/*.tar".to_string(),
        "**/*.tar.gz".to_string(),
        "**/*.tgz".to_string(),
        "**/*.rar".to_string(),
        "**/*.7z".to_string(),
        "**/*.gz".to_string(),
        "**/*.bz2".to_string(),
        "**/*.xz".to_string(),
        "**/*.jar".to_string(),
        "**/*.war".to_string(),
        "**/*.ear".to_string(),
        "**/*.deb".to_string(),
        "**/*.rpm".to_string(),
        "**/*.dmg".to_string(),
        "**/*.pkg".to_string(),
        "**/*.iso".to_string(),
        
        // Machine learning model files (often very large)
        "**/*.pt".to_string(),
        "**/*.pth".to_string(),
        "**/*.pkl".to_string(),
        "**/*.pickle".to_string(),
        "**/*.h5".to_string(),
        "**/*.hdf5".to_string(),
        "**/*.pb".to_string(),
        "**/*.onnx".to_string(),
        "**/*.tflite".to_string(),
        "**/*.caffemodel".to_string(),
        "**/*.weights".to_string(),
        "**/*.model".to_string(),
        "**/*.ckpt".to_string(),
        "**/*.safetensors".to_string(),
        
        // Data files (often large and not code)
        "**/*.csv".to_string(),
        "**/*.tsv".to_string(),
        "**/*.parquet".to_string(),
        "**/*.feather".to_string(),
        "**/*.msgpack".to_string(),
        "**/*.npy".to_string(),
        "**/*.npz".to_string(),
        
        // Office documents
        "**/*.doc".to_string(),
        "**/*.docx".to_string(),
        "**/*.xls".to_string(),
        "**/*.xlsx".to_string(),
        "**/*.ppt".to_string(),
        "**/*.pptx".to_string(),
        "**/*.pdf".to_string(),
        "**/*.odt".to_string(),
        "**/*.ods".to_string(),
        "**/*.odp".to_string(),
        
        // Media files (usually not needed for code analysis)
        "**/*.jpg".to_string(),
        "**/*.jpeg".to_string(),
        "**/*.png".to_string(),
        "**/*.gif".to_string(),
        "**/*.bmp".to_string(),
        "**/*.tiff".to_string(),
        "**/*.tif".to_string(),
        "**/*.svg".to_string(),
        "**/*.webp".to_string(),
        "**/*.ico".to_string(),
        "**/*.mp4".to_string(),
        "**/*.avi".to_string(),
        "**/*.mov".to_string(),
        "**/*.wmv".to_string(),
        "**/*.flv".to_string(),
        "**/*.webm".to_string(),
        "**/*.mkv".to_string(),
        "**/*.mp3".to_string(),
        "**/*.wav".to_string(),
        "**/*.flac".to_string(),
        "**/*.aac".to_string(),
        "**/*.ogg".to_string(),
        "**/*.wma".to_string(),
        
        // Font files
        "**/*.woff".to_string(),
        "**/*.woff2".to_string(),
        "**/*.ttf".to_string(),
        "**/*.otf".to_string(),
        "**/*.eot".to_string(),
        
        // Compiled/binary files
        "**/*.exe".to_string(),
        "**/*.dll".to_string(),
        "**/*.so".to_string(),
        "**/*.dylib".to_string(),
        "**/*.a".to_string(),
        "**/*.lib".to_string(),
        "**/*.o".to_string(),
        "**/*.obj".to_string(),
        "**/*.pyc".to_string(),
        "**/*.pyo".to_string(),
        "**/*.class".to_string(),
        "**/*.elc".to_string(),
        "**/*.beam".to_string(),
    ];

    // Framework-specific exclusions
    for framework in &info.frameworks {
        match framework {
            Framework::NextJS => {
                patterns.extend(vec![
                    "**/.next/**".to_string(),
                    "**/next-env.d.ts".to_string(),
                ]);
            }
            Framework::Django => {
                patterns.extend(vec![
                    "**/migrations/**".to_string(),
                    "**/staticfiles/**".to_string(),
                    "**/media/**".to_string(),
                ]);
            }
            Framework::Rails => {
                patterns.extend(vec![
                    "**/log/**".to_string(),
                    "**/tmp/**".to_string(),
                    "**/storage/**".to_string(),
                    "**/public/assets/**".to_string(),
                ]);
            }
            _ => {}
        }
    }

    // Language-specific exclusions
    for language in &info.languages {
        match language {
            Language::Java => {
                patterns.extend(vec![
                    "**/target/**".to_string(),
                    "**/*.class".to_string(),
                    "**/bin/**".to_string(),
                ]);
            }
            Language::CSharp => {
                patterns.extend(vec![
                    "**/bin/**".to_string(),
                    "**/obj/**".to_string(),
                    "**/packages/**".to_string(),
                ]);
            }
            Language::Go => {
                patterns.extend(vec![
                    "**/vendor/**".to_string(),
                    "**/*.exe".to_string(),
                    "**/*.test".to_string(),
                ]);
            }
            Language::PHP => {
                patterns.extend(vec![
                    "**/vendor/**".to_string(),
                    "**/storage/**".to_string(),
                    "**/bootstrap/cache/**".to_string(),
                ]);
            }
            _ => {}
        }
    }

    patterns.sort();
    patterns.dedup();
    patterns
}