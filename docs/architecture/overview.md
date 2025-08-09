# Architecture Overview

Monorepo with three Rust crates:

- **omnivore-core**: core crawling, parsing, intelligence, graph, and storage modules
- **omnivore-cli**: command-line interface backed by core
- **omnivore-api**: Axum-based HTTP API and GraphQL schema

High-level flow:
1. URLs are scheduled and fetched (static via Reqwest, dynamic via browser automation)
2. Content parsed; metadata and entities extracted
3. Relationships built into a knowledge graph
4. Stats and artifacts exposed via CLI/API
