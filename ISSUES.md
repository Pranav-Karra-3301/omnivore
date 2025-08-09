# Omnivore Issues and Suggestions

This document lists the issues found in the Omnivore project and suggestions for improvement.

## Core Tool

### 1. `parse` command is not intuitive

*   **Issue**: The `parse` command is not intuitive to use. The `README.md` and the documentation mention a `parser-rules.yaml` file but do not explain its format. I had to look at the source code to understand how to create this file.
*   **Suggestion**: Add a clear example of the `parser-rules.yaml` file to the documentation, including an explanation of the available fields (`name`, `selector`, `attribute`, `multiple`, `required`, `transform`).

### 2. `graph` command lacks documentation

*   **Issue**: The `graph` command is not well-documented. The documentation doesn't explain the format of the input file (`crawl-results.json`) or the output file (`graph.db`).
*   **Suggestion**: Add a description of the expected input format for the `graph` command. Also, document the schema of the `graph.db` file.

### 3. `stats` command is not fully implemented

*   **Issue**: The `stats` command is not fully implemented. It only shows a message that no active sessions were found. It's not clear how to start a crawl with a session.
*   **Suggestion**: Implement the session functionality for the `crawl` command and the `stats` command. The `crawl` command should be able to start a crawl with a session ID, and the `stats` command should be able to retrieve the statistics for that session.

## Documentation

### 1. `quickstart.md` is not completely accurate

*   **Issue**: The `quickstart.md` file is not completely accurate. The `--respect-robots` flag in the `crawl` command is incorrect; it should be `--respect-robots-txt`.
*   **Suggestion**: Correct the flag in the documentation.

### 2. Lack of examples

*   **Issue**: The documentation lacks examples for the `parse` and `graph` commands.
*   **Suggestion**: Add complete examples for the `parse` and `graph` commands, including the content of the input files and the expected output.
