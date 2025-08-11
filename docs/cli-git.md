# Omnivore Git Command

The `omnivore git` command is a powerful code extraction and analysis tool that intelligently processes Git repositories, both local and remote. It automatically detects the codebase type, applies smart filtering, and generates organized reports of the source code.

## Overview

The git command clones or analyzes repositories, detects the technology stack, filters out unnecessary files (like dependencies and build artifacts), and produces a clean, organized output of the actual source code.

## Basic Usage

```bash
# Analyze a remote repository
omnivore git https://github.com/user/repo --output analysis.txt

# Analyze a local repository
omnivore git ./my-project --output project-code.txt

# Output to stdout
omnivore git https://github.com/user/repo --stdout

# Quick analysis with default output name
omnivore git https://github.com/user/repo
```

## Features

### Automatic Codebase Detection

The command automatically detects:
- **Project Type**: Web Application, CLI, Library, API, Mobile App, Monorepo, etc.
- **Programming Languages**: Rust, JavaScript, TypeScript, Python, Go, Java, C#, Ruby, PHP, etc.
- **Frameworks**: Next.js, React, Vue, Django, Flask, Rails, Laravel, Spring, etc.
- **Build Tools**: npm, yarn, cargo, pip, maven, gradle, etc.

### Smart Filtering

By default, the command excludes:
- Package directories (`node_modules`, `vendor`, `site-packages`)
- Build outputs (`dist`, `build`, `target`, `bin`)
- Version control directories (`.git`, `.svn`)
- IDE files (`.vscode`, `.idea`)
- Large binary files (images, videos, compiled binaries)
- Machine learning models (`*.pt`, `*.pkl`, `*.h5`)
- Database files (`*.db`, `*.sqlite`)
- Archive files (`*.zip`, `*.tar.gz`)
- Lock files (`package-lock.json`, `yarn.lock`)
- Files larger than 10MB (configurable)

### Organized Output

The output is structured with:
1. **Project metadata** - Type, language, frameworks, build tools
2. **File organization** - Grouped by category (Source Code, Configuration, Tests, Documentation, etc.)
3. **Full source code** - With syntax-aware formatting and line numbers

## Command Options

### Core Options

| Option | Description | Example |
|--------|-------------|---------|
| `--output PATH` | Output file path (supports .txt or .json) | `--output analysis.txt` |
| `--stdout` | Output to stdout instead of file | `--stdout` |
| `--json` | Output in JSON format | `--json` |
| `--verbose` | Show detailed progress information | `--verbose` |

### Filtering Options

| Option | Description | Example |
|--------|-------------|---------|
| `--only PATTERNS` | Include only files matching patterns (comma-separated) | `--only "*.rs,*.toml"` |
| `--include PATTERNS` | Include files matching patterns | `--include "*.js,*.ts"` |
| `--exclude PATTERNS` | Exclude files matching patterns | `--exclude "*.test.js"` |
| `--no-gitignore` | Ignore .gitignore rules | `--no-gitignore` |
| `--allow-binary` | Include binary files | `--allow-binary` |
| `--max-file-size SIZE` | Maximum file size in bytes (default: 10MB) | `--max-file-size 20971520` |

### Repository Options

| Option | Description | Example |
|--------|-------------|---------|
| `--keep` | Keep cloned repository after completion | `--keep` |
| `--depth N` | Clone depth for remote repos (default: 1) | `--depth 10` |

## Examples

### Basic Repository Analysis

```bash
# Analyze a GitHub repository and save to a text file
omnivore git https://github.com/rust-lang/rust-clippy --output clippy-analysis.txt

# Analyze with automatic naming (repo_name_timestamp.txt)
omnivore git https://github.com/facebook/react
```

### Filtering Specific Files

```bash
# Extract only Rust files
omnivore git https://github.com/rust-lang/cargo --only "*.rs,*.toml"

# Include test files that would normally be excluded
omnivore git ./my-project --include "**/tests/**" --output with-tests.txt

# Exclude specific directories
omnivore git ./my-project --exclude "**/experiments/**,**/legacy/**"
```

### Working with Large Repositories

```bash
# Shallow clone for faster processing
omnivore git https://github.com/torvalds/linux --depth 1 --only "*.c,*.h"

# Increase file size limit for larger source files
omnivore git ./ml-project --max-file-size 52428800  # 50MB

# Include normally excluded ML model files
omnivore git ./ml-project --include "*.pkl,*.pt" --allow-binary
```

### Different Output Formats

```bash
# JSON output for programmatic processing
omnivore git https://github.com/user/repo --json --output analysis.json

# Stream to stdout for piping
omnivore git https://github.com/user/repo --stdout | grep "function"

# Pretty text format (default)
omnivore git https://github.com/user/repo --output report.txt
```

### Local Repository Analysis

```bash
# Analyze current directory
omnivore git . --output this-project.txt

# Analyze specific local repository
omnivore git /path/to/project --output project-analysis.txt

# Keep repository structure intact (useful for local repos)
omnivore git ./my-project --output analysis.txt
```

## Output Format

### Text Output (.txt)

The text output includes:

```
================================================================================
                          OMNIVORE CODE ANALYSIS REPORT
================================================================================

PROJECT INFORMATION
-------------------
Type:         Web Application
Description:  A web application written in TypeScript using NextJS
Language:     TypeScript
Frameworks:   NextJS, React
Build Tools:  Npm, Yarn
Total Files:  156

================================================================================

PROJECT STRUCTURE
-----------------

📁 Documentation (4)
   Project documentation and guides
   • README.md
   • CONTRIBUTING.md
   • docs/API.md
   • docs/Setup.md

📁 Configuration (8)
   Project configuration and build files
   • package.json
   • tsconfig.json
   • next.config.js
   • .eslintrc.json
   ...

📁 Source Code (120)
   Main application source code
   • src/pages/index.tsx
   • src/components/Header.tsx
   ...

================================================================================
                              SOURCE CODE
================================================================================

[Full source code with line numbers for each file]
```

### JSON Output (.json)

The JSON output includes:

```json
{
  "metadata": {
    "project_type": "WebApplication",
    "description": "A web application written in TypeScript using NextJS",
    "main_language": "TypeScript",
    "frameworks": ["NextJS", "React"],
    "build_tools": ["Npm"],
    "total_files": 156
  },
  "sections": [
    {
      "name": "Source Code",
      "description": "Main application source code",
      "files": [
        {
          "path": "src/index.ts",
          "content": "// File content here..."
        }
      ]
    }
  ]
}
```

## Smart Defaults

The command uses intelligent defaults based on the detected project type:

### For JavaScript/TypeScript Projects
- Includes: `*.js`, `*.jsx`, `*.ts`, `*.tsx`, `package.json`, `tsconfig.json`
- Excludes: `node_modules/`, `dist/`, `build/`, `.next/`, `*.min.js`

### For Python Projects
- Includes: `*.py`, `requirements.txt`, `pyproject.toml`, `setup.py`
- Excludes: `__pycache__/`, `venv/`, `.venv/`, `*.pyc`, `site-packages/`

### For Rust Projects
- Includes: `*.rs`, `Cargo.toml`, `Cargo.lock`
- Excludes: `target/`, `*.rlib`

### For Go Projects
- Includes: `*.go`, `go.mod`, `go.sum`
- Excludes: `vendor/`, `*.exe`, `*.test`

## Advanced Usage

### Analyzing Machine Learning Projects

```bash
# Exclude large model files but keep training scripts
omnivore git ./ml-project \
  --exclude "*.pt,*.pkl,*.h5,*.safetensors" \
  --include "train.py,model.py" \
  --output ml-code.txt
```

### Analyzing Monorepos

```bash
# Focus on specific packages in a monorepo
omnivore git ./monorepo \
  --include "packages/core/**,packages/cli/**" \
  --exclude "packages/deprecated/**" \
  --output core-packages.txt
```

### CI/CD Integration

```bash
# Generate a code report in CI pipeline
omnivore git . \
  --json \
  --output code-report.json \
  --no-gitignore \
  --verbose
```

### Security Audit

```bash
# Extract only configuration and security-relevant files
omnivore git https://github.com/org/repo \
  --only "*.env.example,*.yml,*.yaml,Dockerfile,*.json" \
  --output security-audit.txt
```

## Performance Tips

1. **Use shallow clones** for large repositories: `--depth 1`
2. **Filter early** with `--only` to avoid processing unnecessary files
3. **Adjust file size limits** based on your needs
4. **Use default smart filtering** - it excludes most unnecessary files automatically

## Troubleshooting

### No files found
- Check your filter patterns
- Use `--verbose` to see what's being excluded
- Try `--no-gitignore` if files might be gitignored
- Verify the repository path/URL is correct

### Clone fails
- Check network connectivity
- Verify repository access (private repos need authentication)
- For SSH URLs, ensure SSH keys are configured

### Output too large
- Use more specific `--only` patterns
- Reduce `--max-file-size`
- Exclude test directories with `--exclude "**/tests/**"`

### Missing files in output
- Check if files exceed size limit (default 10MB)
- Verify files aren't binary (use `--allow-binary` if needed)
- Check if files match exclude patterns

## Integration with Other Omnivore Commands

The git command output can be used with other Omnivore features:

```bash
# Extract code and then analyze with AI
omnivore git https://github.com/user/repo --output code.txt
omnivore analyze code.txt --ai "Find security vulnerabilities"

# Combine with web crawling
omnivore crawl https://docs.example.com --output docs.json
omnivore git https://github.com/example/repo --output code.txt
```

## Best Practices

1. **Start with defaults** - The smart filtering usually works well out of the box
2. **Use `--only` for focused extraction** - More efficient than broad inclusion
3. **Keep output organized** - Use descriptive output filenames
4. **Version control your analyses** - Track code snapshots over time
5. **Respect rate limits** - Don't repeatedly clone large repositories

## Limitations

- Maximum file size: 10MB by default (configurable)
- Memory usage scales with repository size
- Private repositories require proper authentication setup
- Binary file analysis is limited
- Very large repositories may take time to process

## Security Considerations

- The command respects `.gitignore` by default
- Sensitive files should be excluded manually if needed
- Cloned repositories are deleted after processing (unless `--keep`)
- No data is sent to external services
- Authentication credentials are handled by git2 library securely